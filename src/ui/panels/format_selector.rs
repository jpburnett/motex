use gpui::{InteractiveElement, IntoElement, ParentElement, Styled, div, rgb};

use crate::formats::adapter::TextureFormat;
use crate::ui::theme::Theme;

/// Format Selector Panel - lets user choose texture format
/// Now interactive! Click formats to decode differently
pub struct FormatSelectorPanel<'a> {
    theme: &'a Theme,
    selected_format: TextureFormat,
}

impl<'a> FormatSelectorPanel<'a> {
    pub fn new(theme: &'a Theme, selected_format: TextureFormat) -> Self {
        Self {
            theme,
            selected_format,
        }
    }

    fn render_format_option(&self, format: TextureFormat) -> gpui::Div {
        let selected = format == self.selected_format;
        let bg_color = if selected {
            rgb(0x0e639c)
        } else {
            self.theme.surface
        };

        let hover_color = self.theme.hover;

        div()
            .px_3()
            .py_2()
            .rounded_md()
            .bg(bg_color)
            .text_sm()
            .text_color(self.theme.text)
            .cursor_pointer()
            .hover(move |style| {
                if !selected {
                    style.bg(hover_color)
                } else {
                    style
                }
            })
            .child(format.as_str())
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
                div()
                    .mb_2()
                    .text_xs()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_color(self.theme.text_accent)
                    .child("FORMAT"),
            )
            .child(
                div().flex().flex_wrap().gap_2().children(
                    TextureFormat::all()
                        .iter()
                        .map(|&format| self.render_format_option(format)),
                ),
            )
    }
}
