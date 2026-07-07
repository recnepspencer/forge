mod accessors;
mod conflict_family_identities;
mod construction;
mod proof_chain_inputs;
#[cfg(test)]
mod test_overrides;

use schema::facade::platform::authority::replay_undo_semantic_graph::ReplayUndoPlannerRouteFamily;
use topology::certification::{
    TopologyPublicCloseoutFreshnessRequirementPosture,
    TopologyPublicCloseoutRenderedOutputComparisonPosture,
};
use worth_spatial::certification::{
    SpatialPublicCloseoutFreshnessRequirementPosture,
    SpatialPublicCloseoutRenderedOutputComparisonPosture,
};

use crate::workload_composition::planner_owned_routing::{
    BatchAdmissionPlannerRoutePacket, CompiledProductReusePlannerRoutePacket,
    ConflictIndependencePlannerRoutePacket,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictSelectedRoutePacket {
    route_authority_digests: Vec<String>,
    route_lineage_digests: Vec<String>,
    overlap_identity_digests: Vec<String>,
    locality_footprint_digests: Vec<String>,
    selected_conflict_plan_digests: Vec<String>,
    independence_proof_digests: Vec<String>,
    selected_batch_plan_digest: String,
    batch_execution_receipt_digest: String,
    replay_undo_boundary_proof_digests: Vec<String>,
    replay_undo_route_packet_identity: String,
    replay_undo_route_family: ReplayUndoPlannerRouteFamily,
    batch_admission_route_packet: BatchAdmissionPlannerRoutePacket,
    conflict_independence_route_packet: ConflictIndependencePlannerRoutePacket,
    compiled_product_reuse_route_packet: CompiledProductReusePlannerRoutePacket,
    transaction_packet_identities: Vec<String>,
    replay_scope_identities: Vec<String>,
    undo_scope_identities: Vec<String>,
    evidence_lookup_public_closeout_digest: String,
    evidence_lookup_family_coverage_digest: String,
    evidence_lookup_query_surface_matrix_digest: String,
    evidence_lookup_query_consumer_kit_digest: String,
    evidence_lookup_query_boundary_support_digest: String,
    evidence_lookup_query_support_digest: String,
    topology_query_backed_consumer_cutover_digest: String,
    topology_query_public_read_family_row_digest: String,
    topology_query_handle_identity_digest: String,
    topology_query_operating_context_identity_digest: String,
    topology_query_support_snapshot_digest: String,
    topology_query_parity_verified_count: usize,
    selected_family_identity: String,
    selected_product_identity_digest: String,
    selected_equivalence_policy_identity_digest: String,
    selected_equivalence_basis_identity_digest: String,
    selected_compatibility_basis_identity_digest: String,
    selected_reuse_basis_identity_digest: String,
    selected_witness_identity_digest: Option<String>,
    rebuild_denial_identity_digest: Option<String>,
    spatial_selected_family_identity: String,
    spatial_selected_product_identity_digest: String,
    spatial_equivalence_policy_identity_digest: String,
    topology_freshness_requirement_posture: TopologyPublicCloseoutFreshnessRequirementPosture,
    topology_rendered_output_comparison_posture:
        TopologyPublicCloseoutRenderedOutputComparisonPosture,
    spatial_freshness_requirement_posture: SpatialPublicCloseoutFreshnessRequirementPosture,
    spatial_rendered_output_comparison_posture:
        SpatialPublicCloseoutRenderedOutputComparisonPosture,
    topology_query_execution_count: usize,
    topology_row_scan_fallback_count: usize,
    topology_whole_view_fallback_count: usize,
    topology_repeated_rediscovery_denied_count: usize,
    touched_closure_digest: String,
    touched_semantic_family_key: String,
    selected_plan_digest: String,
    touched_aspect_count: usize,
    touched_scope_count: usize,
    selected_row_family_identities: Vec<String>,
    spatial_receipt_proof_row_count: usize,
    spatial_non_ordinary_residue_row_count: usize,
    source_firewall_digest: String,
    deletion_closeout_digest: String,
    selected_route_identity_digest: String,
    decision_trace_identity_digest: String,
    packet_digest: String,
}
