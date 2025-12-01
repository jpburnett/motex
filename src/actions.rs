use crate::texture_view::ScrollMode;

#[derive(Debug, Clone)]
pub enum AppAction {
    // File Operations
    OpenFileDialog,
    SaveFile,
    InsertImageDialog,
    ExportTextureDialog,

    // State Mutations
    UpdateTexture,

    // Navigation / Viewer
    AdjustOffset { delta: i64, mode: ScrollMode },

    // Clipboard
    CopyToClipboard,

    // Exit/Close
    Quit,
}
