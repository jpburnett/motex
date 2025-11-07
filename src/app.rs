use gpui::{
    Context, InteractiveElement, IntoElement, ParentElement, Render, SharedString, Styled, Window,
    div, prelude::FluentBuilder, px,
};

use crate::ui::{canvas::Canvas, inspector::Inspector, theme::Theme, toolbar::Toolbar};

/// Main application state - orchestrates the layout
pub struct TextureViewerApp {
    // UI state
    inspector_open: bool,
    inspector_width: f32,

    // Application state
    _current_file: Option<SharedString>,
    _zoom_level: f32,
}

impl TextureViewerApp {
    pub fn new() -> Self {
        log::info!("Initializing TextureViewerApp");
        Self {
            inspector_open: true,
            inspector_width: 300.0,
            _current_file: None,
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
}

impl Render for TextureViewerApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Fetch the theme from the global context
        let theme = cx.global::<Theme>();

        let inspector_width = if self.inspector_open {
            self.inspector_width
        } else {
            0.0
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.background)
            .child(
                // Main content area with canvas and inspector
                div()
                    .flex()
                    .flex_1()
                    .child(
                        // Canvas area (grows to fill available space)
                        div()
                            .flex()
                            .flex_1()
                            .relative()
                            .child(Canvas::new(theme))
                            .child(Toolbar::new(theme))
                            .child(
                                // Inspector toggle button - floating on canvas (below toolbar)
                                div()
                                    .absolute()
                                    .top(px(68.0)) // Position below the toolbar (48px height + 20px spacing)
                                    .right_4()
                                    .px_3()
                                    .py_2()
                                    .rounded_md()
                                    .bg(theme.surface)
                                    .border_1()
                                    .border_color(theme.border)
                                    .text_sm()
                                    .text_color(theme.text)
                                    .cursor_pointer()
                                    .hover(|style| style.bg(theme.hover))
                                    .on_mouse_down(
                                        gpui::MouseButton::Left,
                                        cx.listener(Self::toggle_inspector),
                                    )
                                    .child(if self.inspector_open { "◀" } else { "▶" }),
                            ),
                    )
                    .when(self.inspector_open, |this| {
                        this.child(Inspector::new(inspector_width, theme))
                    }),
            )
    }
}
