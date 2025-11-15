use gpui::{
    Context, InteractiveElement, IntoElement, MouseButton, ParentElement, Render, Styled, Window,
    div, prelude::FluentBuilder, px,
};
use gpui_component::TitleBar;

use crate::io::loader::LoadedFile;
use crate::ui::{canvas::Canvas, inspector::Inspector, theme::Theme};

/// Main application state - orchestrates the layout
pub struct TextureViewerApp {
    // UI state
    inspector_open: bool,
    inspector_width: f32,
    menu_open: bool,

    // App state
    current_file: Option<LoadedFile>,
    _zoom_level: f32,
}

impl TextureViewerApp {
    pub fn new() -> Self {
        log::info!("Initializing TextureViewerApp");
        Self {
            inspector_open: true,
            inspector_width: 300.0,
            menu_open: false,
            current_file: None,
            _zoom_level: 1.0,
        }
    }

    fn toggle_inspector(
        &mut self,
        _event: &gpui::MouseDownEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        self.inspector_open = !self.inspector_open;
        log::info!(
            "Inspector toggled: {}",
            if self.inspector_open {
                "open"
            } else {
                "closed"
            }
        );
    }

    fn toggle_menu(
        &mut self,
        _event: &gpui::MouseDownEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        self.menu_open = !self.menu_open;
        log::info!(
            "Menu toggled: {}",
            if self.menu_open { "open" } else { "closed" }
        );
    }

    fn open_file(
        &mut self,
        _event: &gpui::MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        log::info!("Open file clicked");

        match crate::io::loader::open_file_dialog() {
            Ok(loaded_file) => {
                log::info!(
                    "File loaded: {} ({} bytes)",
                    loaded_file.name,
                    loaded_file.size
                );
                self.current_file = Some(loaded_file);
                self.menu_open = false;
                cx.notify();
            }
            Err(e) => {
                log::error!("Failed to open file: {}", e);
            }
        }
    }
}

impl Render for TextureViewerApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let inspector_width = if self.inspector_open {
            self.inspector_width
        } else {
            0.0
        };

        // Display file name if loaded
        let file_name = self
            .current_file
            .as_ref()
            .map(|f| f.name.clone())
            .unwrap_or_else(|| "Texture Viewer".to_string());

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.background)
            .child(
                // Custom Title Bar
                TitleBar::new().child(
                    div()
                        .flex()
                        .items_center()
                        .h(px(36.0))
                        .w_full()
                        .bg(theme.surface)
                        .border_b_1()
                        .border_color(theme.border)
                        .child(
                            // Hamburger Menu Button
                            div()
                                .ml_2()
                                .px_3()
                                .py_1()
                                .rounded_md()
                                .bg(theme.surface)
                                .text_sm()
                                .text_color(theme.text)
                                .cursor_pointer()
                                .hover(|style| style.bg(theme.hover))
                                .on_mouse_down(
                                    gpui::MouseButton::Left,
                                    cx.listener(Self::toggle_menu),
                                )
                                .child("☰"),
                        )
                        .child(
                            // Centered Title Text
                            div().flex_1().flex().justify_center().items_center().child(
                                div()
                                    .text_sm()
                                    .text_color(theme.text_muted)
                                    .child(file_name),
                            ),
                        )
                        .child(
                            // Placeholder for window control buttons
                            div().w(px(60.0)).mr_2(),
                        ),
                ),
            )
            .child(
                // Main Content Area
                div()
                    .flex()
                    .flex_1()
                    .child(
                        // Canvas region
                        div()
                            .flex()
                            .flex_1()
                            .relative()
                            .child(Canvas::new(theme, self.current_file.as_ref()))
                            // Zoom Controls
                            .child(
                                div()
                                    .absolute()
                                    .top_4()
                                    .right_4()
                                    .flex()
                                    .gap_2()
                                    .items_center()
                                    .h(px(38.0))
                                    .px_3()
                                    .bg(theme.surface)
                                    .border_1()
                                    .border_color(theme.border)
                                    .rounded_md()
                                    .child(
                                        div().text_sm().text_color(theme.text_muted).child("100%"),
                                    )
                                    .child(
                                        div()
                                            .px_3()
                                            .py_1()
                                            .rounded_md()
                                            .bg(theme.surface)
                                            .text_sm()
                                            .text_color(theme.text)
                                            .cursor_pointer()
                                            .hover(|style| style.bg(theme.hover))
                                            .child("Fit"),
                                    ),
                            )
                            // Collapsible File Menu
                            .child(div().when(self.menu_open, |this| {
                                this.absolute()
                                    .top_0()
                                    .left_0()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .w(px(180.0))
                                    .p_2()
                                    .bg(theme.surface)
                                    .border_1()
                                    .border_color(theme.border)
                                    .rounded_lg()
                                    .shadow_lg()
                                    .child(
                                        // ADD THIS: Toggle Properties button
                                        div()
                                            .px_3()
                                            .py_1()
                                            .rounded_md()
                                            .bg(theme.surface)
                                            .text_sm()
                                            .text_color(theme.text)
                                            .cursor_pointer()
                                            .hover(|style| style.bg(theme.hover))
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(Self::toggle_inspector),
                                            )
                                            .child(if self.inspector_open {
                                                "Hide Properties"
                                            } else {
                                                "Show Properties"
                                            }),
                                    )
                                    .child(
                                        // Open button
                                        div()
                                            .px_3()
                                            .py_1()
                                            .rounded_md()
                                            .bg(theme.surface)
                                            .text_sm()
                                            .text_color(theme.text)
                                            .cursor_pointer()
                                            .hover(|style| style.bg(theme.hover))
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(Self::open_file),
                                            )
                                            .child("Open"),
                                    )
                                    .child(
                                        // Export button
                                        div()
                                            .px_3()
                                            .py_1()
                                            .rounded_md()
                                            .bg(theme.surface)
                                            .text_sm()
                                            .text_color(theme.text)
                                            .cursor_pointer()
                                            .hover(|style| style.bg(theme.hover))
                                            .child("Export"),
                                    )
                            })),
                    )
                    // Right-side Inspector Panel
                    .when(self.inspector_open, |this| {
                        this.child(Inspector::new(
                            inspector_width,
                            theme,
                            self.current_file.as_ref(),
                        ))
                    }),
            )
    }
}
