mod custom_invariant_provenance_diagnostic_projection;
mod diagnostic_projection;
mod failure_diagnostics;
mod invariant_violation_diagnostic_projection;
mod preparation_diagnostics;

use crate::publication::bundle::PublicationStage;
use crate::publication::data::PublicationError;
use crate::runtime::{RelationalRuntime, WorkingState};
use crate::transactions::data::{CommitConflict, MergedCommitPlan, TransactionCommitError};
use crate::validation::engine::InvariantExecutionResult;

use failure_diagnostics::{
    emit_collect_all_failure_diagnostics, emit_conflict_diagnostic, emit_publication_failure,
};
use preparation_diagnostics::emit_preparation_diagnostics;

impl RelationalRuntime {
    pub(crate) fn invariant_authority(&mut self) -> InvariantAuthority<'_> {
        InvariantAuthority::new(self)
    }

    pub fn certify_current_state(&mut self) -> Result<InvariantExecutionResult, PublicationError> {
        self.invariant_authority().enforce_certification_boundary()
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
        let result = self.runtime.validation().commit_boundary(merged_plan);
        emit_preparation_diagnostics(self.runtime, &result);
        let collect_all = emit_collect_all_failure_diagnostics(self.runtime, &result);
        if let Some(failure) = result.summary().blocking_failure() {
            if !collect_all {
                emit_conflict_diagnostic(self.runtime, &result, failure);
            }
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
            self.runtime.validation().mutation_sensitive_for_state(
                overlay_state,
                version_id,
                Some(merged_plan),
            )
        };
        emit_preparation_diagnostics(self.runtime, &result);
        let collect_all = emit_collect_all_failure_diagnostics(self.runtime, &result);
        if let Some(failure) = result.summary().blocking_failure() {
            if !collect_all {
                emit_conflict_diagnostic(self.runtime, &result, failure);
            }
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
            self.runtime.validation().snapshot_publication_for_state(
                overlay_state,
                version_id,
                Some(merged_plan),
            )
        };
        emit_preparation_diagnostics(self.runtime, &result);
        let collect_all = emit_collect_all_failure_diagnostics(self.runtime, &result);
        if let Some(failure) = result.summary().publication_failure() {
            if !collect_all {
                emit_publication_failure(self.runtime, &result, failure);
            }
            return Err(failure
                .clone()
                .into_publication_error(PublicationStage::InvariantCheck));
        }
        Ok(result)
    }

    pub(crate) fn enforce_certification_boundary(
        &mut self,
    ) -> Result<InvariantExecutionResult, PublicationError> {
        let result = self.runtime.validation().certification_state();
        emit_preparation_diagnostics(self.runtime, &result);
        let collect_all = emit_collect_all_failure_diagnostics(self.runtime, &result);
        if let Some(failure) = result.summary().publication_failure() {
            if !collect_all {
                emit_publication_failure(self.runtime, &result, failure);
            }
            return Err(failure
                .clone()
                .into_publication_error(PublicationStage::InvariantCheck));
        }
        Ok(result)
    }
}
