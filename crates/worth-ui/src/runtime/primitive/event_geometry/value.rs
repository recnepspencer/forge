use super::receipt::{
    WorthUiPrimitiveEventContainment, WorthUiPrimitiveEventCursor, WorthUiPrimitiveHitArea,
    WorthUiPrimitivePointerCapture,
};
use super::schema::{WorthUiEventGeometryPropSchema, WorthUiEventGeometryValueKind};
use super::WorthUiEventGeometryValueDenialReceipt;

#[derive(Clone, Debug, PartialEq)]
pub(super) enum WorthUiValidatedEventGeometryValue {
    Cursor(WorthUiPrimitiveEventCursor),
    HitArea(WorthUiPrimitiveHitArea),
    MeasurementToken(String),
    Containment(WorthUiPrimitiveEventContainment),
    Capture(WorthUiPrimitivePointerCapture),
}

impl WorthUiValidatedEventGeometryValue {
    pub(super) fn into_cursor(self) -> WorthUiPrimitiveEventCursor {
        match self {
            Self::Cursor(value) => value,
            _ => unreachable!("event geometry schema value kind guarantees cursor"),
        }
    }

    pub(super) fn into_hit_area(self) -> WorthUiPrimitiveHitArea {
        match self {
            Self::HitArea(value) => value,
            _ => unreachable!("event geometry schema value kind guarantees hit area"),
        }
    }

    pub(super) fn into_measurement_token(self) -> String {
        match self {
            Self::MeasurementToken(value) => value,
            _ => unreachable!("event geometry schema value kind guarantees measurement token"),
        }
    }

    pub(super) fn into_containment(self) -> WorthUiPrimitiveEventContainment {
        match self {
            Self::Containment(value) => value,
            _ => unreachable!("event geometry schema value kind guarantees containment"),
        }
    }

    pub(super) fn into_capture(self) -> WorthUiPrimitivePointerCapture {
        match self {
            Self::Capture(value) => value,
            _ => unreachable!("event geometry schema value kind guarantees capture"),
        }
    }
}

pub(super) fn validate_event_geometry_value(
    surface_id: &str,
    schema: &'static WorthUiEventGeometryPropSchema,
    raw_value: String,
) -> Result<WorthUiValidatedEventGeometryValue, WorthUiEventGeometryValueDenialReceipt> {
    match schema.value_kind() {
        WorthUiEventGeometryValueKind::Cursor => parse_cursor(&raw_value)
            .map(WorthUiValidatedEventGeometryValue::Cursor)
            .ok_or_else(|| {
                WorthUiEventGeometryValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiEventGeometryValueKind::HitArea => parse_hit_area(&raw_value)
            .map(WorthUiValidatedEventGeometryValue::HitArea)
            .ok_or_else(|| {
                WorthUiEventGeometryValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiEventGeometryValueKind::MeasurementToken => parse_measurement_token(&raw_value)
            .map(WorthUiValidatedEventGeometryValue::MeasurementToken)
            .ok_or_else(|| {
                WorthUiEventGeometryValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiEventGeometryValueKind::Containment => parse_containment(&raw_value)
            .map(WorthUiValidatedEventGeometryValue::Containment)
            .ok_or_else(|| {
                WorthUiEventGeometryValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiEventGeometryValueKind::Capture => parse_capture(&raw_value)
            .map(WorthUiValidatedEventGeometryValue::Capture)
            .ok_or_else(|| {
                WorthUiEventGeometryValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiEventGeometryValueKind::Unknown => {
            Err(WorthUiEventGeometryValueDenialReceipt::unknown_prop(
                surface_id,
                "__unknown__",
                raw_value,
                None,
            ))
        }
    }
}

pub(super) fn default_event_geometry_value(
    schema: &'static WorthUiEventGeometryPropSchema,
) -> WorthUiValidatedEventGeometryValue {
    validate_event_geometry_value(
        "__event_geometry_schema_default__",
        schema,
        schema.default_value().to_owned(),
    )
    .expect("event geometry schema defaults must admit")
}

fn parse_cursor(value: &str) -> Option<WorthUiPrimitiveEventCursor> {
    match value.trim_matches('"') {
        "default" => Some(WorthUiPrimitiveEventCursor::Default),
        "pointer" => Some(WorthUiPrimitiveEventCursor::Pointer),
        "text" => Some(WorthUiPrimitiveEventCursor::Text),
        "grab" => Some(WorthUiPrimitiveEventCursor::Grab),
        "grabbing" => Some(WorthUiPrimitiveEventCursor::Grabbing),
        "resize" => Some(WorthUiPrimitiveEventCursor::Resize),
        _ => None,
    }
}

fn parse_hit_area(value: &str) -> Option<WorthUiPrimitiveHitArea> {
    match value.trim_matches('"') {
        "visual_bounds" => Some(WorthUiPrimitiveHitArea::VisualBounds),
        "padded_bounds" => Some(WorthUiPrimitiveHitArea::PaddedBounds),
        "explicit_hit_slop" => Some(WorthUiPrimitiveHitArea::ExplicitHitSlop),
        "disabled_none" => Some(WorthUiPrimitiveHitArea::DisabledNone),
        _ => None,
    }
}

fn parse_measurement_token(value: &str) -> Option<String> {
    let value = value.trim().trim_matches('"');
    let mut chars = value.chars();
    let first = chars.next()?;
    if !first.is_ascii_alphabetic() {
        return None;
    }
    if !chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' || ch == '.') {
        return None;
    }
    value.contains('.').then(|| value.to_owned())
}

fn parse_containment(value: &str) -> Option<WorthUiPrimitiveEventContainment> {
    match value.trim_matches('"') {
        "contain" => Some(WorthUiPrimitiveEventContainment::Contain),
        "bubble" => Some(WorthUiPrimitiveEventContainment::Bubble),
        _ => None,
    }
}

fn parse_capture(value: &str) -> Option<WorthUiPrimitivePointerCapture> {
    match value.trim_matches('"') {
        "none" => Some(WorthUiPrimitivePointerCapture::None),
        "press_drag" => Some(WorthUiPrimitivePointerCapture::PressDrag),
        _ => None,
    }
}
