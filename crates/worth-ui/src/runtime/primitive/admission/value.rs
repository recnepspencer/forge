use crate::runtime::{
    WorthUiPrimitiveAlign, WorthUiPrimitiveAuthoredValueKind, WorthUiPrimitiveColor,
    WorthUiPrimitiveCursorPosture, WorthUiPrimitiveFocusPosture, WorthUiPrimitiveInteractionKind,
    WorthUiPrimitiveMotionEasing, WorthUiPrimitiveMotionKind, WorthUiPrimitiveMotionTarget,
    WorthUiPrimitiveValueDenialReceipt,
};

use super::super::WorthUiPrimitiveAuthoredPropSchema;

#[derive(Clone, Debug, PartialEq)]
pub(super) enum WorthUiValidatedPrimitiveValue {
    Text(String),
    MeasurementToken(String),
    Color(WorthUiPrimitiveColor),
    Align(WorthUiPrimitiveAlign),
    InteractionKind(WorthUiPrimitiveInteractionKind),
    Cursor(WorthUiPrimitiveCursorPosture),
    Focus(WorthUiPrimitiveFocusPosture),
    Boolean(bool),
    MotionKind(WorthUiPrimitiveMotionKind),
    MotionTarget(WorthUiPrimitiveMotionTarget),
    Easing(WorthUiPrimitiveMotionEasing),
}

impl WorthUiValidatedPrimitiveValue {
    pub(super) fn into_text(self) -> String {
        match self {
            Self::Text(value) => value,
            _ => unreachable!("schema value kind guarantees text"),
        }
    }

    pub(super) fn into_measurement_token(self) -> String {
        match self {
            Self::MeasurementToken(value) => value,
            _ => unreachable!("schema value kind guarantees measurement token"),
        }
    }

    pub(super) fn into_color(self) -> WorthUiPrimitiveColor {
        match self {
            Self::Color(value) => value,
            _ => unreachable!("schema value kind guarantees color"),
        }
    }

    pub(super) fn into_align(self) -> WorthUiPrimitiveAlign {
        match self {
            Self::Align(value) => value,
            _ => unreachable!("schema value kind guarantees align"),
        }
    }

    pub(super) fn into_interaction_kind(self) -> WorthUiPrimitiveInteractionKind {
        match self {
            Self::InteractionKind(value) => value,
            _ => unreachable!("schema value kind guarantees interaction kind"),
        }
    }

    pub(super) fn into_cursor(self) -> WorthUiPrimitiveCursorPosture {
        match self {
            Self::Cursor(value) => value,
            _ => unreachable!("schema value kind guarantees cursor"),
        }
    }

    pub(super) fn into_focus(self) -> WorthUiPrimitiveFocusPosture {
        match self {
            Self::Focus(value) => value,
            _ => unreachable!("schema value kind guarantees focus"),
        }
    }

    pub(super) fn into_boolean(self) -> bool {
        match self {
            Self::Boolean(value) => value,
            _ => unreachable!("schema value kind guarantees boolean"),
        }
    }

    pub(super) fn into_motion_kind(self) -> WorthUiPrimitiveMotionKind {
        match self {
            Self::MotionKind(value) => value,
            _ => unreachable!("schema value kind guarantees motion kind"),
        }
    }

    pub(super) fn into_motion_target(self) -> WorthUiPrimitiveMotionTarget {
        match self {
            Self::MotionTarget(value) => value,
            _ => unreachable!("schema value kind guarantees motion target"),
        }
    }

    pub(super) fn into_easing(self) -> WorthUiPrimitiveMotionEasing {
        match self {
            Self::Easing(value) => value,
            _ => unreachable!("schema value kind guarantees easing"),
        }
    }
}

