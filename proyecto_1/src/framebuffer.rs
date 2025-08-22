pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    buffer: Vec<u32>, // lineal, más fácil de manejar con minifb
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    /// Dibuja un pixel blanco (para depuración o líneas)
    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if (x as usize) < self.width && (y as usize) < self.height {
            let idx = y as usize * self.width + x as usize;
            self.buffer[idx] = 0xFFFFFF;
        }
    }

    /// Dibuja un pixel de un color dado
    pub fn set_color_point(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.buffer[idx] = color;
        }
    }

    /// Limpia el framebuffer con un color dado
    pub fn clear(&mut self, color: u32) {
        for pixel in self.buffer.iter_mut() {
            *pixel = color;
        }
    }

    /// Exporta el framebuffer a un buffer plano (para minifb)
    pub fn flush_to(&self, buffer: &mut [u32]) {
        buffer.copy_from_slice(&self.buffer);
    }
}
