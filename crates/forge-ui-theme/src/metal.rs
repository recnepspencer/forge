//! Brushed-gunmetal painting utilities.
//!
//! DOMAIN: Metallic UI surface rendering. Provides both a GPU shader path
//! (via `paint_metal_rect`) and color math utilities (`lerp_color`, `lighten`,
//! `darken`) used throughout the UI.
//! DEPENDENCIES: egui, forge-ui-theme::shader_pipeline.

use egui::{Color32, Rect};

use crate::shader_pipeline::{BlurCaptureCallback, MetalShaderCallback, MetalUniforms};

// ── Color math ───────────────────────────────────────────────────────────────

/// Convert an sRGB Color32 to linear [f32; 4] for GPU shaders.
fn srgb_to_linear(c: Color32) -> [f32; 4] {
    fn ch(v: u8) -> f32 {
        let s = v as f32 / 255.0;
        if s <= 0.04045 {
            s / 12.92
        } else {
            ((s + 0.055) / 1.055).powf(2.4)
        }
    }
    [ch(c.r()), ch(c.g()), ch(c.b()), c.a() as f32 / 255.0]
}

/// Linearly interpolate between two colors.
pub fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    Color32::from_rgba_unmultiplied(
        (a.r() as f32 + (b.r() as f32 - a.r() as f32) * t) as u8,
        (a.g() as f32 + (b.g() as f32 - a.g() as f32) * t) as u8,
        (a.b() as f32 + (b.b() as f32 - a.b() as f32) * t) as u8,
        (a.a() as f32 + (b.a() as f32 - a.a() as f32) * t) as u8,
    )
}

/// Lighten a color by `amount` (0.0 = no change, 1.0 = white).
pub fn lighten(c: Color32, amount: f32) -> Color32 {
    Color32::from_rgb(
        (c.r() as f32 + (255.0 - c.r() as f32) * amount) as u8,
        (c.g() as f32 + (255.0 - c.g() as f32) * amount) as u8,
        (c.b() as f32 + (255.0 - c.b() as f32) * amount) as u8,
    )
}

/// Darken a color by `amount` (0.0 = no change, 1.0 = black).
pub fn darken(c: Color32, amount: f32) -> Color32 {
    let f = 1.0 - amount.clamp(0.0, 1.0);
    Color32::from_rgb(
        (c.r() as f32 * f) as u8,
        (c.g() as f32 * f) as u8,
        (c.b() as f32 * f) as u8,
    )
}

// ── Metallic painting (GPU shader path) ──────────────────────────────────────

/// Options for metallic rect rendering.
pub struct MetalOpts {
    /// How much lighter the top is vs the bottom (0.0..0.25).
    pub gradient_strength: f32,
    /// Shifts the bright band vertically. 0.0 = top, 0.4 = middle.
    pub highlight_shift: f32,
    /// Alpha of the inner-top highlight rim (0.0..1.0).
    pub rim_alpha: f32,
    /// Border radius in pixels.
    pub rounding: f32,
    /// Whether the element is pressed (inverts gradient for concave feel).
    pub pressed: bool,
}

impl Default for MetalOpts {
    fn default() -> Self {
        Self {
            gradient_strength: 0.12,
            highlight_shift: 0.0,
            rim_alpha: 0.08,
            rounding: 8.0,
            pressed: false,
        }
    }
}

/// Emit a GPU shader-rendered metallic rectangle into the painter.
///
/// This adds a `Shape::Callback` that uses the metallic WGSL fragment shader
/// to render a brushed-metal rectangle with gradient, highlight, and rim effects.
pub fn paint_metal_rect(ui: &mut egui::Ui, rect: Rect, base_color: Color32, opts: &MetalOpts) {
    let time = ui.input(|i| i.time) as f32;
    let ppp = ui.ctx().pixels_per_point();

    let uniforms = MetalUniforms {
        rect: [
            rect.min.x * ppp,
            rect.min.y * ppp,
            rect.width() * ppp,
            rect.height() * ppp,
        ],
        base_color: srgb_to_linear(base_color),
        params: [
            opts.gradient_strength,
            opts.highlight_shift,
            opts.rim_alpha,
            opts.rounding * ppp,
        ],
        params2: [
            time,
            if opts.pressed { 1.0 } else { 0.0 },
            ui.input(|i| i.pointer.latest_pos().map_or(-1000.0, |p| p.x * ppp)),
            ui.input(|i| i.pointer.latest_pos().map_or(-1000.0, |p| p.y * ppp)),
        ],
        screen: [
            ui.ctx().screen_rect().width() * ppp,
            ui.ctx().screen_rect().height() * ppp,
            0.0,
            0.0,
        ],
    };

    // Emit the blur-capture callback first — its prepare() submits the
    // two-pass Gaussian blur before the render pass begins, so the blurred
    // backdrop texture is ready when the glass shader runs.
    let blur_trigger = egui_wgpu::Callback::new_paint_callback(rect, BlurCaptureCallback);
    ui.painter().add(blur_trigger);

    let glass_callback =
        egui_wgpu::Callback::new_paint_callback(rect, MetalShaderCallback::new(uniforms));
    ui.painter().add(glass_callback);
}
