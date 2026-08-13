use crate::diagnostics::data::RelationalDiagnosticArtifact;
use crate::runtime::RelationalRuntime;

pub(crate) trait PublicationDiagnosticsSource {
    fn publication_diagnostics(&self) -> &[RelationalDiagnosticArtifact];

    fn publication_diagnostic_artifact_count(&self) -> usize {
        self.publication_diagnostics().len()
    }

    fn publication_diagnostics_since(&self, start: usize) -> Vec<RelationalDiagnosticArtifact> {
        let start = start.min(self.publication_diagnostic_artifact_count());
        self.publication_diagnostics()[start..].to_vec()
    }
}

impl PublicationDiagnosticsSource for RelationalRuntime {
    fn publication_diagnostics(&self) -> &[RelationalDiagnosticArtifact] {
        &self.publication.diagnostics
    }
}
