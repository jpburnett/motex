/// Enumeration of the different texture formats used by the N64
///
use variant_count::VariantCount;

#[derive(Default, VariantCount, Debug, PartialEq, Eq)]
pub enum N64Codec {
    #[default]
    RGBA16,
    RGBA32,
    IA16,
    IA32,
    IA8,
    IA4,
    I8,
    I4,
    CI8,
    CI4,
    ONEBPP,
}

/// Converts a codec enum to a string
pub fn codec_name(codec: N64Codec) -> &'static str {
    match codec {
        N64Codec::RGBA16 => "RGBA16",
        N64Codec::RGBA32 => "RGBA32",
        N64Codec::IA16 => "IA16",
        N64Codec::IA32 => "IA32",
        N64Codec::IA8 => "IA8",
        N64Codec::IA4 => "IA4",
        N64Codec::I8 => "I8",
        N64Codec::I4 => "I4",
        N64Codec::CI8 => "CI8",
        N64Codec::CI4 => "CI4",
        N64Codec::ONEBPP => "1BPP",
    }
}

impl N64Codec {
    pub fn from_index(index: usize) -> N64Codec {
        match index {
            0 => N64Codec::RGBA16,
            1 => N64Codec::RGBA32,
            2 => N64Codec::IA16,
            3 => N64Codec::IA32,
            4 => N64Codec::IA8,
            5 => N64Codec::IA4,
            6 => N64Codec::I8,
            7 => N64Codec::I4,
            8 => N64Codec::CI8,
            9 => N64Codec::CI4,
            10 => N64Codec::ONEBPP,
            _ => N64Codec::RGBA16,
        }
    }
}

pub const CODEC_LENGTH: usize = N64Codec::VARIANT_COUNT;

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
    /// Creates a new color from the given u8 RGBA values
    pub const fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    #[inline]
    /// Creates a new color from 4 u8 values and sets alpha to 255
    pub const fn new_rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 0xFF }
    }

    /// Creates a new color from a 32-bit RGBA pixel.
    pub fn from_u32(pixel: u32) -> Color {
        let r = ((pixel >> 24) & 0xFF) as u8;
        let g = ((pixel >> 16) & 0xFF) as u8;
        let b = ((pixel >> 8) & 0xFF) as u8;
        let a = (pixel & 0xFF) as u8;

        Color { r, g, b, a }
    }
}
