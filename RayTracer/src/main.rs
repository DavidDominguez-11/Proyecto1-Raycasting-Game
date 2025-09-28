// main.rs
#![allow(unused_imports)]
#![allow(dead_code)]

mod framebuffer;
mod maze;
mod player;
mod caster;
mod textures;
mod key;
mod text;
mod audio;

use raylib::prelude::*;
use std::thread;
use std::time::{Duration, Instant};
use player::{Player, process_events};
use framebuffer::Framebuffer;
use maze::{Maze,load_maze};
use caster::{cast_ray, Intersect};
use std::f32::consts::PI;
use textures::TextureManager;
use key::Key;
use text::Font;
use audio::AudioPlayer;

const TRANSPARENT_COLOR: Color = Color::new(0, 0, 0, 0);
const MAX_LIFE: f32 = 999.0; // 60 segundos de vida máxima

struct GameState {
    life: f32,
    game_start_time: Instant,
    has_key: bool,
    flashlight_on: bool,
}

impl GameState {
    fn new() -> Self {
        GameState {
            life: MAX_LIFE,
            game_start_time: Instant::now(),
            has_key: false,
            flashlight_on: false,
        }
    }

    fn update_life(&mut self) {
        let elapsed = self.game_start_time.elapsed().as_secs_f32();
        self.life = (MAX_LIFE - elapsed).max(0.0);
    }

    fn is_alive(&self) -> bool {
        self.life > 0.0
    }

    fn collect_key(&mut self) {
        self.has_key = true;
    }

    fn reset(&mut self) {
        self.life = MAX_LIFE;
        self.game_start_time = Instant::now();
        self.has_key = false;
        self.flashlight_on = false;
    }
}

#[derive(PartialEq)]
enum ScreenState {
    MainMenu,
    Playing,
    Win,
    Lose,
}

fn draw_sprite(
    framebuffer: &mut Framebuffer,
    player: &Player,
    key: &Key,
    texture_manager: &TextureManager
) {
    let sprite_a = (key.pos.y - player.pos.y).atan2(key.pos.x - player.pos.x);
    let mut angle_diff = sprite_a - player.a;
    while angle_diff > PI {
        angle_diff -= 2.0 * PI;
    }
    while angle_diff < -PI {
        angle_diff += 2.0 * PI;
    }

    if angle_diff.abs() > player.fov / 2.0 {
        return;
    }

    let sprite_d = ((player.pos.x - key.pos.x).powi(2) + (player.pos.y - key.pos.y).powi(2)).sqrt();

    // near plane           far plane
    if sprite_d < 50.0 || sprite_d > 1000.0 {
        return;
    }

    let screen_height = framebuffer.height as f32;
    let screen_width = framebuffer.width as f32;

    let sprite_size = (screen_height / sprite_d) * 70.0;
    let screen_x = ((angle_diff / player.fov) + 0.5) * screen_width;

    let start_x = (screen_x - sprite_size / 2.0).max(0.0) as usize;
    let start_y = (screen_height / 2.0 - sprite_size / 2.0).max(0.0) as usize;
    let sprite_size_usize = sprite_size as usize;
    let end_x = (start_x + sprite_size_usize).min(framebuffer.width as usize);
    let end_y = (start_y + sprite_size_usize).min(framebuffer.height as usize);

    for x in start_x..end_x {
        for y in start_y..end_y {
            let tx = ((x - start_x) * 128 / sprite_size_usize) as u32;
            let ty = ((y - start_y) * 128 / sprite_size_usize) as u32;

            let color = texture_manager.get_pixel_color(key.texture_key, tx, ty);
            
            if color != TRANSPARENT_COLOR {
                framebuffer.set_current_color(color);
                framebuffer.set_pixel(x as i32, y as i32);
            }
        }
    }
}

