use anyhow::Result;
use eframe::egui::{
    self, CentralPanel, CollapsingHeader, DragValue, ScrollArea, SidePanel, TopBottomPanel,
    ViewportCommand,
};
use std::path::Path;

// Used for texture
use pigment64::ImageType;
use strum::IntoEnumIterator;

use crate::{
    bin_handler::BinFile,
    motex_options::{Appearance, options_window},
    texview::TexView,
};

#[derive(Default)]
pub struct ViewState {
    show_about: bool,
    show_options: bool,
}

/// The Motex Application.
pub struct Motex {
    appearance: Appearance,
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
    /// View state for the application.
    view_state: ViewState,
    /// Error message to display.
    error_message: Option<String>,
}

impl Motex {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut sample32_tex = TexView::new(cc, "mid_view");
        let preview_tex = TexView::new(cc, "preview_tex");

        // Initialize with a default size. The user will now control this.
        sample32_tex.width = 32;
        sample32_tex.height = 32;

        Self {
            format: ImageType::I8,
            file: BinFile::default(),
            file_pos: 0,
            sample32_tex,
            preview_tex,
            appearance: Appearance::default(),
            view_state: ViewState::default(),
            error_message: None,
        }
    }

    /// Returns data from the currently open file.
    ///
    /// ### Arguments
    /// * `path` - The path to the file to open.
    pub fn open_file(&mut self, path: &Path) -> Result<()> {
        self.file = BinFile::from_path(path)?;
        // Reset position when a new file is opened
        self.file_pos = 0;
        Ok(())
    }

    fn pre_update(&mut self, ctx: &egui::Context) {
        self.appearance.apply_appearance(ctx);
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

    fn render_color_info(&self, ui: &mut egui::Ui) {
        if let Some(color) = self.sample32_tex.hover_color {
            let (r, g, b, a) = color.to_tuple();

            ui.label(format!("Hex: #{:02X}{:02X}{:02X}{:02X}", r, g, b, a));
            ui.label(format!("R: {}", r));
            ui.label(format!("G: {}", g));
            ui.label(format!("B: {}", b));
            ui.label(format!("A: {}", a));

            let color_preview_size = egui::Vec2::new(30.0, 30.0);
            let (rect, _response) =
                ui.allocate_exact_size(color_preview_size, egui::Sense::hover());
            ui.painter().rect_filled(rect, 0.0, color);
            ui.painter()
                .rect_stroke(rect, 0.0, (1.0, egui::Color32::BLACK));
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
            self.render_central_panel_content(ui, ctx);
        });
    }

    fn render_central_panel_content(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        // Draw the texture
        if self.file.data.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.label("No image loaded. Please open a file.");
            });
        } else {
            self.sample32_tex
                .draw(&self.file.data, self.file_pos, ui, ctx);
        }
    }

    /// Renders the left panel of the application.
    /// This panel will contain the image format buttons and color information.
    /// ### Arguments
    /// * `ctx` - The egui context.
    fn render_left_panel(&mut self, ctx: &egui::Context) {
        SidePanel::left("left_panel")
            .resizable(true)
            .default_width(200.0)
            .show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    self.render_left_panel_content(ui);
                });
            });
    }

    fn render_left_panel_content(&mut self, ui: &mut egui::Ui) {
        // NEW: Add controls for texture properties
        CollapsingHeader::new("Texture Properties")
            .default_open(true)
            .show(ui, |ui| {
                egui::Grid::new("texture_properties_grid")
                    .num_columns(2)
                    .spacing([10.0, 4.0])
                    .show(ui, |ui| {
                        // --- Width Control ---
                        ui.label("Width:");
                        ui.add(
                            DragValue::new(&mut self.sample32_tex.width)
                                .speed(1.0)
                                .clamp_range(1..=1024),
                        );
                        ui.end_row();

                        // --- Height Control ---
                        ui.label("Height:");
                        ui.add(
                            DragValue::new(&mut self.sample32_tex.height)
                                .speed(1.0)
                                .clamp_range(1..=1024),
                        );
                        ui.end_row();

                        // --- Offset Control ---
                        ui.label("Offset:");
                        ui.add(
                            DragValue::new(&mut self.file_pos)
                                .speed(1.0)
                                .clamp_range(0..=self.file.data.len())
                                .hexadecimal(8, false, true),
                        );
                        ui.end_row();
                    });

                // --- Offset Stepper Buttons ---
                ui.horizontal(|ui| {
                    if ui.button("<- Row").clicked() {
                        let row_size = (self.sample32_tex.width as f32
                            * bpp_from_image_type(self.format))
                            as usize;
                        self.file_pos = self.file_pos.saturating_sub(row_size);
                    }
                    if ui.button("-").clicked() {
                        self.file_pos = self.file_pos.saturating_sub(1);
                    }
                    if ui.button("+").clicked() {
                        if self.file_pos < self.file.data.len() {
                            self.file_pos += 1;
                        }
                    }
                    if ui.button("Row ->").clicked() {
                        let row_size = (self.sample32_tex.width as f32
                            * bpp_from_image_type(self.format))
                            as usize;
                        if self.file_pos + row_size <= self.file.data.len() {
                            self.file_pos += row_size;
                        }
                    }
                });
                ui.add_space(8.0);

                // --- Zoom Controls ---
                ui.horizontal(|ui| {
                    ui.label("Zoom:");
                    if ui.button("-").clicked() && self.sample32_tex.zoom > 0.5 {
                        self.sample32_tex.zoom /= 2.0;
                    }
                    ui.label(format!("{:.2}x", self.sample32_tex.zoom));
                    if ui.button("+").clicked() && self.sample32_tex.zoom < 8.0 {
                        self.sample32_tex.zoom *= 2.0;
                    }
                    if ui.button("Reset").clicked() {
                        self.sample32_tex.zoom = 1.0;
                    }
                });
            });

        ui.separator();

        CollapsingHeader::new("Image Format")
            .default_open(true)
            .show(ui, |ui| {
                self.render_image_format_buttons(ui);
            });

        ui.separator();

        CollapsingHeader::new("Color Information")
            .default_open(true)
            .show(ui, |ui| {
                self.render_color_info(ui);
            });
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
                ScrollArea::vertical().show(ui, |ui| {
                    self.render_right_panel_content(ui, ctx);
                });
            });
    }

    fn render_right_panel_content(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        if self.file.data.is_empty() {
            ui.label("No file loaded");
            return;
        }

        // Preview with scroll bar
        ScrollArea::vertical().show(ui, |ui| {
            // Ensure the content is taller than the available height to trigger the scroll bar
            ui.set_min_height(2000.0);

            self.preview_tex.width = 128;
            self.preview_tex.height = ui.available_height() as usize - 5;
            self.preview_tex
                .draw(&self.file.data, self.file_pos, ui, ctx);
        });
    }

    /// Opens the About window and renders the contents of the window
    ///
    ///  ### Args
    /// * `ctx` - egui context
    fn show_about_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("About")
            .open(&mut self.view_state.show_about)
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

                if ui.add(egui::Button::new("Options")).clicked() {
                    self.view_state.show_options = true;
                }

                if ui.add(egui::Button::new("About")).clicked() {
                    self.view_state.show_about = true;
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
                ui.horizontal(|ui| {
                    ui.label(format!("File: {}", self.file.path.display()));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(format!("Offset: 0x{:X}", self.file_pos));
                        ui.separator();
                        ui.label(format!("Size: 0x{:X}", self.file.data.len()));
                    });
                });
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

            let scroll_delta = i.raw_scroll_delta.y;
            if scroll_delta != 0.0 {
                let scroll_dir = if scroll_delta > 0.0 { -1 } else { 1 };

                // Scroll by one row of pixels at a time
                let row_size =
                    (self.sample32_tex.width as f32 * bpp_from_image_type(self.format)) as i32;
                let new_pos = (self.file_pos as i32) + (scroll_dir * row_size);

                self.file_pos = new_pos
                    .max(0)
                    .min(self.file.data.len().saturating_sub(1) as i32)
                    as usize;
            }
        });

        // Open dropped files
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                if let Some(path) = i.raw.dropped_files[0].path.clone() {
                    let _ = self.open_file(&path);
                }
            }
        });

        self.pre_update(ctx);

        self.create_top_bar(ctx);
        self.render_left_panel(ctx);
        self.render_right_panel(ctx);
        self.render_central_panel(ctx);
        self.render_bottom_bar(ctx);

        let show_about = &mut self.view_state.show_about;
        if *show_about {
            self.show_about_window(ctx);
        }

        let show_options = &mut self.view_state.show_options;
        if *show_options {
            options_window(ctx, show_options, &mut self.appearance);
        }
    }
}

fn bpp_from_image_type(image_type: ImageType) -> f32 {
    match image_type {
        ImageType::I1 => 0.125,
        ImageType::I4 | ImageType::Ia4 | ImageType::Ci4 => 0.5,
        ImageType::I8 | ImageType::Ia8 | ImageType::Ci8 => 1.0,
        ImageType::Ia16 | ImageType::Rgba16 => 2.0,
        ImageType::Rgba32 => 4.0,
    }
}
