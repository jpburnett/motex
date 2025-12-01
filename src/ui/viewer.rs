use crate::actions::AppAction;
use crate::texture_view::ScrollMode;
use eframe::egui;

pub fn render_viewer(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    texture_handle: &mut Option<egui::TextureHandle>,
    current_rgba: &Option<Vec<u8>>,
    width: u32,
    height: u32,
    scale: f32,
    background_color: egui::Color32,
    hovered_pixel: &mut Option<(usize, usize, egui::Color32)>,
) -> Vec<AppAction> {
    let mut actions = Vec::new();

    // Just a scroll area for the image. No side panels.
    egui::ScrollArea::both()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if let Some(rgba_data) = current_rgba {
                render_texture_image(
                    ui,
                    ctx,
                    texture_handle,
                    rgba_data,
                    width,
                    height,
                    scale,
                    background_color,
                    hovered_pixel,
                    &mut actions,
                );
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label(
                        egui::RichText::new("Drag and drop a file\nor use File > Open")
                            .size(18.0)
                            .weak(),
                    );
                });
            }
        });

    actions
}

#[allow(clippy::too_many_arguments)]
fn render_texture_image(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    texture_handle: &mut Option<egui::TextureHandle>,
    rgba_data: &[u8],
    width: u32,
    height: u32,
    scale: f32,
    background_color: egui::Color32,
    hovered_pixel: &mut Option<(usize, usize, egui::Color32)>,
    actions: &mut Vec<AppAction>,
) {
    let w_usize = width as usize;
    let h_usize = height as usize;

    if texture_handle.is_none() {
        let tex = ctx.load_texture(
            "texture_view",
            egui::ColorImage::from_rgba_unmultiplied([w_usize, h_usize], rgba_data),
            egui::TextureOptions::NEAREST,
        );
        *texture_handle = Some(tex);
    } else if let Some(texture) = texture_handle {
        if rgba_data.len() == w_usize * h_usize * 4 {
            texture.set(
                egui::ColorImage::from_rgba_unmultiplied([w_usize, h_usize], rgba_data),
                egui::TextureOptions::NEAREST,
            );
        }
    }

    let texture_id = texture_handle.as_ref().map(|t| t.id());

    if let Some(texture_id) = texture_id {
        let scaled_size = egui::vec2(width as f32 * scale, height as f32 * scale);

        let frame = egui::Frame::default()
            .fill(background_color)
            .inner_margin(20.0) // More breathing room
            .shadow(egui::Shadow::default()); // Nice drop shadow

        frame.show(ui, |ui| {
            let (rect, response) =
                ui.allocate_exact_size(scaled_size, egui::Sense::click_and_drag());

            ui.painter().image(
                texture_id,
                rect,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            );

            handle_interactions(
                ui,
                &response,
                rect,
                w_usize,
                h_usize,
                scale,
                rgba_data,
                hovered_pixel,
                actions,
            );
        });
    }
}

fn handle_interactions(
    _ui: &egui::Ui,
    response: &egui::Response,
    rect: egui::Rect,
    width: usize,
    height: usize,
    scale: f32,
    rgba_data: &[u8],
    hovered_pixel: &mut Option<(usize, usize, egui::Color32)>,
    actions: &mut Vec<AppAction>,
) {
    response.context_menu(|ui| {
        if ui.button("Export to PNG...").clicked() {
            actions.push(AppAction::ExportTextureDialog);
            ui.close();
        }
        if ui.button("Copy to Clipboard").clicked() {
            actions.push(AppAction::CopyToClipboard);
            ui.close();
        }
    });

    // Handle Scroll
    if response.hovered() {
        let scroll_delta = response.ctx.input(|i| i.smooth_scroll_delta.y);
        if scroll_delta != 0.0 {
            let scroll_amount = if scroll_delta > 0.0 { -1 } else { 1 };
            let modifiers = response.ctx.input(|i| i.modifiers);

            let mode = if modifiers.shift {
                ScrollMode::Row
            } else if modifiers.ctrl {
                ScrollMode::Image
            } else if modifiers.alt {
                ScrollMode::Pixel
            } else {
                ScrollMode::FourRows
            };

            actions.push(AppAction::AdjustOffset {
                delta: scroll_amount,
                mode,
            });
        }
    }

    // Hover Info
    if let Some(pos) = response.hover_pos() {
        let relative_pos = pos - rect.min;
        let x = (relative_pos.x / scale).floor() as usize;
        let y = (relative_pos.y / scale).floor() as usize;

        if x < width && y < height {
            let pixel_index = (y * width + x) * 4;
            if pixel_index + 3 < rgba_data.len() {
                *hovered_pixel = Some((
                    x,
                    y,
                    egui::Color32::from_rgba_unmultiplied(
                        rgba_data[pixel_index],
                        rgba_data[pixel_index + 1],
                        rgba_data[pixel_index + 2],
                        rgba_data[pixel_index + 3],
                    ),
                ));
            }
        }
    } else {
        *hovered_pixel = None;
    }
}