fn draw_cell(
    framebuffer: &mut Framebuffer,
    xo: usize,
    yo: usize,
    block_size: usize,
    cell: char,
) {
    if cell == ' ' {
        return;
    }

    // Asignar colores diferentes según el tipo de celda
    let color = match cell {
        'g' => Color::GREEN,    // Meta - verde
        'k' => Color::GOLD,     // Llave - dorado
        _ => Color::RED,        // Paredes - rojo
    };

    framebuffer.set_current_color(color);

    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            framebuffer.set_pixel(x as i32, y as i32);
        }
    }
}

pub fn render_maze(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
    player: &Player,
) {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = col_index * block_size;
            let yo = row_index * block_size;
            
            draw_cell(framebuffer, xo, yo, block_size, cell);
        }
    }
    //draw player
    framebuffer.set_current_color(Color::WHITE);
    let px = player.pos.x as i32;
    let py = player.pos.y as i32;
    framebuffer.set_pixel(px, py);

    let num_rays = 20;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let  a = (player.a - (player.fov / 2.0)) + (player.fov * current_ray);
        cast_ray(framebuffer, &maze, &player, a, block_size, true);
    }
}

pub fn render_3d(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
    player: &Player,
    texture_cache: &TextureManager,
) {
    let num_rays = framebuffer.width;
    let hh = framebuffer.height as f32/ 2.0;

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = (player.a - (player.fov / 2.0)) + (player.fov * current_ray);
        let angle_diff = a - player.a;
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);
        let d = intersect.distance;
        let c = intersect.impact;
        
        // Saltar si el rayo choca con 'g' (lo renderizaremos como sprite)
        if c == 'g' {
            continue;
        }
        
        let corrected_distance = d * angle_diff.cos() as f32;
        let stake_height = (hh / corrected_distance)*100.0;
        let half_stake_height = stake_height / 2.0;
        let stake_top = (hh - half_stake_height) as usize;
        let stake_bottom = (hh + half_stake_height) as usize;

        for y in stake_top..stake_bottom {
            let tx = intersect.tx;
            let ty = ((y as f32 - stake_top as f32) / (stake_bottom as f32 - stake_top as f32))*128.0;
            let color = texture_cache.get_pixel_color(c, tx as u32, ty as u32);

            framebuffer.set_current_color(color);
            framebuffer.set_pixel(i, y as i32);
        }
    }
}

fn render_key(
    framebuffer: &mut Framebuffer,
    player: &Player,
    texture_cache: &TextureManager,
) {
    let key = vec![
        Key::new(250.0, 250.0, 'k'),
    ];

    for key in key {
        draw_sprite(framebuffer, &player, &key, texture_cache);
    }
}

fn render_minimap(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
    player: &Player,
    minimap_size: usize,
    position: (i32, i32),
) {
    let minimap_block_size = minimap_size / maze[0].len();
    let (pos_x, pos_y) = position;
    
    // Fondo semitransparente del minimapa
    framebuffer.set_current_color(Color::new(0, 0, 0, 180));
    for x in pos_x..pos_x + minimap_size as i32 {
        for y in pos_y..pos_y + minimap_size as i32 {
            framebuffer.set_pixel(x, y);
        }
    }
    
    // Borde del minimapa
    framebuffer.set_current_color(Color::WHITE);
    for x in pos_x..pos_x + minimap_size as i32 {
        framebuffer.set_pixel(x, pos_y);
        framebuffer.set_pixel(x, pos_y + minimap_size as i32 - 1);
    }
    for y in pos_y..pos_y + minimap_size as i32 {
        framebuffer.set_pixel(pos_x, y);
        framebuffer.set_pixel(pos_x + minimap_size as i32 - 1, y);
    }
    
    // Dibujar el laberinto en el minimapa
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = pos_x + (col_index * minimap_block_size) as i32;
            let yo = pos_y + (row_index * minimap_block_size) as i32;
            
            match cell {
                ' ' => framebuffer.set_current_color(Color::DARKGRAY),
                'g' => framebuffer.set_current_color(Color::GREEN), // Meta
                'k' => framebuffer.set_current_color(Color::GOLD),  // Llave
                _ => framebuffer.set_current_color(Color::RED),     // Paredes
            }
            
            for x in xo..xo + minimap_block_size as i32 {
                for y in yo..yo + minimap_block_size as i32 {
                    framebuffer.set_pixel(x, y);
                }
            }
        }
    }
    
    // Dibujar al jugador en el minimapa (punto más grande)
    let player_minimap_x = pos_x + (player.pos.x as usize / block_size * minimap_block_size) as i32;
    let player_minimap_y = pos_y + (player.pos.y as usize / block_size * minimap_block_size) as i32;
    
    framebuffer.set_current_color(Color::BLUE);
    for dx in -1..=1 {
        for dy in -1..=1 {
            framebuffer.set_pixel(player_minimap_x + dx, player_minimap_y + dy);
        }
    }
    
    // Dibujar dirección del jugador
    let direction_x = player_minimap_x + (player.a.cos() * 8.0) as i32;
    let direction_y = player_minimap_y + (player.a.sin() * 8.0) as i32;
    framebuffer.set_current_color(Color::YELLOW);
    framebuffer.draw_line(player_minimap_x, player_minimap_y, direction_x, direction_y);
}

