use crate::replay_undo_transaction_boundary::{
    assemble_replay_undo_transaction_boundary_packet, ReplayUndoTransactionBoundaryAssemblyRequest,
};

use super::super::{CompletedBooleanSplitHandoff, WorkloadCompositionError};
use super::{AdmittedBooleanSplitReplayUndoBoundary, BooleanSplitReplayUndoBoundaryRequest};

pub fn admit_boolean_split_replay_undo_boundary(
    completed_split_handoff: &CompletedBooleanSplitHandoff,
    request: BooleanSplitReplayUndoBoundaryRequest<'_>,
) -> Result<AdmittedBooleanSplitReplayUndoBoundary, WorkloadCompositionError> {
    completed_split_handoff
        .completed_workload()
        .admit_lookup_consumed_workload(
            completed_split_handoff.lookup_consumed_workload_handoff(),
        )?;
    let spatial_touch_authority = completed_split_handoff.admit_split_spatial_touch_authority()?;
    require_split_receipt_matches_lookup_stage_receipt(completed_split_handoff)?;

    let packet = assemble_replay_undo_transaction_boundary_packet(
        ReplayUndoTransactionBoundaryAssemblyRequest::new(
            request.topology_undo_scope_product(),
            request.spatial_replay_scope_product(),
            request.spatial_undo_scope_product(),
            request.support_source(),
        ),
    )
    .map_err(WorkloadCompositionError::ReplayUndoTransactionBoundary)?;

    require_packet_stage_index_matches_split_handoff(&packet, completed_split_handoff)?;
    require_packet_lookup_receipt_matches_split_handoff(&packet, completed_split_handoff)?;
    require_packet_stage_index_matches_split_authority(&packet, &spatial_touch_authority)?;

    Ok(AdmittedBooleanSplitReplayUndoBoundary::new(
        completed_split_handoff.clone(),
        packet,
    ))
}

fn require_split_receipt_matches_lookup_stage_receipt(
    completed_split_handoff: &CompletedBooleanSplitHandoff,
) -> Result<(), WorkloadCompositionError> {
    if completed_split_handoff
        .split_ledger_receipt()
        .receipt_identity()
        == completed_split_handoff
            .lookup_consumed_workload_handoff()
            .stage_receipt_identity()
    {
        Ok(())
    } else {
        Err(WorkloadCompositionError::ReplayUndoBoundary(
            "boolean split replay/undo boundary requires one matching split-ledger receipt and lookup stage receipt identity"
                .to_string(),
        ))
    }
}

fn require_packet_stage_index_matches_split_handoff(
    packet: &crate::replay_undo_transaction_boundary::ReplayUndoTransactionBoundaryPacket,
    completed_split_handoff: &CompletedBooleanSplitHandoff,
) -> Result<(), WorkloadCompositionError> {
    if packet.stage_index_identity().digest()
        == completed_split_handoff.workload_stage_index_identity()
    {
        Ok(())
    } else {
        Err(WorkloadCompositionError::ReplayUndoBoundary(
            "boolean split replay/undo boundary packet must match the completed split workload stage-index identity"
                .to_string(),
        ))
    }
}

fn require_packet_lookup_receipt_matches_split_handoff(
    packet: &crate::replay_undo_transaction_boundary::ReplayUndoTransactionBoundaryPacket,
    completed_split_handoff: &CompletedBooleanSplitHandoff,
) -> Result<(), WorkloadCompositionError> {
    if packet.evidence_lookup_receipt_identity().digest()
        == completed_split_handoff
            .lookup_consumed_workload_handoff()
            .lookup_execution_receipt_digest()
    {
        Ok(())
    } else {
        Err(WorkloadCompositionError::ReplayUndoBoundary(
            "boolean split replay/undo boundary packet must match the completed split lookup execution receipt identity"
                .to_string(),
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
            "boolean split replay/undo boundary packet must match the split spatial touch authority stage-index identity"
                .to_string(),
        ))
    }
}
