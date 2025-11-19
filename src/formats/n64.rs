use crate::texture::Texture;
use pigment64::{ImageType, NativeImage};
use std::io::Cursor;

/// N64 texture formats we support
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum N64Format {
    RGBA16,
    RGBA32,
    CI4,
    CI8,
    IA16,
    IA8,
    IA4,
    I8,
    I4,
}

impl N64Format {
    /// Get display name for UI
    pub fn display_name(&self) -> &str {
        match self {
            Self::RGBA16 => "RGBA16",
            Self::RGBA32 => "RGBA32",
            Self::CI4 => "CI4",
            Self::CI8 => "CI8",
            Self::IA16 => "IA16",
            Self::IA8 => "IA8",
            Self::IA4 => "IA4",
            Self::I8 => "I8",
            Self::I4 => "I4",
        }
    }

    /// Does this format need a palette?
    pub fn needs_palette(&self) -> bool {
        matches!(self, Self::CI4 | Self::CI8)
    }

    /// Convert to pigment64's ImageType (based on actual enum variants)
    fn to_pigment_type(&self) -> ImageType {
        match self {
            Self::RGBA16 => ImageType::Rgba16,
            Self::RGBA32 => ImageType::Rgba32,
            Self::CI4 => ImageType::Ci4,
            Self::CI8 => ImageType::Ci8,
            Self::IA16 => ImageType::Ia16,
            Self::IA8 => ImageType::Ia8,
            Self::IA4 => ImageType::Ia4,
            Self::I8 => ImageType::I8,
            Self::I4 => ImageType::I4,
        }
    }
}

/// Decode an N64 texture using pigment64
pub fn decode_n64_texture(
    data: &[u8],
    format: N64Format,
    width: u32,
    height: u32,
    palette: Option<&[u8]>,
) -> Result<Texture, String> {
    log::info!(
        "Decoding N64 texture: format={:?}, size={}x{}, data_len={}",
        format,
        width,
        height,
        data.len()
    );

    // Validate palette requirement
    if format.needs_palette() && palette.is_none() {
        return Err(format!("{} format requires a palette", format.display_name()));
    }

    let image_type = format.to_pigment_type();
    let mut cursor = Cursor::new(data);

    // Read native N64 image
    let native_image = NativeImage::read(
        &mut cursor,
        image_type,
        width,
        height,
    ).map_err(|e| format!("pigment64 read error: {:?}", e))?;

    // Convert to PNG bytes
    let mut png_bytes = Vec::new();
    native_image
        .as_png(&mut png_bytes, palette)
        .map_err(|e| format!("pigment64 PNG conversion error: {:?}", e))?;

    // Decode PNG to RGBA using image crate
    let png_image = image::load_from_memory(&png_bytes)
        .map_err(|e| format!("PNG decode error: {:?}", e))?;

    let rgba_image = png_image.to_rgba8();
    let rgba_data = rgba_image.into_raw();

    // Create our texture
    let mut texture = Texture::new(width, height);
    texture.data = rgba_data;

    log::info!("Successfully decoded N64 texture");
    Ok(texture)
}

/// Helper to decode without palette (for non-CI formats)
pub fn decode_n64_texture_simple(
    data: &[u8],
    format: N64Format,
    width: u32,
    height: u32,
) -> Result<Texture, String> {
    decode_n64_texture(data, format, width, height, None)
}
