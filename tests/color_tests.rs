// tests/color_tests.rs

#[cfg(test)]
mod tests {

    use motex::{Color, N64Codec, codec_name};

    #[test]
    fn test_color_new_rgba() {
        let color = Color::new_rgba(0, 0, 0, 0);
        assert_eq!(color.r, 0);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
        assert_eq!(color.a, 0);
    }

    #[test]
    fn test_color_new_rgb() {
        let color = Color::new_rgb(0, 0, 0);
        assert_eq!(color.r, 0);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn test_color_from_u32() {
        let color = Color::from_u32(0x00000000);
        assert_eq!(color.r, 0);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
        assert_eq!(color.a, 0);
    }

    #[test]
    fn test_color_black() {
        let color = Color::BLACK;
        assert_eq!(color.r, 0);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn test_color_white() {
        let color = Color::WHITE;
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 255);
        assert_eq!(color.b, 255);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn test_codec_name() {
        assert_eq!(codec_name(N64Codec::RGBA16), "RGBA16");
        assert_eq!(codec_name(N64Codec::RGBA32), "RGBA32");
        assert_eq!(codec_name(N64Codec::IA16), "IA16");
        assert_eq!(codec_name(N64Codec::IA32), "IA32");
        assert_eq!(codec_name(N64Codec::IA8), "IA8");
        assert_eq!(codec_name(N64Codec::IA4), "IA4");
        assert_eq!(codec_name(N64Codec::I8), "I8");
        assert_eq!(codec_name(N64Codec::I4), "I4");
        assert_eq!(codec_name(N64Codec::CI8), "CI8");
        assert_eq!(codec_name(N64Codec::CI4), "CI4");
        assert_eq!(codec_name(N64Codec::ONEBPP), "1BPP");
    }
}