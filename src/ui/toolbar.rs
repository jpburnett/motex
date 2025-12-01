use crate::actions::AppAction;
use eframe::egui;

pub fn render_toolbar(ui: &mut egui::Ui) -> Vec<AppAction> {
    let mut actions = Vec::new();

    egui::MenuBar::new().ui(ui, |ui| {
        ui.menu_button("File", |ui| {
            if ui.button("Open...").clicked() {
                actions.push(AppAction::OpenFileDialog);
                ui.close();
            }
            if ui.button("Save").clicked() {
                actions.push(AppAction::SaveFile);
                ui.close();
            }
            ui.separator();
            if ui.button("Insert Image...").clicked() {
                actions.push(AppAction::InsertImageDialog);
                ui.close();
            }
            if ui.button("Export Image...").clicked() {
                actions.push(AppAction::ExportTextureDialog);
                ui.close();
            }
            ui.separator();
            if ui.button("Quit").clicked() {
                actions.push(AppAction::Quit);
                ui.close();
            }
        });

        ui.menu_button("View", |ui| {
            if ui.button("Zoom In").clicked() {
                // Logic could be handled in AppAction::ZoomIn
                ui.close();
            }
            if ui.button("Zoom Out").clicked() {
                // Logic could be handled in AppAction::ZoomOut
                ui.close();
            }
        });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(egui::RichText::new("Motex v0.1").weak().size(10.0));
        });
    });

    actions
}
