use serde_json::json;

use crate::diagnostics::data::{DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope};
use crate::logic::runtime::{RelationalRuntime, WorkingState};
use crate::publication::data::{PublicationError, PublicationStage};
use crate::publication::logic::publication_failure_diagnostic;
use crate::transactions::data::{CommitConflict, MergedCommitPlan, TransactionCommitError};
use crate::validation::engine::InvariantExecutionResult;

impl RelationalRuntime {
    pub(crate) fn invariant_authority(&mut self) -> InvariantAuthority<'_> {
        InvariantAuthority::new(self)
    }
}

pub(crate) struct InvariantAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

impl<'runtime> InvariantAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub(crate) fn enforce_commit_boundary(
        &mut self,
        merged_plan: &MergedCommitPlan,
    ) -> Result<InvariantExecutionResult, TransactionCommitError> {
        let result = self.runtime.invariant_access().commit_boundary(merged_plan);
        if let Some(failure) = result.summary().blocking_failure() {
            self.emit_conflict_diagnostic(failure.execution_point(), failure.detail().to_string());
            return Err(TransactionCommitError::conflict(
                failure.clone().into_commit_conflict(),
            ));
        }
        Ok(result)
    }

    pub(crate) fn enforce_mutation_sensitive_for_working_state(
        &mut self,
        working_state: &WorkingState,
        version_id: crate::identity::data::VersionId,
        merged_plan: &MergedCommitPlan,
    ) -> Result<InvariantExecutionResult, CommitConflict> {
        let result = {
            let storage = self.runtime.storage_access();
            let overlay_state = storage.overlay_state_view(working_state);
            self.runtime
                .invariant_access()
                .mutation_sensitive_for_state(overlay_state, version_id, Some(merged_plan))
        };
        if let Some(failure) = result.summary().blocking_failure() {
            self.emit_conflict_diagnostic(failure.execution_point(), failure.detail().to_string());
            return Err(failure.clone().into_commit_conflict());
        }
        Ok(result)
    }

    pub(crate) fn enforce_snapshot_publication_for_working_state(
        &mut self,
        working_state: &WorkingState,
        version_id: crate::identity::data::VersionId,
        merged_plan: &MergedCommitPlan,
    ) -> Result<InvariantExecutionResult, PublicationError> {
        let result = {
            let storage = self.runtime.storage_access();
            let overlay_state = storage.overlay_state_view(working_state);
            self.runtime
                .invariant_access()
                .snapshot_publication_for_state(overlay_state, version_id, Some(merged_plan))
        };
        if let Some(failure) = result.summary().publication_failure() {
            self.emit_publication_failure(failure.detail().to_string());
            return Err(failure
                .clone()
                .into_publication_error(PublicationStage::InvariantCheck));
        }
        Ok(result)
    }

    fn emit_conflict_diagnostic(
        &mut self,
        execution_point: crate::validation::data::InvariantExecutionPoint,
        detail: String,
    ) {
        self.runtime
            .publication_authority()
            .diagnostic(DiagnosticsScope::Invariant)
            .failure()
            .emit_entry(
                DiagnosticCode::InvariantViolation,
                detail,
                json!({ "execution_point": execution_point.diagnostic_label() }),
            );
    }

    fn emit_publication_failure(&mut self, detail: String) {
        self.runtime
            .publication_authority()
            .push_bounded_diagnostic(
                DiagnosticsScope::Invariant,
                DiagnosticsArtifactKind::Failure,
                vec![publication_failure_diagnostic(detail)],
            );
    }
}
