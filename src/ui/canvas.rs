use gpui::{IntoElement, ParentElement, Styled, div, rgb};

/// Canvas component - displays the texture
/// Orthogonal: Doesn't know about file formats, decoding, or other UI components
pub struct Canvas {
    // Will hold texture data later
}

impl Canvas {
    pub fn new() -> Self {
        Self {}
    }
}

impl IntoElement for Canvas {
    type Element = gpui::Div;

    fn into_element(self) -> Self::Element {
        div()
            .flex()
            .flex_1()
            .justify_center()
            .items_center()
            .bg(rgb(0x1e1e1e))
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
                            .text_color(rgb(0x666666))
                            .child("Drop a file here to open"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x505050))
                            .child("or use Ctrl+O to browse"),
                    ),
            )
    }
}
