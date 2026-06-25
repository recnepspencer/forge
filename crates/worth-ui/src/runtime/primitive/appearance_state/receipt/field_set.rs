use crate::runtime::WorthUiPrimitiveColor;

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiAppearanceStateFieldSet {
    pub(super) background_color: Option<WorthUiPrimitiveColor>,
    pub(super) foreground_color: Option<WorthUiPrimitiveColor>,
    pub(super) border_color: Option<WorthUiPrimitiveColor>,
    pub(super) border_width_points: Option<f32>,
    pub(super) radius_points: Option<f32>,
    pub(super) opacity: Option<f32>,
    pub(super) focus_ring_color: Option<WorthUiPrimitiveColor>,
    pub(super) focus_ring_width_points: Option<f32>,
    pub(super) icon_color: Option<WorthUiPrimitiveColor>,
    pub(super) text_color: Option<WorthUiPrimitiveColor>,
    pub(super) typography_token: Option<String>,
    pub(super) text_size_points: Option<f32>,
}

impl Default for WorthUiAppearanceStateFieldSet {
    fn default() -> Self {
        Self::empty()
    }
}

impl WorthUiAppearanceStateFieldSet {
    pub fn empty() -> Self {
        Self {
            background_color: None,
            foreground_color: None,
            border_color: None,
            border_width_points: None,
            radius_points: None,
            opacity: None,
            focus_ring_color: None,
            focus_ring_width_points: None,
            icon_color: None,
            text_color: None,
            typography_token: None,
            text_size_points: None,
        }
    }

    pub(crate) fn set_color(&mut self, field: &str, color: WorthUiPrimitiveColor) {
        match field {
            "background" => self.background_color = Some(color),
            "foreground" => self.foreground_color = Some(color),
            "border_color" => self.border_color = Some(color),
            "focus_ring_color" => self.focus_ring_color = Some(color),
            "icon_color" => self.icon_color = Some(color),
            "text_color" => self.text_color = Some(color),
            _ => unreachable!("appearance schema guarantees color field"),
        }
    }

    pub(crate) fn set_points(&mut self, field: &str, points: f32) {
        match field {
            "border_width" => self.border_width_points = Some(points),
            "radius" => self.radius_points = Some(points),
            "focus_ring_width" => self.focus_ring_width_points = Some(points),
            _ => unreachable!("appearance schema guarantees measurement field"),
        }
    }

    pub(crate) fn set_opacity(&mut self, opacity: f32) {
        self.opacity = Some(opacity);
    }

    pub(crate) fn set_typography(&mut self, token: String, text_size_points: f32) {
        self.typography_token = Some(token);
        self.text_size_points = Some(text_size_points);
    }

    pub(super) fn overlay(&mut self, other: &Self) {
        self.background_color = other.background_color.or(self.background_color);
        self.foreground_color = other.foreground_color.or(self.foreground_color);
        self.border_color = other.border_color.or(self.border_color);
        self.border_width_points = other.border_width_points.or(self.border_width_points);
        self.radius_points = other.radius_points.or(self.radius_points);
        self.opacity = other.opacity.or(self.opacity);
        self.focus_ring_color = other.focus_ring_color.or(self.focus_ring_color);
        self.focus_ring_width_points = other
            .focus_ring_width_points
            .or(self.focus_ring_width_points);
        self.icon_color = other.icon_color.or(self.icon_color);
        self.text_color = other.text_color.or(self.text_color);
        self.typography_token = other
            .typography_token
            .clone()
            .or_else(|| self.typography_token.clone());
        self.text_size_points = other.text_size_points.or(self.text_size_points);
    }

    pub fn digest_basis(&self) -> String {
        format!(
            "bg:{:?}|fg:{:?}|border:{:?}|bw:{:?}|radius:{:?}|opacity:{:?}|focus:{:?}|fw:{:?}|icon:{:?}|text:{:?}|type:{:?}:{:?}",
            self.background_color.map(|color| color.hex_triplet()),
            self.foreground_color.map(|color| color.hex_triplet()),
            self.border_color.map(|color| color.hex_triplet()),
            self.border_width_points,
            self.radius_points,
            self.opacity,
            self.focus_ring_color.map(|color| color.hex_triplet()),
            self.focus_ring_width_points,
            self.icon_color.map(|color| color.hex_triplet()),
            self.text_color.map(|color| color.hex_triplet()),
            self.typography_token,
            self.text_size_points
        )
    }
}
