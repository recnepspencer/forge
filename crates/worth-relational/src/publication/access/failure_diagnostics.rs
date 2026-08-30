use crate::diagnostics::data::{
    DiagnosticCode, RelationalDiagnosticFields, RelationalDiagnosticsEntry,
};

pub(crate) fn invariant_failure_diagnostic(
    code: DiagnosticCode,
    detail: String,
    fields: RelationalDiagnosticFields,
) -> RelationalDiagnosticsEntry {
    RelationalDiagnosticsEntry::new(code, detail, fields)
}
