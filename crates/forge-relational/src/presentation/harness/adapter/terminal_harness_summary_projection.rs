use forge_harness::facade::HarnessSummaryProjection;

use crate::diagnostics::data::RelationalDiagnosticValue;

use super::diagnostic_fields_summary_projection::project_diagnostic_fields_for_terminal_harness_summary;

pub(super) type TerminalHarnessSummaryProjection = HarnessSummaryProjection;

pub(super) fn terminal_harness_summary_projection_object(
    fields: impl IntoIterator<Item = (&'static str, TerminalHarnessSummaryProjection)>,
) -> TerminalHarnessSummaryProjection {
    TerminalHarnessSummaryProjection::object(fields)
}

pub(super) fn terminal_harness_summary_projection_dynamic_object(
    fields: impl IntoIterator<Item = (String, TerminalHarnessSummaryProjection)>,
) -> TerminalHarnessSummaryProjection {
    TerminalHarnessSummaryProjection::object(fields)
}

pub(super) fn terminal_harness_summary_projection_array(
    values: impl IntoIterator<Item = TerminalHarnessSummaryProjection>,
) -> TerminalHarnessSummaryProjection {
    TerminalHarnessSummaryProjection::array(values)
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
