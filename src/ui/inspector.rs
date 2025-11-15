use gpui::{InteractiveElement, IntoElement, ParentElement, Styled, div, px};

use super::panels::{
    dimensions::DimensionsPanel, file_info::FileInfoPanel, format_selector::FormatSelectorPanel,
};
use super::theme::Theme;
use crate::io::loader::LoadedFile;

/// Inspector component - right sidebar with collapsible panels
/// Orthogonal: Composes panels but doesn't implement their logic
pub struct Inspector<'a> {
    width: f32,
    theme: &'a Theme,
    file_info: Option<&'a LoadedFile>,
    show_close: bool,
}

impl<'a> Inspector<'a> {
    pub fn new(width: f32, theme: &'a Theme, file_info: Option<&'a LoadedFile>) -> Self {
        Self {
            width,
            theme,
            file_info,
            show_close: true,
        }
    }
}

impl<'a> IntoElement for Inspector<'a> {
    type Element = gpui::Div;

    fn into_element(self) -> Self::Element {
        let hover_color = self.theme.hover;
        let text_color = self.theme.text;

        div()
            .flex()
            .flex_col()
            .w(px(self.width))
            .bg(self.theme.surface)
            .border_l_1()
            .border_color(self.theme.border)
            .child(
                // Inspector header with close button
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_4()
                    .h(px(48.0))
                    .border_b_1()
                    .border_color(self.theme.border)
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(self.theme.text)
                            .child("Properties"),
                    )
                    .child(
                        // Close button (Zed-style) - we'll wire this up differently
                        div()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .text_xs()
                            .text_color(self.theme.text_muted)
                            .cursor_pointer()
                            .hover(move |style| style.bg(hover_color).text_color(text_color))
                            .id("inspector-close") // Add an ID so we can target it
                            .child("✕"),
                    ),
            )
            .child(
                // Inspector panels (each is orthogonal)
                div()
                    .flex()
                    .flex_col()
                    .child(FileInfoPanel::new(self.theme, self.file_info))
                    .child(FormatSelectorPanel::new(self.theme))
                    .child(DimensionsPanel::new(self.theme, 64, 64)), // Default dimensions
            )
    }
}
