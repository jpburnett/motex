use crate::actions::AppAction;
use crate::palette_manager::PaletteManager;
use crate::texture_view::{ImageFormat, ScrollMode, TextureViewState};
use eframe::egui;

pub fn render_sidebar(
    ui: &mut egui::Ui,
    texture_view: &mut TextureViewState,
    palette_manager: &mut PaletteManager,
    scale: &mut f32,
    background_color: &mut egui::Color32,
    file_buffer_len: usize,
    show_pixel_info: &mut bool,
    // NEW: The actual decoded colors to show
    palette_preview: Option<&[egui::Color32]>,
) -> Vec<AppAction> {
    let mut actions = Vec::new();
    let mut changed = false;

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(4.0);

        // --- Decoding Section ---
        ui.heading("Decoding");
        egui::Grid::new("decoding_grid")
            .num_columns(2)
            .spacing([10.0, 4.0])
            .show(ui, |ui| {
                ui.label("Format:");
                egui::ComboBox::from_id_salt("format_combo")
                    .selected_text(texture_view.format.name())
                    .show_ui(ui, |ui| {
                        for &format in ImageFormat::all() {
                            if ui
                                .selectable_value(&mut texture_view.format, format, format.name())
                                .clicked()
                            {
                                changed = true;
                            }
                        }
                    });
                ui.end_row();
            });

        // Pass the preview data to the helper
        if texture_view.format.is_ci() {
            ui.add_space(8.0);
            render_palette_section(
                ui,
                texture_view,
                palette_manager,
                &mut changed,
                palette_preview,
            );
        }

        ui.separator();

        // --- Geometry Section ---
        ui.heading("Geometry");
        egui::Grid::new("geometry_grid")
            .num_columns(2)
            .spacing([10.0, 4.0])
            .show(ui, |ui| {
                ui.label("Width:");
                if ui
                    .add(
                        egui::DragValue::new(&mut texture_view.width)
                            .speed(1)
                            .range(1..=4096),
                    )
                    .changed()
                {
                    changed = true;
                }
                ui.end_row();

                ui.label("Height:");
                if ui
                    .add(
                        egui::DragValue::new(&mut texture_view.height)
                            .speed(1)
                            .range(1..=4096),
                    )
                    .changed()
                {
                    changed = true;
                }
                ui.end_row();
            });

        ui.collapsing("Presets", |ui| {
            ui.horizontal_wrapped(|ui| {
                for &w in &[8, 16, 32, 64, 128, 256, 320, 512, 640] {
                    if ui.button(format!("{}", w)).clicked() {
                        texture_view.width = w;
                        changed = true;
                    }
                }
            });
        });

        ui.separator();

        // --- Navigation Section ---
        ui.heading("Navigation");
        ui.horizontal(|ui| {
            ui.label("Offset:");
            if ui
                .add(
                    egui::DragValue::new(&mut texture_view.offset)
                        .range(0..=file_buffer_len.saturating_sub(1))
                        .hexadecimal(8, false, true),
                )
                .changed()
            {
                actions.push(AppAction::UpdateTexture);
            }
        });

        ui.group(|ui| {
            ui.horizontal(|ui| {
                if ui.small_button("Prev").clicked() {
                    actions.push(AppAction::AdjustOffset {
                        delta: -1,
                        mode: ScrollMode::Image,
                    });
                }
                if ui.small_button("Next").clicked() {
                    actions.push(AppAction::AdjustOffset {
                        delta: 1,
                        mode: ScrollMode::Image,
                    });
                }
            });
        });

        ui.separator();

        // --- Display Section ---
        ui.heading("Display");
        egui::Grid::new("display_grid").show(ui, |ui| {
            ui.label("Scale:");
            ui.add(egui::Slider::new(scale, 0.1..=10.0).text("x"));
            ui.end_row();

            ui.label("Background:");
            ui.color_edit_button_srgba(background_color);
            ui.end_row();
        });

        ui.checkbox(show_pixel_info, "Show Pixel Info");

        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("Shortcuts:\n• Arrows: Move 1 Row\n• Shift+Arrows: Move 1 Pixel")
                .weak()
                .size(11.0),
        );
    });

    if changed {
        actions.push(AppAction::UpdateTexture);
    }

    actions
}

fn render_palette_section(
    ui: &mut egui::Ui,
    texture_view: &mut TextureViewState,
    palette_manager: &mut PaletteManager,
    changed: &mut bool,
    palette_preview: Option<&[egui::Color32]>,
) {
    ui.collapsing("Palette Settings", |ui| {
        ui.label("Preview:");

        egui::Frame::dark_canvas(ui.style())
            .inner_margin(5.0)
            .show(ui, |ui| {
                if let Some(colors) = palette_preview {
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(1.0, 1.0);
                        // Show tooltips on hover for index
                        for (i, &color) in colors.iter().enumerate() {
                            let (rect, response) = ui
                                .allocate_exact_size(egui::vec2(14.0, 14.0), egui::Sense::hover());
                            ui.painter().rect_filled(rect, 1.0, color);

                            // Optional: Highlight if hovered
                            if response.hovered() {
                                ui.painter().rect_stroke(
                                    rect,
                                    1.0,
                                    egui::Stroke::new(1.0, egui::Color32::WHITE),
                                    egui::StrokeKind::Inside,
                                );
                                response.on_hover_text(format!("Index 0x{:02X}", i));
                            }
                        }
                    });
                } else {
                    ui.label("No palette data available");
                }
            });

        ui.add_space(4.0);

        // Controls
        egui::Grid::new("palette_controls").show(ui, |ui| {
            ui.label("Offset:");
            if ui
                .add(
                    egui::DragValue::new(&mut texture_view.palette_offset)
                        .range(0..=usize::MAX)
                        .hexadecimal(8, false, true),
                )
                .changed()
            {
                *changed = true;
            }
            ui.end_row();

            ui.label("Split:");
            if ui
                .checkbox(&mut texture_view.split_palette_enabled, "Enable")
                .changed()
            {
                *changed = true;
            }
            ui.end_row();
        });

        if texture_view.split_palette_enabled {
            ui.horizontal(|ui| {
                ui.label("Split Ofs:");
                if ui
                    .add(
                        egui::DragValue::new(&mut texture_view.split_palette_offset)
                            .hexadecimal(8, false, true),
                    )
                    .changed()
                {
                    *changed = true;
                }
            });
        }

        ui.separator();

        if palette_manager.has_external_palette() {
            if ui.button("Clear External").clicked() {
                palette_manager.clear_external_palette();
                *changed = true;
            }
        } else if ui.button("Load External...").clicked() {
            // Trigger action
        }
    });
}
