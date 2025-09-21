// text.rs
use crate::framebuffer::Framebuffer;
use raylib::prelude::*;

pub struct Font {
    characters: [[[u8; 5]; 5]; 128],
}

impl Font {
    pub fn new() -> Self {
        let mut characters = [[[0; 5]; 5]; 128];
        
        // Definir caracteres básicos (A-Z, 0-9, y algunos símbolos)
        
        // Letra 'A'
        characters['A' as usize] = [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
        ];
        
        // Letra 'B'
        characters['B' as usize] = [
            [1, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 0],
        ];
        
        // Letra 'C'
        characters['C' as usize] = [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
        ];
        
        // Letra 'D'
        characters['D' as usize] = [
            [1, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 0],
        ];
        
        // Letra 'E'
        characters['E' as usize] = [
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0],
            [1, 1, 1, 1, 0],
            [1, 0, 0, 0, 0],
            [1, 1, 1, 1, 1],
        ];
        
        // Letra 'F'
        characters['F' as usize] = [
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0],
            [1, 1, 1, 1, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
        ];
        
        // Letra 'G'
        characters['G' as usize] = [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 1, 1, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
        ];
        
        // Letra 'H'
        characters['H' as usize] = [
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
        ];
        
        // Letra 'I'
        characters['I' as usize] = [
            [1, 1, 1, 1, 1],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [1, 1, 1, 1, 1],
        ];
        
        // Letra 'J'
        characters['J' as usize] = [
            [0, 0, 1, 1, 1],
            [0, 0, 0, 1, 0],
            [0, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [0, 1, 1, 0, 0],
        ];
        
        // Letra 'K'
        characters['K' as usize] = [
            [1, 0, 0, 0, 1],
            [1, 0, 0, 1, 0],
            [1, 1, 1, 0, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 0, 1],
        ];
        
        // Letra 'L'
        characters['L' as usize] = [
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 1, 1, 1, 1],
        ];
        
        // Letra 'M'
        characters['M' as usize] = [
            [1, 0, 0, 0, 1],
            [1, 1, 0, 1, 1],
            [1, 0, 1, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
        ];
        
        // Letra 'N'
        characters['N' as usize] = [
            [1, 0, 0, 0, 1],
            [1, 1, 0, 0, 1],
            [1, 0, 1, 0, 1],
            [1, 0, 0, 1, 1],
            [1, 0, 0, 0, 1],
        ];
        
        // Letra 'O'
        characters['O' as usize] = [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
        ];
        
        // Letra 'P'
        characters['P' as usize] = [
            [1, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
        ];
        
        // Letra 'Q'
        characters['Q' as usize] = [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 1, 1],
            [0, 1, 1, 1, 1],
        ];
        
        // Letra 'R'
        characters['R' as usize] = [
            [1, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 0, 1],
        ];
        
        // Letra 'S'
        characters['S' as usize] = [
            [0, 1, 1, 1, 1],
            [1, 0, 0, 0, 0],
            [0, 1, 1, 1, 0],
            [0, 0, 0, 0, 1],
            [1, 1, 1, 1, 0],
        ];
        
        // Letra 'T'
        characters['T' as usize] = [
            [1, 1, 1, 1, 1],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
        ];
        
        // Letra 'U'
        characters['U' as usize] = [
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
        ];
        
        // Letra 'V'
        characters['V' as usize] = [
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
        ];
        
        // Letra 'W'
        characters['W' as usize] = [
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 1, 0, 1],
            [1, 1, 0, 1, 1],
            [1, 0, 0, 0, 1],
        ];
        
        // Letra 'X'
        characters['X' as usize] = [
            [1, 0, 0, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 0, 1, 0],
            [1, 0, 0, 0, 1],
        ];
        
