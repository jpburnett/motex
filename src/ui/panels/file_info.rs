use gpui::{IntoElement, ParentElement, Styled, div};

use crate::{io::loader::LoadedFile, ui::theme::Theme};

/// File Info Panel - displays file metadata
/// Orthogonal: Only responsible for displaying file information
pub struct FileInfoPanel<'a> {
    theme: &'a Theme,
    file_info: Option<&'a LoadedFile>,
}

impl<'a> FileInfoPanel<'a> {
    pub fn new(theme: &'a Theme, file_info: Option<&'a LoadedFile>) -> Self {
        Self { theme, file_info }
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
        let (name, size, offset) = if let Some(info) = self.file_info {
            (
                info.name.clone(),
                format!("{} bytes", info.size),
                "0x0000".to_string(),
            )
        } else {
            (
                "No file loaded".to_string(),
                "—".to_string(),
                "—".to_string(),
            )
        };
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
                    .child(Self::render_row("Name", name, self.theme))
                    .child(Self::render_row("Size", size, self.theme))
                    .child(Self::render_row("Offset", offset, self.theme)),
            )
    }
}
