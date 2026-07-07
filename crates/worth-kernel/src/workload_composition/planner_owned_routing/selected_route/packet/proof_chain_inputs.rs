use super::WorthTouchedGraphConflictSelectedRoutePacket;
use crate::workload_composition::planner_owned_routing::selected_route::proof_chain_lowering::RoutePacketProofChainLowering;

impl WorthTouchedGraphConflictSelectedRoutePacket {
    pub(crate) fn lower_proof_chain_inputs(&self) -> RoutePacketProofChainLowering {
        RoutePacketProofChainLowering {
            authority_digests: self.route_authority_digests.clone(),
            route_trace_markers: self.overlap_identity_digests.clone(),
            locality_footprint_digests: self.locality_footprint_digests.clone(),
            selected_conflict_plan_digests: self.selected_conflict_plan_digests.clone(),
            independence_proof_digests: self.independence_proof_digests.clone(),
            selected_batch_plan_digest: self.selected_batch_plan_digest.clone(),
            batch_execution_receipt_digest: self.batch_execution_receipt_digest.clone(),
            replay_undo_boundary_proof_digests: self.replay_undo_boundary_proof_digests.clone(),
            transaction_packet_identities: self.transaction_packet_identities.clone(),
            replay_scope_identities: self.replay_scope_identities.clone(),
            undo_scope_identities: self.undo_scope_identities.clone(),
            evidence_lookup_public_closeout_digest: self
                .evidence_lookup_public_closeout_digest
                .clone(),
            evidence_lookup_family_coverage_digest: self
                .evidence_lookup_family_coverage_digest
                .clone(),
            evidence_lookup_query_surface_matrix_digest: self
                .evidence_lookup_query_surface_matrix_digest
                .clone(),
            evidence_lookup_query_consumer_kit_digest: self
                .evidence_lookup_query_consumer_kit_digest
                .clone(),
            evidence_lookup_query_boundary_support_digest: self
                .evidence_lookup_query_boundary_support_digest
                .clone(),
            topology_query_backed_consumer_cutover_digest: self
                .topology_query_backed_consumer_cutover_digest
                .clone(),
            topology_query_public_read_family_row_digest: self
                .topology_query_public_read_family_row_digest
                .clone(),
            topology_query_handle_identity_digest: self
                .topology_query_handle_identity_digest
                .clone(),
            topology_query_operating_context_identity_digest: self
                .topology_query_operating_context_identity_digest
                .clone(),
            topology_query_support_snapshot_digest: self
                .topology_query_support_snapshot_digest
                .clone(),
            topology_query_compiled_product_identity_digest: self
                .selected_product_identity_digest
                .clone(),
            topology_query_equivalence_policy_identity_digest: self
                .selected_equivalence_policy_identity_digest
                .clone(),
            topology_query_selected_equivalence_family_identity: self
                .selected_family_identity
                .clone(),
            topology_query_selected_equivalence_basis_identity_digest: self
                .selected_equivalence_basis_identity_digest
                .clone(),
            topology_query_selected_route_gate_basis_identity_digest: self
                .selected_compatibility_basis_identity_digest
                .clone(),
            topology_query_selected_reuse_basis_identity_digest: self
                .selected_reuse_basis_identity_digest
                .clone(),
            compiled_product_reuse_route_packet_identity: self
                .compiled_product_reuse_route_packet
                .packet_identity()
                .to_string(),
            topology_reuse_posture: self.topology_reuse_posture(),
            spatial_reuse_posture: self.spatial_reuse_posture(),
            topology_query_reuse_decision_identity_digest: self
                .selected_witness_identity_digest
                .clone(),
            topology_query_rebuild_denial_identity_digest: self
                .rebuild_denial_identity_digest
                .clone(),
        }
    }
}