pub(super) fn validate_primitive_value(
    surface_id: &str,
    schema: &'static WorthUiPrimitiveAuthoredPropSchema,
    raw_value: String,
) -> Result<WorthUiValidatedPrimitiveValue, WorthUiPrimitiveValueDenialReceipt> {
    match schema.value_kind() {
        WorthUiPrimitiveAuthoredValueKind::Text => Ok(WorthUiValidatedPrimitiveValue::Text(
            raw_value.trim_matches('"').to_owned(),
        )),
        WorthUiPrimitiveAuthoredValueKind::MeasurementToken => parse_token(&raw_value)
            .map(WorthUiValidatedPrimitiveValue::MeasurementToken)
            .ok_or_else(|| {
                WorthUiPrimitiveValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiPrimitiveAuthoredValueKind::Color => parse_color(&raw_value)
            .map(WorthUiValidatedPrimitiveValue::Color)
            .ok_or_else(|| {
                WorthUiPrimitiveValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiPrimitiveAuthoredValueKind::Align => parse_align(&raw_value)
            .map(WorthUiValidatedPrimitiveValue::Align)
            .ok_or_else(|| {
                WorthUiPrimitiveValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiPrimitiveAuthoredValueKind::InteractionKind => parse_interaction_kind(&raw_value)
            .map(WorthUiValidatedPrimitiveValue::InteractionKind)
            .ok_or_else(|| {
                WorthUiPrimitiveValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiPrimitiveAuthoredValueKind::Cursor => parse_cursor(&raw_value)
            .map(WorthUiValidatedPrimitiveValue::Cursor)
            .ok_or_else(|| {
                WorthUiPrimitiveValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiPrimitiveAuthoredValueKind::Focus => parse_focus(&raw_value)
            .map(WorthUiValidatedPrimitiveValue::Focus)
            .ok_or_else(|| {
                WorthUiPrimitiveValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiPrimitiveAuthoredValueKind::Boolean => parse_boolean(&raw_value)
            .map(WorthUiValidatedPrimitiveValue::Boolean)
            .ok_or_else(|| {
                WorthUiPrimitiveValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiPrimitiveAuthoredValueKind::MotionKind => parse_motion_kind(&raw_value)
            .map(WorthUiValidatedPrimitiveValue::MotionKind)
            .ok_or_else(|| {
                WorthUiPrimitiveValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiPrimitiveAuthoredValueKind::MotionTarget => parse_motion_target(&raw_value)
            .map(WorthUiValidatedPrimitiveValue::MotionTarget)
            .ok_or_else(|| {
                WorthUiPrimitiveValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiPrimitiveAuthoredValueKind::Easing => parse_easing(&raw_value)
            .map(WorthUiValidatedPrimitiveValue::Easing)
            .ok_or_else(|| {
                WorthUiPrimitiveValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiPrimitiveAuthoredValueKind::Unknown => {
            Err(WorthUiPrimitiveValueDenialReceipt::unknown_prop(
                surface_id,
                "__unknown__",
                raw_value,
                None,
            ))
        }
    }
}

pub(super) fn default_primitive_value(
    schema: &'static WorthUiPrimitiveAuthoredPropSchema,
) -> WorthUiValidatedPrimitiveValue {
    validate_primitive_value(
        "__schema_default__",
        schema,
        schema.default_value().to_owned(),
    )
    .expect("primitive schema defaults must admit")
}

fn parse_token(value: &str) -> Option<String> {
    let value = value.trim().trim_matches('"');
    let has_namespace = value.split('.').count() >= 3;
    let valid_chars = value
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-'));
    (has_namespace && valid_chars).then(|| value.to_owned())
}

fn parse_color(value: &str) -> Option<WorthUiPrimitiveColor> {
    let hex = value.strip_prefix('#').unwrap_or(value);
    if hex.len() != 6 {
        return None;
    }
    Some(WorthUiPrimitiveColor::new(
        u8::from_str_radix(&hex[0..2], 16).ok()?,
        u8::from_str_radix(&hex[2..4], 16).ok()?,
        u8::from_str_radix(&hex[4..6], 16).ok()?,
    ))
}

fn parse_align(value: &str) -> Option<WorthUiPrimitiveAlign> {
    match value.trim_matches('"') {
        "start" => Some(WorthUiPrimitiveAlign::Start),
        "center" => Some(WorthUiPrimitiveAlign::Center),
        "end" => Some(WorthUiPrimitiveAlign::End),
        _ => None,
    }
}

fn parse_interaction_kind(value: &str) -> Option<WorthUiPrimitiveInteractionKind> {
    match value.trim_matches('"') {
        "none" => Some(WorthUiPrimitiveInteractionKind::None),
        "submit" => Some(WorthUiPrimitiveInteractionKind::Submit),
        _ => None,
    }
}

fn parse_cursor(value: &str) -> Option<WorthUiPrimitiveCursorPosture> {
    match value.trim_matches('"') {
        "default" => Some(WorthUiPrimitiveCursorPosture::Default),
        "pointer" => Some(WorthUiPrimitiveCursorPosture::Pointer),
        _ => None,
    }
}

fn parse_focus(value: &str) -> Option<WorthUiPrimitiveFocusPosture> {
    match value.trim_matches('"') {
        "none" => Some(WorthUiPrimitiveFocusPosture::None),
        "focusable" => Some(WorthUiPrimitiveFocusPosture::Focusable),
        _ => None,
    }
}

fn parse_boolean(value: &str) -> Option<bool> {
    match value.trim_matches('"') {
        "false" => Some(false),
        "true" => Some(true),
        _ => None,
    }
}

fn parse_motion_kind(value: &str) -> Option<WorthUiPrimitiveMotionKind> {
    match value.trim_matches('"') {
        "none" => Some(WorthUiPrimitiveMotionKind::None),
        "transition" => Some(WorthUiPrimitiveMotionKind::Transition),
        _ => None,
    }
}

fn parse_motion_target(value: &str) -> Option<WorthUiPrimitiveMotionTarget> {
    match value.trim_matches('"') {
        "primitive_background" => Some(WorthUiPrimitiveMotionTarget::Background),
        "primitive_foreground" => Some(WorthUiPrimitiveMotionTarget::Foreground),
        "primitive_radius" => Some(WorthUiPrimitiveMotionTarget::Radius),
        _ => None,
    }
}

fn parse_easing(value: &str) -> Option<WorthUiPrimitiveMotionEasing> {
    match value.trim_matches('"') {
        "linear" => Some(WorthUiPrimitiveMotionEasing::Linear),
        "standard" => Some(WorthUiPrimitiveMotionEasing::Standard),
        "ease_in" => Some(WorthUiPrimitiveMotionEasing::EaseIn),
        "ease_out" => Some(WorthUiPrimitiveMotionEasing::EaseOut),
        _ => None,
    }
}
