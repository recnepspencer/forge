use crate::runtime::live_view::digest::digest_parts;
use crate::runtime::{
    WorthUiFlowLayoutFill, WorthUiFlowLayoutFit, WorthUiLiveViewControlProjectionReceipt,
    WorthUiPrimitiveColor, WorthUiPrimitiveEventCursor,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewControlHostFrameWidthPolicy {
    Hug,
    Fill,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiLiveViewControlHostFrameStyleReceipt {
    padding_top_points: f32,
    padding_right_points: f32,
    padding_bottom_points: f32,
    padding_left_points: f32,
    border_width_points: f32,
    radius_points: f32,
    background_color: WorthUiPrimitiveColor,
    foreground_color: WorthUiPrimitiveColor,
    border_color: WorthUiPrimitiveColor,
    cursor: WorthUiPrimitiveEventCursor,
    width_policy: WorthUiLiveViewControlHostFrameWidthPolicy,
    style_digest: u64,
}

impl WorthUiLiveViewControlHostFrameStyleReceipt {
    pub(super) fn from_receipts(control: &WorthUiLiveViewControlProjectionReceipt) -> Self {
        let padding = control.flow_layout().padding_edges();
        let appearance = control.appearance().resolve_rest();
        let cursor = control.event_geometry().cursor();
        let width_policy =
            width_policy_from_flow(control.flow_layout().fit(), control.flow_layout().fill());
        let background_color = appearance.background_color();
        let foreground_color = appearance.text_color();
        let border_color = appearance.border_color();
        let border_width_points = appearance.border_width_points();
        let radius_points = appearance.radius_points();
        let style_digest = digest_parts([
            control.control_id().to_owned(),
            control.flow_layout().receipt_digest().to_string(),
            control.appearance().receipt_digest().to_string(),
            control.event_geometry().receipt_digest().to_string(),
            format!(
                "padding:{}:{}:{}:{}",
                padding.top(),
                padding.right(),
                padding.bottom(),
                padding.left()
            ),
            format!("border:{border_width_points}"),
            format!("radius:{radius_points}"),
            background_color.hex_triplet(),
            foreground_color.hex_triplet(),
            border_color.hex_triplet(),
            cursor.token().to_owned(),
            width_policy.token().to_owned(),
        ]);
        Self {
            padding_top_points: padding.top(),
            padding_right_points: padding.right(),
            padding_bottom_points: padding.bottom(),
            padding_left_points: padding.left(),
            border_width_points,
            radius_points,
            background_color,
            foreground_color,
            border_color,
            cursor,
            width_policy,
            style_digest,
        }
    }

    pub fn padding_top_points(&self) -> f32 {
        self.padding_top_points
    }

    pub fn padding_right_points(&self) -> f32 {
        self.padding_right_points
    }

    pub fn padding_bottom_points(&self) -> f32 {
        self.padding_bottom_points
    }

    pub fn padding_left_points(&self) -> f32 {
        self.padding_left_points
    }

    pub fn border_width_points(&self) -> f32 {
        self.border_width_points
    }

    pub fn radius_points(&self) -> f32 {
        self.radius_points
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

    pub fn cursor(&self) -> WorthUiPrimitiveEventCursor {
        self.cursor
    }

    pub fn width_policy(&self) -> WorthUiLiveViewControlHostFrameWidthPolicy {
        self.width_policy
    }

    pub fn style_digest(&self) -> u64 {
        self.style_digest
    }
}

impl WorthUiLiveViewControlHostFrameWidthPolicy {
    pub fn token(self) -> &'static str {
        match self {
            Self::Hug => "hug",
            Self::Fill => "fill",
        }
    }
}

impl WorthUiPrimitiveEventCursor {
    fn token(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Pointer => "pointer",
            Self::Text => "text",
            Self::Grab => "grab",
            Self::Grabbing => "grabbing",
            Self::Resize => "resize",
        }
    }
}

fn width_policy_from_flow(
    fit: WorthUiFlowLayoutFit,
    fill: WorthUiFlowLayoutFill,
) -> WorthUiLiveViewControlHostFrameWidthPolicy {
    if fit == WorthUiFlowLayoutFit::Fill
        || matches!(
            fill,
            WorthUiFlowLayoutFill::Width | WorthUiFlowLayoutFill::Both
        )
    {
        WorthUiLiveViewControlHostFrameWidthPolicy::Fill
    } else {
        WorthUiLiveViewControlHostFrameWidthPolicy::Hug
    }
}
