use super::WorthWorkloadOrdinaryConsumerSelectedPlanWitness;
use crate::workload_composition::planner_owned_routing::ordinary_consumer_authority::{
    WorthWorkloadOrdinaryConsumerCurrentRouteWitness, WorthWorkloadOrdinaryConsumerRouteKind,
};

impl WorthWorkloadOrdinaryConsumerSelectedPlanWitness {
    pub(super) fn new(
        route_witness: &WorthWorkloadOrdinaryConsumerCurrentRouteWitness,
        batch_execution_receipt_digest: &str,
    ) -> Self {
        Self {
            route_kind: route_witness.route_kind(),
            route_lineage_digest: route_witness.route_lineage_digest().to_string(),
            route_authority_digest: route_witness.route_authority_digest().to_string(),
            replay_undo_boundary_proof_digest: route_witness
                .replay_undo_boundary_proof_digest()
                .map(str::to_string),
            transaction_packet_identity: route_witness
                .transaction_packet_identity()
                .map(str::to_string),
            replay_scope_identity: route_witness.replay_scope_identity().map(str::to_string),
            undo_scope_identity: route_witness.undo_scope_identity().map(str::to_string),
            batch_execution_receipt_digest: batch_execution_receipt_digest.to_string(),
        }
    }

    pub(crate) const fn route_kind(&self) -> WorthWorkloadOrdinaryConsumerRouteKind {
        self.route_kind
    }

    pub fn batch_execution_receipt_digest(&self) -> &str {
        &self.batch_execution_receipt_digest
    }

    pub fn route_lineage_digest(&self) -> &str {
        &self.route_lineage_digest
    }

    pub fn route_authority_digest(&self) -> &str {
        &self.route_authority_digest
    }

    pub fn replay_undo_boundary_proof_digest(&self) -> Option<&str> {
        self.replay_undo_boundary_proof_digest.as_deref()
    }

    pub fn transaction_packet_identity(&self) -> Option<&str> {
        self.transaction_packet_identity.as_deref()
    }

    pub fn replay_scope_identity(&self) -> Option<&str> {
        self.replay_scope_identity.as_deref()
    }

    pub fn undo_scope_identity(&self) -> Option<&str> {
        self.undo_scope_identity.as_deref()
    }
}
