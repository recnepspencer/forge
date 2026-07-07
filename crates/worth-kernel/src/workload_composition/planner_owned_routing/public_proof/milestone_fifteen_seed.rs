use crate::workload_composition::{
    WorthTouchedGraphConflictAdmittedPublicProofInput, WorthTouchedGraphConflictSelectedRoutePacket,
};

use super::admitted_input::WorthTouchedGraphConflictMilestoneFifteenPlannerProofInput;
use super::milestone_fifteen_seed_support::{
    build_milestone_fifteen_seed_digest, require_planner_proof_input_matches_selected_route_packet,
};
use super::types::WorthTouchedGraphConflictPublicCloseoutError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictMilestoneFifteenSeed {
    overlap_identity_digests: Vec<String>,
    locality_footprint_digests: Vec<String>,
    selected_conflict_plan_digests: Vec<String>,
    independence_proof_digests: Vec<String>,
    selected_batch_plan_digest: String,
    batch_execution_receipt_digest: String,
    replay_undo_boundary_proof_digests: Vec<String>,
    transaction_packet_identities: Vec<String>,
    replay_scope_identities: Vec<String>,
    undo_scope_identities: Vec<String>,
    topology_compiled_product_identity_digest: String,
    topology_equivalence_policy_identity_digest: String,
    evidence_lookup_public_closeout_digest: String,
    evidence_lookup_query_boundary_support_digest: String,
    topology_query_backed_consumer_cutover_digest: String,
    topology_query_public_read_family_row_digest: String,
    topology_query_selected_equivalence_family_identity: String,
    topology_query_selected_equivalence_basis_identity_digest: String,
    topology_query_selected_compatibility_basis_identity_digest: String,
    topology_query_selected_reuse_basis_identity_digest: String,
    topology_query_reuse_decision_identity_digest: Option<String>,
    topology_query_rebuild_denial_identity_digest: Option<String>,
    compiled_product_reuse_route_packet_identity: String,
    spatial_compiled_product_identity_digest: String,
    spatial_equivalence_policy_identity_digest: String,
    spatial_selected_equivalence_family_identity: String,
    spatial_selected_reuse_basis_identity_digest: String,
    spatial_reuse_decision_identity_digest: Option<String>,
    spatial_rebuild_denial_identity_digest: Option<String>,
    residue_digest: String,
    source_firewall_digest: String,
    planner_proof_input: WorthTouchedGraphConflictMilestoneFifteenPlannerProofInput,
    seed_digest: String,
}

impl WorthTouchedGraphConflictMilestoneFifteenSeed {
    pub(crate) fn from_selected_route_packet(
        packet: &WorthTouchedGraphConflictSelectedRoutePacket,
        residue_digest: &str,
        source_firewall_digest: &str,
        admitted_public_proof_input: WorthTouchedGraphConflictAdmittedPublicProofInput,
    ) -> Result<Self, WorthTouchedGraphConflictPublicCloseoutError> {
        let planner_proof_input =
            WorthTouchedGraphConflictMilestoneFifteenPlannerProofInput::from_admitted_input(
                admitted_public_proof_input,
            );
        let lowering = packet.lower_proof_chain_inputs();
        require_planner_proof_input_matches_selected_route_packet(packet, &planner_proof_input)?;
        let seed = Self {
            overlap_identity_digests: lowering.route_trace_markers,
            locality_footprint_digests: lowering.locality_footprint_digests,
            selected_conflict_plan_digests: lowering.selected_conflict_plan_digests,
            independence_proof_digests: lowering.independence_proof_digests,
            selected_batch_plan_digest: lowering.selected_batch_plan_digest,
            batch_execution_receipt_digest: lowering.batch_execution_receipt_digest,
            replay_undo_boundary_proof_digests: lowering.replay_undo_boundary_proof_digests,
            transaction_packet_identities: lowering.transaction_packet_identities,
            replay_scope_identities: lowering.replay_scope_identities,
            undo_scope_identities: lowering.undo_scope_identities,
            topology_compiled_product_identity_digest: lowering
                .topology_query_compiled_product_identity_digest,
            topology_equivalence_policy_identity_digest: lowering
                .topology_query_equivalence_policy_identity_digest,
            evidence_lookup_public_closeout_digest: lowering.evidence_lookup_public_closeout_digest,
            evidence_lookup_query_boundary_support_digest: lowering
                .evidence_lookup_query_boundary_support_digest,
            topology_query_backed_consumer_cutover_digest: lowering
                .topology_query_backed_consumer_cutover_digest,
            topology_query_public_read_family_row_digest: lowering
                .topology_query_public_read_family_row_digest,
            topology_query_selected_equivalence_family_identity: packet
                .selected_family_identity()
                .to_string(),
            topology_query_selected_equivalence_basis_identity_digest: lowering
                .topology_query_selected_equivalence_basis_identity_digest,
            topology_query_selected_compatibility_basis_identity_digest: lowering
                .topology_query_selected_route_gate_basis_identity_digest,
            topology_query_selected_reuse_basis_identity_digest: lowering
                .topology_query_selected_reuse_basis_identity_digest,
            topology_query_reuse_decision_identity_digest: lowering
                .topology_query_reuse_decision_identity_digest,
            topology_query_rebuild_denial_identity_digest: lowering
                .topology_query_rebuild_denial_identity_digest,
            compiled_product_reuse_route_packet_identity: planner_proof_input
                .compiled_product_reuse_route_packet_identity()
                .to_string(),
            spatial_compiled_product_identity_digest: planner_proof_input
                .spatial_compiled_product_identity_digest()
                .to_string(),
            spatial_equivalence_policy_identity_digest: planner_proof_input
                .spatial_equivalence_policy_identity_digest()
                .to_string(),
            spatial_selected_equivalence_family_identity: planner_proof_input
                .spatial_selected_equivalence_family_identity()
                .to_string(),
            spatial_selected_reuse_basis_identity_digest: planner_proof_input
                .spatial_selected_reuse_basis_identity_digest()
                .to_string(),
            spatial_reuse_decision_identity_digest: planner_proof_input
                .spatial_reuse_decision_identity_digest()
                .map(str::to_string),
            spatial_rebuild_denial_identity_digest: planner_proof_input
                .spatial_rebuild_denial_identity_digest()
                .map(str::to_string),
            residue_digest: residue_digest.to_string(),
            source_firewall_digest: source_firewall_digest.to_string(),
            planner_proof_input: planner_proof_input.clone(),
            seed_digest: String::new(),
        };
        Ok(seed.with_seed_digest())
    }