fn draw_life_bar(framebuffer: &mut Framebuffer, game_state: &GameState, font: &Font) {
    let bar_width = 200;
    let bar_height = 20;
    let x = 20;
    let y = 20;
    
    // Fondo de la barra de vida
    framebuffer.set_current_color(Color::DARKGRAY);
    for dx in 0..bar_width {
        for dy in 0..bar_height {
            framebuffer.set_pixel(x + dx, y + dy);
        }
    }
    
    // Vida actual
    let life_width = (bar_width as f32 * (game_state.life / MAX_LIFE)) as i32;
    if life_width > 0 {
        let life_color = if game_state.life > MAX_LIFE * 0.5 {
            Color::GREEN
        } else if game_state.life > MAX_LIFE * 0.25 {
            Color::YELLOW
        } else {
            Color::RED
        };
        
        framebuffer.set_current_color(life_color);
        for dx in 0..life_width {
            for dy in 0..bar_height {
                framebuffer.set_pixel(x + dx, y + dy);
            }
        }
    }
    
    // Borde de la barra
    framebuffer.set_current_color(Color::WHITE);
    for dx in 0..bar_width {
        framebuffer.set_pixel(x + dx, y);
        framebuffer.set_pixel(x + dx, y + bar_height - 1);
    }
    for dy in 0..bar_height {
        framebuffer.set_pixel(x, y + dy);
        framebuffer.set_pixel(x + bar_width - 1, y + dy);
    }
    
    // Texto de la vida - CORRECCIÓN: quitar &mut
    let life_text = format!("TIEMPO: {:.1}s", game_state.life);
    font.draw_text(framebuffer, &life_text, x, y + bar_height + 5, 1, Color::WHITE);
    
    // Indicador de llave - CORRECCIÓN: quitar &mut
    if game_state.has_key {
        font.draw_text(framebuffer, "LLAVE: ✓", x, y + bar_height + 20, 1, Color::GOLD);
    } else {
        font.draw_text(framebuffer, "LLAVE: ✗", x, y + bar_height + 20, 1, Color::GRAY);
    }
}

