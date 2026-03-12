use serde_json::json;

use crate::diagnostics::data::{DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope};
use crate::logic::runtime::{RelationalRuntime, WorkingState};
use crate::publication::data::{PublicationError, PublicationStage};
use crate::publication::logic::publication_failure_diagnostic;
use crate::transactions::data::{
    CommitConflict, MergedCommitPlan, TransactionCommitError,
};

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
    ) -> Result<(), TransactionCommitError> {
        if let Some(failure) = self
            .runtime
            .invariant_access()
            .commit_boundary(merged_plan)
            .first_blocking_failure()
        {
            self.emit_conflict_diagnostic(failure.execution_point(), failure.detail().to_string());
            return Err(TransactionCommitError::Conflict(failure.into_commit_conflict()));
        }
        Ok(())
    }

    pub(crate) fn enforce_mutation_sensitive_for_working_state(
        &mut self,
        working_state: &WorkingState,
        version_id: crate::identity::data::VersionId,
        merged_plan: &MergedCommitPlan,
    ) -> Result<(), CommitConflict> {
        let failure = {
            let overlay_state = self.runtime.overlay_state_view(working_state);
            self.runtime
                .invariant_access()
                .mutation_sensitive_for_state(&overlay_state, version_id, Some(merged_plan))
                .first_blocking_failure()
        };
        if let Some(failure) = failure {
            self.emit_conflict_diagnostic(
                failure.execution_point(),
                failure.detail().to_string(),
            );
            return Err(failure.into_commit_conflict());
        }
        Ok(())
    }

    pub(crate) fn enforce_snapshot_publication_for_working_state(
        &mut self,
        working_state: &WorkingState,
        version_id: crate::identity::data::VersionId,
        merged_plan: &MergedCommitPlan,
    ) -> Result<(), PublicationError> {
        let failure = {
            let overlay_state = self.runtime.overlay_state_view(working_state);
            self.runtime
                .invariant_access()
                .snapshot_publication_for_state(
                    &overlay_state,
                    version_id,
                    Some(merged_plan),
                )
                .first_publication_failure()
        };
        if let Some(failure) = failure {
            self.emit_publication_failure(failure.detail().to_string());
            return Err(failure.into_publication_error(PublicationStage::InvariantCheck));
        }
        Ok(())
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
        self.runtime.publication_authority().push_bounded_diagnostic(
            DiagnosticsScope::Invariant,
            DiagnosticsArtifactKind::Failure,
            vec![publication_failure_diagnostic(detail)],
        );
    }
}
