use crate::diagnostics::data::fields::{
    terminal_projection::{
        project_diagnostic_value_for_terminal_projection, TerminalDiagnosticProjectionValue,
    },
    RelationalDiagnosticValue,
};

use super::external_harness_summary_projection::{
    external_harness_summary_projection_array, external_harness_summary_projection_dynamic_object,
    ExternalHarnessSummaryProjection,
};

pub(super) fn project_diagnostic_fields_for_external_harness_summary(
    value: RelationalDiagnosticValue,
) -> ExternalHarnessSummaryProjection {
    terminal_diagnostic_projection_to_harness_summary(
        project_diagnostic_value_for_terminal_projection(&value),
    )
}

fn terminal_diagnostic_projection_to_harness_summary(
    value: TerminalDiagnosticProjectionValue,
) -> ExternalHarnessSummaryProjection {
    match value {
        TerminalDiagnosticProjectionValue::Null => ExternalHarnessSummaryProjection::Null,
        TerminalDiagnosticProjectionValue::Bool(value) => {
            ExternalHarnessSummaryProjection::Bool(value)
        }
        TerminalDiagnosticProjectionValue::Unsigned(value) => {
            ExternalHarnessSummaryProjection::Unsigned(value)
        }
        TerminalDiagnosticProjectionValue::Signed(value) => {
            ExternalHarnessSummaryProjection::Signed(value)
        }
        TerminalDiagnosticProjectionValue::String(value) => {
            ExternalHarnessSummaryProjection::String(value)
        }
        TerminalDiagnosticProjectionValue::Array(values) => {
            external_harness_summary_projection_array(
                values
                    .into_iter()
                    .map(terminal_diagnostic_projection_to_harness_summary),
            )
        }
        TerminalDiagnosticProjectionValue::Object(fields) => {
            external_harness_summary_projection_dynamic_object(fields.into_iter().map(
                |(field, value)| {
                    (
                        field,
                        terminal_diagnostic_projection_to_harness_summary(value),
                    )
                },
            ))
        }
    }
}

#[cfg(test)]
#[path = "diagnostic_fields_summary_projection/tests.rs"]
mod tests;
