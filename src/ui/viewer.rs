use crate::app::Texture64App;
use crate::texture_view::ScrollMode;
use eframe::egui;

pub fn render_viewer(app: &mut Texture64App, ui: &mut egui::Ui, ctx: &egui::Context) {
    // Left panel for offset controls
    egui::SidePanel::left("left_panel")
        .resizable(true)
        .default_width(200.0)
        .show_inside(ui, |ui| {
            ui.heading("Offset Control");
            ui.separator();

            ui.label("Current Offset:");
            let mut offset_changed = false;
            if ui
                .add(
                    egui::DragValue::new(&mut app.texture_view.offset)
                        .speed(1)
                        .range(0..=app.file_buffer.len().saturating_sub(1))
                        .hexadecimal(8, false, true),
                )
                .changed()
            {
                offset_changed = true;
            }

            ui.separator();

            ui.label("Quick Jump:");
            ui.horizontal(|ui| {
                if ui.button("-1000").clicked() {
                    app.texture_view
                        .adjust_offset(-1000, app.file_buffer.len(), ScrollMode::Pixel);
                    offset_changed = true;
                }
                if ui.button("+1000").clicked() {
                    app.texture_view
                        .adjust_offset(1000, app.file_buffer.len(), ScrollMode::Pixel);
                    offset_changed = true;
                }
            });
            ui.horizontal(|ui| {
                if ui.button("-100").clicked() {
                    app.texture_view
                        .adjust_offset(-100, app.file_buffer.len(), ScrollMode::Pixel);
                    offset_changed = true;
                }
                if ui.button("+100").clicked() {
                    app.texture_view
                        .adjust_offset(100, app.file_buffer.len(), ScrollMode::Pixel);
                    offset_changed = true;
                }
            });
            ui.horizontal(|ui| {
                if ui.button("-10").clicked() {
                    app.texture_view
                        .adjust_offset(-10, app.file_buffer.len(), ScrollMode::Pixel);
                    offset_changed = true;
                }
                if ui.button("+10").clicked() {
                    app.texture_view
                        .adjust_offset(10, app.file_buffer.len(), ScrollMode::Pixel);
                    offset_changed = true;
                }
            });

            ui.separator();

            ui.label("Texture Size:");
            ui.label(format!("{} bytes", app.texture_view.texture_size_bytes()));

            ui.separator();

            ui.label("Mouse Controls:");
            ui.label("• Click: Set offset to pixel");
            ui.label("• Scroll: Move by 4 rows");
            ui.label("• Shift+Scroll: Move by 1 row");
            ui.label("• Ctrl+Scroll: Move by image");
            ui.label("• Alt+Scroll: Move by pixel");

            ui.separator();

            ui.checkbox(&mut app.show_pixel_info, "Show Pixel Info");

            if offset_changed {
                app.update_texture();
            }
        });

    // Central texture viewer
    egui::CentralPanel::default().show_inside(ui, |ui| {
        let available_size = ui.available_size();

        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // Clone the RGBA data to avoid borrow issues
                if let Some(rgba_data) = app.texture_view.current_rgba.clone() {
                    render_texture_image(app, ui, ctx, &rgba_data, available_size);
                } else {
                    ui.centered_and_justified(|ui| {
                        ui.label("No file loaded. Drag and drop a file or use File > Open");
                    });
                }
            });
    });
}

