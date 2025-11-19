use gpui::{InteractiveElement, IntoElement, ParentElement, Styled, div};

use crate::ui::theme::Theme;

/// Dimensions Panel - width/height controls for raw texture data
/// Now with clickable presets!
pub struct DimensionsPanel<'a> {
    theme: &'a Theme,
    width: u32,
    height: u32,
}

impl<'a> DimensionsPanel<'a> {
    pub fn new(theme: &'a Theme, width: u32, height: u32) -> Self {
        Self {
            theme,
            width,
            height,
        }
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

    fn render_preset(&self, size: u32) -> gpui::Div {
        let hover_color = self.theme.hover;

        div()
            .px_2()
            .py_1()
            .rounded_md()
            .bg(self.theme.surface)
            .text_xs()
            .text_color(self.theme.text_accent)
            .cursor_pointer()
            .hover(move |style| style.bg(hover_color))
            .child(format!("{}×{}", size, size))
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
                    .child(Self::render_input_field(
                        "Width",
                        self.width.to_string(),
                        self.theme,
                    ))
                    .child(Self::render_input_field(
                        "Height",
                        self.height.to_string(),
                        self.theme,
                    )),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .mt_3()
                    .flex_wrap()
                    .child(self.render_preset(32))
                    .child(self.render_preset(64))
                    .child(self.render_preset(128))
                    .child(self.render_preset(256)),
            )
    }
}
