// main.rs
#![allow(unused_imports)]
#![allow(dead_code)]

mod framebuffer;
mod maze;
mod player;
mod caster;
mod textures;
mod key;

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
    let maze = load_maze("maze.txt");
    let mut player = Player{
        pos: Vector2::new(150.0,150.0), 
        a: PI/2.0,
        fov: PI / 3.0, 
    };

    let texture_cache = TextureManager::new(&mut window, &raylib_thread);

    let minimap_size = 150; // Tamaño del minimapa en píxeles
    let minimap_position = (window_width as i32 - minimap_size as i32 - 20, 20); // Esquina superior derecha
    
    while !window.window_should_close() {
        // 1. clear framebuffer
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

        // 1.1 process events
        process_events(&window, &mut player, &maze, block_size);

        // 2. draw the maze, passing the maze and block size
        if window.is_key_down(KeyboardKey::KEY_M) {
            // Modo 2D completo (vista desde arriba)
            render_maze(&mut framebuffer, &maze, block_size, &player);
        } else {
            // Modo 3D normal
            render_3d(&mut framebuffer, &maze, block_size, &player, &texture_cache);
            render_key(&mut framebuffer, &player, &texture_cache);
        }
        
        // 3. Dibujar minimapa estático siempre visible (excepto en modo 2D completo)
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
        
        // 4. swap buffers
        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(16));
    }
}