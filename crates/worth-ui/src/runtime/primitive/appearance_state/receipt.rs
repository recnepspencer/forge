use super::digest::hash_text;
use crate::runtime::WorthUiPrimitiveColor;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthUiAppearanceStateName {
    Rest,
    Hover,
    Pressed,
    Focus,
    Disabled,
    Selected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiAppearanceStatePosture {
    hovered: bool,
    pressed: bool,
    focused: bool,
    disabled: bool,
    selected: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiAppearanceStateFieldSet {
    background_color: Option<WorthUiPrimitiveColor>,
    foreground_color: Option<WorthUiPrimitiveColor>,
    border_color: Option<WorthUiPrimitiveColor>,
    border_width_points: Option<f32>,
    radius_points: Option<f32>,
    opacity: Option<f32>,
    focus_ring_color: Option<WorthUiPrimitiveColor>,
    focus_ring_width_points: Option<f32>,
    icon_color: Option<WorthUiPrimitiveColor>,
    text_color: Option<WorthUiPrimitiveColor>,
    typography_token: Option<String>,
    text_size_points: Option<f32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiStatefulAppearanceRecipeReceipt {
    rest: WorthUiAppearanceStateFieldSet,
    hover: WorthUiAppearanceStateFieldSet,
    pressed: WorthUiAppearanceStateFieldSet,
    focus: WorthUiAppearanceStateFieldSet,
    disabled: WorthUiAppearanceStateFieldSet,
    selected: WorthUiAppearanceStateFieldSet,
    receipt_digest: u64,
}

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

impl WorthUiAppearanceStatePosture {
    pub const fn rest() -> Self {
        Self {
            hovered: false,
            pressed: false,
            focused: false,
            disabled: false,
            selected: false,
        }
    }

    pub const fn observed(
        hovered: bool,
        pressed: bool,
        focused: bool,
        disabled: bool,
        selected: bool,
    ) -> Self {
        Self {
            hovered,
            pressed,
            focused,
            disabled,
            selected,
        }
    }

    pub fn hovered(self) -> bool {
        self.hovered
    }

    pub fn pressed(self) -> bool {
        self.pressed
    }

    pub fn focused(self) -> bool {
        self.focused
    }

    pub fn disabled(self) -> bool {
        self.disabled
    }

    pub fn selected(self) -> bool {
        self.selected
    }
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

    fn overlay(&mut self, other: &Self) {
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

impl WorthUiStatefulAppearanceRecipeReceipt {
    pub(crate) fn new(
        rest: WorthUiAppearanceStateFieldSet,
        hover: WorthUiAppearanceStateFieldSet,
        pressed: WorthUiAppearanceStateFieldSet,
        focus: WorthUiAppearanceStateFieldSet,
        disabled: WorthUiAppearanceStateFieldSet,
        selected: WorthUiAppearanceStateFieldSet,
        receipt_digest: u64,
    ) -> Self {
        Self {
            rest,
            hover,
            pressed,
            focus,
            disabled,
            selected,
            receipt_digest,
        }
    }

    pub fn resolve_active(
        &self,
        posture: WorthUiAppearanceStatePosture,
    ) -> WorthUiResolvedAppearanceStateReceipt {
        let mut active = self.rest.clone();
        let mut active_states = vec![WorthUiAppearanceStateName::Rest];
        if posture.disabled() {
            active.overlay(&self.disabled);
            active_states.push(WorthUiAppearanceStateName::Disabled);
            return WorthUiResolvedAppearanceStateReceipt::from_fields(active_states, active);
        }
        if posture.selected() {
            active.overlay(&self.selected);
            active_states.push(WorthUiAppearanceStateName::Selected);
        }
        if posture.hovered() {
            active.overlay(&self.hover);
            active_states.push(WorthUiAppearanceStateName::Hover);
        }
        if posture.pressed() {
            active.overlay(&self.pressed);
            active_states.push(WorthUiAppearanceStateName::Pressed);
        }
        if posture.focused() {
            active.overlay(&self.focus);
            active_states.push(WorthUiAppearanceStateName::Focus);
        }
        WorthUiResolvedAppearanceStateReceipt::from_fields(active_states, active)
    }

    pub fn rest(&self) -> &WorthUiAppearanceStateFieldSet {
        &self.rest
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiResolvedAppearanceStateReceipt {
    fn from_fields(
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
