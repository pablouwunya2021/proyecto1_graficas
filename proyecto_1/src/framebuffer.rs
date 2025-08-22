pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    buffer: Vec<Vec<u32>>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![vec![0; height]; width],
        }
    }

    pub fn clear(&mut self, color: u32) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.buffer[x][y] = color;
            }
        }
    }

    pub fn point(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.buffer[x][y] = color;
        }
    }

    pub fn flush_to(&self, out: &mut [u32]) {
        // out es lineal: fila mayor, luego columna
        for y in 0..self.height {
            for x in 0..self.width {
                out[y * self.width + x] = self.buffer[x][y];
            }
        }
    }
}

