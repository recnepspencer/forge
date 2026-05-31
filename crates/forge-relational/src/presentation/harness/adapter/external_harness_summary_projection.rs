use std::collections::BTreeMap;

use serde_json::{Map as ExternalHarnessJsonObject, Value as ExternalHarnessJson};

use crate::diagnostics::data::{RelationalDiagnosticFields, RelationalDiagnosticValue};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ExternalHarnessSummaryProjection {
    Null,
    Bool(bool),
    Unsigned(u64),
    Signed(i64),
    String(String),
    Array(Vec<ExternalHarnessSummaryProjection>),
    Object(BTreeMap<String, ExternalHarnessSummaryProjection>),
    DiagnosticFields(RelationalDiagnosticValue),
}

impl ExternalHarnessSummaryProjection {
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
                    .map(ExternalHarnessSummaryProjection::into_external_harness_json)
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
                external_harness_summary_projection_from_diagnostic_fields(value)
                    .into_external_harness_json()
            }
        }
    }
}

fn external_harness_summary_projection_from_diagnostic_fields(
    value: RelationalDiagnosticValue,
) -> ExternalHarnessSummaryProjection {
    let external_serde_projection = RelationalDiagnosticFields::from_diagnostic_value(value)
        .to_external_serde_projection_tree();
    external_harness_summary_projection_from_diagnostic_projection(external_serde_projection)
}

fn external_harness_summary_projection_from_diagnostic_projection(
    value: RelationalDiagnosticValue,
) -> ExternalHarnessSummaryProjection {
    match value {
        RelationalDiagnosticValue::Null => ExternalHarnessSummaryProjection::Null,
        RelationalDiagnosticValue::Bool(value) => ExternalHarnessSummaryProjection::Bool(value),
        RelationalDiagnosticValue::Unsigned(value) => {
            ExternalHarnessSummaryProjection::Unsigned(value)
        }
        RelationalDiagnosticValue::Signed(value) => ExternalHarnessSummaryProjection::Signed(value),
        RelationalDiagnosticValue::String(value) => ExternalHarnessSummaryProjection::String(value),
        RelationalDiagnosticValue::Array(values) => ExternalHarnessSummaryProjection::Array(
            values
                .into_iter()
                .map(external_harness_summary_projection_from_diagnostic_projection)
                .collect(),
        ),
        RelationalDiagnosticValue::Object(fields) => ExternalHarnessSummaryProjection::Object(
            fields
                .into_iter()
                .map(|(field, value)| {
                    (
                        field,
                        external_harness_summary_projection_from_diagnostic_projection(value),
                    )
                })
                .collect(),
        ),
        other => external_harness_summary_projection_from_diagnostic_projection(
            RelationalDiagnosticFields::from_diagnostic_value(other)
                .to_external_serde_projection_tree(),
        ),
    }
}

pub(super) fn external_harness_summary_projection_object(
    fields: impl IntoIterator<Item = (&'static str, ExternalHarnessSummaryProjection)>,
) -> ExternalHarnessSummaryProjection {
    ExternalHarnessSummaryProjection::Object(
        fields
            .into_iter()
            .map(|(field, value)| (field.to_string(), value))
            .collect(),
    )
}

pub(super) fn external_harness_summary_projection_array(
    values: impl IntoIterator<Item = ExternalHarnessSummaryProjection>,
) -> ExternalHarnessSummaryProjection {
    ExternalHarnessSummaryProjection::Array(values.into_iter().collect())
}

pub(super) fn external_harness_summary_projection_diagnostic_fields(
    value: RelationalDiagnosticValue,
) -> ExternalHarnessSummaryProjection {
    ExternalHarnessSummaryProjection::DiagnosticFields(value)
}

pub(super) fn external_harness_summary_projection_string(
    value: impl Into<String>,
) -> ExternalHarnessSummaryProjection {
    ExternalHarnessSummaryProjection::String(value.into())
}

pub(super) fn external_harness_summary_projection_u64(
    value: u64,
) -> ExternalHarnessSummaryProjection {
    ExternalHarnessSummaryProjection::Unsigned(value)
}

pub(super) fn external_harness_summary_projection_usize(
    value: usize,
) -> ExternalHarnessSummaryProjection {
    ExternalHarnessSummaryProjection::Unsigned(value as u64)
}

pub(super) fn external_harness_summary_projection_bool(
    value: bool,
) -> ExternalHarnessSummaryProjection {
    ExternalHarnessSummaryProjection::Bool(value)
}

pub(super) fn optional_external_harness_summary_projection_u64(
    value: Option<u64>,
) -> ExternalHarnessSummaryProjection {
    value
        .map(ExternalHarnessSummaryProjection::Unsigned)
        .unwrap_or(ExternalHarnessSummaryProjection::Null)
}

pub(super) fn optional_external_harness_summary_projection_usize(
    value: Option<usize>,
) -> ExternalHarnessSummaryProjection {
    value
        .map(|count| ExternalHarnessSummaryProjection::Unsigned(count as u64))
        .unwrap_or(ExternalHarnessSummaryProjection::Null)
}

pub(super) fn optional_external_harness_summary_projection_string(
    value: Option<String>,
) -> ExternalHarnessSummaryProjection {
    value
        .map(ExternalHarnessSummaryProjection::String)
        .unwrap_or(ExternalHarnessSummaryProjection::Null)
}
