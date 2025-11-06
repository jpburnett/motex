use gpui::{IntoElement, ParentElement, Styled, div, px, rgb};

use super::panels::{
    dimensions::DimensionsPanel, file_info::FileInfoPanel, format_selector::FormatSelectorPanel,
};

/// Inspector component - right sidebar with collapsible panels
/// Orthogonal: Composes panels but doesn't implement their logic
pub struct Inspector {
    width: f32,
}

impl Inspector {
    pub fn new(width: f32) -> Self {
        Self { width }
    }
}

impl IntoElement for Inspector {
    type Element = gpui::Div;

    fn into_element(self) -> Self::Element {
        div()
            .flex()
            .flex_col()
            .w(px(self.width))
            .bg(rgb(0x252526))
            .border_l_1()
            .border_color(rgb(0x3d3d3d))
            .child(
                // Inspector header
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_4()
                    .h(px(48.0))
                    .border_b_1()
                    .border_color(rgb(0x3d3d3d))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(rgb(0xcccccc))
                            .child("Properties"),
                    ),
            )
            .child(
                // Inspector panels (each is orthogonal)
                div()
                    .flex()
                    .flex_col()
                    .child(FileInfoPanel::new())
                    .child(FormatSelectorPanel::new())
                    .child(DimensionsPanel::new()),
            )
    }
}
