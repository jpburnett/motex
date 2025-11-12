use gpui::{IntoElement, ParentElement, Styled, div, px};

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
}

impl<'a> Inspector<'a> {
    pub fn new(width: f32, theme: &'a Theme, file_info: Option<&'a LoadedFile>) -> Self {
        Self {
            width,
            theme,
            file_info,
        }
    }
}

impl<'a> IntoElement for Inspector<'a> {
    type Element = gpui::Div;

    fn into_element(self) -> Self::Element {
        // Default dimensions if no file loaded
        let (width, height) = (64, 64);

        div()
            .flex()
            .flex_col()
            .w(px(self.width))
            .bg(self.theme.surface)
            .border_l_1()
            .border_color(self.theme.border)
            .child(
                // Inspector header
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
                    ),
            )
            .child(
                // Inspector panels (each is orthogonal)
                div()
                    .flex()
                    .flex_col()
                    .child(FileInfoPanel::new(self.theme, self.file_info))
                    .child(FormatSelectorPanel::new(self.theme))
                    .child(DimensionsPanel::new(self.theme, width, height)),
            )
    }
}
