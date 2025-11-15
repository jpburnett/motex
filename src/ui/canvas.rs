use gpui::{IntoElement, ParentElement, Styled, div};

use super::theme::Theme;
use crate::io::loader::LoadedFile;

/// Canvas component - displays the texture
/// Orthogonal: Doesn't know about file formats, decoding, or other UI components
pub struct Canvas<'a> {
    theme: &'a Theme,
    file_data: Option<&'a LoadedFile>,
}

impl<'a> Canvas<'a> {
    pub fn new(theme: &'a Theme, file_data: Option<&'a LoadedFile>) -> Self {
        Self { theme, file_data }
    }

    /// Format bytes as hex dump (like a hex editor)
    /// Shows: offset | hex bytes | ASCII representation
    fn format_hex_line(offset: usize, bytes: &[u8]) -> String {
        let hex: String = bytes
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ");

        let ascii: String = bytes
            .iter()
            .map(|&b| if b >= 32 && b <= 126 { b as char } else { '.' })
            .collect();

        format!("{:08X}  {:<48}  {}", offset, hex, ascii)
    }
}

impl<'a> IntoElement for Canvas<'a> {
    type Element = gpui::Div;

    fn into_element(self) -> Self::Element {
        div()
            .flex()
            .flex_1()
            .justify_center()
            .items_center()
            .bg(self.theme.background)
            .child(match self.file_data {
                None => {
                    // No file loaded - show placeholder
                    div()
                        .flex()
                        .flex_col()
                        .gap_4()
                        .items_center()
                        .child(
                            div()
                                .text_xl()
                                .text_color(self.theme.text_muted)
                                .child("Drop a file here to open"),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(self.theme.text_muted)
                                .child("or use the menu to browse"),
                        )
                }
                Some(file) => {
                    // File loaded - show hex viewer
                    let bytes_to_show = 256.min(file.data.len()); // Show first 256 bytes
                    let lines: Vec<String> = file.data[..bytes_to_show]
                        .chunks(16)
                        .enumerate()
                        .map(|(i, chunk)| Self::format_hex_line(i * 16, chunk))
                        .collect();

                    div()
                        .flex()
                        .flex_col()
                        .p_4()
                        .gap_0()
                        .w_full()
                        .h_full()
                        .child(
                            // Header
                            div()
                                .mb_4()
                                .pb_2()
                                .border_b_1()
                                .border_color(self.theme.border)
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(gpui::FontWeight::BOLD)
                                        .text_color(self.theme.text)
                                        .child(format!(
                                            "Hex View - {} (showing first {} bytes)",
                                            file.name, bytes_to_show
                                        )),
                                ),
                        )
                        .child(
                            // Hex dump
                            div()
                                .flex()
                                .flex_col()
                                .gap_0()
                                .children(lines.into_iter().map(|line| {
                                    div()
                                        .text_xs()
                                        .font_family("monospace")
                                        .text_color(self.theme.text)
                                        .child(line)
                                })),
                        )
                }
            })
    }
}
