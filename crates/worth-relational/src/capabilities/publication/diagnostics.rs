use crate::diagnostics::data::RelationalDiagnosticArtifact;
use crate::runtime::RelationalRuntime;

pub(crate) trait PublicationDiagnosticsSource {
    fn publication_diagnostics(&self) -> Vec<RelationalDiagnosticArtifact>;
    fn publication_diagnostic_artifact_count(&self) -> usize;
    fn publication_diagnostics_since(&self, start: usize) -> Vec<RelationalDiagnosticArtifact>;
}

impl PublicationDiagnosticsSource for RelationalRuntime {
    fn publication_diagnostics(&self) -> Vec<RelationalDiagnosticArtifact> {
        self.publication.diagnostics.snapshot()
    }

    fn publication_diagnostic_artifact_count(&self) -> usize {
        self.publication.diagnostics.count()
    }

    fn publication_diagnostics_since(&self, start: usize) -> Vec<RelationalDiagnosticArtifact> {
        self.publication.diagnostics.since(start)
    }
}
