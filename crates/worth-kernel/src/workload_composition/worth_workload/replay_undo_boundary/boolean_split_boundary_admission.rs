use crate::replay_undo_transaction_boundary::{
    assemble_replay_undo_transaction_boundary_packet, ReplayUndoTransactionBoundaryAssemblyRequest,
};

use super::super::{
    CompletedBooleanSplitBatchExecutionCluster, ReplayUndoBoundaryDenial, WorkloadCompositionError,
};
use super::{AdmittedBooleanSplitReplayUndoBoundary, BooleanSplitReplayUndoBoundaryRequest};

pub fn admit_boolean_split_replay_undo_boundary(
    completed_split_cluster: &CompletedBooleanSplitBatchExecutionCluster,
    request: BooleanSplitReplayUndoBoundaryRequest<'_>,
) -> Result<AdmittedBooleanSplitReplayUndoBoundary, WorkloadCompositionError> {
    let spatial_touch_authority = completed_split_cluster.admit_split_spatial_touch_authority()?;
    require_split_receipt_matches_lookup_stage_receipt(completed_split_cluster)?;

    let packet = assemble_replay_undo_transaction_boundary_packet(
        ReplayUndoTransactionBoundaryAssemblyRequest::new(
            request.topology_undo_scope_product(),
            request.spatial_replay_scope_product(),
            request.spatial_undo_scope_product(),
            request.support_source(),
        ),
    )
    .map_err(WorkloadCompositionError::ReplayUndoTransactionBoundary)?;

    require_packet_stage_index_matches_split_handoff(&packet, completed_split_cluster)?;
    require_packet_lookup_receipt_matches_split_handoff(&packet, completed_split_cluster)?;
    require_packet_stage_index_matches_split_authority(&packet, &spatial_touch_authority)?;

    Ok(AdmittedBooleanSplitReplayUndoBoundary::new(
        completed_split_cluster.split_handoff().clone(),
        packet,
    ))
}

fn require_split_receipt_matches_lookup_stage_receipt(
    completed_split_cluster: &CompletedBooleanSplitBatchExecutionCluster,
) -> Result<(), WorkloadCompositionError> {
    if completed_split_cluster
        .split_handoff()
        .split_ledger_receipt()
        .receipt_identity()
        == completed_split_cluster
            .lookup_consumed_workload_handoff()
            .stage_receipt_identity()
    {
        Ok(())
    } else {
        Err(WorkloadCompositionError::ReplayUndoBoundary(
            ReplayUndoBoundaryDenial::SplitLookupReceiptIdentityMismatch,
        ))
    }
}

fn require_packet_stage_index_matches_split_handoff(
    packet: &crate::replay_undo_transaction_boundary::ReplayUndoTransactionBoundaryPacket,
    completed_split_cluster: &CompletedBooleanSplitBatchExecutionCluster,
) -> Result<(), WorkloadCompositionError> {
    if packet.stage_index_identity().digest()
        == completed_split_cluster.workload_stage_index_identity()
    {
        Ok(())
    } else {
        Err(WorkloadCompositionError::ReplayUndoBoundary(
            ReplayUndoBoundaryDenial::PacketStageIndexMismatchCompletedSplit,
        ))
    }
}

fn require_packet_lookup_receipt_matches_split_handoff(
    packet: &crate::replay_undo_transaction_boundary::ReplayUndoTransactionBoundaryPacket,
    completed_split_cluster: &CompletedBooleanSplitBatchExecutionCluster,
) -> Result<(), WorkloadCompositionError> {
    if packet.evidence_lookup_receipt_identity().digest()
        == completed_split_cluster
            .lookup_consumed_workload_handoff()
            .lookup_execution_receipt_digest()
    {
        Ok(())
    } else {
        Err(WorkloadCompositionError::ReplayUndoBoundary(
            ReplayUndoBoundaryDenial::PacketLookupReceiptMismatchCompletedSplit,
        ))
    }
}

fn require_packet_stage_index_matches_split_authority(
    packet: &crate::replay_undo_transaction_boundary::ReplayUndoTransactionBoundaryPacket,
    spatial_touch_authority: &worth_spatial::facade::workload_vocabulary::SpatialGeometryEvidenceTouchAuthority,
) -> Result<(), WorkloadCompositionError> {
    if packet.stage_index_identity().digest() == spatial_touch_authority.stage_index_identity() {
        Ok(())
    } else {
        Err(WorkloadCompositionError::ReplayUndoBoundary(
            ReplayUndoBoundaryDenial::PacketStageIndexMismatchSpatialTouchAuthority,
        ))
    }
}
