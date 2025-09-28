use raylib::prelude::*;
use crate::textures::TextureManager;

pub struct Key {
    pub pos: Vector2,
    pub texture_key: char,
}

impl Key {
    pub fn new(x: f32, y: f32, texture_key: char) -> Self {
        Key {
            pos: Vector2::new(x, y), 
            texture_key,
        }
    }
}

// Nueva estructura para la batería
pub struct Battery {
    pub pos: Vector2,
    pub texture_keys: [char; 3], // Claves para las 3 texturas de animación
    pub current_frame: usize,    // Índice de la textura actual
    pub frame_timer: f32,        // Tiempo acumulado para cambiar de frame
    pub frame_duration: f32,     // Duración de cada frame en segundos
    pub move_timer: f32,         // Tiempo acumulado para moverse
    pub move_duration: f32,      // Intervalo entre movimientos
    pub target_pos: Vector2,     // Posición objetivo para el movimiento
}

impl Battery {
    pub fn new(x: f32, y: f32, texture_keys: [char; 3]) -> Self {
        Battery {
            pos: Vector2::new(x, y),
            texture_keys,
            current_frame: 0,
            frame_timer: 0.0,
            frame_duration: 0.2, // Cambia frame cada 0.2 segundos
            move_timer: 0.0,
            move_duration: 2.0, // Se mueve cada 2 segundos
            target_pos: Vector2::new(x, y), // Inicialmente se mueve a su propia posición
        }
    }
}

