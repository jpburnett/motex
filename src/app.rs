/// App.rs
use crate::n64_graphics::textures;
use eframe::egui::{self, CentralPanel};
pub struct Motex {
    selected: usize,
    texture: egui::TextureHandle,
}

impl Motex {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            selected: 0,
            texture: cc.egui_ctx.load_texture(
                "Test Tex",
                egui::ColorImage::new([64, 64], egui::Color32::BLACK),
                Default::default(),
            ),
        }
    }
}

impl eframe::App for Motex {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Top menu bar
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        // TODO: Open a file...
                    }
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            });
        });
        CentralPanel::default().show(ctx, |ui| {
            ui.label("A shorter and more convenient way to add a label.");
            if ui.button("Click me").clicked() {
                // take some action here
            }
            egui::ComboBox::from_label("Select Codec!").show_index(
                ui,
                &mut self.selected,
                textures::CODEC_LENGTH,
                |i| textures::codec_name(textures::N64Codec::from_index(i)),
            );

            // Drawing a window
            egui::Window::new("Wow window!")
                .resizable(true)
                .show(ctx, |ui| {
                    ui.label("This is a window");
                    ui.label("You can drag the white space");
                    ui.label("You can resize from the bottom-right corner");
                    ui.label("You can close it with the X-button");
                    ui.image(&self.texture);
                });
        });
    }
}
