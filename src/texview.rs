use std::iter;

use eframe::egui::{self, Color32, ColorImage, Image, TextureHandle, TextureOptions};
use pigment64::{ImageType, NativeImage};

pub struct TexView {
    pub format: ImageType,
    pub width: usize,
    pub height: usize,
    pub bg_color: Color32,
    pub bg_tex: TextureHandle,
    pub tex: TextureHandle,
}

impl TexView {
    pub fn create(cc: &eframe::CreationContext<'_>, tex_name: &str) -> TexView {
        TexView {
            format: ImageType::I8,
            width: 0,
            height: 0,
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
        }
    }
    /// Draw the texture with the given data.
    pub fn draw(&mut self, data: &[u8], offset: usize, ui: &mut egui::Ui) {
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
            TextureOptions::LINEAR,
        );

        // Draw bg and save rect for actual texture location
        let bg_loc = ui.image(&self.bg_tex);

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
        self.tex.set(img, TextureOptions::LINEAR);

        // Put the texture directly over the background
        ui.put(bg_loc.rect, Image::new(&self.tex));
    }
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

/// Convert raw RGBA data to a `ColorImage`, adjusting the color values for color accuracy.
pub fn data_to_color_image(width: usize, height: usize, data: &[u8]) -> ColorImage {
    ColorImage {
        pixels: {
            data.chunks_exact(4)
                .map(|p| {
                    let r = p[0] as u32;
                    let g = p[1] as u32;
                    let b = p[2] as u32;
                    let a = p[3] as u32;

                    Color32::from_rgba_premultiplied(
                        ((r * a + 128) / 256) as u8,
                        ((g * a + 128) / 256) as u8,
                        ((b * a + 128) / 256) as u8,
                        a as u8,
                    )
                })
                .collect()
        },
        size: [width, height],
    }
}
