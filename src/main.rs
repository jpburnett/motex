#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod files;
mod motex_options;

use app::Motex;

fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        ..Default::default()
    };

    if let Err(e) = eframe::run_native(
        "Motex",
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(Motex::new(cc)))
        }),
    ) {
        eprintln!("Error running motex: {}", e);
    }
}
