/// A decoded texture - always stored as RGBA8888
/// Orthogonal: Doesn't know where it came from or how it's displayed

#[derive(Debug, Clone)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>, // RGBA bytes (4 bytes per pixel)
}

impl Texture {
    /// Create a new texture with the given dimensions
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height * 4) as usize;
        Self {
            width,
            height,
            data: vec![0; size],
        }
    }

    /// Get pixel at (x, y) as RGBA
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<[u8; 4]> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let offset = ((y * self.width + x) * 4) as usize;
        Some([
            self.data[offset],
            self.data[offset + 1],
            self.data[offset + 2],
            self.data[offset + 3],
        ])
    }

    /// Set pixel at (x, y) with RGBA
    pub fn set_pixel(&mut self, x: u32, y: u32, rgba: [u8; 4]) {
        if x >= self.width || y >= self.height {
            return;
        }
        let offset = ((y * self.width + x) * 4) as usize;
        self.data[offset..offset + 4].copy_from_slice(&rgba);
    }
}
