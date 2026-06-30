use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::facade::replay_family_catalog::current_spatial_replay_family_catalog;
use worth_spatial::facade::replay_undo_semantic_graph::{
    admit_prepared_spatial_replay_semantic_graph_input, current_boolean_event_ledger_spatial_boundary,
    lower_spatial_replay_scope_product_from_admitted_input,
    lower_spatial_undo_scope_product_from_boolean_event_ledger_request,
    prepare_spatial_replay_semantic_graph_request, BooleanEventLedgerRollbackRequest,
    CurrentReplayUndoSpatialBoundary, SpatialReplaySemanticGraphPreparationRequest,
};
use topology::replay_undo_semantic_graph::current_replay_undo_topology_boundary;

use crate::replay_undo_transaction_boundary::{
    admit_replay_undo_transaction_boundary_packet, assemble_replay_undo_transaction_boundary_input,
    ReplayUndoTransactionBoundaryAssemblyRequest, ReplayUndoTransactionBoundaryInput,
    ReplayUndoTransactionBoundaryPacket, ReplayUndoTransactionBoundarySupportSource,
};

use super::current_cutover::{
    WorthWorkloadOrdinaryConsumerCutoverError, WorthWorkloadOrdinaryConsumerCutoverErrorKind,
};

#[derive(Clone, Debug)]
pub(crate) struct WorthWorkloadCurrentReplayUndoBoundaryProof {
    boundary_proof_digest: String,
    transaction_packet_identity: String,
    replay_scope_identity: String,
    undo_scope_identity: String,
}

pub(crate) fn current_replay_undo_boundary_proof(
    split_boundary: &CurrentReplayUndoSpatialBoundary,
) -> Result<
    WorthWorkloadCurrentReplayUndoBoundaryProof,
    WorthWorkloadOrdinaryConsumerCutoverError,
> {
    let topology_boundary = current_replay_undo_topology_boundary().map_err(current_boundary_error)?;
    let packet_input = current_replay_undo_boundary_packet_input(split_boundary)?;
    let packet = admit_replay_undo_transaction_boundary_packet(packet_input)
        .map_err(current_boundary_error)?;
    lower_replay_undo_boundary_proof_from_packet(
        split_boundary,
        topology_boundary.boundary_digest(),
        &packet,
    )
}

impl WorthWorkloadCurrentReplayUndoBoundaryProof {
    pub(crate) fn boundary_proof_digest(&self) -> &str {
        &self.boundary_proof_digest
    }

    pub(crate) fn transaction_packet_identity(&self) -> &str {
        &self.transaction_packet_identity
    }

    pub(crate) fn replay_scope_identity(&self) -> &str {
        &self.replay_scope_identity
    }

    pub(crate) fn undo_scope_identity(&self) -> &str {
        &self.undo_scope_identity
    }
}

fn current_replay_undo_boundary_packet_input(
    split_boundary: &CurrentReplayUndoSpatialBoundary,
) -> Result<ReplayUndoTransactionBoundaryInput, WorthWorkloadOrdinaryConsumerCutoverError> {
    let lookup_boundary =
        current_boolean_event_ledger_spatial_boundary().map_err(current_boundary_error)?;
    let topology_boundary = current_replay_undo_topology_boundary().map_err(current_boundary_error)?;
    let topology_undo_scope = topology_boundary
        .lower_undo_scope_product()
        .map_err(current_boundary_error)?;
    let retained_replay = split_boundary.retained_replay_receipt().ok_or_else(|| {
        WorthWorkloadOrdinaryConsumerCutoverError::new(
            WorthWorkloadOrdinaryConsumerCutoverErrorKind::MissingCurrentProofChain,
            "phase 13 replay/undo boundary proof requires retained replay authority on the current split boundary",
        )
    })?;
    let replay_request = prepare_spatial_replay_semantic_graph_request(
        SpatialReplaySemanticGraphPreparationRequest::new(
            lookup_boundary.replay_family_identity(),
            split_boundary.authority(),
            split_boundary.execution_receipt(),
            split_boundary.workload_handoff(),
        )
        .with_retained_replay_receipt(retained_replay),
    )
    .map_err(current_boundary_error)?;
    let admitted_replay = admit_prepared_spatial_replay_semantic_graph_input(
        &current_spatial_replay_family_catalog(),
        &replay_request,
    )
    .map_err(current_boundary_error)?;
    let replay_scope = lower_spatial_replay_scope_product_from_admitted_input(&admitted_replay)
        .map_err(current_boundary_error)?;
    let undo_scope = lower_spatial_undo_scope_product_from_boolean_event_ledger_request(
        BooleanEventLedgerRollbackRequest::new(
            split_boundary.authority(),
            split_boundary.execution_receipt(),
            split_boundary.stage_index_product(),
            split_boundary.workload_handoff(),
        ),
    )
    .map_err(current_boundary_error)?;
    assemble_replay_undo_transaction_boundary_input(ReplayUndoTransactionBoundaryAssemblyRequest::new(
        &topology_undo_scope,
        &replay_scope,
        &undo_scope,
        ReplayUndoTransactionBoundarySupportSource::Ordinary,
    ))
    .map_err(current_boundary_error)
}

