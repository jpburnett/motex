use gpui::{IntoElement, ParentElement, Styled, div};

use crate::ui::theme::Theme;

/// File Info Panel - displays file metadata
/// Orthogonal: Only responsible for displaying file information
pub struct FileInfoPanel<'a> {
    theme: &'a Theme,
}

impl<'a> FileInfoPanel<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        Self { theme }
    }

    fn render_row(label: impl Into<String>, value: impl Into<String>, theme: &Theme) -> gpui::Div {
        div()
            .flex()
            .justify_between()
            .py_1()
            .child(
                div()
                    .text_xs()
                    .text_color(theme.text_muted)
                    .child(label.into()),
            )
            .child(div().text_xs().text_color(theme.text).child(value.into()))
    }
}

impl<'a> IntoElement for FileInfoPanel<'a> {
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
                    .child("FILE INFO"),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(Self::render_row("Name", "No file loaded", self.theme))
                    .child(Self::render_row("Size", "—", self.theme))
                    .child(Self::render_row("Offset", "0x0000", self.theme)),
            )
    }
}

