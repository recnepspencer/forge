//! FgIcon — icon identifiers for the Forge UI.
//!
//! DOMAIN: Icon identity and SVG byte access.
//!
//! Workflow: use `node scripts/add-icon.js <lucide-name>` to download an SVG
//! and register it here automatically. Then load it at app startup via
//! `IconStore::load` and render with `IconStore::draw`.
//!
//! DEPENDENCIES: egui, egui_extras.

/// Identifies a Forge UI icon.
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
    X,
    PenLine,
    Eye,
    Layers3,
    Ruler,
    Move3d,
    Grid2x2,
    LoaderCircle,
}

impl FgIcon {
    /// Raw SVG bytes for this icon, or `None` if not yet downloaded.
    pub fn svg_bytes(self) -> Option<&'static [u8]> {
        match self {
            FgIcon::Plus          => Some(include_bytes!("../icons/plus.svg")),
            FgIcon::Minus         => Some(include_bytes!("../icons/minus.svg")),
            FgIcon::Check         => Some(include_bytes!("../icons/check.svg")),
            FgIcon::Search        => Some(include_bytes!("../icons/search.svg")),
            FgIcon::ChevronRight  => Some(include_bytes!("../icons/chevron-right.svg")),
            FgIcon::ChevronDown   => Some(include_bytes!("../icons/chevron-down.svg")),
            FgIcon::Sun           => Some(include_bytes!("../icons/sun.svg")),
            FgIcon::Moon          => Some(include_bytes!("../icons/moon.svg")),
            FgIcon::Box           => Some(include_bytes!("../icons/box.svg")),
            FgIcon::PencilLine    => Some(include_bytes!("../icons/pencil-line.svg")),
            FgIcon::Trash2        => Some(include_bytes!("../icons/trash-2.svg")),
            FgIcon::MessageSquare => Some(include_bytes!("../icons/message-square.svg")),
            FgIcon::X             => Some(include_bytes!("../icons/x.svg")),
            FgIcon::PenLine       => Some(include_bytes!("../icons/pen-line.svg")),
            FgIcon::Eye           => Some(include_bytes!("../icons/eye.svg")),
            FgIcon::Layers3       => Some(include_bytes!("../icons/layers-3.svg")),
            FgIcon::Ruler         => Some(include_bytes!("../icons/ruler.svg")),
            FgIcon::Move3d        => Some(include_bytes!("../icons/move-3d.svg")),
            FgIcon::Grid2x2       => Some(include_bytes!("../icons/grid-2x2.svg")),
            FgIcon::LoaderCircle  => Some(include_bytes!("../icons/loader-circle.svg")),
            _ => None,
        }
    }

    /// Fallback Unicode glyph when SVG is not available.
    pub fn glyph(self) -> &'static str {
        match self {
            FgIcon::Plus          => "+",
            FgIcon::Minus         => "−",
            FgIcon::Edit          => "✎",
            FgIcon::Delete        => "⌫",
            FgIcon::Check         => "✓",
            FgIcon::Close         => "✕",
            FgIcon::Search        => "🔍",
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
            FgIcon::Box           => "☐",
            FgIcon::PencilLine    => "✎",
            FgIcon::Trash2        => "🗑",
            FgIcon::MessageSquare => "💬",
            FgIcon::X             => "✕",
            FgIcon::PenLine       => "✎",
            FgIcon::Eye           => "👁",
            FgIcon::Layers3       => "☰",
            FgIcon::Ruler         => "📏",
            FgIcon::Move3d        => "✥",
            FgIcon::Grid2x2       => "▦",
            FgIcon::LoaderCircle  => "↻",
        }
    }
}

/// Holds loaded egui textures for all icons that have SVG data.
pub struct IconStore {
    pub textures: std::collections::HashMap<FgIcon, egui::TextureHandle>,
}

impl IconStore {
    /// Load every icon with SVG bytes into GPU textures. Call once at startup.
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

    /// Draw an icon. Falls back to glyph if SVG texture is missing.
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

    /// Draw a rotated icon.
    pub fn draw_rotated(&self, ui: &mut egui::Ui, icon: FgIcon, rect: egui::Rect, tint: egui::Color32, angle: f32) {
        if let Some(texture) = self.textures.get(&icon) {
            let sized = egui::load::SizedTexture::new(texture.id(), rect.size());
            let img = egui::Image::from_texture(sized)
                .tint(tint)
                .rotate(angle, egui::vec2(0.5, 0.5));
            ui.put(rect, img);
        } else {
            // fallback for missing svg
            ui.put(rect, egui::Label::new(egui::RichText::new(icon.glyph()).color(tint)));
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
            FgIcon::X, FgIcon::PenLine, FgIcon::Eye, FgIcon::Layers3,
            FgIcon::Ruler, FgIcon::Move3d, FgIcon::Grid2x2, FgIcon::LoaderCircle,
        ]
    }
}
