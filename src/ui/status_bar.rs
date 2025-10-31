use crate::app::Texture64App;
use eframe::egui;

pub fn render_status_bar(app: &Texture64App, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        // File info
        if let Some(filename) = app.file_buffer.filename() {
            ui.label(format!("📁 {}", filename));
            if app.file_buffer.is_modified() {
                ui.label("●");
            }
        } else {
            ui.label("No file loaded");
        }

        ui.separator();

        // File size
        ui.label(format!(
            "Size: {} bytes (0x{:X})",
            app.file_buffer.len(),
            app.file_buffer.len()
        ));

        ui.separator();

        // Current offset with hex
        ui.label(format!(
            "Offset: {} (0x{:08X})",
            app.texture_view.offset, app.texture_view.offset
        ));

        ui.separator();

        // Enhanced pixel info with byte offset and bit display
        if app.show_pixel_info {
            if let Some((x, y, color)) = app.hovered_pixel {
                // Calculate the byte offset for this pixel
                let pixel_index = y * app.texture_view.width as usize + x;
                let bits_per_pixel = app.texture_view.format.bits_per_pixel();
                let byte_offset = app.texture_view.offset + (pixel_index * bits_per_pixel / 8);
                let bit_offset = (pixel_index * bits_per_pixel) % 8;

                ui.separator();

                // Pixel coordinates
                ui.label(format!("Pixel: ({}, {})", x, y));

                ui.separator();

                // Byte offset with bit position if relevant
                if bits_per_pixel < 8 {
                    ui.label(format!("Byte: 0x{:X}+{}b", byte_offset, bit_offset));
                } else {
                    ui.label(format!("Byte: 0x{:X}", byte_offset));
                }

                ui.separator();

                // Color values
                ui.label(format!(
                    "RGBA: ({}, {}, {}, {})",
                    color.r(),
                    color.g(),
                    color.b(),
                    color.a()
                ));

                // Also show hex representation
                ui.label(format!(
                    "#{:02X}{:02X}{:02X}{:02X}",
                    color.r(),
                    color.g(),
                    color.b(),
                    color.a()
                ));

                ui.separator();

                // Show color swatch
                let swatch_size = egui::vec2(20.0, ui.available_height());
                let (swatch_rect, _) = ui.allocate_exact_size(swatch_size, egui::Sense::hover());
                ui.painter().rect_filled(swatch_rect, 2.0, color);
            } else {
                ui.label("Pixel: ---");
            }
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(format!(
                "{}x{} | {}",
                app.texture_view.width,
                app.texture_view.height,
                app.texture_view.format.name()
            ));
        });
    });
}
