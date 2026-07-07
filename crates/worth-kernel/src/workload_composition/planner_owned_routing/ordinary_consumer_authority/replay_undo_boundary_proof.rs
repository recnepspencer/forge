#[cfg(test)]
use schema::facade::platform::authority::replay_undo_semantic_graph::ReplayUndoPlannerRouteFamily;
use topology::replay_undo_semantic_graph::current_replay_undo_topology_ordinary_undo_scope_boundary;

#[cfg(test)]
use crate::workload_composition::planner_owned_routing::{
    current_replay_undo_transaction_route_packet_with_input_override,
};
use crate::workload_composition::planner_owned_routing::{
    lower_replay_undo_boundary_execution_proof, ReplayUndoBoundaryExecutionProof,
    ReplayUndoPlannerRoutePacket,
};
#[cfg(test)]
use crate::workload_composition::planner_owned_routing::current_replay_undo_transaction_route_packet;

use super::cutover::{
    WorthWorkloadOrdinaryConsumerCutoverError, WorthWorkloadOrdinaryConsumerCutoverErrorKind,
};

#[derive(Clone, Debug)]
pub(crate) struct WorthWorkloadCurrentReplayUndoBoundaryProof {
    boundary_proof_digest: String,
    #[cfg(test)]
    route_packet_identity: String,
    #[cfg(test)]
    route_family: ReplayUndoPlannerRouteFamily,
    transaction_packet_identity: String,
    replay_scope_identity: String,
    undo_scope_identity: String,
}

pub(crate) fn lower_current_replay_undo_boundary_proof(
    route_packet: &ReplayUndoPlannerRoutePacket,
    split_boundary: &worth_spatial::facade::replay_undo_semantic_graph::CurrentReplayUndoSpatialBoundary,
) -> Result<WorthWorkloadCurrentReplayUndoBoundaryProof, WorthWorkloadOrdinaryConsumerCutoverError>
{
    let topology_boundary = current_replay_undo_topology_ordinary_undo_scope_boundary()
        .map_err(current_boundary_error)?;
    Ok(lower_current_replay_undo_boundary_proof_from_route_packet(
        route_packet,
        split_boundary,
        topology_boundary.boundary_digest(),
    ))
}

#[cfg(test)]
pub(crate) fn current_replay_undo_boundary_proof(
    split_boundary: &worth_spatial::facade::replay_undo_semantic_graph::CurrentReplayUndoSpatialBoundary,
) -> Result<WorthWorkloadCurrentReplayUndoBoundaryProof, WorthWorkloadOrdinaryConsumerCutoverError>
{
    let route_packet =
        current_replay_undo_transaction_route_packet().map_err(current_boundary_error)?;
    lower_current_replay_undo_boundary_proof(&route_packet, split_boundary)
}

impl WorthWorkloadCurrentReplayUndoBoundaryProof {
    pub(crate) fn boundary_proof_digest(&self) -> &str {
        &self.boundary_proof_digest
    }

    pub(crate) fn transaction_packet_identity(&self) -> &str {
        &self.transaction_packet_identity
    }

    #[cfg(test)]
    pub(crate) fn route_packet_identity(&self) -> &str {
        &self.route_packet_identity
    }

    #[cfg(test)]
    pub(crate) const fn route_family(&self) -> ReplayUndoPlannerRouteFamily {
        self.route_family
    }

    pub(crate) fn replay_scope_identity(&self) -> &str {
        &self.replay_scope_identity
    }

    pub(crate) fn undo_scope_identity(&self) -> &str {
        &self.undo_scope_identity
    }
}

#[cfg(test)]
pub(crate) fn test_current_replay_undo_boundary_proof_with_input_override(
    override_input: impl FnOnce(
        crate::replay_undo_transaction_boundary::ReplayUndoTransactionBoundaryInput,
    ) -> crate::replay_undo_transaction_boundary::ReplayUndoTransactionBoundaryInput,
) -> Result<WorthWorkloadCurrentReplayUndoBoundaryProof, WorthWorkloadOrdinaryConsumerCutoverError>
{
    let split_boundary =
        worth_spatial::facade::replay_undo_semantic_graph::current_boolean_split_spatial_boundary()
            .map_err(current_boundary_error)?;
    let topology_boundary = current_replay_undo_topology_ordinary_undo_scope_boundary()
        .map_err(current_boundary_error)?;
    let route_packet =
        current_replay_undo_transaction_route_packet_with_input_override(override_input)
            .map_err(current_boundary_error)?;
    Ok(lower_current_replay_undo_boundary_proof_from_route_packet(
        &route_packet,
        &split_boundary,
        topology_boundary.boundary_digest(),
    ))
}

fn lower_current_replay_undo_boundary_proof_from_route_packet(
    route_packet: &ReplayUndoPlannerRoutePacket,
    split_boundary: &worth_spatial::facade::replay_undo_semantic_graph::CurrentReplayUndoSpatialBoundary,
    topology_boundary_digest: &str,
) -> WorthWorkloadCurrentReplayUndoBoundaryProof {
    let lowering = lower_replay_undo_boundary_execution_proof(
        route_packet,
        topology_boundary_digest,
        split_boundary.authority().stage_index_identity(),
        split_boundary
            .workload_handoff()
            .lookup_execution_receipt_digest(),
    );
    let lowering = lower_current_replay_undo_boundary_proof_from_execution_lowering(lowering);
    lowering
}

fn lower_current_replay_undo_boundary_proof_from_execution_lowering(
    lowering: ReplayUndoBoundaryExecutionProof,
) -> WorthWorkloadCurrentReplayUndoBoundaryProof {
    WorthWorkloadCurrentReplayUndoBoundaryProof {
        boundary_proof_digest: lowering.boundary_proof_digest().to_string(),
        #[cfg(test)]
        route_packet_identity: lowering.route_packet_identity().to_string(),
        #[cfg(test)]
        route_family: lowering.route_family(),
        transaction_packet_identity: lowering.transaction_packet_identity().to_string(),
        replay_scope_identity: lowering.replay_scope_identity().to_string(),
        undo_scope_identity: lowering.undo_scope_identity().to_string(),
    }
}

fn current_boundary_error<E: std::fmt::Debug>(
    error: E,
) -> WorthWorkloadOrdinaryConsumerCutoverError {
    WorthWorkloadOrdinaryConsumerCutoverError::new(
        WorthWorkloadOrdinaryConsumerCutoverErrorKind::MissingCurrentProofChain,
        format!("phase 13 current replay/undo boundary proof did not assemble: {error:?}"),
    )
}
