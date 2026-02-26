//! FgIcon — icon identifiers for the Forge UI.
//!
//! DOMAIN: Icon identity and SVG byte access.
//!
//! Workflow: use `node scripts/add-icon.js <lucide-name>` to download an SVG
//! and register it here automatically. Then load it at app startup via
//! `IconStore::load` and render with `IconStore::draw`.
//!
//! DEPENDENCIES: egui, egui_extras.

/// Identifies a Forge UI icon. Variants are added via `scripts/add-icon.js`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FgIcon {
    // ── Core UI ─────────────────────────────────────────────────────────
    Plus,
    Minus,
    Edit,
    Delete,
    Check,
    Close,
    Search,
    ChevronRight,
    ChevronDown,
    Sun,
    Moon,
    // ── Domain ──────────────────────────────────────────────────────────
    Plane,
    Boolean,
    Select,
    Measure,
    Sketch,
    Orient,
    Chat,
    Properties,
    Logo,
    // ── Downloaded via scripts/add-icon.js ──────────────────────────────
    Box,
    PencilLine,
    Trash2,
    MessageSquare,
}

impl FgIcon {
    /// Returns the raw bytes of the SVG file for this icon, if available.
    ///
    /// Returns `None` for icons that have not yet been downloaded via the
    /// `scripts/add-icon.js` workflow. Callers fall back to `FgIcon::glyph()`.
    pub fn svg_bytes(self) -> Option<&'static [u8]> {
        match self {
            FgIcon::Box           => Some(include_bytes!("../icons/box.svg")),
            FgIcon::PencilLine    => Some(include_bytes!("../icons/pencil-line.svg")),
            FgIcon::Trash2        => Some(include_bytes!("../icons/trash-2.svg")),
            FgIcon::MessageSquare => Some(include_bytes!("../icons/message-square.svg")),
            _ => None,
        }
    }

    /// Fallback Unicode glyph used when no SVG has been downloaded yet.
    pub fn glyph(self) -> &'static str {
        match self {
            FgIcon::Plus          => "+",
            FgIcon::Minus         => "−",
            FgIcon::Edit          => "✎",
            FgIcon::Delete        => "⌫",
            FgIcon::Check         => "✓",
            FgIcon::Close         => "✕",
            FgIcon::Search        => "⌘K",
            FgIcon::ChevronRight  => "›",
            FgIcon::ChevronDown   => "‹",
            FgIcon::Sun           => "☀",
            FgIcon::Moon          => "☾",
            FgIcon::Plane         => "◈",
            FgIcon::Boolean       => "⊕",
            FgIcon::Select        => "↖",
            FgIcon::Measure       => "⊿",
            FgIcon::Sketch        => "▣",
            FgIcon::Orient        => "✦",
            FgIcon::Chat          => "💬",
            FgIcon::Properties    => "≡",
            FgIcon::Logo          => "◆",
            FgIcon::Box           => "□",
            FgIcon::PencilLine    => "✎",
            FgIcon::Trash2        => "⌫",
            FgIcon::MessageSquare => "💬",
        }
    }
}

/// Holds loaded egui textures for all icons that have SVG data.
///
/// Create one `IconStore` per `ForgeApp` and call `load()` once in `new()`.
/// Then call `draw()` anywhere you have a `&mut egui::Ui`.
pub struct IconStore {
    textures: std::collections::HashMap<FgIcon, egui::TextureHandle>,
}

impl IconStore {
    /// Load all icons that have SVG bytes into GPU textures.
    ///
    /// Call once at app startup from `eframe::CreationContext`.
    pub fn load(ctx: &egui::Context) -> Self {
        let mut textures = std::collections::HashMap::new();

        for &icon in Self::all_icons() {
            if let Some(bytes) = icon.svg_bytes() {
                match egui_extras::image::load_svg_bytes(bytes) {
                    Ok(img) => {
                        let handle = ctx.load_texture(
                            format!("icon_{:?}", icon),
                            img,
                            egui::TextureOptions::LINEAR,
                        );
                        textures.insert(icon, handle);
                    }
                    Err(e) => {
                        eprintln!("forge-ui: failed to load SVG for {icon:?}: {e}");
                    }
                }
            }
        }

        Self { textures }
    }

    /// Draw an icon at the given size. Falls back to the Unicode glyph if no
    /// SVG texture has been loaded for this icon.
    pub fn draw(&self, ui: &mut egui::Ui, icon: FgIcon, size: f32, tint: egui::Color32) {
        if let Some(texture) = self.textures.get(&icon) {
            let sized = egui::load::SizedTexture::new(texture.id(), [size, size]);
            let img = egui::Image::from_texture(sized).tint(tint);
            ui.add(img);
        } else {
            ui.label(
                egui::RichText::new(icon.glyph())
                    .color(tint)
                    .size(size * 0.85),
            );
        }
    }

    fn all_icons() -> &'static [FgIcon] {
        &[
            FgIcon::Plus, FgIcon::Minus, FgIcon::Edit, FgIcon::Delete,
            FgIcon::Check, FgIcon::Close, FgIcon::Search, FgIcon::ChevronRight,
            FgIcon::ChevronDown, FgIcon::Sun, FgIcon::Moon, FgIcon::Plane,
            FgIcon::Boolean, FgIcon::Select, FgIcon::Measure, FgIcon::Sketch,
            FgIcon::Orient, FgIcon::Chat, FgIcon::Properties, FgIcon::Logo,
            FgIcon::Box, FgIcon::PencilLine, FgIcon::Trash2, FgIcon::MessageSquare,
        ]
    }
}