fn lower_replay_undo_boundary_proof_from_packet(
    split_boundary: &CurrentReplayUndoSpatialBoundary,
    topology_boundary_digest: &str,
    packet: &ReplayUndoTransactionBoundaryPacket,
) -> Result<WorthWorkloadCurrentReplayUndoBoundaryProof, WorthWorkloadOrdinaryConsumerCutoverError>
{
    require_packet_matches_current_split_boundary(packet, split_boundary)?;

    let boundary_proof_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-kernel:ordinary-consumer-current-replay-undo-boundary-proof:v1".to_string(),
            format!("packet:{}", packet.packet_identity()),
            format!("replay-scope:{}", packet.replay_scope_identity().digest()),
            format!("undo-scope:{}", packet.undo_scope_identity().digest()),
            format!("topology-boundary:{topology_boundary_digest}"),
            format!("split-stage:{}", split_boundary.authority().stage_index_identity()),
            format!(
                "split-lookup:{}",
                split_boundary
                    .workload_handoff()
                    .lookup_execution_receipt_digest()
            ),
        ],
    );

    Ok(WorthWorkloadCurrentReplayUndoBoundaryProof {
        boundary_proof_digest,
        transaction_packet_identity: packet.packet_identity().to_string(),
        replay_scope_identity: packet.replay_scope_identity().digest().to_string(),
        undo_scope_identity: packet.undo_scope_identity().digest().to_string(),
    })
}

#[cfg(test)]
pub(crate) fn test_current_replay_undo_boundary_packet_input(
    split_boundary: &CurrentReplayUndoSpatialBoundary,
) -> Result<ReplayUndoTransactionBoundaryInput, WorthWorkloadOrdinaryConsumerCutoverError> {
    current_replay_undo_boundary_packet_input(split_boundary)
}

#[cfg(test)]
pub(crate) fn test_lower_replay_undo_boundary_proof_from_packet(
    split_boundary: &CurrentReplayUndoSpatialBoundary,
    topology_boundary_digest: &str,
    packet: &ReplayUndoTransactionBoundaryPacket,
) -> Result<WorthWorkloadCurrentReplayUndoBoundaryProof, WorthWorkloadOrdinaryConsumerCutoverError>
{
    lower_replay_undo_boundary_proof_from_packet(
        split_boundary,
        topology_boundary_digest,
        packet,
    )
}

fn require_packet_matches_current_split_boundary(
    packet: &ReplayUndoTransactionBoundaryPacket,
    split_boundary: &CurrentReplayUndoSpatialBoundary,
) -> Result<(), WorthWorkloadOrdinaryConsumerCutoverError> {
    if packet.stage_index_identity().digest() != split_boundary.authority().stage_index_identity() {
        return Err(WorthWorkloadOrdinaryConsumerCutoverError::new(
            WorthWorkloadOrdinaryConsumerCutoverErrorKind::MissingCurrentProofChain,
            "phase 13 replay/undo boundary proof requires the admitted packet stage index to match the current split spatial authority",
        ));
    }
    if packet.evidence_lookup_receipt_identity().digest()
        != split_boundary
            .workload_handoff()
            .lookup_execution_receipt_digest()
    {
        return Err(WorthWorkloadOrdinaryConsumerCutoverError::new(
            WorthWorkloadOrdinaryConsumerCutoverErrorKind::MissingCurrentProofChain,
            "phase 13 replay/undo boundary proof requires the admitted packet lookup receipt to match the current split workload handoff",
        ));
    }
    Ok(())
}

fn current_boundary_error<E: std::fmt::Debug>(
    error: E,
) -> WorthWorkloadOrdinaryConsumerCutoverError {
    WorthWorkloadOrdinaryConsumerCutoverError::new(
        WorthWorkloadOrdinaryConsumerCutoverErrorKind::MissingCurrentProofChain,
        format!("phase 13 current replay/undo boundary proof did not assemble: {error:?}"),
    )
}
