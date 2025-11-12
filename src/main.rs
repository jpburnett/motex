use gpui::AppContext;
use gpui::{App, Application, Bounds, WindowBounds, WindowOptions, px, size};
use gpui_component::theme::Theme as GpuiTheme;

mod app;
mod io;
mod ui;

use app::TextureViewerApp;
use ui::theme::Theme;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Starting Texture Viewer application");

    Application::new().run(|cx: &mut App| {
        cx.set_global(GpuiTheme::default());
        cx.set_global(Theme::new());

        let bounds = Bounds::centered(None, size(px(1200.0), px(800.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("Texture Viewer".into()),
                    appears_transparent: true,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| TextureViewerApp::new()),
        )
        .unwrap();
    });
}
