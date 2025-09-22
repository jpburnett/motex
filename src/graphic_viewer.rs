use eframe::egui::{self, Color32, Context, Painter, Rect, Vec2};
use pigment64::{ImageType, NativeImage};

pub struct GraphicsViewer {
    data: Vec<u8>,
    palette: Vec<u8>,
    offset: usize,
    auto_pixel_size: bool,
    pix_size: Vec2,
    pix_scale: f32,
    codec: ImageType,
    mode: ImageType,
}

impl GraphicsViewer {
    pub fn new(width: f32, height: f32, codec: ImageType, mode: ImageType) -> Self {
        Self {
            data: Vec::new(),
            palette: Vec::new(),
            offset: 0,
            auto_pixel_size: false,
            pix_size: Vec2::new(width, height),
            pix_scale: 1.0,
            codec,
            mode,
        }
    }

    pub fn set_data(&mut self, raw_data: Vec<u8>) {
        self.data = raw_data;
    }

    pub fn set_palette(&mut self, raw_palette: Vec<u8>) {
        self.palette = raw_palette;
    }

    pub fn set_offset(&mut self, offset: usize) {
        if self.offset != offset {
            self.offset = offset;
        }
    }

    pub fn get_pixel_height(&self) -> f32 {
        if self.auto_pixel_size {
            self.pix_size.y / self.pix_scale
        } else {
            self.pix_size.y
        }
    }

    pub fn get_pixel_width(&self) -> f32 {
        if self.auto_pixel_size {
            self.pix_size.x / self.pix_scale
        } else {
            self.pix_size.x
        }
    }

    pub fn ui(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ui.available_rect_before_wrap();
            let painter = ui.painter();
            self.paint(painter, rect);
        });
    }

    fn paint(&self, painter: &Painter, rect: Rect) {
        if !self.data.is_empty() {
            let image = NativeImage {
                format: self.codec,
                width: self.get_pixel_width() as u32,
                height: self.get_pixel_height() as u32,
                data: self.data.clone(),
            };

            // Implement render_texture here or in a separate function
            // image.render_texture(painter, &self.palette, self.offset, self.pix_scale, self.mode);
        }

        painter.rect_stroke(rect, 0.0, (1.0, Color32::BLACK));
    }
}