fn draw_main_menu(framebuffer: &mut Framebuffer, selected_level: i32) {
    let width = framebuffer.width;
    let height = framebuffer.height;
    
    // Fondo simple
    framebuffer.set_current_color(Color::BLACK);
    for y in 0..height {
        for x in 0..width {
            framebuffer.set_pixel(x, y);
        }
    }
    
    // Título - líneas de píxeles
    framebuffer.set_current_color(Color::RED);
    for x in width/2-80..width/2+80 {
        for y in 90..95 {
            framebuffer.set_pixel(x, y);
        }
    }
    for x in width/2-80..width/2+80 {
        for y in 105..110 {
            framebuffer.set_pixel(x, y);
        }
    }
    
    // Opciones de nivel
    for level in 1..=3 {
        let y_pos = 200 + (level - 1) * 50;
        
        if level == selected_level {
            framebuffer.set_current_color(Color::GREEN);
            // Dibujar selector ">"
            for x in width/2-40..width/2-35 {
                for y in y_pos..y_pos+5 {
                    framebuffer.set_pixel(x, y);
                }
            }
        } else {
            framebuffer.set_current_color(Color::WHITE);
        }
        
        // Dibujar texto "NIVEL X"
        for x in width/2-30..width/2+30 {
            for y in y_pos..y_pos+5 {
                framebuffer.set_pixel(x, y);
            }
        }
        
        // Dibujar número de nivel
        framebuffer.set_current_color(if level == selected_level { Color::GREEN } else { Color::WHITE });
        for x in width/2+35..width/2+40 {
            for y in y_pos..y_pos+5 {
                framebuffer.set_pixel(x, y);
            }
        }
    }
    
    // Instrucciones
    framebuffer.set_current_color(Color::GRAY);
    for x in width/2-60..width/2+60 {
        for y in 380..385 {
            framebuffer.set_pixel(x, y);
        }
    }
    for x in width/2-80..width/2+80 {
        for y in 430..435 {
            framebuffer.set_pixel(x, y);
        }
    }
}

// Función simple para dibujar texto con bloques de píxeles
fn draw_simple_text(framebuffer: &mut Framebuffer, text: &str, x: i32, y: i32, scale: i32) {
    for (i, _c) in text.chars().enumerate() {
        let char_x = x + (i as i32 * 8 * scale);
        
        // Dibujar un bloque rectangular por cada carácter (simplificado)
        for dx in 0..6*scale {
            for dy in 0..8*scale {
                if dx % scale == 0 && dy % scale == 0 {
                    framebuffer.set_pixel(char_x + dx, y + dy);
                }
            }
        }
    }
}

fn check_key_collision(player: &Player, keys: &[Key], game_state: &mut GameState, block_size: usize) -> bool {
    let player_grid_x = (player.pos.x / block_size as f32) as usize;
    let player_grid_y = (player.pos.y / block_size as f32) as usize;
    
    for key in keys {
        let key_grid_x = (key.pos.x / block_size as f32) as usize;
        let key_grid_y = (key.pos.y / block_size as f32) as usize;
        
        if player_grid_x == key_grid_x && player_grid_y == key_grid_y && !game_state.has_key {
            game_state.collect_key();
            return true;
        }
    }
    false
}

fn check_goal_collision(player: &Player, maze: &Maze, game_state: &GameState, block_size: usize) -> bool {
    let player_grid_x = (player.pos.x / block_size as f32) as usize;
    let player_grid_y = (player.pos.y / block_size as f32) as usize;
    
    if player_grid_y < maze.len() && player_grid_x < maze[player_grid_y].len() {
        let cell = maze[player_grid_y][player_grid_x];
        if cell == 'g' && game_state.has_key {
            return true;
        }
    }
    false
}

