use gpui::{IntoElement, ParentElement, Styled, div};

use super::theme::Theme;

/// Canvas component - displays the texture
/// Orthogonal: Doesn't know about file formats, decoding, or other UI components
pub struct Canvas<'a> {
    theme: &'a Theme,
}

impl<'a> Canvas<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        Self { theme }
    }
}

impl<'a> IntoElement for Canvas<'a> {
    type Element = gpui::Div;

    fn into_element(self) -> Self::Element {
        div()
            .flex()
            .flex_1()
            .justify_center()
            .items_center()
            .bg(self.theme.background)
            .child(
                // Placeholder content - will be replaced with actual texture
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .items_center()
                    .child(
                        div()
                            .text_xl()
                            .text_color(self.theme.text_muted)
                            .child("Drop a file here to open"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(self.theme.text_muted)
                            .child("or use Ctrl+O to browse"),
                    ),
            )
    }
}
