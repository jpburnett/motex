// tests/color_tests.rs

#[cfg(test)]
mod tests {

    use motex::{Color, N64Codec, codec_name};

    /// Test that codec names map to the correct enum
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

    #[test]
    fn test_from_index() {
        assert_eq!(N64Codec::from_index(0), N64Codec::RGBA16);
        assert_eq!(N64Codec::from_index(1), N64Codec::RGBA32);
        assert_eq!(N64Codec::from_index(2), N64Codec::IA16);
        assert_eq!(N64Codec::from_index(3), N64Codec::IA32);
        assert_eq!(N64Codec::from_index(4), N64Codec::IA8);
        assert_eq!(N64Codec::from_index(5), N64Codec::IA4);
        assert_eq!(N64Codec::from_index(6), N64Codec::I8);
        assert_eq!(N64Codec::from_index(7), N64Codec::I4);
        assert_eq!(N64Codec::from_index(8), N64Codec::CI8);
        assert_eq!(N64Codec::from_index(9), N64Codec::CI4);
        assert_eq!(N64Codec::from_index(10), N64Codec::ONEBPP);
        assert_eq!(N64Codec::from_index(11), N64Codec::RGBA16);
    }

    /// Test creating a color from RGBA values
    #[test]
    fn test_color_new_rgba() {
        let color = Color::new_rgba(255, 128, 64, 255);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
        assert_eq!(color.a, 255);
    }

    /// Test creating a color from RGB values
    #[test]
    fn test_color_new_rgb() {
        let color = Color::new_rgb(42, 96, 240);
        assert_eq!(color.r, 42);
        assert_eq!(color.g, 96);
        assert_eq!(color.b, 240);
        assert_eq!(color.a, 255);
    }

    /// Test creating a color from a u32
    #[test]
    fn test_from_u32() {
        // Test case 1: Fully opaque red color
        let pixel1: u32 = 0xFF0000FF;
        let color1 = Color::from_u32(pixel1);
        assert_eq!(color1.r, 0xFF);
        assert_eq!(color1.g, 0);
        assert_eq!(color1.b, 0);
        assert_eq!(color1.a, 0xFF);

        // Test case 2: Semi-transparent green color
        let pixel2: u32 = 0x00FF0080;
        let color2 = Color::from_u32(pixel2);
        assert_eq!(color2.r, 0);
        assert_eq!(color2.g, 0xFF);
        assert_eq!(color2.b, 0);
        assert_eq!(color2.a, 0x80);

        // Test case 3: Fully transparent blue color
        let pixel3: u32 = 0x00000000;
        let color3 = Color::from_u32(pixel3);
        assert_eq!(color3.r, 0);
        assert_eq!(color3.g, 0);
        assert_eq!(color3.b, 0);
        assert_eq!(color3.a, 0);
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

}