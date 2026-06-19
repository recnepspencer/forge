#[path = "public_api_planar_boolean_loop_reconstruction_candidate_contract_support/mod.rs"]
#[allow(dead_code)]
mod candidate_contract_support;
#[path = "public_api_planar_boolean_loop_reconstruction_continuation_contract_support/mod.rs"]
#[allow(dead_code)]
mod continuation_contract_support;
#[path = "public_api_planar_boolean_edge_splitting_decision_log_support.rs"]
mod edge_splitting_decision_log_support;
#[path = "public_api_planar_boolean_edge_splitting_endpoint_boundary_support.rs"]
mod edge_splitting_endpoint_boundary_support;
#[path = "public_api_planar_boolean_edge_splitting_interval_subdivision_support.rs"]
mod edge_splitting_interval_subdivision_support;
#[path = "public_api_planar_boolean_edge_splitting_persistent_naming_support.rs"]
mod edge_splitting_persistent_naming_support;
#[path = "public_api_planar_boolean_edge_splitting_raw_schedule_support.rs"]
mod edge_splitting_raw_schedule_support;
#[path = "public_api_planar_boolean_edge_splitting_replay_parity_support.rs"]
mod edge_splitting_replay_parity_support;
#[path = "public_api_planar_boolean_edge_splitting_split_vertex_identity_support.rs"]
mod edge_splitting_split_vertex_identity_support;
#[path = "public_api_planar_boolean_edge_splitting_support.rs"]
mod edge_splitting_support;
#[path = "public_api_planar_boolean_event_extraction_metaboss_support/mod.rs"]
mod metaboss_support;
#[path = "public_api_planar_boolean_common_plane_reduced_operand_pair_support.rs"]
mod reduced_pair_support;

#[test]
fn loop_reconstruction_candidate_contract_preserves_real_promotion_boundary() {
    reduced_pair_support::run_with_large_stack(|| {
        candidate_contract_support::assert_loop_reconstruction_candidate_contract_preserves_real_promotion_boundary();
    });
}
