use std::collections::BTreeMap;

use serde_json::{Map as ExternalHarnessJsonObject, Value as ExternalHarnessJson};

use crate::diagnostics::data::RelationalDiagnosticValue;

use super::diagnostic_fields_summary_projection::project_diagnostic_fields_for_external_harness_summary;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ExternalHarnessSummaryProjection {
    Null,
    Bool(bool),
    Unsigned(u64),
    Signed(i64),
    String(String),
    Array(Vec<ExternalHarnessSummaryProjection>),
    Object(BTreeMap<String, ExternalHarnessSummaryProjection>),
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
        }
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

pub(super) fn external_harness_summary_projection_dynamic_object(
    fields: impl IntoIterator<Item = (String, ExternalHarnessSummaryProjection)>,
) -> ExternalHarnessSummaryProjection {
    ExternalHarnessSummaryProjection::Object(fields.into_iter().collect())
}

pub(super) fn external_harness_summary_projection_array(
    values: impl IntoIterator<Item = ExternalHarnessSummaryProjection>,
) -> ExternalHarnessSummaryProjection {
    ExternalHarnessSummaryProjection::Array(values.into_iter().collect())
}

pub(super) fn external_harness_summary_projection_diagnostic_fields(
    value: RelationalDiagnosticValue,
) -> ExternalHarnessSummaryProjection {
    project_diagnostic_fields_for_external_harness_summary(value)
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
