use crate::merge::data::{
    MergeExecutionRequest, MergePlanningRequest, NormalizedRelationalMergeRequest,
    RelationalMergeRequestNormalizationDenial,
};

pub(super) fn normalize_merge_planning_request(
    request: MergePlanningRequest,
) -> Result<NormalizedRelationalMergeRequest, RelationalMergeRequestNormalizationDenial> {
    NormalizedRelationalMergeRequest::from_planning_request(request)
}

pub(super) fn normalize_merge_execution_request(
    request: MergeExecutionRequest,
) -> Result<NormalizedRelationalMergeRequest, RelationalMergeRequestNormalizationDenial> {
    NormalizedRelationalMergeRequest::from_execution_request(request)
}
