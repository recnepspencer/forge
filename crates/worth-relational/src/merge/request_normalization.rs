use crate::merge::data::{
    NormalizedRelationalMergeRequest, OwnerBoundMergeExecutionRequest,
    OwnerBoundMergePlanningRequest, RelationalMergeRequestNormalizationDenial,
};

pub(super) fn normalize_bound_merge_planning_request(
    request: OwnerBoundMergePlanningRequest,
) -> Result<NormalizedRelationalMergeRequest, RelationalMergeRequestNormalizationDenial> {
    NormalizedRelationalMergeRequest::from_owner_bound_planning_request(request)
}

pub(super) fn normalize_bound_merge_execution_request(
    request: OwnerBoundMergeExecutionRequest,
) -> Result<NormalizedRelationalMergeRequest, RelationalMergeRequestNormalizationDenial> {
    NormalizedRelationalMergeRequest::from_owner_bound_execution_request(request)
}