fn draw_win_screen(framebuffer: &mut Framebuffer, font: &Font, game_state: &GameState) {
    let width = framebuffer.width;
    let height = framebuffer.height;
    
    // Fondo verde de victoria
    framebuffer.set_current_color(Color::new(0, 100, 0, 255));
    for y in 0..height {
        for x in 0..width {
            framebuffer.set_pixel(x, y);
        }
    }
    
    // Mensaje de victoria - CORRECCIÓN: quitar &mut en todas las llamadas
    font.draw_text(framebuffer, "¡VICTORIA!", width / 2 - 60, height / 2 - 50, 3, Color::GOLD);
    font.draw_text(framebuffer, "Encontraste la llave y escapaste!", width / 2 - 120, height / 2, 1, Color::WHITE);
    
    let time_used = MAX_LIFE - game_state.life;
    font.draw_text(framebuffer, &format!("Tiempo: {:.1} segundos", time_used), width / 2 - 80, height / 2 + 30, 1, Color::YELLOW);
    
    font.draw_text(framebuffer, "Presiona ESPACIO para jugar otra vez", width / 2 - 140, height / 2 + 80, 1, Color::LIGHTGRAY);
    font.draw_text(framebuffer, "Presiona ESC para salir al menu", width / 2 - 120, height / 2 + 110, 1, Color::LIGHTGRAY);
}

fn draw_lose_screen(framebuffer: &mut Framebuffer, font: &Font) {
    let width = framebuffer.width;
    let height = framebuffer.height;
    
    // Fondo rojo de derrota
    framebuffer.set_current_color(Color::new(100, 0, 0, 255));
    for y in 0..height {
        for x in 0..width {
            framebuffer.set_pixel(x, y);
        }
    }
    
    // Mensaje de derrota - CORRECCIÓN: quitar &mut en todas las llamadas
    font.draw_text(framebuffer, "¡GAME OVER!", width / 2 - 60, height / 2 - 50, 3, Color::RED);
    font.draw_text(framebuffer, "Se te acabó el tiempo...", width / 2 - 80, height / 2, 1, Color::WHITE);
    font.draw_text(framebuffer, "No lograste encontrar la llave a tiempo", width / 2 - 140, height / 2 + 30, 1, Color::WHITE);
    
    font.draw_text(framebuffer, "Presiona ESPACIO para intentar otra vez", width / 2 - 160, height / 2 + 80, 1, Color::LIGHTGRAY);
    font.draw_text(framebuffer, "Presiona ESC para salir al menu", width / 2 - 120, height / 2 + 110, 1, Color::LIGHTGRAY);
}

fn get_keys() -> Vec<Key> {
    vec![
        Key::new(250.0, 250.0, 'k'),
    ]
}

fn draw_goal_sprite(
    framebuffer: &mut Framebuffer,
    player: &Player,
    maze: &Maze,
    texture_manager: &TextureManager,
    block_size: usize,
) {
    // Buscar la posición de la meta 'g' en el laberinto
    for (j, row) in maze.iter().enumerate() {
        for (i, &cell) in row.iter().enumerate() {
            if cell == 'g' {
                let goal_pos = Vector2::new(
                    (i * block_size + block_size / 2) as f32,
                    (j * block_size + block_size / 2) as f32
                );
                
                // Crear un "sprite" temporal para la meta
                let goal_sprite = Key {
                    pos: goal_pos,
                    texture_key: 'g', // Usar 'g' como identificador de textura
                };
                
                // Usar la misma función que para dibujar la llave
                draw_sprite(framebuffer, player, &goal_sprite, texture_manager);
            }
        }
    }
}

