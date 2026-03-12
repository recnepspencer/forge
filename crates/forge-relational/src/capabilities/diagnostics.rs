use crate::diagnostics::data::{DiagnosticCode, DiagnosticsScope};
use crate::logic::runtime::RelationalRuntime;

#[allow(dead_code)]
pub(crate) trait DiagnosticsSink {
    fn emit_diagnostic_entry(
        &mut self,
        scope: DiagnosticsScope,
        code: DiagnosticCode,
        message: impl Into<String>,
        fields: serde_json::Value,
    );
}

impl DiagnosticsSink for RelationalRuntime {
    fn emit_diagnostic_entry(
        &mut self,
        scope: DiagnosticsScope,
        code: DiagnosticCode,
        message: impl Into<String>,
        fields: serde_json::Value,
    ) {
        self.diagnostic(scope).failure().emit_entry(code, message, fields);
    }
}