fn render_texture_image(
    app: &mut Texture64App,
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    rgba_data: &[u8],
    _available_size: egui::Vec2,
) {
    let width = app.texture_view.width as usize;
    let height = app.texture_view.height as usize;
    let scale = app.scale;
    let background_color = app.background_color;

    // Create or update texture - do this first to avoid borrow issues
    if app.texture_handle.is_none() {
        let tex = ctx.load_texture(
            "texture_view",
            egui::ColorImage::from_rgba_unmultiplied([width, height], rgba_data),
            egui::TextureOptions::NEAREST,
        );
        app.texture_handle = Some(tex);
    }

    // Update existing texture
    if let Some(texture) = &mut app.texture_handle {
        if rgba_data.len() == width * height * 4 {
            texture.set(
                egui::ColorImage::from_rgba_unmultiplied([width, height], rgba_data),
                egui::TextureOptions::NEAREST,
            );
        }
    }

    // Get texture ID - extract it before the closure
    let texture_id = app.texture_handle.as_ref().map(|t| t.id());

    if let Some(texture_id) = texture_id {
        let scaled_size = egui::vec2(width as f32 * scale, height as f32 * scale);

        // Create a frame with the background color
        let frame = egui::Frame::default()
            .fill(background_color)
            .inner_margin(10.0);

        frame.show(ui, |ui| {
            let (rect, response) =
                ui.allocate_exact_size(scaled_size, egui::Sense::click_and_drag());

            // Draw the texture
            ui.painter().image(
                texture_id,
                rect,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            );

            // Handle mouse interactions
            handle_mouse_interaction(app, &response, rect, width, height);

            // Show context menu on right-click
            response.context_menu(|ui| {
                if ui.button("Export to PNG...").clicked() {
                    let _ = app.export_texture();
                    ui.close();
                }
                if ui.button("Copy to Clipboard").clicked() {
                    let _ = app.copy_to_clipboard();
                    ui.close();
                }
                ui.separator();
                if ui.button("Set Palette Offset Here").clicked() {
                    if app.texture_view.format.is_ci() {
                        app.texture_view.palette_offset = app.texture_view.offset;
                        app.update_texture();
                    }
                    ui.close();
                }
            });
        });
    }
}

fn handle_mouse_interaction(
    app: &mut Texture64App,
    response: &egui::Response,
    rect: egui::Rect,
    width: usize,
    height: usize,
) {
    // Handle left click to set offset
    if response.clicked() {
        if let Some(pos) = response.interact_pointer_pos() {
            let relative_pos = pos - rect.min;
            let x = (relative_pos.x / app.scale).floor() as usize;
            let y = (relative_pos.y / app.scale).floor() as usize;

            if x < width && y < height {
                app.texture_view.set_offset_from_click(x, y);
                app.update_texture();
            }
        }
    }

    // Handle mouse wheel scrolling
    if response.hovered() {
        let scroll_delta = response.ctx.input(|i| i.smooth_scroll_delta.y);

        if scroll_delta != 0.0 {
            let scroll_amount = if scroll_delta > 0.0 { -1 } else { 1 };

            let modifiers = response.ctx.input(|i| i.modifiers);

            let scroll_mode = if modifiers.shift {
                ScrollMode::Row
            } else if modifiers.ctrl {
                ScrollMode::Image
            } else if modifiers.alt {
                ScrollMode::Pixel
            } else {
                ScrollMode::FourRows
            };

            app.texture_view
                .adjust_offset(scroll_amount, app.file_buffer.len(), scroll_mode);
            app.update_texture();
        }
    }

    // Track hovered pixel for info display
    if let Some(pos) = response.hover_pos() {
        let relative_pos = pos - rect.min;
        let x = (relative_pos.x / app.scale).floor() as usize;
        let y = (relative_pos.y / app.scale).floor() as usize;

        if x < width && y < height {
            if let Some(rgba_data) = &app.texture_view.current_rgba {
                let pixel_index = (y * width + x) * 4;
                if pixel_index + 3 < rgba_data.len() {
                    let r = rgba_data[pixel_index];
                    let g = rgba_data[pixel_index + 1];
                    let b = rgba_data[pixel_index + 2];
                    let a = rgba_data[pixel_index + 3];

                    app.hovered_pixel =
                        Some((x, y, egui::Color32::from_rgba_unmultiplied(r, g, b, a)));
                }
            }
        }
    } else {
        app.hovered_pixel = None;
    }
}
