use gpui::{IntoElement, ParentElement, Styled, div};

use crate::io::loader::LoadedFile;
use crate::ui::theme::Theme;

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

    fn format_size(bytes: u64) -> String {
        if bytes < 1024 {
            format!("{} B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.2} KB", bytes as f64 / 1024.0)
        } else {
            format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
        }
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
                    .child(Self::render_row(
                        "Name",
                        self.file_info
                            .map(|f| f.name.clone())
                            .unwrap_or_else(|| "No file loaded".to_string()),
                        self.theme,
                    ))
                    .child(Self::render_row(
                        "Size",
                        self.file_info
                            .map(|f| Self::format_size(f.size))
                            .unwrap_or_else(|| "—".to_string()),
                        self.theme,
                    ))
                    .child(Self::render_row(
                        "Path",
                        self.file_info
                            .and_then(|f| f.path.parent())
                            .and_then(|p| p.to_str())
                            .unwrap_or("—"),
                        self.theme,
                    )),
            )
    }
}
