//! Forge design token system.
//!
//! DOMAIN: All color, spacing, radius, and typography tokens. Two built-in
//! themes: dark and light. Custom themes are new functions returning ForgeTheme.
//! DEPENDENCIES: egui only.

pub mod backdrop;
pub mod metal;
pub mod shader_pipeline;

use egui::Color32;

/// Complete design token set for one theme.
///
/// Every color, radius, and spacing value the UI needs lives here.
/// Zero hardcoded values anywhere downstream — everything flows through this struct.
#[derive(Debug, Clone)]
pub struct ForgeTheme {
    // ── Background layers ────────────────────────────────────────────────
    /// Deepest background — the window/canvas.
    pub bg_base: Color32,
    /// Default panel / card background.
    pub bg_surface: Color32,
    /// Elevated chrome: menus, dropdowns, tooltips.
    pub bg_raised: Color32,
    /// Modal scrim / overlay backdrop.
    pub bg_overlay: Color32,

    // ── Borders ──────────────────────────────────────────────────────────
    pub border_subtle: Color32,
    pub border_default: Color32,
    /// Accent border shown when an input is focused.
    pub border_focus: Color32,

    // ── Text ─────────────────────────────────────────────────────────────
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub text_muted: Color32,
    /// Text drawn on a colored (accent) surface.
    pub text_inverse: Color32,

    // ── Accent / brand ───────────────────────────────────────────────────
    /// Primary brand accent (Forge violet).
    pub accent_primary: Color32,
    /// Accent with alpha — used for glow, bloom, and selection rings.
    pub accent_glow: Color32,
    /// Softer accent used for hover states.
    pub accent_hover: Color32,
    /// Very subtle accent tint for section labels and active-state pill backgrounds.
    pub accent_subtle: Color32,

    // ── Semantic ─────────────────────────────────────────────────────────
    pub success: Color32,
    pub warning: Color32,
    pub danger: Color32,
    pub info: Color32,

    /// Muted tint versions for badge/chip backgrounds.
    pub success_surface: Color32,
    pub warning_surface: Color32,
    pub danger_surface: Color32,
    pub info_surface: Color32,

    // ── Structural surfaces ──────────────────────────────────────────────
    /// Slightly darker inset bg for the sidebar panel.
    pub bg_sidebar: Color32,
    /// Dot color for the viewport grid pattern.
    pub viewport_grid: Color32,
    /// Background for user chat messages (plain, no card).
    pub chat_user_bg: Color32,
    /// Background card for agent/AI chat messages.
    pub chat_agent_bg: Color32,

    // ── Geometry (border radii) ──────────────────────────────────────────
    pub radius_sm: f32,
    pub radius_md: f32,
    pub radius_lg: f32,
    pub radius_pill: f32,

    // ── Spacing scale (4-based, in logical pixels) ───────────────────────
    /// `[4, 8, 12, 16, 20, 24, 32, 48]`
    pub space: [f32; 8],

    // ── Typography ───────────────────────────────────────────────────────
    pub font_size_xs: f32,
    pub font_size_sm: f32,
    pub font_size_md: f32,
    pub font_size_lg: f32,
    pub font_size_xl: f32,

    // ── Metallic effect tokens ───────────────────────────────────────────
    /// Top-to-bottom gradient strength for metallic surfaces (0.0..0.25).
    pub metal_gradient: f32,
    /// Inner rim highlight alpha (0..255). Higher = shinier top edge.
    pub metal_rim_alpha: u8,
    /// Shadow blur radius for elevated metallic elements.
    pub metal_shadow_blur: f32,
    /// Shadow alpha for elevated metallic elements.
    pub metal_shadow_alpha: u8,
}

impl ForgeTheme {
    /// Convenience accessor: returns `self.space[idx]`.
    pub fn sp(&self, idx: usize) -> f32 {
        self.space[idx.min(7)]
    }

    /// Configure egui `Visuals` from this theme so egui's own widgets
    /// adopt our color system as closely as possible.
    pub fn apply_to_egui(&self, ctx: &egui::Context) {
        let mut visuals = egui::Visuals::dark();

        visuals.override_text_color = Some(self.text_primary);
        visuals.panel_fill = self.bg_surface;
        visuals.window_fill = self.bg_raised;
        visuals.extreme_bg_color = self.bg_base;
        visuals.faint_bg_color = self.bg_surface;

        visuals.widgets.noninteractive.bg_fill = self.bg_surface;
        visuals.widgets.noninteractive.fg_stroke =
            egui::Stroke::new(1.0, self.text_secondary);
        visuals.widgets.noninteractive.bg_stroke =
            egui::Stroke::new(1.0, self.border_subtle);

        visuals.widgets.inactive.bg_fill = self.bg_surface;
        visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, self.text_primary);
        visuals.widgets.inactive.bg_stroke =
            egui::Stroke::new(1.0, self.border_default);

        visuals.widgets.hovered.bg_fill = self.accent_hover;
        visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, self.text_primary);
        visuals.widgets.hovered.bg_stroke =
            egui::Stroke::new(1.5, self.border_focus);

        visuals.widgets.active.bg_fill = self.accent_primary;
        visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, self.text_inverse);
        visuals.widgets.active.bg_stroke =
            egui::Stroke::new(2.0, self.accent_primary);

        visuals.selection.bg_fill =
            Color32::from_rgba_unmultiplied(108, 99, 255, 60);
        visuals.selection.stroke = egui::Stroke::new(1.0, self.accent_primary);

        visuals.window_corner_radius = egui::CornerRadius::same(self.radius_md as u8);
        visuals.window_stroke = egui::Stroke::new(1.0, self.border_default);

        ctx.set_visuals(visuals);
    }
}

