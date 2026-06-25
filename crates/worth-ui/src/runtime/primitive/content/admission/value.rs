use crate::capability::{IconId, ImageAssetId};

use super::super::participation::WorthUiPrimitiveContentParticipationPosture;
use super::super::receipt::WorthUiPrimitiveContentItemKind;
use super::denial_receipt::WorthUiPrimitiveContentValueDenialReceipt;
use super::schema::{
    WorthUiPrimitiveContentPropSchema, WorthUiPrimitiveContentValueKind, CONTENT_SLOT_PROP,
};

#[derive(Clone, Debug, PartialEq)]
pub(super) enum WorthUiValidatedPrimitiveContentValue {
    Kind(WorthUiPrimitiveContentKind),
    Order(Vec<WorthUiPrimitiveContentItemKind>),
    Text(String),
    IconId(IconId),
    ImageAsset(ImageAssetId),
    MeasurementToken(String),
    AccessibilityName(String),
    ContentRole(WorthUiPrimitiveContentRole),
    ParticipationPosture(WorthUiPrimitiveContentParticipationPosture),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveContentKind {
    Plain,
    Inline,
    Stack,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveContentRole {
    Body,
    Label,
    HelperText,
    ErrorText,
    PrefixAdornment,
    SuffixAdornment,
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

    pub(super) fn into_image_asset(self) -> ImageAssetId {
        match self {
            Self::ImageAsset(value) => value,
            _ => unreachable!("content schema value kind guarantees image asset"),
        }
    }

    pub(super) fn into_measurement_token(self) -> String {
        match self {
            Self::MeasurementToken(value) => value,
            _ => unreachable!("content schema value kind guarantees measurement token"),
        }
    }

    pub(super) fn into_participation_posture(self) -> WorthUiPrimitiveContentParticipationPosture {
        match self {
            Self::ParticipationPosture(value) => value,
            _ => unreachable!("content schema value kind guarantees participation posture"),
        }
    }

    pub(super) fn into_content_role(self) -> WorthUiPrimitiveContentRole {
        match self {
            Self::ContentRole(value) => value,
            _ => unreachable!("content schema value kind guarantees content role"),
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
        WorthUiPrimitiveContentValueKind::ImageAsset => parse_image_asset(&raw_value)
            .map(WorthUiValidatedPrimitiveContentValue::ImageAsset)
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
        WorthUiPrimitiveContentValueKind::ContentRole => parse_content_role(&raw_value)
            .map(WorthUiValidatedPrimitiveContentValue::ContentRole)
            .ok_or_else(|| {
                WorthUiPrimitiveContentValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiPrimitiveContentValueKind::ParticipationPosture => {
            parse_participation_posture(&raw_value)
                .map(WorthUiValidatedPrimitiveContentValue::ParticipationPosture)
                .ok_or_else(|| {
                    WorthUiPrimitiveContentValueDenialReceipt::new(
                        surface_id, schema, raw_value, None,
                    )
                })
        }
        WorthUiPrimitiveContentValueKind::UnsupportedReference => {
            let mut denial = WorthUiPrimitiveContentValueDenialReceipt::new(
                surface_id,
                schema,
                raw_value.clone(),
                None,
            );
            if schema.prop_key() == CONTENT_SLOT_PROP {
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
            "image" => WorthUiPrimitiveContentItemKind::Image,
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

fn parse_image_asset(value: &str) -> Option<ImageAssetId> {
    ImageAssetId::new(value.trim().trim_matches('"')).ok()
}

fn parse_token(value: &str) -> Option<String> {
    let value = value.trim().trim_matches('"');
    let has_namespace = value.split('.').count() >= 3;
    let valid_chars = value
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-'));
    (has_namespace && valid_chars).then(|| value.to_owned())
}

fn parse_participation_posture(value: &str) -> Option<WorthUiPrimitiveContentParticipationPosture> {
    match value.trim().trim_matches('"') {
        "present" => Some(WorthUiPrimitiveContentParticipationPosture::Present),
        "absent" => Some(WorthUiPrimitiveContentParticipationPosture::Absent),
        "hidden_from_paint" => Some(WorthUiPrimitiveContentParticipationPosture::HiddenFromPaint),
        "hidden_from_accessibility" => {
            Some(WorthUiPrimitiveContentParticipationPosture::HiddenFromAccessibility)
        }
        "inert" => Some(WorthUiPrimitiveContentParticipationPosture::Inert),
        "loading" => Some(WorthUiPrimitiveContentParticipationPosture::Loading),
        "unsupported" => Some(WorthUiPrimitiveContentParticipationPosture::Unsupported),
        "denied" => Some(WorthUiPrimitiveContentParticipationPosture::Denied),
        _ => None,
    }
}

fn parse_content_role(value: &str) -> Option<WorthUiPrimitiveContentRole> {
    match value.trim().trim_matches('"') {
        "body" => Some(WorthUiPrimitiveContentRole::Body),
        "label" => Some(WorthUiPrimitiveContentRole::Label),
        "helper_text" => Some(WorthUiPrimitiveContentRole::HelperText),
        "error_text" => Some(WorthUiPrimitiveContentRole::ErrorText),
        "prefix_adornment" => Some(WorthUiPrimitiveContentRole::PrefixAdornment),
        "suffix_adornment" => Some(WorthUiPrimitiveContentRole::SuffixAdornment),
        _ => None,
    }
}

impl WorthUiPrimitiveContentRole {
    pub fn token(self) -> &'static str {
        match self {
            Self::Body => "body",
            Self::Label => "label",
            Self::HelperText => "helper_text",
            Self::ErrorText => "error_text",
            Self::PrefixAdornment => "prefix_adornment",
            Self::SuffixAdornment => "suffix_adornment",
        }
    }
}
