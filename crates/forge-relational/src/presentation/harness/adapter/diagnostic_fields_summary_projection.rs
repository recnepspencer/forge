use crate::diagnostics::data::fields::{
    terminal_projection::{
        project_diagnostic_value_for_terminal_projection, TerminalDiagnosticProjectionValue,
    },
    RelationalDiagnosticValue,
};

use super::terminal_harness_summary_projection::{
    terminal_harness_summary_projection_array, terminal_harness_summary_projection_dynamic_object,
    TerminalHarnessSummaryProjection,
};

pub(super) fn project_diagnostic_fields_for_terminal_harness_summary(
    value: RelationalDiagnosticValue,
) -> TerminalHarnessSummaryProjection {
    terminal_diagnostic_projection_to_harness_summary(
        project_diagnostic_value_for_terminal_projection(&value),
    )
}

fn terminal_diagnostic_projection_to_harness_summary(
    value: TerminalDiagnosticProjectionValue,
) -> TerminalHarnessSummaryProjection {
    match value {
        TerminalDiagnosticProjectionValue::Null => TerminalHarnessSummaryProjection::Null,
        TerminalDiagnosticProjectionValue::Bool(value) => {
            TerminalHarnessSummaryProjection::Bool(value)
        }
        TerminalDiagnosticProjectionValue::Unsigned(value) => {
            TerminalHarnessSummaryProjection::Unsigned(value)
        }
        TerminalDiagnosticProjectionValue::Signed(value) => {
            TerminalHarnessSummaryProjection::Signed(value)
        }
        TerminalDiagnosticProjectionValue::String(value) => {
            TerminalHarnessSummaryProjection::String(value)
        }
        TerminalDiagnosticProjectionValue::Array(values) => {
            terminal_harness_summary_projection_array(
                values
                    .into_iter()
                    .map(terminal_diagnostic_projection_to_harness_summary),
            )
        }
        TerminalDiagnosticProjectionValue::Object(fields) => {
            terminal_harness_summary_projection_dynamic_object(fields.into_iter().map(
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
