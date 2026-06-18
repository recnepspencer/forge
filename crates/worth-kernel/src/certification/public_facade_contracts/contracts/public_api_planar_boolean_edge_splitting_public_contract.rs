#[path = "public_api_planar_boolean_collinear_relations_support/mod.rs"]
#[allow(dead_code)]
mod collinear_relation_support;
#[path = "public_api_planar_boolean_edge_splitting_decision_log_support.rs"]
#[allow(dead_code)]
mod edge_splitting_decision_log_support;
#[path = "public_api_planar_boolean_edge_splitting_endpoint_boundary_support.rs"]
#[allow(dead_code)]
mod edge_splitting_endpoint_boundary_support;
#[path = "public_api_planar_boolean_edge_splitting_interval_subdivision_support.rs"]
#[allow(dead_code)]
mod edge_splitting_interval_subdivision_support;
#[path = "public_api_planar_boolean_edge_splitting_normalized_schedule_support.rs"]
#[allow(dead_code)]
mod edge_splitting_normalized_schedule_support;
#[path = "public_api_planar_boolean_edge_splitting_ordered_schedule_support.rs"]
#[allow(dead_code)]
mod edge_splitting_ordered_schedule_support;
#[path = "public_api_planar_boolean_edge_splitting_persistent_naming_support.rs"]
#[allow(dead_code)]
mod edge_splitting_persistent_naming_support;
#[path = "public_api_planar_boolean_edge_splitting_public_contract_support/mod.rs"]
#[allow(dead_code)]
mod edge_splitting_public_contract_support;
#[path = "public_api_planar_boolean_edge_splitting_raw_schedule_support.rs"]
#[allow(dead_code)]
mod edge_splitting_raw_schedule_support;
#[path = "public_api_planar_boolean_edge_splitting_replay_parity_support.rs"]
#[allow(dead_code)]
mod edge_splitting_replay_parity_support;
#[path = "public_api_planar_boolean_edge_splitting_split_vertex_identity_support.rs"]
#[allow(dead_code)]
mod edge_splitting_split_vertex_identity_support;
#[path = "public_api_planar_boolean_edge_splitting_support.rs"]
#[allow(dead_code)]
mod edge_splitting_support;
#[path = "public_api_planar_boolean_event_ledger_support.rs"]
#[allow(dead_code)]
mod event_ledger_support;
#[path = "public_api_planar_boolean_event_extraction_metaboss_support/mod.rs"]
#[allow(dead_code, unused_imports)]
mod metaboss_support;
#[path = "public_api_planar_boolean_point_events_support/mod.rs"]
#[allow(dead_code)]
mod point_event_support;
#[path = "public_api_planar_boolean_event_predicate_binding_support.rs"]
#[allow(dead_code)]
mod predicate_binding_support;
#[path = "public_api_planar_boolean_common_plane_reduced_operand_pair_support.rs"]
mod reduced_pair_support;

#[test]
fn split_public_contract_requires_real_ledger_and_rejects_manual_split_evidence() {
    reduced_pair_support::run_with_large_stack(|| {
        edge_splitting_public_contract_support::assert_split_public_contract_requires_real_ledger_and_rejects_manual_evidence();
    });
}
