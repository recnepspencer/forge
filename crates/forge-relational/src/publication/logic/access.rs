use crate::diagnostics::data::RelationalDiagnosticsEntry;
use crate::diagnostics::facade::RelationalDiagnosticsFacade;
use crate::logic::runtime::{RelationalReplayRecord, RelationalRuntime};
use crate::publication::data::diff::{
    PatchStreamBatch, PatchStreamReadError, PatchStreamReadErrorClass, PatchStreamRequest,
};
use crate::publication::data::PublicationBundle;
use crate::validation::data::InvariantExecutionPoint;

pub struct PublicationAccess<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl RelationalRuntime {
    pub fn publication_access(&self) -> PublicationAccess<'_> {
        PublicationAccess::new(self)
    }
}

impl<'runtime> PublicationAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn diagnostics(&self) -> RelationalDiagnosticsFacade {
        RelationalDiagnosticsFacade {
            artifacts: self.diagnostic_artifacts().to_vec(),
        }
    }

    pub fn diagnostic_artifacts(
        &self,
    ) -> &[crate::diagnostics::data::RelationalDiagnosticArtifact] {
        &self.runtime.publication.diagnostics
    }

    pub fn diagnostics_since(
        &self,
        start: usize,
    ) -> Vec<crate::diagnostics::data::RelationalDiagnosticArtifact> {
        self.runtime.publication.diagnostics[start..].to_vec()
    }

    pub fn latest_bundle(&self) -> Option<&PublicationBundle<RelationalReplayRecord>> {
        self.runtime.publication.latest_bundle.as_ref()
    }

    pub fn latest_patch(&self) -> Option<&crate::publication::data::diff::RelationalPatchRecord> {
        self.latest_bundle().map(|bundle| &bundle.patch)
    }

    pub fn latest_replay(&self) -> Option<&RelationalReplayRecord> {
        self.latest_bundle().map(|bundle| &bundle.replay)
    }

    pub fn read_patch_stream(
        &self,
        request: PatchStreamRequest,
    ) -> Result<PatchStreamBatch, PatchStreamReadError> {
        if request.max_commits == 0 {
            return Err(PatchStreamReadError {
                class: PatchStreamReadErrorClass::InvalidBatchSize,
                detail: "patch stream request must ask for at least one commit".to_string(),
            });
        }

        let latest_position = self.runtime.history_access().latest_patch_stream_position();
        let latest_commit_id = self
            .runtime
            .publication
            .latest_bundle
            .as_ref()
            .map(|bundle| bundle.commit.commit_id)
            .or_else(|| {
                self.runtime
                    .history_access()
                    .latest_commit()
                    .map(|commit| commit.commit_id)
            });

        if let Some(after_position) = request.after_position {
            if !self
                .runtime
                .history_access()
                .contains_patch_stream_position(after_position)
            {
                return Err(PatchStreamReadError {
                    class: PatchStreamReadErrorClass::UnknownResumePosition,
                    detail: format!("unknown patch stream resume position {}", after_position.0),
                });
            }
        }

        let patches = self
            .runtime
            .history_access()
            .patches_after(request.after_position, request.max_commits);

        Ok(PatchStreamBatch {
            resumed_after: request.after_position,
            next_position: patches.last().map(|patch| patch.position),
            latest_position,
            latest_commit_id,
            patches,
        })
    }
}

pub(crate) fn publication_failure_diagnostic(detail: String) -> RelationalDiagnosticsEntry {
    RelationalDiagnosticsEntry {
        code: crate::diagnostics::data::DiagnosticCode::InvariantViolation,
        message: detail,
        fields: serde_json::json!({
            "execution_point": InvariantExecutionPoint::SnapshotPublication.diagnostic_label()
        }),
    }
}
