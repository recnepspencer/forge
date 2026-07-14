mod aspect_plan_compilation;
mod authority_verification;
mod lineage_continuity;
mod plan_compilation;
mod record_plan_compilation;

use crate::merge::data::{
    ExecutionReadyLoweredMergePlan, MergeExecutionError, MergeExecutionPreparationError,
    MergeExecutionRequest, PreparedMergeExecution,
};

use super::planning_artifact::materialize_planning_artifact;
use super::MergeAccess;

impl<'runtime> MergeAccess<'runtime> {
    pub fn prepare_merge_execution(
        &self,
        request: MergeExecutionRequest,
    ) -> Result<PreparedMergeExecution, MergeExecutionPreparationError> {
        let normalized_request = self
            .normalize_merge_request(request)
            .map_err(crate::merge::data::MergePlanningError::from)
            .map_err(MergeExecutionPreparationError::Planning)?;
        let lowered_plan = self
            .lower_planning_scope(normalized_request.clone())
            .map_err(MergeExecutionPreparationError::Planning)?;
        let artifact = materialize_planning_artifact(self.runtime, lowered_plan.clone());
        let execution_ready_plan = ExecutionReadyLoweredMergePlan::try_from_lowered(
            lowered_plan,
            artifact.schema_snapshot.clone(),
        )
        .map_err(MergeExecutionPreparationError::NotExecutionReady)?;
        let bound_executable_plan = plan_compilation::compile_bound_executable_plan(
            self.runtime,
            &normalized_request,
            &execution_ready_plan,
        )
        .map_err(MergeExecutionPreparationError::Compilation)?;

        Ok(PreparedMergeExecution::new(
            normalized_request,
            artifact,
            execution_ready_plan,
            bound_executable_plan,
        ))
    }

    pub fn verify_prepared_merge_execution(
        &self,
        prepared: &PreparedMergeExecution,
    ) -> Result<(), MergeExecutionError> {
        authority_verification::verify_prepared_merge_execution(self.runtime, prepared)
    }

    #[cfg(test)]
    pub(crate) fn compile_execution_ready_merge_plan_for_test(
        &self,
        execution_ready: &ExecutionReadyLoweredMergePlan,
    ) -> Result<
        crate::merge::data::BoundExecutableMergePlan,
        crate::merge::data::MergeExecutionCompilationError,
    > {
        plan_compilation::compile_bound_executable_plan(
            self.runtime,
            &crate::merge::data::NormalizedRelationalMergeRequest::admit_full_branch(
                execution_ready.basis.target_head.branch_id.clone(),
                execution_ready.basis.source_head.branch_id.clone(),
                crate::merge::data::MergeIntent::ReconcileIntoTarget,
                crate::merge::data::RelationalMergeCorrespondencePosture::Advisory,
                crate::merge::data::RelationalMergeSchemaReconciliationPosture::Participate,
                crate::merge::data::RelationalMergeTopologyIntent::PreserveTopologySemantics,
            )
            .expect("test compilation request should admit"),
            execution_ready,
        )
    }
}
