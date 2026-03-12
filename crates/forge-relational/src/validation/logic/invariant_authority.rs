use serde_json::json;

use crate::diagnostics::data::{DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope};
use crate::logic::runtime::{RelationalRuntime, WorkingState};
use crate::publication::data::{PublicationError, PublicationStage};
use crate::publication::logic::publication_failure_diagnostic;
use crate::transactions::data::{
    CommitConflict, MergedCommitPlan, TransactionCommitError,
};

impl RelationalRuntime {
    pub fn invariant_authority(&mut self) -> InvariantAuthority<'_> {
        InvariantAuthority::new(self)
    }
}

pub struct InvariantAuthority<'runtime> {
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
        if let Some(conflict) = self
            .runtime
            .invariant_access()
            .commit_boundary_conflict(merged_plan)
        {
            self.runtime
                .publication_authority()
                .diagnostic(DiagnosticsScope::Invariant)
                .failure()
                .emit_entry(
                    DiagnosticCode::InvariantViolation,
                    conflict.detail(),
                    json!({ "execution_point": "commit_boundary" }),
                );
            return Err(TransactionCommitError::Conflict(conflict));
        }
        Ok(())
    }

    pub(crate) fn enforce_mutation_sensitive_for_working_state(
        &mut self,
        working_state: &WorkingState,
        version_id: crate::identity::data::VersionId,
        merged_plan: &MergedCommitPlan,
    ) -> Result<(), CommitConflict> {
        let conflict = {
            let overlay_state = self.runtime.overlay_state_view(working_state);
            self.runtime
                .invariant_access()
                .mutation_sensitive_conflict_for_state(&overlay_state, version_id, Some(merged_plan))
        };
        if let Some(conflict) = conflict {
            self.runtime
                .publication_authority()
                .diagnostic(DiagnosticsScope::Invariant)
                .failure()
                .emit_entry(
                    DiagnosticCode::InvariantViolation,
                    conflict.detail(),
                    json!({ "execution_point": "mutation_sensitive" }),
                );
            return Err(conflict);
        }
        Ok(())
    }

    pub(crate) fn enforce_snapshot_publication_for_working_state(
        &mut self,
        working_state: &WorkingState,
        version_id: crate::identity::data::VersionId,
        merged_plan: &MergedCommitPlan,
    ) -> Result<(), PublicationError> {
        let error = {
            let overlay_state = self.runtime.overlay_state_view(working_state);
            self.runtime
                .invariant_access()
                .snapshot_publication_error_for_state(
                    &overlay_state,
                    version_id,
                    Some(merged_plan),
                    PublicationStage::InvariantCheck,
                )
        };
        if let Some(error) = error {
            self.runtime.publication_authority().push_bounded_diagnostic(
                DiagnosticsScope::Invariant,
                DiagnosticsArtifactKind::Failure,
                vec![publication_failure_diagnostic(error.detail.clone())],
            );
            return Err(error);
        }
        Ok(())
    }
}