        // Letra 'Y'
        characters['Y' as usize] = [
            [1, 0, 0, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
        ];
        
        // Letra 'Z'
        characters['Z' as usize] = [
            [1, 1, 1, 1, 1],
            [0, 0, 0, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 0, 0, 0],
            [1, 1, 1, 1, 1],
        ];
        
        // Números 0-9
        for c in '0'..='9' {
            let i = c as usize;
            characters[i] = match c {
                '0' => [
                    [0, 1, 1, 1, 0],
                    [1, 0, 0, 0, 1],
                    [1, 0, 0, 0, 1],
                    [1, 0, 0, 0, 1],
                    [0, 1, 1, 1, 0],
                ],
                '1' => [
                    [0, 0, 1, 0, 0],
                    [0, 1, 1, 0, 0],
                    [0, 0, 1, 0, 0],
                    [0, 0, 1, 0, 0],
                    [0, 1, 1, 1, 0],
                ],
                '2' => [
                    [0, 1, 1, 1, 0],
                    [1, 0, 0, 0, 1],
                    [0, 0, 1, 1, 0],
                    [0, 1, 0, 0, 0],
                    [1, 1, 1, 1, 1],
                ],
                '3' => [
                    [1, 1, 1, 1, 0],
                    [0, 0, 0, 0, 1],
                    [0, 1, 1, 1, 0],
                    [0, 0, 0, 0, 1],
                    [1, 1, 1, 1, 0],
                ],
                '4' => [
                    [0, 0, 0, 1, 0],
                    [0, 0, 1, 1, 0],
                    [0, 1, 0, 1, 0],
                    [1, 1, 1, 1, 1],
                    [0, 0, 0, 1, 0],
                ],
                '5' => [
                    [1, 1, 1, 1, 1],
                    [1, 0, 0, 0, 0],
                    [1, 1, 1, 1, 0],
                    [0, 0, 0, 0, 1],
                    [1, 1, 1, 1, 0],
                ],
                '6' => [
                    [0, 1, 1, 1, 0],
                    [1, 0, 0, 0, 0],
                    [1, 1, 1, 1, 0],
                    [1, 0, 0, 0, 1],
                    [0, 1, 1, 1, 0],
                ],
                '7' => [
                    [1, 1, 1, 1, 1],
                    [0, 0, 0, 0, 1],
                    [0, 0, 0, 1, 0],
                    [0, 0, 1, 0, 0],
                    [0, 0, 1, 0, 0],
                ],
                '8' => [
                    [0, 1, 1, 1, 0],
                    [1, 0, 0, 0, 1],
                    [0, 1, 1, 1, 0],
                    [1, 0, 0, 0, 1],
                    [0, 1, 1, 1, 0],
                ],
                '9' => [
                    [0, 1, 1, 1, 0],
                    [1, 0, 0, 0, 1],
                    [0, 1, 1, 1, 1],
                    [0, 0, 0, 0, 1],
                    [0, 1, 1, 1, 0],
                ],
                _ => [[0; 5]; 5],
            };
        }
        
        // Espacio
        characters[' ' as usize] = [[0; 5]; 5];
        
        // Símbolo '>'
        characters['>' as usize] = [
            [1, 0, 0, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 0, 1, 1, 1],
            [0, 1, 0, 0, 0],
            [1, 0, 0, 0, 0],
        ];
        
        // Símbolo '<'
        characters['<' as usize] = [
            [0, 0, 0, 0, 1],
            [0, 0, 0, 1, 0],
            [1, 1, 1, 0, 0],
            [0, 0, 0, 1, 0],
            [0, 0, 0, 0, 1],
        ];
        
        Font { characters }
    }
    
    pub fn draw_text(&self, framebuffer: &mut Framebuffer, text: &str, x: i32, y: i32, scale: i32, color: Color) {
        let mut current_x = x;
        
        for c in text.chars() {
            if c as usize >= self.characters.len() {
                current_x += 6 * scale;
                continue;
            }
            
            let character = self.characters[c as usize];
            
            for row in 0..5 {
                for col in 0..5 {
                    if character[row][col] == 1 {
                        for dx in 0..scale {
                            for dy in 0..scale {
                                let px = current_x + col as i32 * scale + dx;
                                let py = y + row as i32 * scale + dy;
                                
                                // Dibujar el píxel directamente sin cambiar el color global
                                if px >= 0 && px < framebuffer.width && py >= 0 && py < framebuffer.height {
                                    framebuffer.color_buffer.draw_pixel(px, py, color);
                                }
                            }
                        }
                    }
                }
            }
            
            current_x += 6 * scale;
        }
    }
}