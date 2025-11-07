use gpui::{IntoElement, ParentElement, Styled, div};

use crate::ui::theme::Theme;

/// Dimensions Panel - width/height controls for raw texture data
/// Orthogonal: Just handles dimension input, doesn't do decoding
pub struct DimensionsPanel<'a> {
    theme: &'a Theme,
}

impl<'a> DimensionsPanel<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        Self { theme }
    }

    fn render_input_field(
        label: impl Into<String>,
        value: impl Into<String>,
        theme: &Theme,
    ) -> gpui::Div {
        div()
            .flex()
            .flex_col()
            .gap_1()
            .child(
                div()
                    .text_xs()
                    .text_color(theme.text_muted)
                    .child(label.into()),
            )
            .child(
                div()
                    .px_3()
                    .py_2()
                    .rounded_md()
                    .bg(theme.background)
                    .border_1()
                    .border_color(theme.border)
                    .text_sm()
                    .text_color(theme.text)
                    .child(value.into()),
            )
    }
}

impl<'a> IntoElement for DimensionsPanel<'a> {
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
                    .child("DIMENSIONS"),
            )
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(Self::render_input_field("Width", "64", self.theme))
                    .child(Self::render_input_field("Height", "64", self.theme)),
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
                            .bg(self.theme.surface)
                            .text_xs()
                            .text_color(self.theme.text_accent)
                            .child("32×32"),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .bg(self.theme.surface)
                            .text_xs()
                            .text_color(self.theme.text_accent)
                            .child("64×64"),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .bg(self.theme.surface)
                            .text_xs()
                            .text_color(self.theme.text_accent)
                            .child("128×128"),
                    ),
            )
    }
}
