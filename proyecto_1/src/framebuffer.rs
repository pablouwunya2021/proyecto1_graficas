pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    buffer: Vec<u32>, // 0xAARRGGBB -> minifb usa 0x00RRGGBB
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.buffer.fill(0x000000); // negro
    }

    #[inline]
    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.buffer[idx] = 0xFFFFFF; // blanco
        }
    }

    #[inline]
    pub fn put(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.buffer[idx] = color;
        }
    }

    pub fn flush_to(&self, out: &mut [u32]) {
        out.copy_from_slice(&self.buffer);
    }
}
