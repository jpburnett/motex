use gpui::{IntoElement, ParentElement, Styled, div, rgb};

/// Format Selector Panel - lets user choose texture format
/// Orthogonal: Doesn't know how formats are decoded, just presents options
pub struct FormatSelectorPanel {
    // Will hold selected format later
}

impl FormatSelectorPanel {
    pub fn new() -> Self {
        Self {}
    }

    fn render_format_option(name: impl Into<String>, selected: bool) -> gpui::Div {
        let bg_color = if selected {
            rgb(0x0e639c)
        } else {
            rgb(0x2d2d2d)
        };

        div()
            .px_3()
            .py_2()
            .rounded_md()
            .bg(bg_color)
            .text_sm()
            .text_color(rgb(0xcccccc))
            .child(name.into())
    }
}

impl IntoElement for FormatSelectorPanel {
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
                    .child("FORMAT"),
            )
            .child(
                // Format dropdown (mockup with common N64 formats)
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(Self::render_format_option("RGBA16", true))
                    .child(
                        // Expandable format list
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_2()
                            .child(Self::render_format_option("RGBA32", false))
                            .child(Self::render_format_option("CI4", false))
                            .child(Self::render_format_option("CI8", false))
                            .child(Self::render_format_option("IA16", false))
                            .child(Self::render_format_option("IA8", false))
                            .child(Self::render_format_option("I8", false)),
                    ),
            )
    }
}
