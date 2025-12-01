use crate::file_buffer::FileBuffer;
use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum ImageFormat {
    I4,
    I8,
    IA4,
    IA8,
    IA16,
    CI4,
    CI8,
    RGBA16,
    RGBA32,
}

impl ImageFormat {
    pub fn all() -> &'static [ImageFormat] {
        &[
            ImageFormat::RGBA16,
            ImageFormat::RGBA32,
            ImageFormat::IA16,
            ImageFormat::IA8,
            ImageFormat::IA4,
            ImageFormat::I8,
            ImageFormat::I4,
            ImageFormat::CI8,
            ImageFormat::CI4,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            ImageFormat::I4 => "I4",
            ImageFormat::I8 => "I8",
            ImageFormat::IA4 => "IA4",
            ImageFormat::IA8 => "IA8",
            ImageFormat::IA16 => "IA16",
            ImageFormat::CI4 => "CI4",
            ImageFormat::CI8 => "CI8",
            ImageFormat::RGBA16 => "RGBA16",
            ImageFormat::RGBA32 => "RGBA32",
        }
    }

    pub fn is_ci(&self) -> bool {
        matches!(self, ImageFormat::CI4 | ImageFormat::CI8)
    }

    pub fn bits_per_pixel(&self) -> usize {
        match self {
            ImageFormat::I4 | ImageFormat::IA4 | ImageFormat::CI4 => 4,
            ImageFormat::I8 | ImageFormat::IA8 | ImageFormat::CI8 => 8,
            ImageFormat::IA16 | ImageFormat::RGBA16 => 16,
            ImageFormat::RGBA32 => 32,
        }
    }

    pub fn bytes_per_pixel(&self) -> f32 {
        self.bits_per_pixel() as f32 / 8.0
    }

    pub fn to_pigment64(&self) -> pigment64::ImageType {
        match self {
            ImageFormat::I4 => pigment64::ImageType::I4,
            ImageFormat::I8 => pigment64::ImageType::I8,
            ImageFormat::IA4 => pigment64::ImageType::Ia4,
            ImageFormat::IA8 => pigment64::ImageType::Ia8,
            ImageFormat::IA16 => pigment64::ImageType::Ia16,
            ImageFormat::CI4 => pigment64::ImageType::Ci4,
            ImageFormat::CI8 => pigment64::ImageType::Ci8,
            ImageFormat::RGBA16 => pigment64::ImageType::Rgba16,
            ImageFormat::RGBA32 => pigment64::ImageType::Rgba32,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct TextureViewState {
    pub format: ImageFormat,
    pub width: u32,
    pub height: u32,
    pub offset: usize,
    pub palette_offset: usize,
    pub split_palette_enabled: bool,
    pub split_palette_offset: usize,
    pub use_alpha_for_intensity: bool,

    #[serde(skip)]
    pub current_rgba: Option<Vec<u8>>,
}

impl Default for TextureViewState {
    fn default() -> Self {
        Self {
            format: ImageFormat::RGBA16,
            width: 32,
            height: 32,
            offset: 0,
            palette_offset: 0,
            split_palette_enabled: false,
            split_palette_offset: 0,
            use_alpha_for_intensity: false,
            current_rgba: None,
        }
    }
}

impl TextureViewState {
    pub fn update_from_buffer(&mut self, buffer: &FileBuffer, palette: Option<&[u8]>) {
        if buffer.is_empty() {
            self.current_rgba = None;
            return;
        }

        let size = (self.width * self.height) as usize;
        let bytes_needed = (size as f32 * self.format.bytes_per_pixel()).ceil() as usize;

        // Attempt to get the data slice
        let raw_slice = buffer.get_slice(self.offset, bytes_needed);

        match raw_slice {
            Some(data) if data.len() == bytes_needed => {
                // Case 1: We have all the data we need
                match self.decode_texture(data, palette) {
                    Ok(rgba) => self.current_rgba = Some(rgba),
                    Err(e) => {
                        log::warn!("Failed to decode texture: {}", e);
                        self.current_rgba = None;
                    }
                }
            }
            Some(data) => {
                // Case 2: Partial data (End of file). Pad with zeros.
                let mut padded = data.to_vec();
                padded.resize(bytes_needed, 0);

                match self.decode_texture(&padded, palette) {
                    Ok(rgba) => self.current_rgba = Some(rgba),
                    Err(e) => {
                        log::warn!("Failed to decode texture (padded): {}", e);
                        self.current_rgba = None;
                    }
                }
            }
            None => {
                // Case 3: Offset is completely out of bounds
                self.current_rgba = None;
            }
        }
    }

    fn decode_texture(&self, data: &[u8], palette: Option<&[u8]>) -> Result<Vec<u8>> {
        let img_type = self.format.to_pigment64();

        // Read NativeImage from raw data
        let native_img = pigment64::NativeImage::read(data, img_type, self.width, self.height)?;

        // Convert to PNG and write to buffer
        let mut png_bytes = Vec::new();
        native_img.as_png(&mut png_bytes, palette)?;

        // Read PNG back to get RGBA data
        let img = image::load_from_memory(&png_bytes)?;
        let rgba_img = img.to_rgba8();

        Ok(rgba_img.into_raw())
    }

    pub fn encode_to_n64(&self, rgba_img: &image::RgbaImage) -> Result<Vec<u8>> {
        let img_type = self.format.to_pigment64();

        // Create a PNG in memory from the RGBA data
        let mut png_bytes = Vec::new();
        rgba_img.write_to(
            &mut std::io::Cursor::new(&mut png_bytes),
            image::ImageFormat::Png,
        )?;

        // Read as pigment64 PNGImage
        let png_img = pigment64::PNGImage::read(&png_bytes[..])?;

        // Convert to N64 format
        let mut native_bytes = Vec::new();
        png_img.as_native(&mut native_bytes, img_type)?;

        Ok(native_bytes)
    }

    pub fn get_current_rgba(&self) -> Option<Vec<u8>> {
        self.current_rgba.clone()
    }

    pub fn texture_size_bytes(&self) -> usize {
        let pixels = (self.width * self.height) as usize;
        (pixels as f32 * self.format.bytes_per_pixel()).ceil() as usize
    }

    pub fn adjust_offset(&mut self, delta: i64, buffer_size: usize, mode: ScrollMode) {
        let bytes_per_row = (self.width as f32 * self.format.bytes_per_pixel()).ceil() as i64;
        let bytes_per_image = self.texture_size_bytes() as i64;

        let offset_delta = match mode {
            ScrollMode::Pixel => (self.format.bytes_per_pixel().ceil() as i64) * delta,
            ScrollMode::Row => bytes_per_row * delta,
            ScrollMode::FourRows => bytes_per_row * 4 * delta,
            ScrollMode::Image => bytes_per_image * delta,
        };

        // Apply delta
        let new_offset = self.offset as i64 + offset_delta;

        // Clamp
        self.offset = new_offset.clamp(0, buffer_size.saturating_sub(1) as i64) as usize;
    }

    // pub fn set_offset_from_click(&mut self, x: usize, y: usize) {
    //     let pixel_index = y * self.width as usize + x;
    //     let byte_offset = (pixel_index as f32 * self.format.bytes_per_pixel()).floor() as usize;
    //     self.offset += byte_offset;
    // }
}

#[derive(Debug, Clone, Copy)]
pub enum ScrollMode {
    Pixel,
    Row,
    FourRows,
    Image,
}
