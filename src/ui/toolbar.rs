use gpui::{InteractiveElement, IntoElement, MouseButton, ParentElement, Styled, div, px};

use super::theme::Theme;

/// Toolbar component - floating action bar
/// Orthogonal: Just provides UI controls, doesn't handle business logic
pub struct Toolbar<'a> {
    theme: &'a Theme,
}

impl<'a> Toolbar<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        Self { theme }
    }

    fn render_button(label: impl Into<String>, theme: &Theme) -> gpui::Div {
        let label_str = label.into();
        let label_clone = label_str.clone();
        let hover_color = theme.hover;

        div()
            .px_3()
            .py_1()
            .rounded_md()
            .bg(theme.surface)
            .text_sm()
            .text_color(theme.text)
            .cursor_pointer()
            .hover(move |style| style.bg(hover_color))
            .on_mouse_down(MouseButton::Left, move |_event, _window, _cx| {
                log::info!("Button clicked: {}", label_clone);
            })
            .child(label_str)
    }
}

impl<'a> IntoElement for Toolbar<'a> {
    type Element = gpui::Div;

    fn into_element(self) -> Self::Element {
        div()
            .absolute()
            .top_4()
            .left_4()
            .right_4()
            .flex()
            .items_center()
            .justify_between()
            .px_4()
            .h(px(48.0))
            .bg(self.theme.surface)
            .border_1()
            .border_color(self.theme.border)
            .rounded_lg()
            .shadow_lg()
            .child(
                // Left side - file operations
                div()
                    .flex()
                    .gap_2()
                    .child(Self::render_button("Open", self.theme))
                    .child(Self::render_button("Export", self.theme)),
            )
            .child(
                // Right side - view controls
                div()
                    .flex()
                    .gap_2()
                    .items_center()
                    .child(
                        div()
                            .text_sm()
                            .text_color(self.theme.text_muted)
                            .child("100%"),
                    )
                    .child(Self::render_button("Fit", self.theme)),
            )
    }
}
