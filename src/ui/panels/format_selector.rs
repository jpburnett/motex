use gpui::{IntoElement, ParentElement, Styled, div, rgb};

use crate::ui::theme::Theme;

/// Format Selector Panel - lets user choose texture format
/// Orthogonal: Doesn't know how formats are decoded, just presents options
pub struct FormatSelectorPanel<'a> {
    theme: &'a Theme,
}

impl<'a> FormatSelectorPanel<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        Self { theme }
    }

    fn render_format_option(name: impl Into<String>, selected: bool, theme: &Theme) -> gpui::Div {
        let bg_color = if selected {
            rgb(0x0e639c)
        } else {
            theme.surface
        };

        div()
            .px_3()
            .py_2()
            .rounded_md()
            .bg(bg_color)
            .text_sm()
            .text_color(theme.text)
            .child(name.into())
    }
}

impl<'a> IntoElement for FormatSelectorPanel<'a> {
    type Element = gpui::Div;

    fn into_element(self) -> Self::Element {
        div()
            .flex()
            .flex_col()
            .px_4()
            .py_3()
            .border_b_1()
            .border_color(self.theme.border)
            .child(
                // Panel title
                div()
                    .mb_2()
                    .text_xs()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_color(self.theme.text_accent)
                    .child("FORMAT"),
            )
            .child(
                // Format dropdown (mockup with common N64 formats)
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(Self::render_format_option("RGBA16", true, self.theme))
                    .child(
                        // Expandable format list
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_2()
                            .child(Self::render_format_option("RGBA32", false, self.theme))
                            .child(Self::render_format_option("CI4", false, self.theme))
                            .child(Self::render_format_option("CI8", false, self.theme))
                            .child(Self::render_format_option("IA16", false, self.theme))
                            .child(Self::render_format_option("IA8", false, self.theme))
                            .child(Self::render_format_option("I8", false, self.theme)),
                    ),
            )
    }
}
