use eframe::egui::{self};

pub struct Appearance {
    pub theme: eframe::Theme,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            theme: eframe::Theme::Dark,
        }
    }
}

impl Appearance {
    pub fn pre_update(&mut self, ctx: &egui::Context) {
        if ctx.style().visuals.dark_mode != (self.theme == eframe::Theme::Dark) {
            let mut style = ctx.style().as_ref().clone();
            style.visuals = match self.theme {
                eframe::Theme::Dark => egui::Visuals::dark(),
                eframe::Theme::Light => egui::Visuals::light(),
            };
            ctx.set_style(style);
        }
    }
}

pub fn options_window(ctx: &egui::Context, show: &mut bool, options: &mut Appearance) {
    egui::Window::new("Options").open(show).show(ctx, |ui| {
        egui::ComboBox::from_label("Theme")
            .selected_text(format!("{:?}", options.theme))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut options.theme, eframe::Theme::Dark, "Dark");
                ui.selectable_value(&mut options.theme, eframe::Theme::Light, "Light");
            });
    });
}
