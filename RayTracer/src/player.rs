use raylib::prelude::*;
use std::f32::consts::PI;
use crate::maze::Maze;

pub struct Player {
    pub pos: Vector2,
    pub a: f32,
    pub fov: f32,
}

pub fn process_events(
    window: &RaylibHandle,
    player: &mut Player,
    maze: &Maze,
    block_size: usize,
) -> bool {
    const MOVE_SPEED: f32 = 8.0;
    const ROTATION_SPEED: f32 = PI / 20.0;
    const MOUSE_SENSITIVITY: f32 = 0.003;

    if window.is_key_down(KeyboardKey::KEY_LEFT) {
        player.a -= ROTATION_SPEED;
    }
    if window.is_key_down(KeyboardKey::KEY_RIGHT) {
        player.a += ROTATION_SPEED;
    }

    // Mouse rotation (only affects angle)
    let mouse_delta = window.get_mouse_delta();
    if mouse_delta.x != 0.0 {
        player.a += mouse_delta.x * MOUSE_SENSITIVITY;
    }

    // Normalize angle to [-PI, PI]
    if player.a > PI { player.a -= 2.0 * PI; }
    if player.a < -PI { player.a += 2.0 * PI; }

    let mut next_pos = player.pos;
    let mut moved = false;

    if window.is_key_down(KeyboardKey::KEY_UP) {
        next_pos.x += MOVE_SPEED * player.a.cos();
        next_pos.y += MOVE_SPEED * player.a.sin();
        moved = true;
    }
    if window.is_key_down(KeyboardKey::KEY_DOWN) {
        next_pos.x -= MOVE_SPEED * player.a.cos();
        next_pos.y -= MOVE_SPEED * player.a.sin();
        moved = true;
    }

    if moved {

        let grid_x = next_pos.x as usize / block_size;
        let grid_y = next_pos.y as usize / block_size;

        if grid_y < maze.len() && grid_x < maze[grid_y].len()
            && (maze[grid_y][grid_x] == ' ' || maze[grid_y][grid_x] == 'g')
        {
            player.pos = next_pos;
            return true;
        }
    }
    false
}
