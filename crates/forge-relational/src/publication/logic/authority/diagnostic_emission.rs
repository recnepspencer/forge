use crate::diagnostics::data::{
    DeterminismExpectation, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry,
};
use crate::publication::logic::diagnostics::{emit_filtered_artifact, DiagnosticArtifactBuilder};
use crate::publication::logic::PublicationAuthority;

impl<'runtime> PublicationAuthority<'runtime> {
    pub(crate) fn push_diagnostic_artifact(&mut self, artifact: RelationalDiagnosticArtifact) {
        let _ = emit_filtered_artifact(self.runtime, artifact);
    }

    pub(crate) fn push_bounded_diagnostic(
        &mut self,
        scope: DiagnosticsScope,
        kind: DiagnosticsArtifactKind,
        entries: Vec<RelationalDiagnosticsEntry>,
    ) -> RelationalDiagnosticArtifact {
        emit_filtered_artifact(
            self.runtime,
            RelationalDiagnosticArtifact::new(
                scope,
                kind,
                DeterminismExpectation::Required,
                entries,
            ),
        )
    }

    pub(crate) fn diagnostic(self, scope: DiagnosticsScope) -> DiagnosticArtifactBuilder<'runtime> {
        DiagnosticArtifactBuilder::new(self.runtime, scope)
    }
}
