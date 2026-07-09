use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticFields,
    RelationalDiagnosticsEntry,
};
use crate::logic::runtime::RelationalRuntime;

pub(crate) trait DiagnosticArtifactSink {
    fn push_diagnostic_entries(
        &mut self,
        scope: DiagnosticsScope,
        kind: DiagnosticsArtifactKind,
        entries: Vec<RelationalDiagnosticsEntry>,
    );

    fn emit_failure_diagnostic(
        &mut self,
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
        &mut self,
        scope: DiagnosticsScope,
        kind: DiagnosticsArtifactKind,
        entries: Vec<RelationalDiagnosticsEntry>,
    ) {
        self.publication_authority()
            .diagnostic(scope)
            .kind(kind)
            .entries(entries)
            .emit();
    }
}
