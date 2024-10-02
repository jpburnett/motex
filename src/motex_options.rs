use eframe::egui::{self, Theme};

pub struct Appearance {
    pub theme_pref: Theme,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            theme_pref: Theme::Dark,
        }
    }
}

impl Appearance {
    pub fn pre_update(&mut self, ctx: &egui::Context) {
        self.set_theme(ctx);
    }

    fn set_theme(&self, ctx: &egui::Context) {
        // Use egui::Theme directly for simplicity
        match self.theme_pref {
            egui::Theme::Dark => ctx.set_visuals(egui::Visuals::dark()),
            egui::Theme::Light => ctx.set_visuals(egui::Visuals::light()),
        }
    }
}

pub fn options_window(ctx: &egui::Context, show: &mut bool, options: &mut Appearance) {
    egui::Window::new("Options").open(show).show(ctx, |ui| {
        ui.label("Theme"); // Place the label above the ComboBox
        egui::ComboBox::from_id_salt("theme_selection")
            .selected_text(match options.theme_pref {
                Theme::Dark => "Dark".to_string(),
                Theme::Light => "Light".to_string(),
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut options.theme_pref, Theme::Dark, "Dark");
                ui.selectable_value(&mut options.theme_pref, Theme::Light, "Light");
            });
    });
}
