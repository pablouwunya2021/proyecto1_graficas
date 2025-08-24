use crate::framebuffer::Framebuffer;
use crate::player::Player;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView};
use rand::Rng;

pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub sprite_index: usize,
    pub sprites: Vec<DynamicImage>,
}

impl Enemy {
    pub fn new(x: f32, y: f32, sprite_paths: &[&str]) -> Self {
        let mut sprites = Vec::new();
        for path in sprite_paths {
            let img = ImageReader::open(path)
                .expect("No se pudo abrir sprite del enemigo")
                .decode()
                .expect("No se pudo decodificar imagen");
            sprites.push(img);
        }

        Self {
            x,
            y,
            sprite_index: 0,
            sprites,
        }
    }

    /// Alterna sprite para animación
    pub fn animate(&mut self) {
        self.sprite_index = (self.sprite_index + 1) % self.sprites.len();
    }

    /// Movimiento aleatorio
    pub fn update(&mut self, maze: &Vec<Vec<char>>, block_size: usize) {
        let mut rng = rand::thread_rng();
        let dx = rng.gen_range(-1.0..1.0);
        let dy = rng.gen_range(-1.0..1.0);

        let new_x = self.x + dx * 1.5;
        let new_y = self.y + dy * 1.5;

        if !is_wall(new_x, new_y, maze, block_size) {
            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn draw_3d(&self, fb: &mut Framebuffer, player: &Player, width: usize, height: usize) {
        let sprite = &self.sprites[self.sprite_index];

        // Calcular la posición relativa al jugador
        let dx = self.x - player.pos.x;
        let dy = self.y - player.pos.y;

        let angle = player.a.atan2(1.0);
        let inv_det = 1.0 / (player.a.cos() * 1.0 - player.a.sin() * 0.0);

        // Simple proyección (versión básica)
        let dist = (dx*dx + dy*dy).sqrt();
        if dist < 5.0 { return; } // evitar que se dibuje encima

        let scale = (200.0 / dist).max(5.0); // tamaño depende de la distancia
        let screen_x = (width as f32 / 2.0 + dx - dy).round() as i32;
        let screen_y = (height as f32 / 2.0 - scale / 2.0).round() as i32;

        // Dibujar sprite como rectángulo escalado
        let (sw, sh) = sprite.dimensions();
        for sy in 0..sh {
            for sx in 0..sw {
                let pixel = sprite.get_pixel(sx, sy);
                if pixel[3] == 0 { continue; } // transparencia

                let px = (screen_x + (sx as f32 * scale / sw as f32) as i32) as usize;
                let py = (screen_y + (sy as f32 * scale / sh as f32) as i32) as usize;

                if px < fb.width && py < fb.height {
                    let color = ((pixel[0] as u32) << 16)
                        | ((pixel[1] as u32) << 8)
                        | (pixel[2] as u32);
                    fb.point(px, py, color);
                }
            }
        }
    }
}

/// Chequea si es pared
fn is_wall(x: f32, y: f32, maze: &Vec<Vec<char>>, block_size: usize) -> bool {
    let i = (x / block_size as f32) as usize;
    let j = (y / block_size as f32) as usize;
    if j >= maze.len() || i >= maze[0].len() {
        return true;
    }
    maze[j][i] != ' '
}
