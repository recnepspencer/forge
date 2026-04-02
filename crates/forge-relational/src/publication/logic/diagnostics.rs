use serde_json::Value;

use crate::diagnostics::data::{
    DeterminismExpectation, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry,
};
use crate::logic::runtime::RelationalRuntime;

pub(crate) struct DiagnosticArtifactBuilder<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
    scope: DiagnosticsScope,
    kind: DiagnosticsArtifactKind,
    entries: Vec<RelationalDiagnosticsEntry>,
}

impl<'runtime> DiagnosticArtifactBuilder<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime, scope: DiagnosticsScope) -> Self {
        Self {
            runtime,
            scope,
            kind: DiagnosticsArtifactKind::MinimalSummary,
            entries: Vec::new(),
        }
    }

    pub(crate) fn kind(mut self, kind: DiagnosticsArtifactKind) -> Self {
        self.kind = kind;
        self
    }

    pub(crate) fn minimal_summary(self) -> Self {
        self.kind(DiagnosticsArtifactKind::MinimalSummary)
    }

    pub(crate) fn failure(self) -> Self {
        self.kind(DiagnosticsArtifactKind::Failure)
    }

    pub(crate) fn rollback(self) -> Self {
        self.kind(DiagnosticsArtifactKind::Rollback)
    }

    pub(crate) fn comparison(self) -> Self {
        self.kind(DiagnosticsArtifactKind::Comparison)
    }

    pub(crate) fn entry(
        mut self,
        code: crate::diagnostics::data::DiagnosticCode,
        message: impl Into<String>,
        fields: Value,
    ) -> Self {
        self.entries.push(RelationalDiagnosticsEntry {
            code,
            message: message.into(),
            fields,
        });
        self
    }

    pub(crate) fn entries(
        mut self,
        entries: impl IntoIterator<Item = RelationalDiagnosticsEntry>,
    ) -> Self {
        self.entries.extend(entries);
        self
    }

    pub(crate) fn emit_entry(
        self,
        code: crate::diagnostics::data::DiagnosticCode,
        message: impl Into<String>,
        fields: Value,
    ) -> RelationalDiagnosticArtifact {
        self.entry(code, message, fields).emit()
    }

    pub(crate) fn emit(self) -> RelationalDiagnosticArtifact {
        let profile = &self.runtime.config.diagnostics.profile;
        let artifact = RelationalDiagnosticArtifact {
            scope: self.scope,
            kind: self.kind,
            determinism: DeterminismExpectation::Required,
            entries: self.entries,
        };
        let filtered = profile
            .filter_artifact(artifact.clone())
            .unwrap_or_else(|| RelationalDiagnosticArtifact {
                scope: artifact.scope,
                kind: artifact.kind,
                determinism: artifact.determinism,
                entries: Vec::new(),
            });
        if !filtered.entries.is_empty() {
            self.runtime.publication.diagnostics.push(filtered.clone());
        }
        filtered
    }
}
