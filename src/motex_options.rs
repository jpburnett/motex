use eframe::egui::{self, Theme, Visuals};

/// Represents the available theme choices for the application.
/// Light and Dark are explicit choices, while System will
/// default to the system theme.
#[derive(Debug, Clone, PartialEq)]
pub enum ThemeChoice {
    Light,
    Dark,
    System,
}

impl Default for ThemeChoice {
    fn default() -> Self {
        // default to System theme
        Self::System
    }
}

/// Manages the visual appearance settings for the application.
pub struct Appearance {
    /// The current theme choice for the application.
    pub theme_choice: ThemeChoice,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            theme_choice: ThemeChoice::default(),
        }
    }
}

impl Appearance {
    /// Applies the appearance settings to the egui context.
    ///
    /// # Arguments
    /// * `ctx` - The egui context to apply the appearance settings to.
    pub fn apply_appearance(&mut self, ctx: &egui::Context) {
        self.set_theme(ctx);
    }

    /// Updates the application theme based on the current theme choice.
    /// Defaults to Dark theme if system theme can't be detected.
    ///
    /// # Arguments
    /// * `ctx` - The egui context to apply the appearance settings to.
    pub fn set_theme(&self, ctx: &egui::Context) {
        let visuals = match self.theme_choice {
            ThemeChoice::Light => Visuals::light(),
            ThemeChoice::Dark => Visuals::dark(),
            ThemeChoice::System => {
                // Default to dark theme if system theme can't be detected
                if ctx.system_theme() != Some(Theme::Light) {
                    Visuals::dark()
                } else {
                    Visuals::light()
                }
            }
        };
        ctx.set_visuals(visuals);
    }
}

/// Displays the options window containing appearance settings.
///
/// # Arguments
/// * `ctx` - The egui context
/// * `show` - Mutable reference to control window visibility
/// * `options` - Mutable reference to the appearance settings
///
/// # Example
/// ```
/// use eframe::egui;
/// use motex::motex_options::{Appearance, options_window};  // Add this import
///
/// let ctx = egui::Context::default();
/// let mut show_options = true;
/// let mut appearance = Appearance::default();
/// let _ = ctx.run(Default::default(), |ctx| {
///     options_window(ctx, &mut show_options, &mut appearance);
/// });
/// ```
pub fn options_window(ctx: &egui::Context, show: &mut bool, options: &mut Appearance) {
    egui::Window::new("Options").open(show).show(ctx, |ui| {
        ui.label("Theme");
        egui::ComboBox::from_id_salt("theme_selection")
            .selected_text(match options.theme_choice {
                ThemeChoice::Dark => "Dark",
                ThemeChoice::Light => "Light",
                ThemeChoice::System => "System Default",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut options.theme_choice,
                    ThemeChoice::System,
                    "System Default",
                );
                ui.selectable_value(&mut options.theme_choice, ThemeChoice::Dark, "Dark");
                ui.selectable_value(&mut options.theme_choice, ThemeChoice::Light, "Light");
            });
    });
}
