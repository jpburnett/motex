use crate::app::Texture64App;
use crate::texture_view::ImageFormat;
use eframe::egui;

pub fn render_toolbar(app: &mut Texture64App, ui: &mut egui::Ui) {
    egui::MenuBar::new().ui(ui, |ui| {
        ui.menu_button("File", |ui| {
            if ui.button("Open... (Ctrl+O)").clicked() {
                app.ui_open_file();
                ui.close();
            }
            if ui.button("Save (Ctrl+S)").clicked() {
                let _ = app.save_file();
                ui.close();
            }
            if ui.button("Insert... (Ctrl+I)").clicked() {
                let _ = app.insert_image();
                ui.close();
            }
            ui.separator();
            if ui.button("Export to PNG...").clicked() {
                let _ = app.export_texture();
                ui.close();
            }
        });

        ui.separator();

        // Format selection
        ui.label("Format:");
        let mut changed = false;
        egui::ComboBox::from_id_salt("format_combo")
            .selected_text(app.texture_view.format.name())
            .show_ui(ui, |ui| {
                for &format in ImageFormat::all() {
                    if ui
                        .selectable_value(&mut app.texture_view.format, format, format.name())
                        .clicked()
                    {
                        changed = true;
                    }
                }
            });

        ui.separator();

        // Width control with presets
        ui.label("Width:");
        if ui
            .add(
                egui::DragValue::new(&mut app.texture_view.width)
                    .speed(1)
                    .range(1..=4096),
            )
            .changed()
        {
            changed = true;
        }

        // Width presets
        ui.horizontal(|ui| {
            if ui.small_button("8").clicked() {
                app.texture_view.width = 8;
                changed = true;
            }
            if ui.small_button("16").clicked() {
                app.texture_view.width = 16;
                changed = true;
            }
            if ui.small_button("32").clicked() {
                app.texture_view.width = 32;
                changed = true;
            }
            if ui.small_button("64").clicked() {
                app.texture_view.width = 64;
                changed = true;
            }
            if ui.small_button("128").clicked() {
                app.texture_view.width = 128;
                changed = true;
            }
            if ui.small_button("256").clicked() {
                app.texture_view.width = 256;
                changed = true;
            }
            if ui.small_button("512").clicked() {
                app.texture_view.width = 512;
                changed = true;
            }
            if ui.small_button("1024").clicked() {
                app.texture_view.width = 1024;
                changed = true;
            }
        });

        ui.separator();

        // Height control with presets
        ui.label("Height:");
        if ui
            .add(
                egui::DragValue::new(&mut app.texture_view.height)
                    .speed(1)
                    .range(1..=4096),
            )
            .changed()
        {
            changed = true;
        }

        // Height presets
        ui.horizontal(|ui| {
            if ui.small_button("8").clicked() {
                app.texture_view.height = 8;
                changed = true;
            }
            if ui.small_button("16").clicked() {
                app.texture_view.height = 16;
                changed = true;
            }
            if ui.small_button("32").clicked() {
                app.texture_view.height = 32;
                changed = true;
            }
            if ui.small_button("64").clicked() {
                app.texture_view.height = 64;
                changed = true;
            }
            if ui.small_button("128").clicked() {
                app.texture_view.height = 128;
                changed = true;
            }
            if ui.small_button("256").clicked() {
                app.texture_view.height = 256;
                changed = true;
            }
        });

        ui.separator();

        // Scale control
        ui.label("Scale:");
        ui.add(
            egui::DragValue::new(&mut app.scale)
                .speed(0.1)
                .range(0.1..=10.0),
        );

        ui.separator();

        // Background color picker
        ui.label("BG:");
        ui.color_edit_button_srgba(&mut app.background_color);

        if changed {
            app.update_texture();
        }
    });

    // Second row for palette controls (only shown for CI formats)
    if app.texture_view.format.is_ci() {
        ui.horizontal(|ui| {
            ui.label("Palette Controls:");

            ui.separator();

            ui.label("Palette Offset:");
            if ui
                .add(
                    egui::DragValue::new(&mut app.texture_view.palette_offset)
                        .speed(1)
                        .range(0..=usize::MAX)
                        .hexadecimal(8, false, true),
                )
                .changed()
            {
                app.update_texture();
            }

            ui.separator();

            let mut split_changed = false;
            if ui
                .checkbox(&mut app.texture_view.split_palette_enabled, "Split Palette")
                .changed()
            {
                split_changed = true;
            }

            if app.texture_view.split_palette_enabled {
                ui.label("Split Offset:");
                if ui
                    .add(
                        egui::DragValue::new(&mut app.texture_view.split_palette_offset)
                            .speed(1)
                            .range(0..=usize::MAX)
                            .hexadecimal(8, false, true),
                    )
                    .changed()
                {
                    split_changed = true;
                }
            }

            ui.separator();

            if app.palette_manager.has_external_palette() {
                ui.label(format!(
                    "External: {}",
                    app.palette_manager
                        .external_palette_filename()
                        .unwrap_or_default()
                ));
                if ui.button("Clear").clicked() {
                    app.palette_manager.clear_external_palette();
                    app.update_texture();
                }
            } else {
                if ui.button("Load External Palette...").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Binary files", &["bin", "pal"])
                        .pick_file()
                    {
                        let _ = app.palette_manager.load_external_palette(path);
                        app.update_texture();
                    }
                }
            }

            if split_changed {
                app.update_texture();
            }
        });
    }
}
