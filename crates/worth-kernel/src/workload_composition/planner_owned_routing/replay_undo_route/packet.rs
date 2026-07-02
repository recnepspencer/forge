use schema::facade::platform::authority::replay_undo_semantic_graph::ReplayUndoPlannerRouteFamily;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::replay_undo_transaction_boundary::ReplayUndoTransactionBoundaryPacket;

use super::admitted_input::AdmittedReplayUndoPlannerRouteInput;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ReplayUndoPlannerRoutePacket {
    family: ReplayUndoPlannerRouteFamily,
    transaction_boundary_packet: ReplayUndoTransactionBoundaryPacket,
    route_authority_digest: String,
    route_lineage_digest: String,
    route_packet_identity: String,
}

pub(crate) fn lower_replay_undo_planner_route_packet(
    admitted_input: AdmittedReplayUndoPlannerRouteInput,
) -> ReplayUndoPlannerRoutePacket {
    let transaction_boundary_packet = admitted_input.transaction_boundary_packet().clone();
    let route_authority_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-kernel:planner-owned-replay-undo-route-authority:v1".to_string(),
            format!("family:{}", admitted_input.family().as_str()),
            format!("packet:{}", transaction_boundary_packet.packet_identity()),
            format!(
                "replay-scope:{}",
                transaction_boundary_packet.replay_scope_identity().digest()
            ),
            format!(
                "undo-scope:{}",
                transaction_boundary_packet.undo_scope_identity().digest()
            ),
            format!(
                "scope-route-product:{}",
                admitted_input.scope_route_product_identity()
            ),
            format!("source:{}", admitted_input.source_identity().as_str()),
            format!("source-path:{}", admitted_input.source_path()),
            format!("inventory-rows:{}", admitted_input.inventory_row_count()),
            format!(
                "forbidden-surface-denials:{}",
                admitted_input.forbidden_surface_denial_count()
            ),
        ],
    );
    let route_lineage_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-kernel:planner-owned-replay-undo-route-lineage:v1".to_string(),
            format!("family:{}", admitted_input.family().as_str()),
            format!("authority:{route_authority_digest}"),
            format!(
                "scope-route-product:{}",
                admitted_input.scope_route_product_identity()
            ),
            format!(
                "stage:{}",
                transaction_boundary_packet.stage_index_identity().digest()
            ),
            format!(
                "lookup:{}",
                transaction_boundary_packet
                    .evidence_lookup_receipt_identity()
                    .digest()
            ),
        ],
    );
    let route_packet_identity = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-kernel:planner-owned-replay-undo-route-packet:v1".to_string(),
            format!("family:{}", admitted_input.family().as_str()),
            format!("authority:{route_authority_digest}"),
            format!("lineage:{route_lineage_digest}"),
        ],
    );

    ReplayUndoPlannerRoutePacket {
        family: admitted_input.family(),
        transaction_boundary_packet,
        route_authority_digest,
        route_lineage_digest,
        route_packet_identity,
    }
}

impl ReplayUndoPlannerRoutePacket {
    pub(crate) const fn family(&self) -> ReplayUndoPlannerRouteFamily {
        self.family
    }

    pub(crate) const fn transaction_boundary_packet(&self) -> &ReplayUndoTransactionBoundaryPacket {
        &self.transaction_boundary_packet
    }

    pub(crate) fn route_authority_digest(&self) -> &str {
        &self.route_authority_digest
    }

    pub(crate) fn route_lineage_digest(&self) -> &str {
        &self.route_lineage_digest
    }

    pub(crate) fn route_packet_identity(&self) -> &str {
        &self.route_packet_identity
    }
}
