use crate::logic::runtime::{PartitionAccess, RelationalRuntime};
use crate::transactions::data::MergedCommitPlan;
use crate::validation::engine::{
    HarnessAuditMode, InvariantEngine, InvariantExecutionRequest, InvariantExecutionResult,
    InvariantRequestProfile,
};

impl RelationalRuntime {
    pub fn invariant_access(&self) -> InvariantAccess<'_> {
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

    pub fn harness_audit(&self, mode: HarnessAuditMode) -> InvariantExecutionResult {
        mode.request_profile()
            .map_or_else(|| InvariantExecutionResult::new(Vec::new()), |profile| {
                self.execute_for_runtime(profile)
            })
    }

    pub fn mutation_sensitive_state(&self) -> InvariantExecutionResult {
        self.execute_for_runtime(InvariantRequestProfile::MutationSensitive)
    }

    pub fn snapshot_publication_state(&self) -> InvariantExecutionResult {
        self.execute_for_runtime(InvariantRequestProfile::SnapshotPublication)
    }

    pub(crate) fn mutation_sensitive_for_state(
        &self,
        state: &impl PartitionAccess,
        version_id: crate::identity::data::VersionId,
        merged_plan: Option<&MergedCommitPlan>,
    ) -> InvariantExecutionResult {
        self.execute_for_state(
            InvariantRequestProfile::MutationSensitive,
            state,
            version_id,
            merged_plan,
        )
    }

    pub(crate) fn commit_boundary(&self, merged_plan: &MergedCommitPlan) -> InvariantExecutionResult {
        self.execute_for_runtime_plan(
            InvariantRequestProfile::CommitBoundary,
            merged_plan,
        )
    }

    pub(crate) fn snapshot_publication_for_state(
        &self,
        state: &impl PartitionAccess,
        version_id: crate::identity::data::VersionId,
        merged_plan: Option<&MergedCommitPlan>,
    ) -> InvariantExecutionResult {
        self.execute_for_state(
            InvariantRequestProfile::SnapshotPublication,
            state,
            version_id,
            merged_plan,
        )
    }

    fn execute_for_runtime(
        &self,
        profile: InvariantRequestProfile,
    ) -> InvariantExecutionResult {
        self.execute_for_state(
            profile,
            &self.runtime.current_state(),
            self.runtime.current_version_id(),
            None,
        )
    }

    fn execute_for_runtime_plan(
        &self,
        profile: InvariantRequestProfile,
        merged_plan: &'runtime MergedCommitPlan,
    ) -> InvariantExecutionResult {
        self.execute_for_state(
            profile,
            &self.runtime.current_state(),
            self.runtime.current_version_id(),
            Some(merged_plan),
        )
    }

    fn execute_for_state(
        &self,
        profile: InvariantRequestProfile,
        state: &'runtime dyn PartitionAccess,
        version_id: crate::identity::data::VersionId,
        merged_plan: Option<&'runtime MergedCommitPlan>,
    ) -> InvariantExecutionResult {
        let request = InvariantExecutionRequest::from_profile(
            profile,
            state,
            version_id,
            merged_plan,
        );
        InvariantEngine::new(self.runtime).execute(request)
    }
}
