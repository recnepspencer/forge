use crate::capability::IconId;

use super::denial_receipt::WorthUiPrimitiveContentValueDenialReceipt;
use super::receipt::WorthUiPrimitiveContentItemKind;
use super::schema::{
    WorthUiPrimitiveContentPropSchema, WorthUiPrimitiveContentValueKind, CONTENT_IMAGE_PROP,
    CONTENT_SLOT_PROP,
};

#[derive(Clone, Debug, PartialEq)]
pub(super) enum WorthUiValidatedPrimitiveContentValue {
    Kind(WorthUiPrimitiveContentKind),
    Order(Vec<WorthUiPrimitiveContentItemKind>),
    Text(String),
    IconId(IconId),
    MeasurementToken(String),
    AccessibilityName(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveContentKind {
    Plain,
    Inline,
    Stack,
}

impl WorthUiValidatedPrimitiveContentValue {
    pub(super) fn into_kind(self) -> WorthUiPrimitiveContentKind {
        match self {
            Self::Kind(value) => value,
            _ => unreachable!("content schema value kind guarantees kind"),
        }
    }

    pub(super) fn into_order(self) -> Vec<WorthUiPrimitiveContentItemKind> {
        match self {
            Self::Order(value) => value,
            _ => unreachable!("content schema value kind guarantees order"),
        }
    }

    pub(super) fn into_text(self) -> String {
        match self {
            Self::Text(value) | Self::AccessibilityName(value) => value,
            _ => unreachable!("content schema value kind guarantees text"),
        }
    }

    pub(super) fn into_icon_id(self) -> IconId {
        match self {
            Self::IconId(value) => value,
            _ => unreachable!("content schema value kind guarantees icon id"),
        }
    }

    pub(super) fn into_measurement_token(self) -> String {
        match self {
            Self::MeasurementToken(value) => value,
            _ => unreachable!("content schema value kind guarantees measurement token"),
        }
    }
}

pub(super) fn validate_primitive_content_value(
    surface_id: &str,
    schema: &'static WorthUiPrimitiveContentPropSchema,
    raw_value: String,
) -> Result<WorthUiValidatedPrimitiveContentValue, WorthUiPrimitiveContentValueDenialReceipt> {
    match schema.value_kind() {
        WorthUiPrimitiveContentValueKind::Kind => parse_kind(&raw_value)
            .map(WorthUiValidatedPrimitiveContentValue::Kind)
            .ok_or_else(|| {
                WorthUiPrimitiveContentValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiPrimitiveContentValueKind::Order => parse_order(&raw_value)
            .map(WorthUiValidatedPrimitiveContentValue::Order)
            .ok_or_else(|| {
                WorthUiPrimitiveContentValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiPrimitiveContentValueKind::Text => parse_text(&raw_value)
            .map(WorthUiValidatedPrimitiveContentValue::Text)
            .ok_or_else(|| {
                WorthUiPrimitiveContentValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiPrimitiveContentValueKind::IconId => parse_icon_id(&raw_value)
            .map(WorthUiValidatedPrimitiveContentValue::IconId)
            .ok_or_else(|| {
                WorthUiPrimitiveContentValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiPrimitiveContentValueKind::MeasurementToken => parse_token(&raw_value)
            .map(WorthUiValidatedPrimitiveContentValue::MeasurementToken)
            .ok_or_else(|| {
                WorthUiPrimitiveContentValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiPrimitiveContentValueKind::AccessibilityName => parse_text(&raw_value)
            .map(WorthUiValidatedPrimitiveContentValue::AccessibilityName)
            .ok_or_else(|| {
                WorthUiPrimitiveContentValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiPrimitiveContentValueKind::UnsupportedReference => {
            let mut denial = WorthUiPrimitiveContentValueDenialReceipt::new(
                surface_id,
                schema,
                raw_value.clone(),
                None,
            );
            if schema.prop_key() == CONTENT_IMAGE_PROP || schema.prop_key() == CONTENT_SLOT_PROP {
                return Err(denial);
            }
            denial.attach_source_span(None);
            Err(denial)
        }
        WorthUiPrimitiveContentValueKind::Unknown => {
            Err(WorthUiPrimitiveContentValueDenialReceipt::unknown_prop(
                surface_id,
                "__unknown__",
                raw_value,
                None,
            ))
        }
    }
}

pub(super) fn default_primitive_content_value(
    schema: &'static WorthUiPrimitiveContentPropSchema,
) -> Option<WorthUiValidatedPrimitiveContentValue> {
    let raw = schema.default_value()?.to_owned();
    validate_primitive_content_value("__content_schema_default__", schema, raw).ok()
}

fn parse_kind(value: &str) -> Option<WorthUiPrimitiveContentKind> {
    match value.trim().trim_matches('"') {
        "plain" => Some(WorthUiPrimitiveContentKind::Plain),
        "inline" => Some(WorthUiPrimitiveContentKind::Inline),
        "stack" => Some(WorthUiPrimitiveContentKind::Stack),
        _ => None,
    }
}

fn parse_order(value: &str) -> Option<Vec<WorthUiPrimitiveContentItemKind>> {
    let value = value.trim().trim_matches('"');
    let mut items = Vec::new();
    for part in value.split(',') {
        let item = match part.trim() {
            "text" => WorthUiPrimitiveContentItemKind::Text,
            "icon" => WorthUiPrimitiveContentItemKind::Icon,
            "spacer" => WorthUiPrimitiveContentItemKind::Spacer,
            "badge" => WorthUiPrimitiveContentItemKind::Badge,
            "divider" => WorthUiPrimitiveContentItemKind::Divider,
            _ => return None,
        };
        if !items.contains(&item) {
            items.push(item);
        }
    }
    (!items.is_empty()).then_some(items)
}

fn parse_text(value: &str) -> Option<String> {
    let value = value.trim().trim_matches('"');
    (!value.trim().is_empty()).then(|| value.to_owned())
}

fn parse_icon_id(value: &str) -> Option<IconId> {
    IconId::new(value.trim().trim_matches('"')).ok()
}

fn parse_token(value: &str) -> Option<String> {
    let value = value.trim().trim_matches('"');
    let has_namespace = value.split('.').count() >= 3;
    let valid_chars = value
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-'));
    (has_namespace && valid_chars).then(|| value.to_owned())
}
