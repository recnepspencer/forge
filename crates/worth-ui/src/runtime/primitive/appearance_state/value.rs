use crate::runtime::WorthUiPrimitiveColor;

use super::denial_receipt::WorthUiAppearanceStateValueDenialReceipt;
use super::schema::{WorthUiAppearanceStatePropSchema, WorthUiAppearanceStateValueKind};

#[derive(Clone, Debug, PartialEq)]
pub(super) enum WorthUiValidatedAppearanceStateValue {
    Color(WorthUiPrimitiveColor),
    ColorToken(String),
    MeasurementToken(String),
    Opacity(f32),
    TypographyToken(String),
}

impl WorthUiValidatedAppearanceStateValue {
    pub(super) fn into_color_or_token(self) -> Result<WorthUiPrimitiveColor, String> {
        match self {
            Self::Color(value) => Ok(value),
            Self::ColorToken(value) => Err(value),
            _ => unreachable!("appearance schema value kind guarantees color"),
        }
    }

    pub(super) fn into_measurement_token(self) -> String {
        match self {
            Self::MeasurementToken(value) => value,
            _ => unreachable!("appearance schema value kind guarantees measurement token"),
        }
    }

    pub(super) fn into_opacity(self) -> f32 {
        match self {
            Self::Opacity(value) => value,
            _ => unreachable!("appearance schema value kind guarantees opacity"),
        }
    }

    pub(super) fn into_typography_token(self) -> String {
        match self {
            Self::TypographyToken(value) => value,
            _ => unreachable!("appearance schema value kind guarantees typography token"),
        }
    }
}

pub(super) fn validate_appearance_state_value(
    surface_id: &str,
    schema: &WorthUiAppearanceStatePropSchema,
    raw_value: String,
) -> Result<WorthUiValidatedAppearanceStateValue, WorthUiAppearanceStateValueDenialReceipt> {
    match schema.value_kind() {
        WorthUiAppearanceStateValueKind::Color => parse_color(&raw_value)
            .map(|color| match color {
                ParsedAppearanceColor::Literal(value) => {
                    WorthUiValidatedAppearanceStateValue::Color(value)
                }
                ParsedAppearanceColor::Token(value) => {
                    WorthUiValidatedAppearanceStateValue::ColorToken(value)
                }
            })
            .ok_or_else(|| {
                WorthUiAppearanceStateValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiAppearanceStateValueKind::MeasurementToken => parse_token(&raw_value)
            .map(WorthUiValidatedAppearanceStateValue::MeasurementToken)
            .ok_or_else(|| {
                WorthUiAppearanceStateValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiAppearanceStateValueKind::Opacity => parse_opacity(&raw_value)
            .map(WorthUiValidatedAppearanceStateValue::Opacity)
            .ok_or_else(|| {
                WorthUiAppearanceStateValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiAppearanceStateValueKind::TypographyToken => parse_token(&raw_value)
            .map(WorthUiValidatedAppearanceStateValue::TypographyToken)
            .ok_or_else(|| {
                WorthUiAppearanceStateValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiAppearanceStateValueKind::Unknown => {
            Err(WorthUiAppearanceStateValueDenialReceipt::unknown_prop(
                surface_id,
                "__unknown__",
                raw_value,
                None,
            ))
        }
    }
}

pub(super) fn default_appearance_state_value(
    schema: &WorthUiAppearanceStatePropSchema,
) -> Option<WorthUiValidatedAppearanceStateValue> {
    let raw = schema.default_value()?.to_owned();
    validate_appearance_state_value("__appearance_state_schema_default__", schema, raw).ok()
}

enum ParsedAppearanceColor {
    Literal(WorthUiPrimitiveColor),
    Token(String),
}

fn parse_color(value: &str) -> Option<ParsedAppearanceColor> {
    let value = value.trim().trim_matches('"');
    if value == "transparent" {
        return Some(ParsedAppearanceColor::Literal(
            WorthUiPrimitiveColor::transparent(),
        ));
    }
    let hex = value.strip_prefix('#').unwrap_or(value);
    if matches!(hex.len(), 6 | 8) && hex.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Some(ParsedAppearanceColor::Literal(WorthUiPrimitiveColor::new(
            u8::from_str_radix(&hex[0..2], 16).ok()?,
            u8::from_str_radix(&hex[2..4], 16).ok()?,
            u8::from_str_radix(&hex[4..6], 16).ok()?,
        )));
    }
    parse_token(value).map(ParsedAppearanceColor::Token)
}

fn parse_token(value: &str) -> Option<String> {
    let value = value.trim().trim_matches('"');
    let has_namespace = value.split('.').count() >= 3;
    let valid_chars = value
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-'));
    (has_namespace && valid_chars).then(|| value.to_owned())
}

fn parse_opacity(value: &str) -> Option<f32> {
    let value = value.trim().trim_matches('"').parse::<f32>().ok()?;
    (0.0..=1.0).contains(&value).then_some(value)
}
