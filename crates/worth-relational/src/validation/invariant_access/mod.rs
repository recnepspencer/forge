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
use crate::validation::engine::{InvariantExecutionResult, InvariantRequestProfile};

impl RelationalRuntime {
    pub(crate) fn invariant_access(&self) -> InvariantAccess<'_> {
        InvariantAccess::new(self)
    }
}

pub struct InvariantAccess<'runtime> {
    runtime: &'runtime RelationalRuntime,
    view: crate::validation::engine::InvariantRuntimeView<'runtime>,
}

impl<'runtime> InvariantAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self {
            runtime,
            view: crate::validation::engine::InvariantRuntimeView::from_runtime(runtime),
        }
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
}
