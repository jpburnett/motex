use anyhow::Result;
use eframe::egui::{
    self, CentralPanel, Color32, Sense, SidePanel, TopBottomPanel, Vec2, ViewportCommand,
};
use std::path::Path;

use crate::{files::bin_handler::BinFile, texview::TexView};
// Used for texture
use pigment64::ImageType;
use strum::IntoEnumIterator;

/// The Motex Application.
pub struct Motex {
    /// The selected codec.
    format: ImageType,
    /// The file that is opened.
    file: BinFile,
    /// The current position into the file.
    file_pos: usize,
    // Middle panel stuff
    sample32_tex: TexView,

    // Preview panel stuff
    preview_tex: TexView,

    hover_color: Option<egui::Color32>,

    /// Flag indicating if the About window is open, true if open, false if closed.
    show_about_open: bool,
    error_message: Option<String>,
}

impl Motex {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            format: ImageType::I8,
            file: BinFile::default(),
            file_pos: 0,
            sample32_tex: TexView::create(cc, "mid_view"),
            hover_color: None,
            preview_tex: TexView::create(cc, "preview_tex"),
            show_about_open: false,
            error_message: None,
        }
    }

    /// Returns data from the currently open file.
    ///
    /// ### Arguments
    /// * `path` - The path to the file to open.
    pub fn open_file(&mut self, path: &Path) -> Result<()> {
        self.file = BinFile::from_path(path)?;
        Ok(())
    }

    /// Renders the buttons for selecting the image format.
    ///
    /// ### Arguments
    /// * `ui` - The egui context.
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
                            if self.format == img_type {
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

    // TODO: finish implementing this function...
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

    fn update_image_format(&mut self, format: ImageType) {
        self.format = format;
        self.sample32_tex.format = format;
        self.preview_tex.format = format;
    }

    /// Renders the central "main" panel of the application.
    /// This panel will display the image that is currently open in a variety of dimensions.
    /// ### Arguments
    /// * `ctx` - The egui context.
    fn render_central_panel(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            // Display the texture for a 32 x 32 image

            self.sample32_tex.width = 32;
            self.sample32_tex.height = 32;

            self.sample32_tex
                .draw(&self.file.data, self.file_pos, ui, ctx);
        });
    }

    /// Renders the left panel of the application.
    /// This panel will contain the image format buttons and color information.
    /// ### Arguments
    /// * `ctx` - The egui context.
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

    /// Renders the right panel of the application.
    /// This panel will contain the preview of the file data as well as
    /// displaying the current file position in hexadecimal.
    /// ### Arguments
    /// * `ctx` - The egui context.
    fn render_right_panel(&mut self, ctx: &egui::Context) {
        SidePanel::right("right_panel")
            .max_width(150.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    self.render_right_panel_content(ui, ctx);
                });
            });
    }

    fn render_right_panel_content(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        if self.file.data.is_empty() {
            ui.label("No image data to display");
            return;
        }

        ui.horizontal(|ui| {
            ui.label(format!("Pos: 0x{:X}", self.file_pos));
        });

        self.preview_tex.width = 128;
        self.preview_tex.height = ui.available_height() as usize - 5;

        self.preview_tex
            .draw(&self.file.data, self.file_pos, ui, ctx);
    }

    /// Opens the About window and renders the contents of the window
    ///
    ///  ### Args
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
                Ok(()) => {}
                Err(e) => {
                    eprintln!("Failed to open file: {}", e);
                    self.error_message = Some(format!("Failed to open file: {}", e));
                }
            }
        }
    }

    /// This function is responsible for rendering the bottom bar of the application.
    /// The bar displays the path and size of the file that is open.
    ///
    /// ### Args
    /// * `ctx` - egui context
    fn render_bottom_bar(&self, ctx: &egui::Context) {
        TopBottomPanel::bottom("bottom_bar").show(ctx, |ui| {
            // If a file is open, display the path.
            if self.file.path.exists() {
                ui.label(format!(
                    "File path: {:?} - Size: 0x{:X}",
                    self.file.path,
                    self.file.data.len()
                ));
            }
        });
    }
}

impl eframe::App for Motex {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.input(|i| {
            if self.file.data.is_empty() {
                return;
            }

            // TODO we may want to just use the smooth scroll delta and normalize it somehow, rather than the shenanigans below
            let scroll_dir = match i.raw_scroll_delta.y.partial_cmp(&0.0) {
                Some(std::cmp::Ordering::Greater) => -1,
                Some(std::cmp::Ordering::Less) => 1,
                _ => 0,
            };

            // Scroll 4 lines at a time
            let scroll_factor = 4;

            self.file_pos = (self.file_pos as i32
                + (scroll_dir
                    * scroll_factor
                    * (self.preview_tex.width as f32 * bpp_from_image_type(self.preview_tex.format)) // TODO maybe we don't want to change the scroll speed based on the currently-selected format
                        as i32))
                .max(0)
                .min(self.file.data.len() as i32) as usize;
        });

        // Open dropped files
        if ctx.input(|i| !i.raw.dropped_files.is_empty()) {
            for file in ctx.input(|i| i.raw.dropped_files.clone()) {
                let _ = self.open_file(&file.path.unwrap());
            }
        }

        self.create_top_bar(ctx);

        self.render_left_panel(ctx);

        self.render_right_panel(ctx);

        self.render_central_panel(ctx);

        self.render_bottom_bar(ctx);

        self.show_about_window(ctx);
    }
}

fn bpp_from_image_type(image_type: ImageType) -> f32 {
    match image_type {
        ImageType::I1 => 0.125,
        ImageType::I4 => 0.5,
        ImageType::I8 => 1.0,
        ImageType::Ia4 => 0.5,
        ImageType::Ia8 => 1.0,
        ImageType::Ia16 => 2.0,
        ImageType::Ci4 => 0.5,
        ImageType::Ci8 => 1.0,
        ImageType::Rgba16 => 2.0,
        ImageType::Rgba32 => 4.0,
    }
}
