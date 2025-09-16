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

