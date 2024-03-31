// tests/color_tests.rs

#[cfg(test)]
mod tests {

    use motex::{Color, ImgFormat};

    #[test]
    fn test_get_all_formats() {
        let all_formats = ImgFormat::get_all_formats();
        assert_eq!(all_formats.len(), 11);
        assert!(all_formats.contains(&ImgFormat::RGBA16));
        assert!(all_formats.contains(&ImgFormat::RGBA32));
        assert!(all_formats.contains(&ImgFormat::IA32));
        assert!(all_formats.contains(&ImgFormat::IA16));
        assert!(all_formats.contains(&ImgFormat::IA8));
        assert!(all_formats.contains(&ImgFormat::IA4));
        assert!(all_formats.contains(&ImgFormat::I8));
        assert!(all_formats.contains(&ImgFormat::I4));
        assert!(all_formats.contains(&ImgFormat::CI8));
        assert!(all_formats.contains(&ImgFormat::CI4));
        assert!(all_formats.contains(&ImgFormat::OneBPP));
    }
    
    /// Test that codec names map to the correct enum
    #[test]
    fn test_format_name() {
        assert_eq!(ImgFormat::RGBA16.to_string(), "RGBA16");
        assert_eq!(ImgFormat::RGBA32.to_string(), "RGBA32");
        assert_eq!(ImgFormat::IA16.to_string(), "IA16");
        assert_eq!(ImgFormat::IA32.to_string(), "IA32");
        assert_eq!(ImgFormat::IA8.to_string(), "IA8");
        assert_eq!(ImgFormat::IA4.to_string(), "IA4");
        assert_eq!(ImgFormat::I8.to_string(), "I8");
        assert_eq!(ImgFormat::I4.to_string(), "I4");
        assert_eq!(ImgFormat::CI8.to_string(), "CI8");
        assert_eq!(ImgFormat::CI4.to_string(), "CI4");
        assert_eq!(ImgFormat::OneBPP.to_string(), "OneBPP");
    }

    #[test]
    fn test_img_format_to_usize() {
        assert_eq!(usize::from(ImgFormat::RGBA16), 0);
        assert_eq!(usize::from(ImgFormat::RGBA32), 1);
        assert_eq!(usize::from(ImgFormat::IA32), 2);
        assert_eq!(usize::from(ImgFormat::IA16), 3);
        assert_eq!(usize::from(ImgFormat::IA8), 4);
        assert_eq!(usize::from(ImgFormat::IA4), 5);
        assert_eq!(usize::from(ImgFormat::I8), 6);
        assert_eq!(usize::from(ImgFormat::I4), 7);
        assert_eq!(usize::from(ImgFormat::CI8), 8);
        assert_eq!(usize::from(ImgFormat::CI4), 9);
        assert_eq!(usize::from(ImgFormat::OneBPP), 10);
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

    /// Test creating a color from an u32
    #[test]
    fn test_from_u32() {
        // Test case 1: Fully opaque red color
        let pixel1: u32 = 0xFF0000FF;
        let color1 = Color::rgba_from_u32(pixel1);
        assert_eq!(color1.r, 0xFF);
        assert_eq!(color1.g, 0);
        assert_eq!(color1.b, 0);
        assert_eq!(color1.a, 0xFF);

        // Test case 2: Semi-transparent green color
        let pixel2: u32 = 0x00FF0080;
        let color2 = Color::rgba_from_u32(pixel2);
        assert_eq!(color2.r, 0);
        assert_eq!(color2.g, 0xFF);
        assert_eq!(color2.b, 0);
        assert_eq!(color2.a, 0x80);

        // Test case 3: Fully transparent blue color
        let pixel3: u32 = 0x00000000;
        let color3 = Color::rgba_from_u32(pixel3);
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