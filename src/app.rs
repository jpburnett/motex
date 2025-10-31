use crate::file_buffer::FileBuffer;
use crate::palette_manager::PaletteManager;
use crate::texture_view::TextureViewState;
use anyhow::Result;
use eframe::egui;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Texture64App {
    #[serde(skip)]
    pub file_buffer: FileBuffer,

    pub texture_view: TextureViewState,

    #[serde(skip)]
    pub palette_manager: PaletteManager,

    // UI state
    #[serde(with = "color32_serde")]
    pub background_color: egui::Color32,
    pub scale: f32,
    pub show_pixel_info: bool,

    #[serde(skip)]
    pub hovered_pixel: Option<(usize, usize, egui::Color32)>,

    #[serde(skip)]
    pub texture_handle: Option<egui::TextureHandle>,

    #[serde(skip)]
    show_save_confirmation: bool,

    #[serde(skip)]
    pending_action: Option<PendingAction>,
}

// Custom serialization for egui::Color32
mod color32_serde {
    use eframe::egui;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(color: &egui::Color32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (color.r(), color.g(), color.b(), color.a()).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<egui::Color32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (r, g, b, a) = <(u8, u8, u8, u8)>::deserialize(deserializer)?;
        Ok(egui::Color32::from_rgba_unmultiplied(r, g, b, a))
    }
}

#[derive(Clone)]
enum PendingAction {
    OpenFile(std::path::PathBuf),
    InsertImage,
    NewFile,
}

impl Default for Texture64App {
    fn default() -> Self {
        Self {
            file_buffer: FileBuffer::default(),
            texture_view: TextureViewState::default(),
            palette_manager: PaletteManager::default(),
            background_color: egui::Color32::from_rgb(128, 128, 128),
            scale: 1.0,
            show_pixel_info: true,
            hovered_pixel: None,
            texture_handle: None,
            show_save_confirmation: false,
            pending_action: None,
        }
    }
}

impl Texture64App {
    pub fn open_file(&mut self, path: std::path::PathBuf) -> Result<()> {
        self.file_buffer.load_file(path)?;
        self.texture_view.offset = 0;
        self.texture_handle = None; // Clear old texture
        self.update_texture();
        Ok(())
    }

    pub fn ui_open_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("All files", &["*"])
            .pick_file()
        {
            if self.file_buffer.is_modified() {
                self.pending_action = Some(PendingAction::OpenFile(path));
                self.show_save_confirmation = true;
            } else {
                let _ = self.open_file(path);
            }
        }
    }

    pub fn save_file(&mut self) -> Result<()> {
        self.file_buffer.save()?;
        Ok(())
    }

    pub fn insert_image(&mut self) -> Result<()> {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Image files", &["png", "jpg", "jpeg", "bmp"])
            .pick_file()
        {
            let img = image::open(path)?;
            let rgba = img.to_rgba8();

            // Convert RGBA to N64 format at current offset
            let n64_data = self.texture_view.encode_to_n64(&rgba)?;

            self.file_buffer
                .insert_data(self.texture_view.offset, &n64_data)?;
            self.update_texture();
        }
        Ok(())
    }

    pub fn export_texture(&mut self) -> Result<()> {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("PNG Image", &["png"])
            .add_filter("JPEG Image", &["jpg", "jpeg"])
            .add_filter("BMP Image", &["bmp"])
            .save_file()
        {
            if let Some(rgba_data) = self.texture_view.get_current_rgba() {
                let img = image::RgbaImage::from_raw(
                    self.texture_view.width,
                    self.texture_view.height,
                    rgba_data,
                )
                .ok_or_else(|| anyhow::anyhow!("Failed to create image"))?;

                img.save(path)?;
            }
        }
        Ok(())
    }

    pub fn copy_to_clipboard(&mut self) -> Result<()> {
        if let Some(rgba_data) = self.texture_view.get_current_rgba() {
            let img = image::RgbaImage::from_raw(
                self.texture_view.width,
                self.texture_view.height,
                rgba_data,
            )
            .ok_or_else(|| anyhow::anyhow!("Failed to create image"))?;

            // Try to copy to clipboard, but don't fail if it doesn't work
            match arboard::Clipboard::new() {
                Ok(mut clipboard) => {
                    let img_data = arboard::ImageData {
                        width: img.width() as usize,
                        height: img.height() as usize,
                        bytes: img.as_raw().into(),
                    };

                    if let Err(e) = clipboard.set_image(img_data) {
                        log::warn!("Failed to copy to clipboard: {}", e);
                    }
                }
                Err(e) => {
                    log::warn!("Failed to initialize clipboard: {}", e);
                }
            }
        }
        Ok(())
    }

    pub fn update_texture(&mut self) {
        let palette_data = if self.texture_view.format.is_ci() {
            Some(self.palette_manager.get_palette_data(
                &self.file_buffer,
                self.texture_view.palette_offset,
                self.texture_view.split_palette_enabled,
                self.texture_view.split_palette_offset,
            ))
        } else {
            None
        };

        self.texture_view
            .update_from_buffer(&self.file_buffer, palette_data.as_deref());
    }

    fn handle_file_drop(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                if let Some(dropped_file) = i.raw.dropped_files.first() {
                    if let Some(path) = &dropped_file.path {
                        if self.file_buffer.is_modified() {
                            self.pending_action = Some(PendingAction::OpenFile(path.clone()));
                            self.show_save_confirmation = true;
                        } else {
                            let _ = self.open_file(path.clone());
                        }
                    }
                }
            }
        });
    }

    fn check_shortcuts(&mut self, ctx: &egui::Context) {
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::O)) {
            self.ui_open_file();
        }
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::S)) {
            if self.file_buffer.path().is_some() {
                let _ = self.save_file();
            }
        }
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::I)) {
            let _ = self.insert_image();
        }
    }

    fn ui_save_confirmation(&mut self, ctx: &egui::Context) {
        if self.show_save_confirmation {
            egui::Window::new("Unsaved Changes")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label("The current file has unsaved changes.");
                    ui.label("Do you want to save before continuing?");

                    ui.horizontal(|ui| {
                        if ui.button("Save").clicked() {
                            let _ = self.save_file();
                            self.execute_pending_action();
                            self.show_save_confirmation = false;
                        }
                        if ui.button("Don't Save").clicked() {
                            self.execute_pending_action();
                            self.show_save_confirmation = false;
                        }
                        if ui.button("Cancel").clicked() {
                            self.pending_action = None;
                            self.show_save_confirmation = false;
                        }
                    });
                });
        }
    }

    fn execute_pending_action(&mut self) {
        if let Some(action) = self.pending_action.take() {
            match action {
                PendingAction::OpenFile(path) => {
                    let _ = self.open_file(path);
                }
                PendingAction::InsertImage => {
                    let _ = self.insert_image();
                }
                PendingAction::NewFile => {
                    // Future: create new file
                }
            }
        }
    }
}

impl eframe::App for Texture64App {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // Settings persistence disabled for now - Color32 serialization issues
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_file_drop(ctx);
        self.check_shortcuts(ctx);
        self.ui_save_confirmation(ctx);

        crate::ui::render_ui(self, ctx);
    }
}
