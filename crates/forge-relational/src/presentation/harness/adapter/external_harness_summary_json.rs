use std::collections::BTreeMap;

use serde_json::{Map as ExternalHarnessJsonObject, Value as ExternalHarnessJson};

use crate::diagnostics::data::{RelationalDiagnosticFields, RelationalDiagnosticValue};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ExternalHarnessSummaryJson {
    Null,
    Bool(bool),
    Unsigned(u64),
    Signed(i64),
    String(String),
    Array(Vec<ExternalHarnessSummaryJson>),
    Object(BTreeMap<String, ExternalHarnessSummaryJson>),
    DiagnosticFields(RelationalDiagnosticValue),
}

impl ExternalHarnessSummaryJson {
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
                    .map(ExternalHarnessSummaryJson::into_external_harness_json)
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
                external_harness_summary_diagnostic_projection(value).into_external_harness_json()
            }
        }
    }
}

fn external_harness_summary_diagnostic_projection(
    value: RelationalDiagnosticValue,
) -> ExternalHarnessSummaryJson {
    let external_serde_projection_json = RelationalDiagnosticFields::from_diagnostic_value(value)
        .to_external_serde_projection_tree();
    external_harness_summary_json_from_diagnostic_projection(external_serde_projection_json)
}

fn external_harness_summary_json_from_diagnostic_projection(
    value: RelationalDiagnosticValue,
) -> ExternalHarnessSummaryJson {
    match value {
        RelationalDiagnosticValue::Null => ExternalHarnessSummaryJson::Null,
        RelationalDiagnosticValue::Bool(value) => ExternalHarnessSummaryJson::Bool(value),
        RelationalDiagnosticValue::Unsigned(value) => ExternalHarnessSummaryJson::Unsigned(value),
        RelationalDiagnosticValue::Signed(value) => ExternalHarnessSummaryJson::Signed(value),
        RelationalDiagnosticValue::String(value) => ExternalHarnessSummaryJson::String(value),
        RelationalDiagnosticValue::Array(values) => ExternalHarnessSummaryJson::Array(
            values
                .into_iter()
                .map(external_harness_summary_json_from_diagnostic_projection)
                .collect(),
        ),
        RelationalDiagnosticValue::Object(fields) => ExternalHarnessSummaryJson::Object(
            fields
                .into_iter()
                .map(|(field, value)| {
                    (
                        field,
                        external_harness_summary_json_from_diagnostic_projection(value),
                    )
                })
                .collect(),
        ),
        other => external_harness_summary_json_from_diagnostic_projection(
            RelationalDiagnosticFields::from_diagnostic_value(other)
                .to_external_serde_projection_tree(),
        ),
    }
}

pub(super) fn external_harness_summary_object(
    fields: impl IntoIterator<Item = (&'static str, ExternalHarnessSummaryJson)>,
) -> ExternalHarnessSummaryJson {
    ExternalHarnessSummaryJson::Object(
        fields
            .into_iter()
            .map(|(field, value)| (field.to_string(), value))
            .collect(),
    )
}

pub(super) fn external_harness_summary_array(
    values: impl IntoIterator<Item = ExternalHarnessSummaryJson>,
) -> ExternalHarnessSummaryJson {
    ExternalHarnessSummaryJson::Array(values.into_iter().collect())
}

pub(super) fn external_harness_summary_diagnostic_fields(
    value: RelationalDiagnosticValue,
) -> ExternalHarnessSummaryJson {
    ExternalHarnessSummaryJson::DiagnosticFields(value)
}

pub(super) fn external_harness_summary_string(
    value: impl Into<String>,
) -> ExternalHarnessSummaryJson {
    ExternalHarnessSummaryJson::String(value.into())
}

pub(super) fn external_harness_summary_u64(value: u64) -> ExternalHarnessSummaryJson {
    ExternalHarnessSummaryJson::Unsigned(value)
}

pub(super) fn external_harness_summary_usize(value: usize) -> ExternalHarnessSummaryJson {
    ExternalHarnessSummaryJson::Unsigned(value as u64)
}

pub(super) fn external_harness_summary_bool(value: bool) -> ExternalHarnessSummaryJson {
    ExternalHarnessSummaryJson::Bool(value)
}

pub(super) fn optional_external_harness_summary_u64(
    value: Option<u64>,
) -> ExternalHarnessSummaryJson {
    value
        .map(ExternalHarnessSummaryJson::Unsigned)
        .unwrap_or(ExternalHarnessSummaryJson::Null)
}

pub(super) fn optional_external_harness_summary_usize(
    value: Option<usize>,
) -> ExternalHarnessSummaryJson {
    value
        .map(|count| ExternalHarnessSummaryJson::Unsigned(count as u64))
        .unwrap_or(ExternalHarnessSummaryJson::Null)
}

pub(super) fn optional_external_harness_summary_string(
    value: Option<String>,
) -> ExternalHarnessSummaryJson {
    value
        .map(ExternalHarnessSummaryJson::String)
        .unwrap_or(ExternalHarnessSummaryJson::Null)
}
