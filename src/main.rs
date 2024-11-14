#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use motex::app::Motex;
fn main() {
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
