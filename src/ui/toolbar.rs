use gpui::{InteractiveElement, IntoElement, MouseButton, ParentElement, Styled, div, px, rgb};

/// Toolbar component - floating action bar
/// Orthogonal: Just provides UI controls, doesn't handle business logic
pub struct Toolbar {
    // Will hold toolbar state later
}

impl Toolbar {
    pub fn new() -> Self {
        Self {}
    }

    fn render_button(label: impl Into<String>) -> gpui::Div {
        let label_str = label.into();
        let label_clone = label_str.clone();

        div()
            .px_3()
            .py_1()
            .rounded_md()
            .bg(rgb(0x2d2d2d))
            .text_sm()
            .text_color(rgb(0xcccccc))
            .cursor_pointer()
            .hover(|style| style.bg(rgb(0x3d3d3d)))
            // --- CHANGED: ---
            // The closure signature now includes `_window` as the third argument
            // to match what `on_mouse_down` expects (event, window, cx).
            .on_mouse_down(MouseButton::Left, move |_event, _window, _cx| {
                // --- END CHANGE ---
                log::info!("Button clicked: {}", label_clone);
            })
            .child(label_str)
    }
}

impl IntoElement for Toolbar {
    type Element = gpui::Div;

    fn into_element(self) -> Self::Element {
        div()
            .absolute()
            .top_4()
            .left_4()
            .right_4()
            .flex()
            .items_center()
            .justify_between()
            .px_4()
            .h(px(48.0))
            .bg(rgb(0x252526))
            .border_1()
            .border_color(rgb(0x3d3d3d))
            .rounded_lg()
            .shadow_lg()
            .child(
                // Left side - file operations
                div()
                    .flex()
                    .gap_2()
                    .child(Self::render_button("Open"))
                    .child(Self::render_button("Export")),
            )
            .child(
                // Right side - view controls
                div()
                    .flex()
                    .gap_2()
                    .items_center()
                    .child(div().text_sm().text_color(rgb(0x888888)).child("100%"))
                    .child(Self::render_button("Fit")),
            )
    }
}
