use super::admission_error::SpatialReplaySemanticGraphAdmissionError;
use super::replay_request::{
    SpatialReplaySemanticGraphPreparationRequest, SpatialReplaySemanticGraphPreparedRequest,
};
use super::workload_authority::prepare_workload_authority;

pub fn prepare_spatial_replay_semantic_graph_request<'a>(
    request: SpatialReplaySemanticGraphPreparationRequest<'a>,
) -> Result<SpatialReplaySemanticGraphPreparedRequest<'a>, SpatialReplaySemanticGraphAdmissionError>
{
    let (prior_proof_identity, stage_index_identity) = prepare_workload_authority(&request)?;
    Ok(SpatialReplaySemanticGraphPreparedRequest::new(
        request.family_identity(),
        request.spatial_touch_authority(),
        prior_proof_identity,
        stage_index_identity,
        request.lookup_consumed_workload_handoff(),
        request.retained_replay_receipt(),
    ))
}
