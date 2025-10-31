mod app;
mod file_buffer;
mod palette_manager;
mod texture_view;
mod ui;

use eframe::egui;

fn main() -> eframe::Result<()> {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };

    eframe::run_native(
        "Motex - N64 Texture Viewer",
        native_options,
        Box::new(|cc| Ok(Box::<app::Texture64App>::default())),
    )
}
