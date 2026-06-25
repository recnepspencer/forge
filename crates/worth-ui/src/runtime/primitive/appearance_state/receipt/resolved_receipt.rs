use super::super::digest::hash_text;
use super::{field_set::WorthUiAppearanceStateFieldSet, state_name::WorthUiAppearanceStateName};
use crate::runtime::WorthUiPrimitiveColor;

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiResolvedAppearanceStateReceipt {
    active_states: Vec<WorthUiAppearanceStateName>,
    background_color: WorthUiPrimitiveColor,
    foreground_color: WorthUiPrimitiveColor,
    border_color: WorthUiPrimitiveColor,
    border_width_points: f32,
    radius_points: f32,
    opacity: f32,
    focus_ring_color: WorthUiPrimitiveColor,
    focus_ring_width_points: f32,
    icon_color: WorthUiPrimitiveColor,
    text_color: WorthUiPrimitiveColor,
    typography_token: String,
    text_size_points: f32,
    receipt_digest: u64,
}

impl WorthUiResolvedAppearanceStateReceipt {
    pub(crate) fn from_fields(
        active_states: Vec<WorthUiAppearanceStateName>,
        fields: WorthUiAppearanceStateFieldSet,
    ) -> Self {
        let background_color = fields
            .background_color
            .expect("rest background color is always admitted");
        let foreground_color = fields
            .foreground_color
            .expect("rest foreground color is always admitted");
        let border_color = fields.border_color.unwrap_or(background_color);
        let border_width_points = fields.border_width_points.unwrap_or(0.0);
        let radius_points = fields.radius_points.unwrap_or(8.0);
        let opacity = fields.opacity.unwrap_or(1.0);
        let focus_ring_color = fields.focus_ring_color.unwrap_or(foreground_color);
        let focus_ring_width_points = fields.focus_ring_width_points.unwrap_or(0.0);
        let icon_color = fields.icon_color.unwrap_or(foreground_color);
        let text_color = fields.text_color.unwrap_or(foreground_color);
        let typography_token = fields
            .typography_token
            .unwrap_or_else(|| "worth.primitive.appearance.default.font_size".to_owned());
        let text_size_points = fields.text_size_points.unwrap_or(13.0);
        let receipt_digest = hash_text(&format!(
            "resolved-appearance|states:{active_states:?}|bg:{}|fg:{}|border:{}|bw:{}|radius:{}|opacity:{}|focus:{}|fw:{}|icon:{}|text:{}|type:{}:{}",
            background_color.hex_triplet(),
            foreground_color.hex_triplet(),
            border_color.hex_triplet(),
            border_width_points,
            radius_points,
            opacity,
            focus_ring_color.hex_triplet(),
            focus_ring_width_points,
            icon_color.hex_triplet(),
            text_color.hex_triplet(),
            typography_token,
            text_size_points
        ));
        Self {
            active_states,
            background_color,
            foreground_color,
            border_color,
            border_width_points,
            radius_points,
            opacity,
            focus_ring_color,
            focus_ring_width_points,
            icon_color,
            text_color,
            typography_token,
            text_size_points,
            receipt_digest,
        }
    }

    pub fn active_states(&self) -> &[WorthUiAppearanceStateName] {
        &self.active_states
    }

    pub fn background_color(&self) -> WorthUiPrimitiveColor {
        self.background_color
    }

    pub fn foreground_color(&self) -> WorthUiPrimitiveColor {
        self.foreground_color
    }

    pub fn border_color(&self) -> WorthUiPrimitiveColor {
        self.border_color
    }

    pub fn border_width_points(&self) -> f32 {
        self.border_width_points
    }

    pub fn radius_points(&self) -> f32 {
        self.radius_points
    }

    pub fn opacity(&self) -> f32 {
        self.opacity
    }

    pub fn focus_ring_color(&self) -> WorthUiPrimitiveColor {
        self.focus_ring_color
    }

    pub fn focus_ring_width_points(&self) -> f32 {
        self.focus_ring_width_points
    }

    pub fn icon_color(&self) -> WorthUiPrimitiveColor {
        self.icon_color
    }

    pub fn text_color(&self) -> WorthUiPrimitiveColor {
        self.text_color
    }

    pub fn typography_token(&self) -> &str {
        &self.typography_token
    }

    pub fn text_size_points(&self) -> f32 {
        self.text_size_points
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}
