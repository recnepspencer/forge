mod execution;
mod metadata;
#[cfg(test)]
mod test_support;
#[cfg(test)]
mod tests;

use crate::logic::runtime::RelationalRuntime;
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
                    None,
                    None,
                    crate::validation::data::InvariantGroupSet::empty(),
                    crate::validation::data::InvariantCostClass::Global,
                    crate::validation::engine::InvariantExecutionDisposition::SkippedByMayBreakMask,
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

    pub(crate) fn mutation_sensitive_for_state(
        &self,
        state: crate::storage::overlay::OverlayStateView<
            'runtime,
            crate::logic::runtime::WorkingState,
        >,
        version_id: crate::identity::data::VersionId,
        merged_plan: Option<&MergedCommitPlan>,
    ) -> InvariantExecutionResult {
        self.execute_for_state(
            InvariantRequestProfile::MutationSensitive,
            InvariantObservation::speculative(state),
            version_id,
            merged_plan,
        )
    }

    pub(crate) fn commit_boundary(
        &self,
        merged_plan: &'runtime MergedCommitPlan,
    ) -> InvariantExecutionResult {
        self.execute_for_runtime_plan(InvariantRequestProfile::CommitBoundary, merged_plan)
    }

    pub(crate) fn snapshot_publication_for_state(
        &self,
        state: crate::storage::overlay::OverlayStateView<
            'runtime,
            crate::logic::runtime::WorkingState,
        >,
        version_id: crate::identity::data::VersionId,
        merged_plan: Option<&MergedCommitPlan>,
    ) -> InvariantExecutionResult {
        self.execute_for_state(
            InvariantRequestProfile::SnapshotPublication,
            InvariantObservation::speculative(state),
            version_id,
            merged_plan,
        )
    }
}