// ── Built-in themes ──────────────────────────────────────────────────────────

/// Dark theme — the default Forge experience.
pub fn dark_theme() -> ForgeTheme {
    ForgeTheme {
        bg_base:    Color32::from_rgb(10, 11, 14),
        bg_surface: Color32::from_rgb(17, 19, 24),
        bg_raised:  Color32::from_rgb(26, 29, 36),
        bg_overlay: Color32::from_rgba_unmultiplied(0, 0, 0, 180),

        border_subtle:  Color32::from_rgb(30, 33, 42),
        border_default: Color32::from_rgb(37, 40, 48),
        border_focus:   Color32::from_rgb(108, 99, 255),

        text_primary:   Color32::from_rgb(232, 233, 238),
        text_secondary: Color32::from_rgb(123, 127, 142),
        text_muted:     Color32::from_rgb(69, 74, 88),
        text_inverse:   Color32::from_rgb(255, 255, 255),

        accent_primary: Color32::from_rgb(108, 99, 255),
        accent_glow:    Color32::from_rgba_unmultiplied(108, 99, 255, 100),
        accent_hover:   Color32::from_rgb(37, 40, 48),
        accent_subtle:  Color32::from_rgba_unmultiplied(108, 99, 255, 22),

        success: Color32::from_rgb(46, 204, 143),
        warning: Color32::from_rgb(245, 166, 35),
        danger:  Color32::from_rgb(255, 77, 106),
        info:    Color32::from_rgb(56, 182, 255),

        success_surface: Color32::from_rgba_unmultiplied(46, 204, 143, 25),
        warning_surface: Color32::from_rgba_unmultiplied(245, 166, 35, 25),
        danger_surface:  Color32::from_rgba_unmultiplied(255, 77, 106, 25),
        info_surface:    Color32::from_rgba_unmultiplied(56, 182, 255, 25),

        bg_sidebar:     Color32::from_rgb(13, 14, 18),
        viewport_grid:  Color32::from_rgba_unmultiplied(255, 255, 255, 12),
        chat_user_bg:   Color32::TRANSPARENT,
        chat_agent_bg:  Color32::from_rgb(22, 25, 32),

        radius_sm:   4.0,
        radius_md:   8.0,
        radius_lg:   12.0,
        radius_pill: 9999.0,

        space: [4.0, 8.0, 12.0, 16.0, 20.0, 24.0, 32.0, 48.0],

        font_size_xs: 11.0,
        font_size_sm: 12.0,
        font_size_md: 14.0,
        font_size_lg: 16.0,
        font_size_xl: 20.0,

        metal_gradient: 0.35,
        metal_rim_alpha: 100,
        metal_shadow_blur: 6.0,
        metal_shadow_alpha: 40,
    }
}

/// Light theme — clean, high-contrast studio look.
pub fn light_theme() -> ForgeTheme {
    ForgeTheme {
        bg_base:    Color32::from_rgb(248, 249, 251),
        bg_surface: Color32::from_rgb(255, 255, 255),
        bg_raised:  Color32::from_rgb(244, 245, 248),
        bg_overlay: Color32::from_rgba_unmultiplied(0, 0, 0, 80),

        border_subtle:  Color32::from_rgb(226, 228, 234),
        border_default: Color32::from_rgb(210, 213, 222),
        border_focus:   Color32::from_rgb(108, 99, 255),

        text_primary:   Color32::from_rgb(15, 17, 26),
        text_secondary: Color32::from_rgb(80, 87, 108),
        text_muted:     Color32::from_rgb(155, 160, 178),
        text_inverse:   Color32::from_rgb(255, 255, 255),

        accent_primary: Color32::from_rgb(108, 99, 255),
        accent_glow:    Color32::from_rgba_unmultiplied(108, 99, 255, 80),
        accent_hover:   Color32::from_rgb(237, 236, 255),
        accent_subtle:  Color32::from_rgba_unmultiplied(108, 99, 255, 18),

        success: Color32::from_rgb(22, 163, 111),
        warning: Color32::from_rgb(202, 120, 10),
        danger:  Color32::from_rgb(220, 38, 75),
        info:    Color32::from_rgb(14, 140, 210),

        success_surface: Color32::from_rgba_unmultiplied(22, 163, 111, 20),
        warning_surface: Color32::from_rgba_unmultiplied(202, 120, 10, 20),
        danger_surface:  Color32::from_rgba_unmultiplied(220, 38, 75, 20),
        info_surface:    Color32::from_rgba_unmultiplied(14, 140, 210, 20),

        bg_sidebar:     Color32::from_rgb(240, 241, 245),
        viewport_grid:  Color32::from_rgba_unmultiplied(0, 0, 0, 12),
        chat_user_bg:   Color32::TRANSPARENT,
        chat_agent_bg:  Color32::from_rgb(244, 245, 250),

        radius_sm:   4.0,
        radius_md:   8.0,
        radius_lg:   12.0,
        radius_pill: 9999.0,

        space: [4.0, 8.0, 12.0, 16.0, 20.0, 24.0, 32.0, 48.0],

        font_size_xs: 11.0,
        font_size_sm: 12.0,
        font_size_md: 14.0,
        font_size_lg: 16.0,
        font_size_xl: 20.0,

        metal_gradient: 0.30,
        metal_rim_alpha: 80,
        metal_shadow_blur: 4.0,
        metal_shadow_alpha: 30,
    }
}
