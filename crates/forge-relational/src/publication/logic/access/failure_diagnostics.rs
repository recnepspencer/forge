use crate::diagnostics::data::{
    DiagnosticCode, RelationalDiagnosticFields, RelationalDiagnosticValue,
    RelationalDiagnosticsEntry,
};
use crate::validation::data::InvariantExecutionPoint;

pub(crate) fn publication_failure_diagnostic(
    code: DiagnosticCode,
    detail: String,
    fields: RelationalDiagnosticFields,
) -> RelationalDiagnosticsEntry {
    RelationalDiagnosticsEntry::new(code, detail, publication_failure_fields(fields))
}

fn publication_failure_fields(fields: RelationalDiagnosticFields) -> RelationalDiagnosticFields {
    RelationalDiagnosticValue::object([
        (
            "execution_point",
            RelationalDiagnosticValue::string(
                InvariantExecutionPoint::SnapshotPublication.diagnostic_label(),
            ),
        ),
        ("failure", fields.root().clone()),
    ])
    .into()
}
