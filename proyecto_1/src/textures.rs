use image::{DynamicImage, GenericImageView};
use std::collections::HashMap;

pub struct Textures {
    pub map: HashMap<char, DynamicImage>, // asociamos char de pared -> imagen
}

impl Textures {
    pub fn new() -> Self {
        let mut map = HashMap::new();

        map.insert('#', image::open("textures/pared.png").unwrap());
        map.insert('A', image::open("textures/twilight.png").unwrap());
        map.insert('B', image::open("textures/pinky.png").unwrap());
        map.insert('C', image::open("textures/apple.png").unwrap());

        Textures { map }
    }

    /// Obtener color de la textura en coordenada x, y (0..1)
    pub fn sample(&self, c: char, u: f32, v: f32) -> u32 {
        if let Some(img) = self.map.get(&c) {
            let w = img.width() as f32;
            let h = img.height() as f32;

            let px = (u.clamp(0.0, 0.999) * w) as u32;
            let py = (v.clamp(0.0, 0.999) * h) as u32;

            let rgba = img.get_pixel(px, py).0;
            ((rgba[0] as u32) << 16) | ((rgba[1] as u32) << 8) | (rgba[2] as u32)
        } else {
            0xFFFFFF
        }
    }
}
