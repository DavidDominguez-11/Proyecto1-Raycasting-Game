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

use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use player::{Player, process_events};
use framebuffer::Framebuffer;
use maze::{Maze,load_maze};
use caster::{cast_ray, Intersect};
use std::f32::consts::PI;
use textures::TextureManager;
use key::Key;
use text::Font;

const TRANSPARENT_COLOR: Color = Color::new(0, 0, 0, 0);

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

    framebuffer.set_current_color(Color::RED);

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

    framebuffer.set_current_color(Color::RED);

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = (player.a - (player.fov / 2.0)) + (player.fov * current_ray);
        let angle_diff = a - player.a;
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);
        let d = intersect.distance;
        let c = intersect.impact;
        let corrected_distance = d * angle_diff.cos() as f32;
        let stake_height = (hh / corrected_distance)*100.0; //factor de escala rendering
        let half_stake_height = stake_height / 2.0;
        let stake_top = (hh - half_stake_height) as usize;
        let stake_bottom = (hh + half_stake_height) as usize;

        for y in stake_top..stake_bottom {
            let tx = intersect.tx;
            let ty = ((y as f32 - stake_top as f32) / (stake_bottom as f32 - stake_top as f32))*128.0; //el 128 tiene que ver con el tamaño de la textura (el ancho), cambiar tanto en main como en caster
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

#[derive(PartialEq)]
enum GameState {
    MainMenu,
    Playing,
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

fn main() {
    let window_width = 1300;
    let window_height = 900;
    let block_size = 100;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raycaster Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(
        window_width as i32, 
        window_height as i32, 
        Color::new(50, 50, 100, 255)
    );

    framebuffer.set_background_color(Color::new(80, 80, 200, 255));

    // Load the maze once before the loop
    let font = Font::new();
    let mut game_state = GameState::MainMenu;
    let mut selected_level = 1;
    let mut maze: Maze = Vec::new();
    let mut player = Player{
        pos: Vector2::new(150.0,150.0), 
        a: PI/2.0,
        fov: PI / 3.0, 
    };

    let texture_cache = TextureManager::new(&mut window, &raylib_thread);

    let minimap_size = 150; // Tamaño del minimapa en píxeles
    let minimap_position = (window_width as i32 - minimap_size as i32 - 20, 20); // Esquina superior derecha
    
    while !window.window_should_close() {
        match game_state {
            GameState::MainMenu => {
                // Procesar entrada en el menú
                if window.is_key_pressed(KeyboardKey::KEY_UP) {
                    selected_level = if selected_level > 1 { selected_level - 1 } else { 3 };
                }
                if window.is_key_pressed(KeyboardKey::KEY_DOWN) {
                    selected_level = if selected_level < 3 { selected_level + 1 } else { 1 };
                }
                if window.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    // Cargar el nivel seleccionado
                    let maze_file = match selected_level {
                        1 => "maze1.txt",
                        2 => "maze2.txt",
                        3 => "maze3.txt",
                        _ => "maze1.txt",
                    };
                    
                    maze = load_maze(maze_file);
                    game_state = GameState::Playing;
                    
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
                }
                
                // Dibujar menú principal
                framebuffer.clear();
                
                // Fondo
                framebuffer.set_current_color(Color::new(20, 20, 40, 255));
                for y in 0..framebuffer.height {
                    for x in 0..framebuffer.width {
                        framebuffer.set_pixel(x, y);
                    }
                }
                
                // Guardar el ancho en una variable local para evitar problemas de préstamo
                let screen_width = framebuffer.width;
                
                // Título
                font.draw_text(&mut framebuffer, "RAYCASTING GAME", 
                    screen_width / 2 - 45, 100, 2, Color::YELLOW);
                
                // Subtítulo
                font.draw_text(&mut framebuffer, "SELECCIONA NIVEL", 
                    screen_width / 2 - 45, 180, 1, Color::WHITE);
                
                // Opciones de nivel
                for level in 1..=3 {
                    let y_pos = 250 + (level - 1) * 50;
                    
                    if level == selected_level {
                        font.draw_text(&mut framebuffer, &format!("> NIVEL {} <", level), 
                            screen_width / 2 - 35, y_pos, 1, Color::GREEN);
                    } else {
                        font.draw_text(&mut framebuffer, &format!("  NIVEL {}  ", level), 
                            screen_width / 2 - 35, y_pos, 1, Color::LIGHTGRAY);
                    }
                }
            }
            
            GameState::Playing => {
                // Código de juego existente (sin cambios)
                framebuffer.clear();
                
                let half_height = window_height as u32 / 2;
                //cielo
                framebuffer.set_current_color(Color::new(135, 206, 235, 255));
                for y in 0..half_height {
                    for x in 0..window_width as u32 {
                        framebuffer.set_pixel(x as i32, y as i32);
                    }
                }
                //piso
                framebuffer.set_current_color(Color::new(168, 168, 168, 168));
                for y in half_height..window_height as u32 {
                    for x in 0..window_width as u32 {
                        framebuffer.set_pixel(x as i32, y as i32);
                    }
                }

                process_events(&window, &mut player, &maze, block_size);

                if window.is_key_down(KeyboardKey::KEY_M) {
                    render_maze(&mut framebuffer, &maze, block_size, &player);
                } else {
                    render_3d(&mut framebuffer, &maze, block_size, &player, &texture_cache);
                    render_key(&mut framebuffer, &player, &texture_cache);
                }
                
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
                
                // Volver al menú si se presiona ESC
                if window.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                    game_state = GameState::MainMenu;
                }
            }
        }
        
        framebuffer.swap_buffers(&mut window, &raylib_thread);
        thread::sleep(Duration::from_millis(16));
    }
}