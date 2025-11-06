use gpui::{Rgba, rgb};

/// A struct to hold all the colors for the application.
/// We `#[derive(Clone)]` so it can be easily stored
/// and retrieved from the `gpui::AppContext`.
#[derive(Clone)]
pub struct Theme {
    pub background: Rgba,
    pub surface: Rgba,
    pub border: Rgba,
    pub hover: Rgba,
    pub text: Rgba,
    pub text_muted: Rgba,
    pub text_accent: Rgba,
}

impl Theme {
    /// Creates a new instance of the default theme.
    pub fn new() -> Self {
        Self {
            background: rgb(0x1e1e1e),  // Dark background
            surface: rgb(0x252526),     // Slightly lighter surface (panels, buttons)
            border: rgb(0x3d3d3d),      // Borders
            hover: rgb(0x3d3d3d),       // Hover background (same as border for now)
            text: rgb(0xcccccc),        // Primary text
            text_muted: rgb(0x888888),  // Muted text (labels, secondary info)
            text_accent: rgb(0xaaaaaa), // Accent text (panel titles)
        }
    }
}

// Add this implementation to satisfy the `gpui::Global` trait bound.
// This tells gpui that Theme can be stored and retrieved as a global.
impl gpui::Global for Theme {}
