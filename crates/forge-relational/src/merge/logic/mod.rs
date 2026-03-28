mod aspect_plan_lookup;
mod causal;
mod conflicts;
mod execution;
mod execution_diagnostics;
mod execution_mutation_plan;
mod identity;
mod lowering;
mod planning;
mod planning_artifact;
mod policy;

use std::time::Instant;

use crate::logic::runtime::RelationalRuntime;
use crate::merge::data::{MergePlanningArtifactCore, MergePlanningError, MergePlanningRequest};
pub(crate) use execution_diagnostics::{
    merge_execution_failure_artifact, merge_execution_success_artifact,
    merge_execution_summary_entry,
};
use planning_artifact::materialize_planning_artifact;

pub struct MergeAccess<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl<'runtime> MergeAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn runtime(&self) -> &'runtime RelationalRuntime {
        self.runtime
    }

    pub fn inspect_history_scope(
        &self,
        request: MergePlanningRequest,
    ) -> Result<MergePlanningArtifactCore, MergePlanningError> {
        self.inspect_planning_scope(request)
    }

    pub fn inspect_planning_scope(
        &self,
        request: MergePlanningRequest,
    ) -> Result<MergePlanningArtifactCore, MergePlanningError> {
        let started_at = Instant::now();
        let plan = self.lower_planning_scope(request)?;
        let artifact = materialize_planning_artifact(self.runtime, plan);
        self.runtime
            .performance_access()
            .count_merge_planning_elapsed(started_at.elapsed().as_nanos());
        Ok(artifact)
    }
}
