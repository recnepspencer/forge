use schema::facade::platform::authority::replay_undo_semantic_graph::{
    ReplayUndoSemanticGraphPriorProofIdentity, ReplayUndoSemanticGraphStageIndexIdentity,
};
use schema::facade::platform::authority::replay_undo_semantic_graph_internal::{
    admit_replay_undo_stage_index_identity, admit_spatial_evidence_lookup_prior_proof_identity,
};

use super::admission_error::SpatialReplaySemanticGraphAdmissionError;
use super::replay_request::SpatialReplaySemanticGraphPreparationRequest;
pub(crate) fn prepare_workload_authority<'a>(
    request: &SpatialReplaySemanticGraphPreparationRequest<'a>,
) -> Result<
    (
        ReplayUndoSemanticGraphPriorProofIdentity,
        ReplayUndoSemanticGraphStageIndexIdentity,
    ),
    SpatialReplaySemanticGraphAdmissionError,
> {
    require_matching_stage_index_identity(request)?;
    require_matching_stage_receipt_identity(request)?;
    require_matching_lookup_execution_receipt(request)?;

    Ok((
        admit_spatial_evidence_lookup_prior_proof_identity(
            request.evidence_lookup_receipt().execution_receipt_digest(),
        ),
        admit_replay_undo_stage_index_identity(
            request
                .lookup_consumed_workload_handoff()
                .workload_stage_index_identity(),
        ),
    ))
}

fn require_matching_stage_index_identity(
    request: &SpatialReplaySemanticGraphPreparationRequest<'_>,
) -> Result<(), SpatialReplaySemanticGraphAdmissionError> {
    let authority_stage_index_identity = request.spatial_touch_authority().stage_index_identity();
    let handoff_stage_index_identity = request
        .lookup_consumed_workload_handoff()
        .workload_stage_index_identity();
    if authority_stage_index_identity == handoff_stage_index_identity {
        return Ok(());
    }
    Err(
        SpatialReplaySemanticGraphAdmissionError::StageIndexIdentityMismatch {
            authority_stage_index_identity: authority_stage_index_identity.to_string(),
            product_stage_index_identity: handoff_stage_index_identity.to_string(),
        },
    )
}

fn require_matching_stage_receipt_identity(
    request: &SpatialReplaySemanticGraphPreparationRequest<'_>,
) -> Result<(), SpatialReplaySemanticGraphAdmissionError> {
    let authority_stage_receipt_identity = request
        .spatial_touch_authority()
        .evidence_identity()
        .to_string();
    let handoff_stage_receipt_identity = request
        .lookup_consumed_workload_handoff()
        .stage_receipt_identity();
    if authority_stage_receipt_identity == handoff_stage_receipt_identity {
        return Ok(());
    }
    Err(
        SpatialReplaySemanticGraphAdmissionError::StageReceiptIdentityMismatch {
            authority_stage_receipt_identity,
            handoff_stage_receipt_identity: handoff_stage_receipt_identity.to_string(),
        },
    )
}

fn require_matching_lookup_execution_receipt(
    request: &SpatialReplaySemanticGraphPreparationRequest<'_>,
) -> Result<(), SpatialReplaySemanticGraphAdmissionError> {
    let receipt_execution_digest = request.evidence_lookup_receipt().execution_receipt_digest();
    let handoff_execution_digest = request
        .lookup_consumed_workload_handoff()
        .lookup_execution_receipt_digest();
    if receipt_execution_digest == handoff_execution_digest {
        return Ok(());
    }
    Err(
        SpatialReplaySemanticGraphAdmissionError::LookupExecutionReceiptMismatch {
            receipt_execution_digest: receipt_execution_digest.to_string(),
            handoff_execution_digest: handoff_execution_digest.to_string(),
        },
    )
}
