use crate::capability::SurfaceId;
use crate::runtime::{WorthUiResolvedIcon, WorthUiRuntimeFactId, WorthUiRuntimeHost};

use super::component_visual::{
    authored_prop_number_or_text, authored_prop_text, resolve_component_icon,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiInlineContentReceipt {
    items: Vec<WorthUiInlineContentItem>,
    gap_points: f32,
    dependency_fact: WorthUiRuntimeFactId,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WorthUiInlineContentItem {
    Icon(WorthUiInlineIconItem),
    Text(WorthUiInlineTextItem),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiInlineIconItem {
    icon: WorthUiResolvedIcon,
    size_points: f32,
    stroke_width_points: f32,
    rest_style: WorthUiInlineIconStyle,
    pressed_style: WorthUiInlineIconStyle,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiInlineIconStyle {
    color: String,
    background_color: String,
    border_color: String,
    border_width_points: f32,
    border_radius_points: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiInlineTextItem {
    text: String,
    color: String,
    pressed_color: String,
    size_points: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct WorthUiInlineContentDefaults<'a> {
    pub(crate) text: &'a str,
    pub(crate) text_color: &'a str,
    pub(crate) icon_color: &'a str,
    pub(crate) text_size_points: f32,
    pub(crate) icon_size_points: f32,
    pub(crate) icon_stroke_width_points: f32,
    pub(crate) gap_points: f32,
}

pub(crate) fn resolve_inline_content(
    runtime: &WorthUiRuntimeHost,
    surface_id: &SurfaceId,
    defaults: WorthUiInlineContentDefaults<'_>,
) -> WorthUiInlineContentReceipt {
    let text = authored_prop_text(runtime, surface_id, "label")
        .unwrap_or(defaults.text)
        .to_owned();
    let text_color = authored_prop_text(runtime, surface_id, "foreground_color")
        .unwrap_or(defaults.text_color)
        .to_owned();
    let pressed_text_color = authored_prop_text(runtime, surface_id, "pressed_foreground_color")
        .unwrap_or(&text_color)
        .to_owned();
    let text_size_points = authored_prop_number_or_text(runtime, surface_id, "text_size")
        .as_deref()
        .and_then(parse_points)
        .unwrap_or(defaults.text_size_points);
    let icon_size_points = authored_prop_number_or_text(runtime, surface_id, "icon_size")
        .as_deref()
        .and_then(parse_points)
        .unwrap_or(defaults.icon_size_points);
    let icon_stroke_width_points =
        authored_prop_number_or_text(runtime, surface_id, "icon_stroke_width")
            .as_deref()
            .and_then(parse_points)
            .unwrap_or(defaults.icon_stroke_width_points);
    let rest_style = WorthUiInlineIconStyle {
        color: authored_prop_text(runtime, surface_id, "icon_color")
            .unwrap_or(defaults.icon_color)
            .to_owned(),
        background_color: authored_prop_text(runtime, surface_id, "icon_background_color")
            .unwrap_or("transparent")
            .to_owned(),
        border_color: authored_prop_text(runtime, surface_id, "icon_border_color")
            .unwrap_or("transparent")
            .to_owned(),
        border_width_points: authored_prop_number_or_text(runtime, surface_id, "icon_border_width")
            .as_deref()
            .and_then(parse_points)
            .unwrap_or(0.0),
        border_radius_points: authored_prop_number_or_text(
            runtime,
            surface_id,
            "icon_border_radius",
        )
        .as_deref()
        .and_then(parse_points)
        .unwrap_or(0.0),
    };
    let pressed_style = WorthUiInlineIconStyle {
        color: authored_prop_text(runtime, surface_id, "pressed_icon_color")
            .unwrap_or(rest_style.color())
            .to_owned(),
        background_color: authored_prop_text(runtime, surface_id, "pressed_icon_background_color")
            .unwrap_or(rest_style.background_color())
            .to_owned(),
        border_color: authored_prop_text(runtime, surface_id, "pressed_icon_border_color")
            .unwrap_or(rest_style.border_color())
            .to_owned(),
        border_width_points: authored_prop_number_or_text(
            runtime,
            surface_id,
            "pressed_icon_border_width",
        )
        .as_deref()
        .and_then(parse_points)
        .unwrap_or(rest_style.border_width_points()),
        border_radius_points: authored_prop_number_or_text(
            runtime,
            surface_id,
            "pressed_icon_border_radius",
        )
        .as_deref()
        .and_then(parse_points)
        .unwrap_or(rest_style.border_radius_points()),
    };
    let gap_points = authored_prop_number_or_text(runtime, surface_id, "content_gap")
        .as_deref()
        .and_then(parse_points)
        .unwrap_or(defaults.gap_points);

    let mut items = Vec::new();
    if let Some(icon) = resolve_component_icon(runtime, surface_id, "icon", defaults.icon_color) {
        items.push(WorthUiInlineContentItem::Icon(WorthUiInlineIconItem {
            icon,
            size_points: icon_size_points,
            stroke_width_points: icon_stroke_width_points,
            rest_style,
            pressed_style,
        }));
    }
    if !text.is_empty() {
        items.push(WorthUiInlineContentItem::Text(WorthUiInlineTextItem {
            text,
            color: text_color,
            pressed_color: pressed_text_color,
            size_points: text_size_points,
        }));
    }

    WorthUiInlineContentReceipt {
        items,
        gap_points,
        dependency_fact: WorthUiRuntimeFactId::authored_surface_props(surface_id.as_str()),
    }
}

impl WorthUiInlineContentReceipt {
    pub fn items(&self) -> &[WorthUiInlineContentItem] {
        &self.items
    }

    pub fn gap_points(&self) -> f32 {
        self.gap_points
    }

    pub fn dependency_fact(&self) -> &WorthUiRuntimeFactId {
        &self.dependency_fact
    }
}

impl WorthUiInlineIconItem {
    pub fn icon(&self) -> &WorthUiResolvedIcon {
        &self.icon
    }

    pub fn size_points(&self) -> f32 {
        self.size_points
    }

    pub fn stroke_width_points(&self) -> f32 {
        self.stroke_width_points
    }

    pub fn rest_style(&self) -> &WorthUiInlineIconStyle {
        &self.rest_style
    }

    pub fn pressed_style(&self) -> &WorthUiInlineIconStyle {
        &self.pressed_style
    }
}

impl WorthUiInlineIconStyle {
    pub fn color(&self) -> &str {
        &self.color
    }

    pub fn background_color(&self) -> &str {
        &self.background_color
    }

    pub fn border_color(&self) -> &str {
        &self.border_color
    }

    pub fn border_width_points(&self) -> f32 {
        self.border_width_points
    }

    pub fn border_radius_points(&self) -> f32 {
        self.border_radius_points
    }
}

impl WorthUiInlineTextItem {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn color(&self) -> &str {
        &self.color
    }

    pub fn pressed_color(&self) -> &str {
        &self.pressed_color
    }

    pub fn size_points(&self) -> f32 {
        self.size_points
    }
}

fn parse_points(value: &str) -> Option<f32> {
    value.trim().trim_end_matches("px").parse::<f32>().ok()
}
