use gpui::{
    Context, InteractiveElement, IntoElement, ParentElement, Render, SharedString, Styled, Window,
    div, prelude::FluentBuilder,
};

// 1. Import the theme alongside your other UI components
use crate::ui::{canvas::Canvas, inspector::Inspector, theme::Theme, toolbar::Toolbar};

/// Main application state - orchestrates the layout
pub struct TextureViewerApp {
    // UI state
    inspector_open: bool,
    inspector_width: f32,

    // Application state
    // We prefix these with `_` to tell the Rust compiler
    // that we intentionally aren't using them yet.
    // This silences the "dead_code" warning.
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
        // --- FIX 1: ---
        // The event type is changed from `&gpui::ClickEvent`
        // to `&gpui::MouseDownEvent` to match the event provided
        // by the `on_mouse_down` handler.
        _event: &gpui::MouseDownEvent,
        // --- END FIX 1 ---
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
        // 2. Fetch the theme from the global context
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
            // 3. Use the theme colors
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
                            .child(Canvas::new())
                            .child(Toolbar::new())
                            .child(
                                // Inspector toggle button - floating on canvas
                                div()
                                    .absolute()
                                    .top_4()
                                    .right_4()
                                    .px_3()
                                    .py_2()
                                    .rounded_md()
                                    // 4. Use the theme colors for the button
                                    .bg(theme.surface)
                                    .border_1()
                                    .border_color(theme.border)
                                    .text_sm()
                                    .text_color(theme.text)
                                    .cursor_pointer()
                                    .hover(|style| style.bg(theme.hover))
                                    // --- FIX 2: ---
                                    // The `on_click` method doesn't exist on `gpui::Div`.
                                    // The correct method is `on_mouse_down`, which
                                    // takes a `MouseButton` as the first argument.
                                    .on_mouse_down(
                                        gpui::MouseButton::Left,
                                        cx.listener(Self::toggle_inspector),
                                    )
                                    // --- END FIX 2 ---
                                    .child(if self.inspector_open { "◀" } else { "▶" }),
                            ),
                    )
                    .when(self.inspector_open, |this| {
                        this.child(Inspector::new(inspector_width))
                    }),
            )
    }
}
