// mod.rs

use crate::app::Texture64App;
use eframe::egui;

mod status_bar;
mod toolbar;
mod viewer;

pub fn render_ui(app: &mut Texture64App, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        toolbar::render_toolbar(app, ui);
    });

    egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
        status_bar::render_status_bar(app, ui);
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        viewer::render_viewer(app, ui, ctx);
    });
}
