use gpui::{IntoElement, ParentElement, Styled, div, rgb};

/// Dimensions Panel - width/height controls for raw texture data
/// Orthogonal: Just handles dimension input, doesn't do decoding
pub struct DimensionsPanel {
    // Will hold dimension state later
}

impl DimensionsPanel {
    pub fn new() -> Self {
        Self {}
    }

    fn render_input_field(label: impl Into<String>, value: impl Into<String>) -> gpui::Div {
        div()
            .flex()
            .flex_col()
            .gap_1()
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x888888))
                    .child(label.into()),
            )
            .child(
                div()
                    .px_3()
                    .py_2()
                    .rounded_md()
                    .bg(rgb(0x1e1e1e))
                    .border_1()
                    .border_color(rgb(0x3d3d3d))
                    .text_sm()
                    .text_color(rgb(0xcccccc))
                    .child(value.into()),
            )
    }
}

impl IntoElement for DimensionsPanel {
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
                    .child("DIMENSIONS"),
            )
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(Self::render_input_field("Width", "64"))
                    .child(Self::render_input_field("Height", "64")),
            )
            .child(
                // Common presets
                div()
                    .flex()
                    .gap_2()
                    .mt_3()
                    .flex_wrap()
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .bg(rgb(0x2d2d2d))
                            .text_xs()
                            .text_color(rgb(0xaaaaaa))
                            .child("32×32"),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .bg(rgb(0x2d2d2d))
                            .text_xs()
                            .text_color(rgb(0xaaaaaa))
                            .child("64×64"),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .bg(rgb(0x2d2d2d))
                            .text_xs()
                            .text_color(rgb(0xaaaaaa))
                            .child("128×128"),
                    ),
            )
    }
}
