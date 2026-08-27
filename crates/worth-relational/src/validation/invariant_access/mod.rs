mod execution;
mod metadata;
#[cfg(test)]
pub(crate) mod test_support;
#[cfg(test)]
mod tests;

use crate::branch::SelectedRelationalBranchState;
use crate::runtime::RelationalRuntime;
use crate::transactions::data::MergedCommitPlan;
#[cfg(test)]
use crate::validation::engine::HarnessAuditMode;
use crate::validation::engine::{
    InvariantExecutionResult, InvariantObservation, InvariantRequestProfile,
};

impl RelationalRuntime {
    pub(crate) fn invariant_access(&self) -> InvariantAccess<'_> {
        InvariantAccess::new(self)
    }
}

pub struct InvariantAccess<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl<'runtime> InvariantAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    #[cfg(test)]
    pub fn harness_audit(&self, mode: HarnessAuditMode) -> InvariantExecutionResult {
        mode.request_profile().map_or_else(
            || {
                InvariantExecutionResult::skipped(self.execution_metadata(
                    InvariantRequestProfile::HarnessAudit,
                    crate::validation::engine::InvariantObservationKind::Committed,
                    self.runtime.current_version_id(),
                    self.runtime.current_version_id(),
                    None,
                    None,
                    crate::validation::data::InvariantGroupSet::empty(),
                    crate::validation::data::InvariantCostClass::Global,
                    crate::validation::engine::InvariantExecutionDisposition::SkippedByMayBreakMask,
                    None,
                ))
            },
            |profile| self.execute_for_runtime(profile),
        )
    }

    pub fn mutation_sensitive_state(&self) -> InvariantExecutionResult {
        self.execute_for_runtime(InvariantRequestProfile::MutationSensitive)
    }

    pub fn snapshot_publication_state(&self) -> InvariantExecutionResult {
        self.execute_for_runtime(InvariantRequestProfile::SnapshotPublication)
    }

    pub fn certification_state(&self) -> InvariantExecutionResult {
        self.execute_for_runtime(InvariantRequestProfile::CertificationBoundary)
    }

    pub(crate) fn mutation_sensitive_for_state_with_proposal<'state>(
        &self,
        state: crate::storage::overlay::OverlayStateView<'state, crate::runtime::WorkingState>,
        version_id: crate::identity::data::VersionId,
        merged_plan: Option<&'state MergedCommitPlan>,
        proposal_identity: Option<&crate::mvcc::RelationalMutationProposalIdentity>,
    ) -> InvariantExecutionResult {
        self.execute_for_state(
            InvariantRequestProfile::MutationSensitive,
            InvariantObservation::speculative_with_proposal(state, proposal_identity.cloned()),
            version_id,
            merged_plan,
        )
    }

    pub(crate) fn commit_boundary_for_selected_branch(
        &self,
        selected_state: &SelectedRelationalBranchState,
        proposed_working_state: &crate::storage::overlay::WorkingState,
        proposed_version_id: crate::identity::data::VersionId,
        merged_plan: &'runtime MergedCommitPlan,
        proposal_identity: Option<&crate::mvcc::RelationalMutationProposalIdentity>,
    ) -> InvariantExecutionResult {
        self.execute_for_selected_branch_plan(
            InvariantRequestProfile::CommitBoundary,
            selected_state,
            proposed_working_state,
            proposed_version_id,
            merged_plan,
            proposal_identity,
        )
    }

    pub(crate) fn commit_boundary_for_selected_branch_plan(
        &self,
        selected_state: &SelectedRelationalBranchState,
        merged_plan: &MergedCommitPlan,
    ) -> InvariantExecutionResult {
        self.execute_for_selected_branch_committed_plan(
            InvariantRequestProfile::CommitBoundary,
            selected_state,
            merged_plan,
        )
    }

    pub(crate) fn graph_composition_for_selected_branch_plan(
        &self,
        selected_state: &SelectedRelationalBranchState,
        merged_plan: &MergedCommitPlan,
    ) -> InvariantExecutionResult {
        self.execute_for_selected_branch_committed_plan(
            InvariantRequestProfile::GraphComposition,
            selected_state,
            merged_plan,
        )
    }

    pub(crate) fn snapshot_publication_for_state_with_proposal<'state>(
        &self,
        state: crate::storage::overlay::OverlayStateView<'state, crate::runtime::WorkingState>,
        version_id: crate::identity::data::VersionId,
        merged_plan: Option<&'state MergedCommitPlan>,
        proposal_identity: Option<&crate::mvcc::RelationalMutationProposalIdentity>,
    ) -> InvariantExecutionResult {
        self.execute_for_state(
            InvariantRequestProfile::SnapshotPublication,
            InvariantObservation::speculative_with_proposal(state, proposal_identity.cloned()),
            version_id,
            merged_plan,
        )
    }
}
