use super::denial_receipt::WorthUiInteractionValueDenialReceipt;
use super::payload::{WorthUiInteractionFieldValue, WorthUiInteractionKind};
use super::receipt::WorthUiInteractionReadiness;
use super::schema::{WorthUiInteractionPropSchema, WorthUiInteractionValueKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum WorthUiValidatedInteractionValue {
    Kind(WorthUiInteractionKind),
    Identifier(String),
    Payload(WorthUiInteractionFieldValue),
    Readiness(WorthUiInteractionReadiness),
}

impl WorthUiValidatedInteractionValue {
    pub(super) fn into_kind(self) -> WorthUiInteractionKind {
        match self {
            Self::Kind(value) => value,
            _ => unreachable!("schema value kind guarantees interaction kind"),
        }
    }

    pub(super) fn into_identifier(self) -> String {
        match self {
            Self::Identifier(value) => value,
            _ => unreachable!("schema value kind guarantees identifier"),
        }
    }

    pub(super) fn into_payload(self) -> WorthUiInteractionFieldValue {
        match self {
            Self::Payload(value) => value,
            _ => unreachable!("schema value kind guarantees payload"),
        }
    }

    pub(super) fn into_readiness(self) -> WorthUiInteractionReadiness {
        match self {
            Self::Readiness(value) => value,
            _ => unreachable!("schema value kind guarantees readiness"),
        }
    }
}

pub(super) fn validate_interaction_value(
    surface_id: &str,
    schema: &'static WorthUiInteractionPropSchema,
    raw_value: String,
    source_span: Option<crate::runtime::WorthUiPrimitiveSourceSpan>,
) -> Result<WorthUiValidatedInteractionValue, WorthUiInteractionValueDenialReceipt> {
    match schema.value_kind() {
        WorthUiInteractionValueKind::Kind => parse_kind(&raw_value)
            .map(WorthUiValidatedInteractionValue::Kind)
            .ok_or_else(|| {
                WorthUiInteractionValueDenialReceipt::new(
                    surface_id,
                    schema,
                    raw_value,
                    source_span,
                )
            }),
        WorthUiInteractionValueKind::Identifier => parse_identifier(&raw_value)
            .map(WorthUiValidatedInteractionValue::Identifier)
            .ok_or_else(|| {
                WorthUiInteractionValueDenialReceipt::new(
                    surface_id,
                    schema,
                    raw_value,
                    source_span,
                )
            }),
        WorthUiInteractionValueKind::Payload => parse_payload(&raw_value)
            .map(WorthUiValidatedInteractionValue::Payload)
            .ok_or_else(|| {
                WorthUiInteractionValueDenialReceipt::new(
                    surface_id,
                    schema,
                    raw_value,
                    source_span,
                )
            }),
        WorthUiInteractionValueKind::Readiness => parse_readiness(&raw_value)
            .map(WorthUiValidatedInteractionValue::Readiness)
            .ok_or_else(|| {
                WorthUiInteractionValueDenialReceipt::new(
                    surface_id,
                    schema,
                    raw_value,
                    source_span,
                )
            }),
        WorthUiInteractionValueKind::Unknown => {
            Err(WorthUiInteractionValueDenialReceipt::unknown_prop(
                surface_id,
                "__unknown__",
                raw_value,
                source_span,
            ))
        }
    }
}

fn parse_kind(value: &str) -> Option<WorthUiInteractionKind> {
    match value.trim_matches('"') {
        "click" => Some(WorthUiInteractionKind::Click),
        "submit" => Some(WorthUiInteractionKind::Submit),
        "command" => Some(WorthUiInteractionKind::Command),
        "toggle" => Some(WorthUiInteractionKind::Toggle),
        "open" => Some(WorthUiInteractionKind::Open),
        "focus" => Some(WorthUiInteractionKind::Focus),
        _ => None,
    }
}

fn parse_identifier(value: &str) -> Option<String> {
    let value = value.trim().trim_matches('"');
    let valid = !value.is_empty()
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-'));
    valid.then(|| value.to_owned())
}

fn parse_payload(value: &str) -> Option<WorthUiInteractionFieldValue> {
    let value = value.trim();
    if value.trim_matches('"').is_empty() {
        return None;
    }
    if let Ok(number) = value.parse::<u32>() {
        return Some(WorthUiInteractionFieldValue::Number(number));
    }
    let trimmed = value.trim_matches('"').to_owned();
    if trimmed.split('.').count() >= 2
        && trimmed
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-'))
    {
        Some(WorthUiInteractionFieldValue::Identifier(trimmed))
    } else {
        Some(WorthUiInteractionFieldValue::Text(trimmed))
    }
}

fn parse_readiness(value: &str) -> Option<WorthUiInteractionReadiness> {
    match value.trim_matches('"') {
        "enabled" => Some(WorthUiInteractionReadiness::Enabled),
        "disabled" => Some(WorthUiInteractionReadiness::Disabled),
        _ => None,
    }
}
