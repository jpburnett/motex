/// Enumeration of the different texture formats
///
use variant_count::VariantCount;

#[derive(Default, VariantCount, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ImgFormat {
    #[default]
    RGBA16,
    RGBA32,
    IA32,
    IA16,
    IA8,
    IA4,
    I8,
    I4,
    CI8,
    CI4,
    OneBPP,
}

impl ImgFormat {
    pub fn get_all_formats() -> Vec<ImgFormat> {
        vec![
            ImgFormat::RGBA16,
            ImgFormat::RGBA32,
            ImgFormat::IA32,
            ImgFormat::IA16,
            ImgFormat::IA8,
            ImgFormat::IA4,
            ImgFormat::I8,
            ImgFormat::I4,
            ImgFormat::CI8,
            ImgFormat::CI4,
            ImgFormat::OneBPP,
        ]
    }
}

impl ToString for ImgFormat {
    fn to_string(&self) -> String {
        match self {
            Self::RGBA16 => "RGBA16",
            Self::RGBA32 => "RGBA32",
            Self::IA32 => "IA32",
            Self::IA16 => "IA16",
            Self::IA8 => "IA8",
            Self::IA4 => "IA4",
            Self::I8 => "I8",
            Self::I4 => "I4",
            Self::CI8 => "CI8",
            Self::CI4 => "CI4",
            Self::OneBPP => "OneBPP"
        }.to_string()
    }
}

impl From<ImgFormat> for usize {
    fn from(value: ImgFormat) -> Self {
        match value {
            ImgFormat::RGBA16 => 0,
            ImgFormat::RGBA32 => 1,
            ImgFormat::IA32 => 2,
            ImgFormat::IA16 => 3,
            ImgFormat::IA8 => 4,
            ImgFormat::IA4 => 5,
            ImgFormat::I8 => 6,
            ImgFormat::I4 => 7,
            ImgFormat::CI8 => 8,
            ImgFormat::CI4 => 9,
            ImgFormat::OneBPP => 10,   
        }
    }
}

pub const IMG_FORMAT_LENGTH: usize = ImgFormat::VARIANT_COUNT;

/// Structure modeling Color data made up of 4 u8 values
/// r: Red
/// g: Green
/// b: Blue
/// a: Alpha
///
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// Sample colors
    pub const TRANSPARENT: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };

    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };

    #[inline]
    /// Create a new color from the given u8 RGBA values
    pub const fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    #[inline]
    /// Create a new color from 4 u8 values and sets alpha to 255
    pub const fn new_rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 0xFF }
    }

    /// Create a new color from a 32-bit RGBA pixel.
    pub fn rgba_from_u32(pixel: u32) -> Color {
        let r = ((pixel >> 24) & 0xFF) as u8;
        let g = ((pixel >> 16) & 0xFF) as u8;
        let b = ((pixel >> 8) & 0xFF) as u8;
        let a = (pixel & 0xFF) as u8;

        Color { r, g, b, a }
    }

    /// Convert a 16-bit RGBA pixel to a 32-bit RGBA color.
    /// 
    #[inline]
    pub fn from_u16(pixel: u16) -> Color {
        let r = ((pixel >> 11) & 0x1F) as u8;
        let g = ((pixel >> 6) & 0x1F) as u8;
        let b = ((pixel >> 1) & 0x1F) as u8;
        let a = (pixel & 0x01) as u8;

        let r = (r << 3) | (r >> 2);
        let g = (g << 3) | (g >> 2);
        let b = (b << 3) | (b >> 2);
        let a = 255 * a;

        Color { r, g, b, a }
    }

    /// Convert a 32-bit RGBA color to a 16-bit RGBA pixel.
    #[inline]
    pub fn rgba_to_u16(&self) -> u16 {
        // Could divide by 8, but shifting is faster
        let r = (self.r >> 3) as u16;
        let g = (self.g >> 3) as u16;
        let b = (self.b >> 3) as u16;
        // Normalize alpha then scale down to a single bit
        let a = (self.a / 255) as u16;
        // Couldn't this also work?...
        // let a = if self.a > 0 { 1 } else { 0 };

        // Combining the components into a single 16-bit value
        (r << 11) | (g << 6) | (b << 1) | a
    }

}
