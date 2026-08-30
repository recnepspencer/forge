use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticFields,
    RelationalDiagnosticsEntry,
};
use crate::runtime::RelationalRuntime;

pub(crate) trait DiagnosticArtifactSink {
    fn push_diagnostic_entries(
        &self,
        scope: DiagnosticsScope,
        kind: DiagnosticsArtifactKind,
        entries: Vec<RelationalDiagnosticsEntry>,
    );

    fn emit_failure_diagnostic(
        &self,
        scope: DiagnosticsScope,
        code: DiagnosticCode,
        message: impl Into<String>,
        fields: impl Into<RelationalDiagnosticFields>,
    ) {
        self.push_diagnostic_entries(
            scope,
            DiagnosticsArtifactKind::Failure,
            vec![RelationalDiagnosticsEntry::new(
                code,
                message,
                fields.into(),
            )],
        );
    }
}

impl DiagnosticArtifactSink for RelationalRuntime {
    fn push_diagnostic_entries(
        &self,
        scope: DiagnosticsScope,
        kind: DiagnosticsArtifactKind,
        entries: Vec<RelationalDiagnosticsEntry>,
    ) {
        self.push_bounded_preparation_diagnostic(scope, kind, entries);
    }
}
