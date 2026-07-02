use schema::facade::platform::authority::replay_undo_semantic_graph::ReplayUndoPlannerRouteFamily;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::packet::ReplayUndoPlannerRoutePacket;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ReplayUndoBoundaryExecutionProof {
    boundary_proof_digest: String,
    route_packet_identity: String,
    route_family: ReplayUndoPlannerRouteFamily,
    transaction_packet_identity: String,
    replay_scope_identity: String,
    undo_scope_identity: String,
}

pub(crate) fn lower_replay_undo_boundary_execution_proof(
    route_packet: &ReplayUndoPlannerRoutePacket,
    topology_boundary_digest: &str,
    split_stage_index_identity: &str,
    split_lookup_receipt_identity: &str,
) -> ReplayUndoBoundaryExecutionProof {
    let packet = route_packet.transaction_boundary_packet();
    let boundary_proof_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-kernel:planner-owned-replay-undo-boundary-proof:v1".to_string(),
            format!("route-family:{}", route_packet.family().as_str()),
            format!("route-packet:{}", route_packet.route_packet_identity()),
            format!("packet:{}", packet.packet_identity()),
            format!("replay-scope:{}", packet.replay_scope_identity().digest()),
            format!("undo-scope:{}", packet.undo_scope_identity().digest()),
            format!("topology-boundary:{topology_boundary_digest}"),
            format!("split-stage:{split_stage_index_identity}"),
            format!("split-lookup:{split_lookup_receipt_identity}"),
        ],
    );

    ReplayUndoBoundaryExecutionProof {
        boundary_proof_digest,
        route_packet_identity: route_packet.route_packet_identity().to_string(),
        route_family: route_packet.family(),
        transaction_packet_identity: packet.packet_identity().to_string(),
        replay_scope_identity: packet.replay_scope_identity().digest().to_string(),
        undo_scope_identity: packet.undo_scope_identity().digest().to_string(),
    }
}

impl ReplayUndoBoundaryExecutionProof {
    pub(crate) fn boundary_proof_digest(&self) -> &str {
        &self.boundary_proof_digest
    }

    pub(crate) fn route_packet_identity(&self) -> &str {
        &self.route_packet_identity
    }

    pub(crate) const fn route_family(&self) -> ReplayUndoPlannerRouteFamily {
        self.route_family
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
