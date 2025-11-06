use gpui::{IntoElement, ParentElement, Styled, div, rgb};

/// File Info Panel - displays file metadata
/// Orthogonal: Only responsible for displaying file information
pub struct FileInfoPanel {
    // Will hold file metadata later
}

impl FileInfoPanel {
    pub fn new() -> Self {
        Self {}
    }

    fn render_row(label: impl Into<String>, value: impl Into<String>) -> gpui::Div {
        div()
            .flex()
            .justify_between()
            .py_1()
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x888888))
                    .child(label.into()),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0xcccccc))
                    .child(value.into()),
            )
    }
}

impl IntoElement for FileInfoPanel {
    type Element = gpui::Div;

    fn into_element(self) -> Self::Element {
        div()
            .flex()
            .flex_col()
            .px_4()
            .py_3()
            .border_b_1()
            .border_color(rgb(0x3d3d3d))
            .child(
                // Panel title
                div()
                    .mb_2()
                    .text_xs()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_color(rgb(0xaaaaaa))
                    .child("FILE INFO"),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(Self::render_row("Name", "No file loaded"))
                    .child(Self::render_row("Size", "—"))
                    .child(Self::render_row("Offset", "0x0000")),
            )
    }
}
