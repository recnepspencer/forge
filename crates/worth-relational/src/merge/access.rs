//! Relational merge planning and inspection entry capability.

use std::time::Instant;

use worth_foundational::FoundationalMergeAdmissionOutcome;

use crate::merge::data::{
    MergePlanningArtifactCore, MergePlanningError, MergePlanningRequest,
    NormalizedRelationalMergeRequest, RelationalFoundationalMergeRequest,
    RelationalMergeInspectionArtifact, RelationalMergeRequestNormalizationDenial,
};
use crate::runtime::RelationalRuntime;

pub struct MergeAccess<'runtime> {
    pub(super) runtime: &'runtime RelationalRuntime,
}

impl<'runtime> MergeAccess<'runtime> {
    pub fn normalize_merge_planning_request(
        &self,
        request: MergePlanningRequest,
    ) -> Result<NormalizedRelationalMergeRequest, RelationalMergeRequestNormalizationDenial> {
        super::request_normalization::normalize_merge_planning_request(request)
    }

    pub fn normalize_merge_request(
        &self,
        request: crate::merge::data::MergeExecutionRequest,
    ) -> Result<NormalizedRelationalMergeRequest, RelationalMergeRequestNormalizationDenial> {
        super::request_normalization::normalize_merge_execution_request(request)
    }

    pub fn lower_merge_request_to_foundational(
        &self,
        request: NormalizedRelationalMergeRequest,
    ) -> FoundationalMergeAdmissionOutcome<RelationalFoundationalMergeRequest> {
        super::request_foundational_lowering::lower_merge_request_to_foundational(request)
    }

    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
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
        let normalized_request = self.normalize_merge_planning_request(request)?;
        let plan = self.lower_planning_scope(normalized_request)?;
        let artifact = super::planning_artifact::materialize_planning_artifact(self.runtime, plan);
        self.runtime
            .performance_access()
            .count_merge_planning_elapsed(started_at.elapsed().as_nanos());
        Ok(artifact)
    }

    pub fn inspect_execution_surface(
        &self,
        request: MergePlanningRequest,
    ) -> Result<RelationalMergeInspectionArtifact, MergePlanningError> {
        let artifact = self.inspect_planning_scope(request)?;
        Ok(RelationalMergeInspectionArtifact::from_input(
            artifact.inspection_input(),
        ))
    }
}
