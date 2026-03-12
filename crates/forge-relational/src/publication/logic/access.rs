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
            artifacts: self.runtime.publication.diagnostics.clone(),
        }
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

        let latest_position = self
            .runtime
            .history
            .patch_stream_index
            .last_key_value()
            .map(|(position, _)| *position);
        let latest_commit_id = self
            .runtime
            .publication
            .latest_bundle
            .as_ref()
            .map(|bundle| bundle.commit.commit_id)
            .or_else(|| {
                self.runtime
                    .history
                    .commit_envelopes
                    .values()
                    .max_by_key(|envelope| envelope.commit.commit_id)
                    .map(|envelope| envelope.commit.commit_id)
            });

        if let Some(after_position) = request.after_position {
            if !self
                .runtime
                .history
                .patch_stream_index
                .contains_key(&after_position)
            {
                return Err(PatchStreamReadError {
                    class: PatchStreamReadErrorClass::UnknownResumePosition,
                    detail: format!("unknown patch stream resume position {}", after_position.0),
                });
            }
        }

        let start = request
            .after_position
            .map(std::ops::Bound::Excluded)
            .unwrap_or(std::ops::Bound::Unbounded);
        let patches = self
            .runtime
            .history
            .patch_stream_index
            .range((start, std::ops::Bound::Unbounded))
            .filter_map(|(_, commit_id)| self.runtime.history.commit_envelopes.get(commit_id))
            .map(|envelope| envelope.patch.clone())
            .take(request.max_commits)
            .collect::<Vec<_>>();

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
