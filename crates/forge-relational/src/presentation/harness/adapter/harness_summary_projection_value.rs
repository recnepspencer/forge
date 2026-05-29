use std::collections::BTreeMap;

use serde_json::{Map as ExternalHarnessJsonObject, Value as ExternalHarnessJson};

use crate::diagnostics::data::{RelationalDiagnosticFields, RelationalDiagnosticValue};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum HarnessSummaryProjectionValue {
    Null,
    Bool(bool),
    Unsigned(u64),
    Signed(i64),
    String(String),
    Array(Vec<HarnessSummaryProjectionValue>),
    Object(BTreeMap<String, HarnessSummaryProjectionValue>),
    DiagnosticFields(RelationalDiagnosticValue),
}

impl HarnessSummaryProjectionValue {
    pub(super) fn into_external_harness_json(self) -> ExternalHarnessJson {
        match self {
            Self::Null => ExternalHarnessJson::Null,
            Self::Bool(value) => ExternalHarnessJson::Bool(value),
            Self::Unsigned(value) => ExternalHarnessJson::from(value),
            Self::Signed(value) => ExternalHarnessJson::from(value),
            Self::String(value) => ExternalHarnessJson::String(value),
            Self::Array(values) => ExternalHarnessJson::Array(
                values
                    .into_iter()
                    .map(HarnessSummaryProjectionValue::into_external_harness_json)
                    .collect(),
            ),
            Self::Object(fields) => {
                ExternalHarnessJson::Object(ExternalHarnessJsonObject::from_iter(
                    fields
                        .into_iter()
                        .map(|(field, value)| (field, value.into_external_harness_json())),
                ))
            }
            Self::DiagnosticFields(value) => {
                diagnostic_fields_projection(value).into_external_harness_json()
            }
        }
    }
}

fn diagnostic_fields_projection(value: RelationalDiagnosticValue) -> HarnessSummaryProjectionValue {
    let projected =
        RelationalDiagnosticFields::from_diagnostic_value(value).to_external_projection_value();
    harness_projection_value_from_diagnostic_projection(projected)
}

fn harness_projection_value_from_diagnostic_projection(
    value: RelationalDiagnosticValue,
) -> HarnessSummaryProjectionValue {
    match value {
        RelationalDiagnosticValue::Null => HarnessSummaryProjectionValue::Null,
        RelationalDiagnosticValue::Bool(value) => HarnessSummaryProjectionValue::Bool(value),
        RelationalDiagnosticValue::Unsigned(value) => {
            HarnessSummaryProjectionValue::Unsigned(value)
        }
        RelationalDiagnosticValue::Signed(value) => HarnessSummaryProjectionValue::Signed(value),
        RelationalDiagnosticValue::String(value) => HarnessSummaryProjectionValue::String(value),
        RelationalDiagnosticValue::Array(values) => HarnessSummaryProjectionValue::Array(
            values
                .into_iter()
                .map(harness_projection_value_from_diagnostic_projection)
                .collect(),
        ),
        RelationalDiagnosticValue::Object(fields) => HarnessSummaryProjectionValue::Object(
            fields
                .into_iter()
                .map(|(field, value)| {
                    (
                        field,
                        harness_projection_value_from_diagnostic_projection(value),
                    )
                })
                .collect(),
        ),
        other => harness_projection_value_from_diagnostic_projection(
            RelationalDiagnosticFields::from_diagnostic_value(other).to_external_projection_value(),
        ),
    }
}

pub(super) fn harness_summary_object(
    fields: impl IntoIterator<Item = (&'static str, HarnessSummaryProjectionValue)>,
) -> HarnessSummaryProjectionValue {
    HarnessSummaryProjectionValue::Object(
        fields
            .into_iter()
            .map(|(field, value)| (field.to_string(), value))
            .collect(),
    )
}

pub(super) fn harness_summary_array(
    values: impl IntoIterator<Item = HarnessSummaryProjectionValue>,
) -> HarnessSummaryProjectionValue {
    HarnessSummaryProjectionValue::Array(values.into_iter().collect())
}

pub(super) fn harness_summary_diagnostic_fields(
    value: RelationalDiagnosticValue,
) -> HarnessSummaryProjectionValue {
    HarnessSummaryProjectionValue::DiagnosticFields(value)
}

pub(super) fn harness_summary_string(value: impl Into<String>) -> HarnessSummaryProjectionValue {
    HarnessSummaryProjectionValue::String(value.into())
}

pub(super) fn harness_summary_u64(value: u64) -> HarnessSummaryProjectionValue {
    HarnessSummaryProjectionValue::Unsigned(value)
}

pub(super) fn harness_summary_usize(value: usize) -> HarnessSummaryProjectionValue {
    HarnessSummaryProjectionValue::Unsigned(value as u64)
}

pub(super) fn harness_summary_bool(value: bool) -> HarnessSummaryProjectionValue {
    HarnessSummaryProjectionValue::Bool(value)
}

pub(super) fn optional_harness_summary_u64(value: Option<u64>) -> HarnessSummaryProjectionValue {
    value
        .map(HarnessSummaryProjectionValue::Unsigned)
        .unwrap_or(HarnessSummaryProjectionValue::Null)
}

pub(super) fn optional_harness_summary_usize(
    value: Option<usize>,
) -> HarnessSummaryProjectionValue {
    value
        .map(|count| HarnessSummaryProjectionValue::Unsigned(count as u64))
        .unwrap_or(HarnessSummaryProjectionValue::Null)
}

pub(super) fn optional_harness_summary_string(
    value: Option<String>,
) -> HarnessSummaryProjectionValue {
    value
        .map(HarnessSummaryProjectionValue::String)
        .unwrap_or(HarnessSummaryProjectionValue::Null)
}
