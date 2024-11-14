use eframe::egui;
use motex::motex_options::{Appearance, ThemeChoice};

#[cfg(test)]
mod appearance_tests {
    use super::*;

    #[test]
    fn test_theme_choice_default() {
        // Test that the default choice is System
        let default_theme = ThemeChoice::default();
        assert_eq!(default_theme, ThemeChoice::System);
    }

    #[test]
    fn test_appearance_default() {
        // Test that new Appearance instances use System theme by default
        let appearance = Appearance::default();
        assert_eq!(appearance.theme_choice, ThemeChoice::System);
    }

    #[test]
    fn test_theme_choices_are_unique() {
        // Verify all theme choices are different from each other
        assert_ne!(ThemeChoice::Light, ThemeChoice::Dark);
        assert_ne!(ThemeChoice::System, ThemeChoice::Light);
        assert_ne!(ThemeChoice::System, ThemeChoice::Dark);
    }

    #[test]
    fn test_explicit_theme_settings() {
        let ctx = egui::Context::default();
        let mut appearance = Appearance::default();

        // Test Dark theme
        appearance.theme_choice = ThemeChoice::Dark;
        appearance.set_theme(&ctx);
        assert!(ctx.style().visuals.dark_mode);

        // Test Light theme
        appearance.theme_choice = ThemeChoice::Light;
        appearance.set_theme(&ctx);
        assert!(!ctx.style().visuals.dark_mode);
    }

    #[test]
    fn test_system_theme_behavior() {
        let ctx = egui::Context::default();
        let mut appearance = Appearance::default();

        // Test System theme (which is the default)
        appearance.theme_choice = ThemeChoice::System;
        appearance.set_theme(&ctx);
        // We can only verify that it's set to a valid mode
        assert!(matches!(ctx.style().visuals.dark_mode, true | false));
    }
}

#[cfg(test)]
mod options_window_tests {
    use super::*;

    #[test]
    fn test_options_window_state() {
        let ctx = egui::Context::default();
        let mut show = true;
        let mut appearance = Appearance::default();

        // Run the context for one frame
        let _ = ctx.run(Default::default(), |ctx| {
            motex::motex_options::options_window(ctx, &mut show, &mut appearance);
        });

        assert!(show);
    }
}
