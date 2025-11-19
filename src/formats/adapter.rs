use crate::texture::texture::Texture;
// use pigment64::ImageFormat;

/// N64 texture formats we support
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextureFormat {
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

impl TextureFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            TextureFormat::RGBA16 => "RGBA16",
            TextureFormat::RGBA32 => "RGBA32",
            TextureFormat::CI4 => "CI4",
            TextureFormat::CI8 => "CI8",
            TextureFormat::IA16 => "IA16",
            TextureFormat::IA8 => "IA8",
            TextureFormat::IA4 => "IA4",
            TextureFormat::I8 => "I8",
            TextureFormat::I4 => "I4",
        }
    }

    pub fn all() -> &'static [TextureFormat] {
        &[
            TextureFormat::RGBA16,
            TextureFormat::RGBA32,
            TextureFormat::CI4,
            TextureFormat::CI8,
            TextureFormat::IA16,
            TextureFormat::IA8,
            TextureFormat::IA4,
            TextureFormat::I8,
            TextureFormat::I4,
        ]
    }
}

/// Decode N64 texture using pigment64
pub fn decode_texture(
    data: &[u8],
    format: TextureFormat,
    width: u32,
    height: u32,
    palette: Option<&[u8]>,
) -> Result<Texture, String> {
    log::info!(
        "Decoding texture: format={:?}, size={}x{}, data_len={}",
        format,
        width,
        height,
        data.len()
    );

    // For now, let's just create a simple RGBA16 decoder manually
    // We'll expand this once we understand pigment64's actual API better
    match format {
        TextureFormat::RGBA16 => decode_rgba16(data, width, height),
        _ => Err(format!("Format {:?} not yet implemented", format)),
    }
}

/// Simple RGBA16 decoder (5-5-5-1 format)
fn decode_rgba16(data: &[u8], width: u32, height: u32) -> Result<Texture, String> {
    let expected_size = (width * height * 2) as usize;

    if data.len() < expected_size {
        return Err(format!(
            "Not enough data: expected {} bytes for {}x{}, got {}",
            expected_size,
            width,
            height,
            data.len()
        ));
    }

    let mut texture = Texture::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel_index = (y * width + x) as usize;
            let byte_offset = pixel_index * 2;

            // Read 16-bit value (big-endian, N64 is big-endian)
            let pixel = u16::from_be_bytes([data[byte_offset], data[byte_offset + 1]]);

            // Extract components (5-5-5-1)
            let r = ((pixel >> 11) & 0x1F) as u8;
            let g = ((pixel >> 6) & 0x1F) as u8;
            let b = ((pixel >> 1) & 0x1F) as u8;
            let a = (pixel & 0x01) as u8;

            // Scale from 5-bit to 8-bit
            let r8 = (r << 3) | (r >> 2);
            let g8 = (g << 3) | (g >> 2);
            let b8 = (b << 3) | (b >> 2);
            let a8 = if a == 1 { 255 } else { 0 };

            let offset = ((y * width + x) * 4) as usize;
            texture.data[offset] = r8;
            texture.data[offset + 1] = g8;
            texture.data[offset + 2] = b8;
            texture.data[offset + 3] = a8;
        }
    }

    Ok(texture)
}
