use crate::actions::AppAction;
use crate::app::Texture64App;
use eframe::egui;

mod sidebar;
mod status_bar;
mod toolbar;
mod viewer;

pub fn render_ui(
    app: &mut Texture64App,
    ctx: &egui::Context,
    palette_preview: Option<&[egui::Color32]>,
) -> Vec<AppAction> {
    let mut actions = Vec::new();

    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        let toolbar_actions = toolbar::render_toolbar(ui);
        actions.extend(toolbar_actions);
    });

    egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
        status_bar::render_status_bar(app, ui);
    });

    egui::SidePanel::left("inspector_panel")
        .resizable(true)
        .default_width(240.0)
        .show(ctx, |ui| {
            let sidebar_actions = sidebar::render_sidebar(
                ui,
                &mut app.texture_view,
                &mut app.palette_manager,
                &mut app.scale,
                &mut app.background_color,
                app.file_buffer.len(),
                &mut app.show_pixel_info,
                palette_preview,
            );
            actions.extend(sidebar_actions);
        });

    egui::CentralPanel::default().show(ctx, |ui| {
        let viewer_actions = viewer::render_viewer(
            ui,
            ctx,
            &mut app.texture_handle,
            &app.texture_view.current_rgba,
            app.texture_view.width,
            app.texture_view.height,
            app.scale,
            app.background_color,
            &mut app.hovered_pixel,
        );
        actions.extend(viewer_actions);
    });

    actions
}
