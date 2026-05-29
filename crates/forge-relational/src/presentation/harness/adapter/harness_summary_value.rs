use std::collections::BTreeMap;

use serde_json::{Map, Value};

use crate::diagnostics::data::{RelationalDiagnosticFields, RelationalDiagnosticValue};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum HarnessSummaryValue {
    Null,
    Bool(bool),
    Unsigned(u64),
    String(String),
    Array(Vec<HarnessSummaryValue>),
    Object(BTreeMap<String, HarnessSummaryValue>),
    DiagnosticFields(RelationalDiagnosticValue),
}

impl HarnessSummaryValue {
    pub(super) fn into_json(self) -> Value {
        match self {
            Self::Null => Value::Null,
            Self::Bool(value) => Value::Bool(value),
            Self::Unsigned(value) => Value::from(value),
            Self::String(value) => Value::String(value),
            Self::Array(values) => Value::Array(
                values
                    .into_iter()
                    .map(HarnessSummaryValue::into_json)
                    .collect(),
            ),
            Self::Object(fields) => Value::Object(Map::from_iter(
                fields
                    .into_iter()
                    .map(|(field, value)| (field, value.into_json())),
            )),
            Self::DiagnosticFields(value) => {
                RelationalDiagnosticFields::from_diagnostic_value(value).into_serde_projection()
            }
        }
    }
}

pub(super) fn harness_summary_object(
    fields: impl IntoIterator<Item = (&'static str, HarnessSummaryValue)>,
) -> HarnessSummaryValue {
    HarnessSummaryValue::Object(
        fields
            .into_iter()
            .map(|(field, value)| (field.to_string(), value))
            .collect(),
    )
}

pub(super) fn harness_summary_array(
    values: impl IntoIterator<Item = HarnessSummaryValue>,
) -> HarnessSummaryValue {
    HarnessSummaryValue::Array(values.into_iter().collect())
}

pub(super) fn harness_summary_diagnostic_fields(
    value: RelationalDiagnosticValue,
) -> HarnessSummaryValue {
    HarnessSummaryValue::DiagnosticFields(value)
}

pub(super) fn harness_summary_string(value: impl Into<String>) -> HarnessSummaryValue {
    HarnessSummaryValue::String(value.into())
}

pub(super) fn harness_summary_u64(value: u64) -> HarnessSummaryValue {
    HarnessSummaryValue::Unsigned(value)
}

pub(super) fn harness_summary_usize(value: usize) -> HarnessSummaryValue {
    HarnessSummaryValue::Unsigned(value as u64)
}

pub(super) fn harness_summary_bool(value: bool) -> HarnessSummaryValue {
    HarnessSummaryValue::Bool(value)
}

pub(super) fn optional_harness_summary_u64(value: Option<u64>) -> HarnessSummaryValue {
    value
        .map(HarnessSummaryValue::Unsigned)
        .unwrap_or(HarnessSummaryValue::Null)
}

pub(super) fn optional_harness_summary_usize(value: Option<usize>) -> HarnessSummaryValue {
    value
        .map(|count| HarnessSummaryValue::Unsigned(count as u64))
        .unwrap_or(HarnessSummaryValue::Null)
}

pub(super) fn optional_harness_summary_string(value: Option<String>) -> HarnessSummaryValue {
    value
        .map(HarnessSummaryValue::String)
        .unwrap_or(HarnessSummaryValue::Null)
}
