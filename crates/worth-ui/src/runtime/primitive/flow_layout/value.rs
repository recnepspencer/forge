use super::receipt::{
    WorthUiFlowLayoutAlign, WorthUiFlowLayoutCrossAlign, WorthUiFlowLayoutFill,
    WorthUiFlowLayoutFit, WorthUiFlowLayoutKind,
};
use super::schema::{WorthUiFlowLayoutPropSchema, WorthUiFlowLayoutValueKind};
use super::WorthUiFlowLayoutValueDenialReceipt;

#[derive(Clone, Debug, PartialEq)]
pub(super) enum WorthUiValidatedFlowLayoutValue {
    Kind(WorthUiFlowLayoutKind),
    MeasurementToken(String),
    Align(WorthUiFlowLayoutAlign),
    CrossAlign(WorthUiFlowLayoutCrossAlign),
    Fit(WorthUiFlowLayoutFit),
    Fill(WorthUiFlowLayoutFill),
}

impl WorthUiValidatedFlowLayoutValue {
    pub(super) fn into_kind(self) -> WorthUiFlowLayoutKind {
        match self {
            Self::Kind(value) => value,
            _ => unreachable!("flow schema value kind guarantees kind"),
        }
    }

    pub(super) fn into_measurement_token(self) -> String {
        match self {
            Self::MeasurementToken(value) => value,
            _ => unreachable!("flow schema value kind guarantees measurement token"),
        }
    }

    pub(super) fn into_align(self) -> WorthUiFlowLayoutAlign {
        match self {
            Self::Align(value) => value,
            _ => unreachable!("flow schema value kind guarantees align"),
        }
    }

    pub(super) fn into_cross_align(self) -> WorthUiFlowLayoutCrossAlign {
        match self {
            Self::CrossAlign(value) => value,
            _ => unreachable!("flow schema value kind guarantees cross align"),
        }
    }

    pub(super) fn into_fit(self) -> WorthUiFlowLayoutFit {
        match self {
            Self::Fit(value) => value,
            _ => unreachable!("flow schema value kind guarantees fit"),
        }
    }

    pub(super) fn into_fill(self) -> WorthUiFlowLayoutFill {
        match self {
            Self::Fill(value) => value,
            _ => unreachable!("flow schema value kind guarantees fill"),
        }
    }
}

pub(super) fn validate_flow_layout_value(
    surface_id: &str,
    schema: &'static WorthUiFlowLayoutPropSchema,
    raw_value: String,
) -> Result<WorthUiValidatedFlowLayoutValue, WorthUiFlowLayoutValueDenialReceipt> {
    match schema.value_kind() {
        WorthUiFlowLayoutValueKind::Kind => parse_kind(&raw_value)
            .map(WorthUiValidatedFlowLayoutValue::Kind)
            .ok_or_else(|| {
                WorthUiFlowLayoutValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiFlowLayoutValueKind::MeasurementToken => parse_measurement_token(&raw_value)
            .map(WorthUiValidatedFlowLayoutValue::MeasurementToken)
            .ok_or_else(|| {
                WorthUiFlowLayoutValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiFlowLayoutValueKind::Align => parse_align(&raw_value)
            .map(WorthUiValidatedFlowLayoutValue::Align)
            .ok_or_else(|| {
                WorthUiFlowLayoutValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiFlowLayoutValueKind::CrossAlign => parse_cross_align(&raw_value)
            .map(WorthUiValidatedFlowLayoutValue::CrossAlign)
            .ok_or_else(|| {
                WorthUiFlowLayoutValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiFlowLayoutValueKind::Fit => parse_fit(&raw_value)
            .map(WorthUiValidatedFlowLayoutValue::Fit)
            .ok_or_else(|| {
                WorthUiFlowLayoutValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiFlowLayoutValueKind::Fill => parse_fill(&raw_value)
            .map(WorthUiValidatedFlowLayoutValue::Fill)
            .ok_or_else(|| {
                WorthUiFlowLayoutValueDenialReceipt::new(surface_id, schema, raw_value, None)
            }),
        WorthUiFlowLayoutValueKind::Unknown => {
            Err(WorthUiFlowLayoutValueDenialReceipt::unknown_prop(
                surface_id,
                "__unknown__",
                raw_value,
                None,
            ))
        }
    }
}

pub(super) fn default_flow_layout_value(
    schema: &'static WorthUiFlowLayoutPropSchema,
) -> WorthUiValidatedFlowLayoutValue {
    validate_flow_layout_value(
        "__flow_schema_default__",
        schema,
        schema.default_value().to_owned(),
    )
    .expect("flow layout schema defaults must admit")
}

fn parse_kind(value: &str) -> Option<WorthUiFlowLayoutKind> {
    match value.trim_matches('"') {
        "row" => Some(WorthUiFlowLayoutKind::Row),
        "column" => Some(WorthUiFlowLayoutKind::Column),
        "inline" => Some(WorthUiFlowLayoutKind::Inline),
        "stack" => Some(WorthUiFlowLayoutKind::Stack),
        "grid" => Some(WorthUiFlowLayoutKind::Grid),
        "spacer" => Some(WorthUiFlowLayoutKind::Spacer),
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

fn parse_align(value: &str) -> Option<WorthUiFlowLayoutAlign> {
    match value.trim_matches('"') {
        "start" => Some(WorthUiFlowLayoutAlign::Start),
        "center" => Some(WorthUiFlowLayoutAlign::Center),
        "end" => Some(WorthUiFlowLayoutAlign::End),
        _ => None,
    }
}

fn parse_cross_align(value: &str) -> Option<WorthUiFlowLayoutCrossAlign> {
    match value.trim_matches('"') {
        "start" => Some(WorthUiFlowLayoutCrossAlign::Start),
        "center" => Some(WorthUiFlowLayoutCrossAlign::Center),
        "end" => Some(WorthUiFlowLayoutCrossAlign::End),
        "baseline" => Some(WorthUiFlowLayoutCrossAlign::Baseline),
        _ => None,
    }
}

fn parse_fit(value: &str) -> Option<WorthUiFlowLayoutFit> {
    match value.trim_matches('"') {
        "hug" => Some(WorthUiFlowLayoutFit::Hug),
        "fill" => Some(WorthUiFlowLayoutFit::Fill),
        _ => None,
    }
}

fn parse_fill(value: &str) -> Option<WorthUiFlowLayoutFill> {
    match value.trim_matches('"') {
        "none" => Some(WorthUiFlowLayoutFill::None),
        "width" => Some(WorthUiFlowLayoutFill::Width),
        "height" => Some(WorthUiFlowLayoutFill::Height),
        "both" => Some(WorthUiFlowLayoutFill::Both),
        _ => None,
    }
}
