pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    buffer: Vec<u32>, // Buffer lineal para mejor rendimiento
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    pub fn clear(&mut self, color: u32) {
        for pixel in &mut self.buffer {
            *pixel = color;
        }
    }

    pub fn point(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    pub fn flush_to(&self, out: &mut [u32]) {
        out.copy_from_slice(&self.buffer);
    }
    
    // Nuevo método para dibujar rectángulos rellenos
    pub fn fill_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        for dy in 0..height {
            let current_y = y + dy;
            if current_y >= self.height {
                continue;
            }
            
            for dx in 0..width {
                let current_x = x + dx;
                if current_x < self.width {
                    self.buffer[current_y * self.width + current_x] = color;
                }
            }
        }
    }
}