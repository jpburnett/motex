use std::iter;

use eframe::egui::{self, Color32, ColorImage, Sense, TextureHandle, TextureOptions};
use pigment64::{ImageType, NativeImage};

pub struct TexView {
    pub format: ImageType,
    pub width: usize,
    pub height: usize,
    pub zoom: f32, // Add zoom field
    pub bg_color: Color32,
    pub bg_tex: TextureHandle,
    pub tex: TextureHandle,
    pub hover_color: Option<Color32>,
}

impl TexView {
    pub fn new(cc: &eframe::CreationContext<'_>, tex_name: &str) -> Self {
        Self {
            format: ImageType::I8,
            width: 0,
            height: 0,
            zoom: 1.0, // Initialize zoom to 1.0
            bg_color: Color32::from_rgba_premultiplied(0, 0, 0, 255),
            bg_tex: cc.egui_ctx.load_texture(
                tex_name.to_owned() + "_bg",
                egui::ColorImage::new([1, 1], egui::Color32::WHITE),
                Default::default(),
            ),
            tex: cc.egui_ctx.load_texture(
                tex_name,
                egui::ColorImage::new([1, 1], egui::Color32::WHITE),
                Default::default(),
            ),
            hover_color: Some(Color32::from_rgba_premultiplied(0, 0, 0, 0)),
        }
    }

    pub fn draw(&mut self, data: &[u8], offset: usize, ui: &mut egui::Ui, ctx: &egui::Context) {
        if offset > data.len() {
            return;
        }

        let siz: usize = self.width * self.height * 4;

        // Create a black background
        let bg_data: Vec<u8> = std::iter::repeat(self.bg_color.to_array())
            .flatten()
            .take(siz)
            .collect();
        self.bg_tex.set(
            data_to_color_image(self.width, self.height, bg_data.as_slice()),
            TextureOptions::NEAREST, // Use nearest neighbor filtering for the background
        );

        let ni: NativeImage = NativeImage::read(
            &data[offset..],
            self.format,
            self.width as u32,
            self.height as u32,
        )
        .unwrap();

        let mut decoded_data: Vec<u8> = vec![];
        let _ = ni.decode(&mut decoded_data, None);

        let img_data = pad_to_length(decoded_data, siz);
        let img = data_to_color_image(self.width, self.height, img_data.as_slice());

        // Use NEAREST filtering for crisp pixels
        let tex_options = TextureOptions {
            magnification: egui::TextureFilter::Nearest,
            minification: egui::TextureFilter::Nearest,
            ..Default::default()
        };
        self.tex.set(img, tex_options);

        // Apply zoom to the texture size
        let zoomed_size = egui::vec2(
            self.width as f32 * self.zoom,
            self.height as f32 * self.zoom,
        );

        // Create a group to contain the image
        egui::Frame::none().fill(self.bg_color).show(ui, |ui| {
            // Use a fixed size area that matches our zoomed dimensions
            let (res, painter) = ui.allocate_painter(zoomed_size, Sense::hover());

            // Draw the texture scaled to our zoomed size
            painter.image(
                self.tex.id(),
                res.rect,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                Color32::WHITE,
            );

            // Handle hover detection
            if let Some(cursor_pos) = ctx.input(|i| i.pointer.hover_pos()) {
                if res.rect.contains(cursor_pos) {
                    let relative_pos = cursor_pos - res.rect.min;
                    // Adjust pixel calculation based on zoom
                    let pixel_x = ((relative_pos.x / zoomed_size.x) * self.width as f32) as usize;
                    let pixel_y = ((relative_pos.y / zoomed_size.y) * self.height as f32) as usize;
                    let index = (pixel_y * self.width + pixel_x) * 4;

                    if index + 3 < img_data.len() {
                        let r = img_data[index];
                        let g = img_data[index + 1];
                        let b = img_data[index + 2];
                        let a = img_data[index + 3];
                        self.hover_color = Some(Color32::from_rgba_premultiplied(r, g, b, a));
                    }
                }
            }
        });
    }

    pub fn update_dimensions(&mut self, format: ImageType, data_size: usize) {
        // Calculate reasonable dimensions based on format and available data
        let bpp = match format {
            ImageType::I1 => 1,
            ImageType::I4 | ImageType::Ia4 | ImageType::Ci4 => 4,
            ImageType::I8 | ImageType::Ia8 | ImageType::Ci8 => 8,
            ImageType::Ia16 | ImageType::Rgba16 => 16,
            ImageType::Rgba32 => 32,
        };

        // Calculate maximum square dimensions that would fit in the data
        let max_pixels = (data_size * 8) / bpp;
        let max_square = (max_pixels as f64).sqrt().floor() as usize;

        // Set dimensions to power of 2, max 64x64
        self.width = max_square.min(64).next_power_of_two();
        self.height = self.width;
    }
}

// Helper function to convert raw image data to egui ColorImage
fn data_to_color_image(width: usize, height: usize, data: &[u8]) -> ColorImage {
    assert!(data.len() >= width * height * 4);
    let _pixels: Vec<Color32> = data
        .chunks_exact(4)
        .map(|chunk| Color32::from_rgba_unmultiplied(chunk[0], chunk[1], chunk[2], chunk[3]))
        .collect();
    ColorImage::from_rgba_unmultiplied([width, height], data)
}

// TODO this can probably be implemented better and avoid allocating a Vec or something
pub fn pad_to_length(data: Vec<u8>, length: usize) -> Vec<u8> {
    match data.len() < length {
        true => {
            let mut ret: Vec<u8> = data.clone();
            ret.extend(iter::repeat(0).take(length - data.len()));

            ret
        }
        false => data[..length].to_vec(),
    }
}
