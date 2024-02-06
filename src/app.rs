use crate::files::file_loading::N64Image;

/// App.rs
use crate::n64_graphics::textures;
use anyhow::Result;
use eframe::egui::{self, CentralPanel, TopBottomPanel, ViewportCommand};
use motex::BinFile;
use std::path::{
    Path,
    PathBuf,
};

/// The main application struct.
///
///
pub struct Motex {
    /// The file that is opened.
    file_path: PathBuf,
    /// The selected codec.
    selected: usize,
    /// The texture to display.
    texture: egui::TextureHandle,
    /// The data from the currently open file.
    file_data: Vec<u8>,
    /// The image to display.
    image: N64Image,
}

impl Motex {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            file_path: PathBuf::default(),
            selected: 0,
            texture: cc.egui_ctx.load_texture(
                "Test Tex",
                egui::ColorImage::new([64, 64], egui::Color32::BLACK),
                Default::default(),
            ),
            file_data: vec![],
            image: N64Image::default(),
        }
    }

    /// Returns data from the currently open file.
    ///
    /// ### Arguments
    /// * `path` - The path to the file to open.
    pub fn open_file(&mut self, path: &Path) -> Result<()> {
        let selected_file = BinFile::from_path(path)?;
        self.file_data = selected_file.data;
        self.file_path = selected_file.path;
        Ok(())
    }
}

impl eframe::App for Motex {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Menu Bar
        TopBottomPanel::top("top_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            let _ = self.open_file(&path);

                            // TODO: Parker - Move this to elsewhere?...loading can grab the data
                            // self.image = N64Image::read(
                            //     &self.file_data[..],
                            //     textures::N64Codec::from_index(self.selected),
                            //     32,
                            //     32,
                            // ).unwrap();
                        }

                        ui.close_menu();
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(ViewportCommand::Close)
                    }
                });

                ui.button("About")
                    .on_hover_text("Show about dialog")
                    .clicked();
            });
        });

        // Main panel
        CentralPanel::default().show(ctx, |ui| {
            egui::ComboBox::from_label("Select Codec!").show_index(
                ui,
                &mut self.selected,
                textures::CODEC_LENGTH,
                |i| textures::codec_name(textures::N64Codec::from_index(i)),
            );

            // Right panel -- image data / preview will live here
            egui::SidePanel::right("right_panel")
                .resizable(false)
                .default_width(200.0)
                .show_inside(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Right Panel");
                        ui.label(format!("File data size: {:#X}", self.file_data.len()));
                        // TODO: Parker -- Add a function to decode data here?
                    });
                });

            let mut decoded_data: Vec<u8> = vec![];
            let _ = self.image.decode(&mut decoded_data);
            self.texture.set(
                egui::ColorImage::from_rgba_unmultiplied(
                    [self.image.width, self.image.height],
                    &decoded_data,
                ),
                Default::default(),
            );
            ui.image(&self.texture);

        });

        // Bottom panel
        TopBottomPanel::bottom("bottom_bar").show(ctx, |ui| {
            ui.label("Bottom bar");

            // If a file is open, display the path.
            if (self.file_path).exists() {
                ui.label(format!("File path: {:?}", self.file_path));
            }
        });

    }
}