// --- NUEVAS FUNCIONES PARA EL EFECTO LINTERNA---
// Aplica un efecto de linterna más realista: gradiente radial
fn apply_flashlight_effect(framebuffer: &mut Framebuffer, window_width: i32, window_height: i32) {
    let center_x = (window_width / 2) as f32;
    let center_y = (window_height / 2) as f32;
    // Definir el radio máximo de la linterna (ajusta este valor)
    let max_radius = (window_width.min(window_height) as f32) * 0.6; // 60% del lado más corto
    // Definir el radio donde la luz comienza a atenuarse fuertemente (ángulo cónico)
    let inner_radius = max_radius * 0.3; // 30% del radio máximo

    for y in 0..window_height {
        for x in 0..window_width {
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            let distance = (dx.powi(2) + dy.powi(2)).sqrt();

            // Calcular intensidad basada en la distancia
            let intensity = if distance <= inner_radius {
                1.0 // Área central completamente iluminada
            } else if distance <= max_radius {
                // Gradiente suave entre inner_radius y max_radius
                // Usamos una curva cuadrática para una caída más natural
                let t = (distance - inner_radius) / (max_radius - inner_radius);
                (1.0 - t).powi(2).max(0.0) // Asegura que no sea negativo
            } else {
                0.0 // Fuera del radio, completamente oscuro
            };

            // Obtener el color actual del píxel
            if let Some(current_color) = framebuffer.get_pixel_color(x, y) {
                // Calcular el factor de oscuridad (opuesto a la intensidad)
                let darkness_factor = 1.0 - intensity;

                // Definir el color base de la oscuridad (negro)
                let dark_r = 0.0;
                let dark_g = 0.0;
                let dark_b = 0.0;

                // Mezclar el color actual con el negro basado en el factor de oscuridad
                // Usamos una interpolación lineal ponderada
                let r = (current_color.r as f32 * intensity + dark_r * darkness_factor) as u8;
                let g = (current_color.g as f32 * intensity + dark_g * darkness_factor) as u8;
                let b = (current_color.b as f32 * intensity + dark_b * darkness_factor) as u8;
                let a = current_color.a; // Mantener la transparencia original

                framebuffer.set_current_color(Color::new(r, g, b, a));
                framebuffer.set_pixel(x, y);
            }
        }
    }
}

// Opcional: Aplica una oscuridad general cuando la linterna está apagada
fn apply_general_darkness(framebuffer: &mut Framebuffer, window_width: i32, window_height: i32) {
    // Definir el color base de la oscuridad (negro con cierta transparencia)
    let darkness_color = Color::new(0, 0, 0, 200); // Negro semi-transparente

    for y in 0..window_height {
        for x in 0..window_width {
            // Obtener el color actual del píxel
            if let Some(current_color) = framebuffer.get_pixel_color(x, y) {
                // Mezclar el color actual con el color de oscuridad
                // Usamos una fórmula simple de mezcla (puedes probar otras)
                let r = ((current_color.r as u16 + darkness_color.r as u16) / 2) as u8;
                let g = ((current_color.g as u16 + darkness_color.g as u16) / 2) as u8;
                let b = ((current_color.b as u16 + darkness_color.b as u16) / 2) as u8;
                let a = current_color.a; // Mantener la transparencia original
                framebuffer.set_current_color(Color::new(r, g, b, a));
                framebuffer.set_pixel(x, y);
            }
        }
    }
}