    fn with_seed_digest(mut self) -> Self {
        self.seed_digest = build_milestone_fifteen_seed_digest(&self);
        self
    }

    pub fn overlap_identity_digests(&self) -> &[String] {
        &self.overlap_identity_digests
    }
    pub fn locality_footprint_digests(&self) -> &[String] {
        &self.locality_footprint_digests
    }
    pub fn selected_conflict_plan_digests(&self) -> &[String] {
        &self.selected_conflict_plan_digests
    }
    pub fn independence_proof_digests(&self) -> &[String] {
        &self.independence_proof_digests
    }
    pub fn selected_batch_plan_digest(&self) -> &str {
        &self.selected_batch_plan_digest
    }
    pub fn batch_execution_receipt_digest(&self) -> &str {
        &self.batch_execution_receipt_digest
    }
    pub fn replay_undo_boundary_proof_digests(&self) -> &[String] {
        &self.replay_undo_boundary_proof_digests
    }
    pub fn transaction_packet_identities(&self) -> &[String] {
        &self.transaction_packet_identities
    }
    pub fn replay_scope_identities(&self) -> &[String] {
        &self.replay_scope_identities
    }
    pub fn undo_scope_identities(&self) -> &[String] {
        &self.undo_scope_identities
    }
    pub fn topology_compiled_product_identity_digest(&self) -> &str {
        &self.topology_compiled_product_identity_digest
    }
    pub fn topology_equivalence_policy_identity_digest(&self) -> &str {
        &self.topology_equivalence_policy_identity_digest
    }
    pub fn evidence_lookup_public_closeout_digest(&self) -> &str {
        &self.evidence_lookup_public_closeout_digest
    }
    pub fn evidence_lookup_query_boundary_support_digest(&self) -> &str {
        &self.evidence_lookup_query_boundary_support_digest
    }
    pub fn topology_query_backed_consumer_cutover_digest(&self) -> &str {
        &self.topology_query_backed_consumer_cutover_digest
    }
    pub fn topology_query_public_read_family_row_digest(&self) -> &str {
        &self.topology_query_public_read_family_row_digest
    }
    pub fn topology_query_selected_equivalence_family_identity(&self) -> &str {
        &self.topology_query_selected_equivalence_family_identity
    }
    pub fn topology_query_selected_equivalence_basis_identity_digest(&self) -> &str {
        &self.topology_query_selected_equivalence_basis_identity_digest
    }
    pub fn topology_query_selected_compatibility_basis_identity_digest(&self) -> &str {
        &self.topology_query_selected_compatibility_basis_identity_digest
    }
    pub fn topology_query_selected_reuse_basis_identity_digest(&self) -> &str {
        &self.topology_query_selected_reuse_basis_identity_digest
    }
    pub fn topology_query_reuse_decision_identity_digest(&self) -> Option<&str> {
        self.topology_query_reuse_decision_identity_digest
            .as_deref()
    }
    pub fn topology_query_rebuild_denial_identity_digest(&self) -> Option<&str> {
        self.topology_query_rebuild_denial_identity_digest
            .as_deref()
    }
    pub fn compiled_product_reuse_route_packet_identity(&self) -> &str {
        &self.compiled_product_reuse_route_packet_identity
    }
    pub fn spatial_compiled_product_identity_digest(&self) -> &str {
        &self.spatial_compiled_product_identity_digest
    }
    pub fn spatial_equivalence_policy_identity_digest(&self) -> &str {
        &self.spatial_equivalence_policy_identity_digest
    }
    pub fn spatial_selected_equivalence_family_identity(&self) -> &str {
        &self.spatial_selected_equivalence_family_identity
    }
    pub fn spatial_selected_reuse_basis_identity_digest(&self) -> &str {
        &self.spatial_selected_reuse_basis_identity_digest
    }
    pub fn spatial_reuse_decision_identity_digest(&self) -> Option<&str> {
        self.spatial_reuse_decision_identity_digest.as_deref()
    }
    pub fn spatial_rebuild_denial_identity_digest(&self) -> Option<&str> {
        self.spatial_rebuild_denial_identity_digest.as_deref()
    }
    pub fn residue_digest(&self) -> &str {
        &self.residue_digest
    }
    pub fn source_firewall_digest(&self) -> &str {
        &self.source_firewall_digest
    }
    #[cfg(test)]
    pub(crate) fn planner_proof_input(
        &self,
    ) -> &WorthTouchedGraphConflictMilestoneFifteenPlannerProofInput {
        &self.planner_proof_input
    }
    pub fn seed_digest(&self) -> &str {
        &self.seed_digest
    }
}
