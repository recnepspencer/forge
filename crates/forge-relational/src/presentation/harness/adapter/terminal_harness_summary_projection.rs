use std::collections::BTreeMap;

use serde_json::{Map as ExternalHarnessJsonObject, Value as ExternalHarnessJson};

use crate::diagnostics::data::RelationalDiagnosticValue;

use super::diagnostic_fields_summary_projection::project_diagnostic_fields_for_terminal_harness_summary;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum TerminalHarnessSummaryProjection {
    Null,
    Bool(bool),
    Unsigned(u64),
    Signed(i64),
    String(String),
    Array(Vec<TerminalHarnessSummaryProjection>),
    Object(BTreeMap<String, TerminalHarnessSummaryProjection>),
}

impl TerminalHarnessSummaryProjection {
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
                    .map(TerminalHarnessSummaryProjection::into_external_harness_json)
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

pub(super) fn terminal_harness_summary_projection_object(
    fields: impl IntoIterator<Item = (&'static str, TerminalHarnessSummaryProjection)>,
) -> TerminalHarnessSummaryProjection {
    TerminalHarnessSummaryProjection::Object(
        fields
            .into_iter()
            .map(|(field, value)| (field.to_string(), value))
            .collect(),
    )
}

pub(super) fn terminal_harness_summary_projection_dynamic_object(
    fields: impl IntoIterator<Item = (String, TerminalHarnessSummaryProjection)>,
) -> TerminalHarnessSummaryProjection {
    TerminalHarnessSummaryProjection::Object(fields.into_iter().collect())
}

pub(super) fn terminal_harness_summary_projection_array(
    values: impl IntoIterator<Item = TerminalHarnessSummaryProjection>,
) -> TerminalHarnessSummaryProjection {
    TerminalHarnessSummaryProjection::Array(values.into_iter().collect())
}

pub(super) fn terminal_harness_summary_projection_diagnostic_fields(
    value: RelationalDiagnosticValue,
) -> TerminalHarnessSummaryProjection {
    project_diagnostic_fields_for_terminal_harness_summary(value)
}

pub(super) fn terminal_harness_summary_projection_string(
    value: impl Into<String>,
) -> TerminalHarnessSummaryProjection {
    TerminalHarnessSummaryProjection::String(value.into())
}

pub(super) fn terminal_harness_summary_projection_u64(
    value: u64,
) -> TerminalHarnessSummaryProjection {
    TerminalHarnessSummaryProjection::Unsigned(value)
}

pub(super) fn terminal_harness_summary_projection_usize(
    value: usize,
) -> TerminalHarnessSummaryProjection {
    TerminalHarnessSummaryProjection::Unsigned(value as u64)
}

pub(super) fn terminal_harness_summary_projection_bool(
    value: bool,
) -> TerminalHarnessSummaryProjection {
    TerminalHarnessSummaryProjection::Bool(value)
}

pub(super) fn optional_terminal_harness_summary_projection_u64(
    value: Option<u64>,
) -> TerminalHarnessSummaryProjection {
    value
        .map(TerminalHarnessSummaryProjection::Unsigned)
        .unwrap_or(TerminalHarnessSummaryProjection::Null)
}

pub(super) fn optional_terminal_harness_summary_projection_usize(
    value: Option<usize>,
) -> TerminalHarnessSummaryProjection {
    value
        .map(|count| TerminalHarnessSummaryProjection::Unsigned(count as u64))
        .unwrap_or(TerminalHarnessSummaryProjection::Null)
}

pub(super) fn optional_terminal_harness_summary_projection_string(
    value: Option<String>,
) -> TerminalHarnessSummaryProjection {
    value
        .map(TerminalHarnessSummaryProjection::String)
        .unwrap_or(TerminalHarnessSummaryProjection::Null)
}