fn main() {
    let window_width = 1300;
    let window_height = 900;
    let block_size = 100;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raycaster Game - Encuentra la Llave!")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(
        window_width as i32, 
        window_height as i32, 
        Color::new(50, 50, 100, 255)
    );

    framebuffer.set_background_color(Color::new(80, 80, 200, 255));

    let font = Font::new();
    let mut screen_state = ScreenState::MainMenu;
    let mut selected_level = 1;
    let mut maze: Maze = Vec::new();
    let mut player = Player{
        pos: Vector2::new(150.0,150.0), 
        a: PI/2.0,
        fov: PI / 3.0, 
    };

    let texture_cache = TextureManager::new(&mut window, &raylib_thread);
    let mut game_state = GameState::new();

    let minimap_size = 150;
    let minimap_position = (window_width as i32 - minimap_size as i32 - 20, 20);

    let audio_player = AudioPlayer::default();
    if let Err(e) = audio_player.play_background_music("assets/sounds/game_music.mp3") {
        eprintln!("Error al cargar la música de fondo: {}", e);
    }
    audio_player.set_volume(0.3);

    // Control de cadencia para el sonido de pasos
    let mut last_step_time = Instant::now();
    let step_cooldown = Duration::from_millis(250);

    let mut cursor_hidden = false;
    
    while !window.window_should_close() {
        match screen_state {
            ScreenState::MainMenu => {
                // Mostrar cursor en el menú
                if cursor_hidden {
                    window.enable_cursor();
                    cursor_hidden = false;
                }

                if window.is_key_pressed(KeyboardKey::KEY_UP) {
                    selected_level = if selected_level > 1 { selected_level - 1 } else { 3 };
                }
                if window.is_key_pressed(KeyboardKey::KEY_DOWN) {
                    selected_level = if selected_level < 3 { selected_level + 1 } else { 1 };
                }
                if window.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    let maze_file = match selected_level {
                        1 => "maze1.txt",
                        2 => "maze2.txt",
                        3 => "maze3.txt",
                        _ => "maze1.txt",
                    };
                    
                    maze = load_maze(maze_file);
                    game_state.reset();
                    
                    // Posicionar al jugador en un lugar seguro
                    for (j, row) in maze.iter().enumerate() {
                        for (i, &cell) in row.iter().enumerate() {
                            if cell == ' ' {
                                player.pos.x = (i * block_size + block_size / 2) as f32;
                                player.pos.y = (j * block_size + block_size / 2) as f32;
                                break;
                            }
                        }
                    }
                    
                    screen_state = ScreenState::Playing;
                }
                
                framebuffer.clear();
                
                // Dibujar menú principal
                framebuffer.set_current_color(Color::new(20, 20, 40, 255));
                for y in 0..framebuffer.height {
                    for x in 0..framebuffer.width {
                        framebuffer.set_pixel(x, y);
                    }
                }
                
                let screen_width = framebuffer.width;
                
                font.draw_text(&mut framebuffer, "RAYCASTING GAME", 
                    screen_width / 2 - 45, 100, 2, Color::YELLOW);
                font.draw_text(&mut framebuffer, "ENCUENTRA LA LLAVE A TIEMPO!", 
                    screen_width / 2 - 80, 150, 1, Color::GOLD);
                font.draw_text(&mut framebuffer, "SELECCIONA NIVEL", 
                    screen_width / 2 - 45, 220, 1, Color::WHITE);

                for level in 1..=3 {
                    let y_pos = 280 + (level - 1) * 50;
                    
                    if level == selected_level {
                        font.draw_text(&mut framebuffer, &format!("> NIVEL {} <", level), 
                            screen_width / 2 - 35, y_pos, 1, Color::GREEN);
                    } else {
                        font.draw_text(&mut framebuffer, &format!("  NIVEL {}  ", level), 
                            screen_width / 2 - 35, y_pos, 1, Color::LIGHTGRAY);
                    }
                }

                font.draw_text(&mut framebuffer, "Tienes 60 segundos para encontrar la llave", 
                    screen_width / 2 - 150, 450, 1, Color::LIGHTGRAY);
                font.draw_text(&mut framebuffer, "y llegar a la salida (casilla verde)", 
                    screen_width / 2 - 120, 470, 1, Color::LIGHTGRAY);
            }
            
            ScreenState::Playing => {
                framebuffer.clear();
                
                // Actualizar vida
                game_state.update_life();
                
                // Ocultar cursor cuando se está jugando
                if !cursor_hidden {
                    window.disable_cursor();
                    cursor_hidden = true;
                }

                // Verificar condiciones de fin de juego
                if !game_state.is_alive() {
                    screen_state = ScreenState::Lose;
                    continue;
                }
                
                let keys = get_keys();
                if check_key_collision(&player, &keys, &mut game_state, block_size) {
                    // La llave fue recolectada (se muestra en la UI)
                }
                
                if check_goal_collision(&player, &maze, &game_state, block_size) {
                    screen_state = ScreenState::Win;
                    continue;
                }

                // --- GESTIÓN DE LA LINTERNA ---
                if window.is_key_pressed(KeyboardKey::KEY_E) { // Usamos 'E' para encender/apagar
                    game_state.flashlight_on = !game_state.flashlight_on;
                }
                
                // Renderizado normal del juego
                let half_height = window_height as u32 / 2;
                
                // Cielo
                framebuffer.set_current_color(Color::new(20, 20, 20, 255));
                for y in 0..half_height {
                    for x in 0..window_width as u32 {
                        framebuffer.set_pixel(x as i32, y as i32);
                    }
                }
                
                // Piso
                framebuffer.set_current_color(Color::new(10, 10, 10, 255));
                for y in half_height..window_height as u32 {
                    for x in 0..window_width as u32 {
                        framebuffer.set_pixel(x as i32, y as i32);
                    }
                }

                let moved = process_events(&window, &mut player, &maze, block_size);
                if moved && (window.is_key_down(KeyboardKey::KEY_UP) || window.is_key_down(KeyboardKey::KEY_DOWN) || window.is_key_down(KeyboardKey::KEY_A) || window.is_key_down(KeyboardKey::KEY_S)) {
                    let now = Instant::now();
                    if now.duration_since(last_step_time) >= step_cooldown {
                        if let Err(e) = audio_player.play_sfx_once("assets/sounds/step.mp3") {
                            eprintln!("Error al reproducir sonido de paso: {}", e);
                        }
                        last_step_time = now;
                    }
                }

                if window.is_key_down(KeyboardKey::KEY_M) {
                    render_maze(&mut framebuffer, &maze, block_size, &player);
                } else {
                    render_3d(&mut framebuffer, &maze, block_size, &player, &texture_cache);
                    
                    // Renderizar llaves si no han sido recolectadas
                    if !game_state.has_key {
                        for key in &keys {
                            draw_sprite(&mut framebuffer, &player, key, &texture_cache);
                        }
                    }
                    
                    // Renderizar la meta como sprite (siempre visible)
                    draw_goal_sprite(&mut framebuffer, &player, &maze, &texture_cache, block_size);
                }

                // --- APLICAR EFECTO DE LINTERNA ---
                if game_state.flashlight_on {
                    apply_flashlight_effect(&mut framebuffer, window_width, window_height);
                } else {
                    // Opcional: Si la linterna está apagada, aplicar un efecto de oscuridad general
                    apply_general_darkness(&mut framebuffer, window_width, window_height);
                }
                // --- FIN EFECTO ---
                
                // Dibujar barra de vida
                draw_life_bar(&mut framebuffer, &game_state, &font);
                
                if !window.is_key_down(KeyboardKey::KEY_M) {
                    render_minimap(
                        &mut framebuffer, 
                        &maze, 
                        block_size, 
                        &player, 
                        minimap_size, 
                        minimap_position
                    );
                }
                
                if window.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                    screen_state = ScreenState::MainMenu;
                }
            }
            
            ScreenState::Win => {
                framebuffer.clear();
                draw_win_screen(&mut framebuffer, &font, &game_state);

                // Mostrar cursor en pantalla de victoria
                if cursor_hidden {
                    window.enable_cursor();
                    cursor_hidden = false;
                }
                
                if window.is_key_pressed(KeyboardKey::KEY_SPACE) {
                    screen_state = ScreenState::MainMenu;
                }
                if window.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                    screen_state = ScreenState::MainMenu;
                }
            }
            
            ScreenState::Lose => {
                framebuffer.clear();
                draw_lose_screen(&mut framebuffer, &font);

                // Mostrar cursor en pantalla de derrota
                if cursor_hidden {
                    window.enable_cursor();
                    cursor_hidden = false;
                }
                
                if window.is_key_pressed(KeyboardKey::KEY_SPACE) {
                    screen_state = ScreenState::MainMenu;
                }
                if window.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                    screen_state = ScreenState::MainMenu;
                }
            }
        }
        
        framebuffer.swap_buffers(&mut window, &raylib_thread);
        thread::sleep(Duration::from_millis(16));
    }
    // Asegurarse de mostrar el cursor al salir
    if cursor_hidden {
        window.enable_cursor();
    }
}