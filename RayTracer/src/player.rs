// src/player.rs
use raylib::prelude::*;
use std::f32::consts::PI;
use crate::maze::Maze;

pub struct Player {
    pub pos: Vector2,
    pub a: f32,
    pub fov: f32,
}

// Modificamos la función para que maneje solo movimiento y rotación, y devuelva si se movió o no.
pub fn process_events(
    window: &RaylibHandle, // Cambiado a &RaylibHandle para no necesitar &mut
    player: &mut Player,
    maze: &Maze,
    block_size: usize,
) -> bool { // Devuelve true si el jugador se movió físicamente
    const MOVE_SPEED: f32 = 8.0;
    const ROTATION_SPEED: f32 = PI / 40.0; // Ajustado según ejemplo
    const MOUSE_SENSITIVITY: f32 = 0.002; // Ajustado según ejemplo

    // Obtener el delta del mouse directamente aquí
    let mouse_delta = window.get_mouse_delta();

    // Rotación con mouse (solo ángulo)
    player.a += mouse_delta.x * MOUSE_SENSITIVITY;

    // Rotación con teclado (opcional, para redundancia o controles alternativos)
    if window.is_key_down(KeyboardKey::KEY_LEFT) || window.is_key_down(KeyboardKey::KEY_A) {
        player.a -= ROTATION_SPEED;
    }
    if window.is_key_down(KeyboardKey::KEY_RIGHT) || window.is_key_down(KeyboardKey::KEY_D) {
        player.a += ROTATION_SPEED;
    }

    // Normalizar el ángulo para evitar overflow
    if player.a > PI {
        player.a -= 2.0 * PI;
    } else if player.a < -PI {
        player.a += 2.0 * PI;
    }

    let mut next_pos = player.pos;
    let mut moved = false; // Indica si se intentó mover (antes de colisión)

    // Movimiento con teclado (WASD o Flechas)
    if window.is_key_down(KeyboardKey::KEY_UP) || window.is_key_down(KeyboardKey::KEY_W) {
        next_pos.x += MOVE_SPEED * player.a.cos();
        next_pos.y += MOVE_SPEED * player.a.sin();
        moved = true;
    }
    if window.is_key_down(KeyboardKey::KEY_DOWN) || window.is_key_down(KeyboardKey::KEY_S) {
        next_pos.x -= MOVE_SPEED * player.a.cos();
        next_pos.y -= MOVE_SPEED * player.a.sin();
        moved = true;
    }

    // Verificar colisiones y aplicar movimiento
    if moved {
        let grid_x = next_pos.x as usize / block_size;
        let grid_y = next_pos.y as usize / block_size;

        // Verificar límites del laberinto
        if grid_y < maze.len() && grid_x < maze[grid_y].len() {
            // Permitir movimiento si es espacio vacío o la meta (cuando se tiene la llave se verifica en main)
            if maze[grid_y][grid_x] == ' ' || maze[grid_y][grid_x] == 'g' {
                player.pos = next_pos;
                return true; // El jugador se movió físicamente
            }
        }
        // Si llegó aquí, hubo intento de movimiento pero colisión, devolver false
        return false;
    }

    // Si no se intentó mover, devolver false
    false
}