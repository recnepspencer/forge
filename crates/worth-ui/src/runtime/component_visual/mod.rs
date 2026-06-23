use crate::capability::{IconId, SurfaceId};
use crate::runtime::{WorthUiAuthoredSurfacePropValue, WorthUiRuntimeFactId, WorthUiRuntimeHost};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiResolvedIcon {
    icon_id: String,
    source_kind: String,
    provider: String,
    source_key: String,
    family: String,
    theme_posture: String,
    accessibility_posture: String,
    color: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiResolvedBoxStyle {
    background_color: String,
    foreground_color: String,
    icon_color: String,
    border_color: String,
    border_width_points: f32,
    border_radius_points: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct WorthUiComponentStyleDefaults<'a> {
    pub(crate) background_color: &'a str,
    pub(crate) foreground_color: &'a str,
    pub(crate) icon_color: &'a str,
    pub(crate) border_color: &'a str,
    pub(crate) border_width_points: f32,
    pub(crate) border_radius_points: f32,
}

pub(crate) fn resolve_component_icon(
    runtime: &WorthUiRuntimeHost,
    surface_id: &SurfaceId,
    key: &str,
    color: &str,
) -> Option<WorthUiResolvedIcon> {
    let icon_id = authored_prop_text(runtime, surface_id, key)?;
    let icon_id = IconId::new(icon_id).ok()?;
    let descriptor = runtime.active_capability_snapshot().icons().get(&icon_id)?;
    let source = descriptor.source()?;
    Some(WorthUiResolvedIcon {
        icon_id: icon_id.as_str().to_owned(),
        source_kind: format!("{:?}", source.kind()),
        provider: source.provider().to_owned(),
        source_key: source.source_key().to_owned(),
        family: format!("{:?}", descriptor.family()),
        theme_posture: format!("{:?}", descriptor.theme_posture()),
        accessibility_posture: format!("{:?}", descriptor.accessibility_posture()),
        color: color.to_owned(),
    })
}

pub(crate) fn resolve_component_box_style(
    runtime: &WorthUiRuntimeHost,
    surface_id: &SurfaceId,
    defaults: WorthUiComponentStyleDefaults<'_>,
) -> WorthUiResolvedBoxStyle {
    resolve_component_box_style_with_prefix(runtime, surface_id, "", defaults)
}

pub(crate) fn resolve_component_box_style_with_prefix(
    runtime: &WorthUiRuntimeHost,
    surface_id: &SurfaceId,
    prefix: &str,
    defaults: WorthUiComponentStyleDefaults<'_>,
) -> WorthUiResolvedBoxStyle {
    WorthUiResolvedBoxStyle {
        background_color: authored_prop_text(
            runtime,
            surface_id,
            &prop_key(prefix, "background_color"),
        )
        .unwrap_or(defaults.background_color)
        .to_owned(),
        foreground_color: authored_prop_text(
            runtime,
            surface_id,
            &prop_key(prefix, "foreground_color"),
        )
        .unwrap_or(defaults.foreground_color)
        .to_owned(),
        icon_color: authored_prop_text(runtime, surface_id, &prop_key(prefix, "icon_color"))
            .unwrap_or(defaults.icon_color)
            .to_owned(),
        border_color: authored_prop_text(runtime, surface_id, &prop_key(prefix, "border_color"))
            .unwrap_or(defaults.border_color)
            .to_owned(),
        border_width_points: authored_prop_number_or_text(
            runtime,
            surface_id,
            &prop_key(prefix, "border_width"),
        )
        .as_deref()
        .and_then(parse_points)
        .unwrap_or(defaults.border_width_points),
        border_radius_points: authored_prop_number_or_text(
            runtime,
            surface_id,
            &prop_key(prefix, "border_radius"),
        )
        .as_deref()
        .and_then(parse_points)
        .unwrap_or(defaults.border_radius_points),
    }
}

fn prop_key(prefix: &str, name: &str) -> String {
    if prefix.is_empty() {
        name.to_owned()
    } else {
        format!("{prefix}_{name}")
    }
}

pub(crate) fn authored_prop_text<'a>(
    runtime: &'a WorthUiRuntimeHost,
    surface_id: &'a SurfaceId,
    key: &str,
) -> Option<&'a str> {
    runtime
        .inspect_active_authored_surface_props(surface_id)
        .find_map(|entry| {
            (entry.key() == key).then_some(match entry.value() {
                WorthUiAuthoredSurfacePropValue::Identifier(value)
                | WorthUiAuthoredSurfacePropValue::StringLiteral(value) => value.as_str(),
                WorthUiAuthoredSurfacePropValue::NumberLiteral(_) => return None,
            })
        })
}

pub(crate) fn authored_prop_number_or_text(
    runtime: &WorthUiRuntimeHost,
    surface_id: &SurfaceId,
    key: &str,
) -> Option<String> {
    runtime
        .inspect_active_authored_surface_props(surface_id)
        .find_map(|entry| {
            (entry.key() == key).then(|| match entry.value() {
                WorthUiAuthoredSurfacePropValue::Identifier(value)
                | WorthUiAuthoredSurfacePropValue::StringLiteral(value) => value.clone(),
                WorthUiAuthoredSurfacePropValue::NumberLiteral(value) => value.to_string(),
            })
        })
}

impl WorthUiResolvedIcon {
    pub fn icon_id(&self) -> &str {
        &self.icon_id
    }

    pub fn source_kind(&self) -> &str {
        &self.source_kind
    }

    pub fn provider(&self) -> &str {
        &self.provider
    }

    pub fn source_key(&self) -> &str {
        &self.source_key
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn theme_posture(&self) -> &str {
        &self.theme_posture
    }

    pub fn accessibility_posture(&self) -> &str {
        &self.accessibility_posture
    }

    pub fn color(&self) -> &str {
        &self.color
    }
}

impl WorthUiResolvedBoxStyle {
    pub fn background_color(&self) -> &str {
        &self.background_color
    }

    pub fn foreground_color(&self) -> &str {
        &self.foreground_color
    }

    pub fn icon_color(&self) -> &str {
        &self.icon_color
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

    pub fn dependency_fact(surface_id: &SurfaceId) -> WorthUiRuntimeFactId {
        WorthUiRuntimeFactId::authored_surface_props(surface_id.as_str())
    }
}

fn parse_points(value: &str) -> Option<f32> {
    value.trim().trim_end_matches("px").parse::<f32>().ok()
}
