/// App.rs
use anyhow::Result;
use eframe::egui::{
    self, CentralPanel, Color32, Sense, SidePanel, TextureHandle, TopBottomPanel, Vec2,
    ViewportCommand,
};
use open;
use std::path::{Path, PathBuf};

use crate::files::bin_handler::BinFile;
// Used for texture
use pigment64::{ImageType, NativeImage};
use strum::IntoEnumIterator;

/// The main application struct.
pub struct Motex {
    /// The selected codec.
    selected: ImageType,
    /// The texture to display.
    texture: TextureHandle,
    /// The file that is opened.
    file_path: PathBuf,
    /// The data from the currently open file.
    file_data: Vec<u8>,
    image: NativeImage,
    hover_color: Option<egui::Color32>,

    /// Flag indicating if the About window is open, true if open, false if closed.
    show_about_open: bool,
    error_message: Option<String>,
}

impl Motex {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            file_path: PathBuf::default(),
            selected: ImageType::I1,
            texture: cc.egui_ctx.load_texture(
                "Test Tex",
                egui::ColorImage::new([64, 64], egui::Color32::WHITE),
                Default::default(),
            ),
            file_data: vec![],
            image: NativeImage {
                format: ImageType::I1,
                width: 0,
                height: 0,
                data: vec![],
            },
            hover_color: None,
            show_about_open: false,
            error_message: None,
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

    fn render_image_format_buttons(&mut self, ui: &mut egui::Ui) {
        ui.heading("Image Formats");

        let button_size = egui::vec2(50.0, 30.0);
        let highlight_color = egui::Color32::from_rgb(0, 100, 255);

        egui::Grid::new("image_format_grid")
            .num_columns(4) // Adjust this number to change the number of columns
            .spacing([4.0, 4.0])
            .show(ui, |ui| {
                for (index, img_type) in ImageType::iter().enumerate() {
                    let button = ui.add_sized(
                        button_size,
                        egui::Button::new(format!("{:?}", img_type)).fill(
                            if self.selected == img_type {
                                highlight_color
                            } else {
                                egui::Color32::TRANSPARENT
                            },
                        ),
                    );

                    if button.clicked() {
                        self.update_image_format(img_type);
                    }

                    if (index + 1) % 4 == 0 {
                        ui.end_row();
                    }
                }
            });
    }

    fn render_color_info(&self, ui: &mut egui::Ui) {
        ui.heading("Color Info");

        if let Some(color) = self.hover_color {
            let (r, g, b, a) = color.to_tuple();

            // Display hex representation
            ui.label(format!("Hex: #{:02X}{:02X}{:02X}{:02X}", r, g, b, a));

            // Display individual R, G, B, A values
            ui.label(format!("R: {}", r));
            ui.label(format!("G: {}", g));
            ui.label(format!("B: {}", b));
            ui.label(format!("A: {}", a));

            // Display color preview
            let color_preview_size = Vec2::new(30.0, 30.0);
            let (rect, _response) = ui.allocate_exact_size(color_preview_size, Sense::hover());
            ui.painter().rect_filled(rect, 0.0, color);

            // Optional: Add a border around the color preview
            ui.painter().rect_stroke(rect, 0.0, (1.0, Color32::BLACK));
        } else {
            ui.label("Hover over the image to see color info");
        }
    }

    fn update_image_format(&mut self, img_type: ImageType) {
        self.selected = img_type;
        self.image.format = img_type;
        println!("Option Selected: {:?}", img_type);
    }

    fn render_left_panel(&mut self, ctx: &egui::Context) {
        SidePanel::left("left_panel")
            .resizable(false)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    self.render_left_panel_content(ui);
                });
            });
    }

    fn render_left_panel_content(&mut self, ui: &mut egui::Ui) {
        self.render_image_format_buttons(ui);
        self.render_color_info(ui);
    }

    fn render_right_panel(&self, ctx: &egui::Context) {
        SidePanel::right("right_panel")
            .resizable(false)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Right Panel");
                    self.render_right_panel_content(ui);
                });
            });
    }

    fn render_right_panel_content(&self, ui: &mut egui::Ui) {
        ui.label(format!("File data size: {:#X}", self.file_data.len()));
        // TODO: Add a function to decode data here...
    }

    /// Opens the About window and renders the contents of the window
    ///
    ///  ### Args
    /// * `self` - Motex struct
    /// * `ctx` - egui context
    fn show_about_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("About")
            .open(&mut self.show_about_open)
            .default_open(true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.image(egui::include_image!("../assets/purplefrog-bg-512.png"));
                    ui.add_space(20.0);
                    ui.label("Motex");
                    ui.label(format!("Version: {}", env!("CARGO_PKG_VERSION")));
                    ui.add_space(10.0);
                    ui.label("© 2024 Ampier / Decompals");
                    if ui.link("GitHub Repo").clicked() {
                        if let Err(e) = open::that("https://github.com/jpburnett/motex") {
                            eprintln!("Failed to open URL: {}", e);
                            println!("Opening URL in browser");
                        }
                    }
                });
            });
    }

    /// Creates and displays the top bar menu of the application.
    ///  ### Args
    /// * `self` - Motex struct
    /// * `ctx` - egui context
    fn create_top_bar(&mut self, ctx: &egui::Context) {
        TopBottomPanel::top("top_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.add(egui::Button::new("Open")).clicked() {
                        self.open_file_dialog();
                        ui.close_menu();
                    }
                    if ui.add(egui::Button::new("Quit")).clicked() {
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                });

                if ui.add(egui::Button::new("About")).clicked() {
                    self.show_about_open = true;
                }
            });
        });
    }

    fn open_file_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            match self.open_file(&path) {
                Ok(()) => {
                    match NativeImage::read(&self.file_data[..], self.selected, 32, 32) {
                        Ok(image) => {
                            self.image = image;
                            self.error_message = None; // Clear any previous error
                        }
                        Err(e) => {
                            eprintln!("Failed to read image: {}", e);
                            self.error_message = Some(format!("Failed to read image: {}", e));
                            // Optionally, there could be a blank image here?
                            // self.image = NativeImage {
                            //     format: ImageType::default(), // Assuming ImageType has a default
                            //     width: 32,
                            //     height: 32,
                            //     data: vec![0; 32 * 32 * 4], // Assuming RGBA
                            // };
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to open file: {}", e);
                    self.error_message = Some(format!("Failed to open file: {}", e));
                }
            }
        }
    }
}

impl eframe::App for Motex {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Menu Bar
        self.create_top_bar(ctx);

        self.render_left_panel(ctx);

        self.render_right_panel(ctx);

        // Main panel
        CentralPanel::default().show(ctx, |ui| {
            // Display the texture for a 32 x 32 image
            let mut decoded_data: Vec<u8> = vec![];
            let _ = self.image.decode(&mut decoded_data, None);
            self.texture.set(
                egui::ColorImage::from_rgba_unmultiplied(
                    [self.image.width as usize, self.image.height as usize],
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
            if self.file_path.exists() {
                ui.label(format!("File path: {:?}", self.file_path));
            }
        });

        // About window
        self.show_about_window(ctx);
    }
}
