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
#[path = "public_api_planar_boolean_loop_reconstruction_workload_evidence_support.rs"]
#[allow(dead_code)]
mod loop_workload_evidence_support;
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
#[path = "public_api_spatial_touch_receipt_admission_support.rs"]
mod spatial_touch_receipt_admission_support;

#[test]
fn phase4_spatial_touch_segment_pair_handoff_admits_real_completed_workload_ledger() {
    reduced_pair_support::run_with_large_stack(|| {
        spatial_touch_receipt_admission_support::
            assert_segment_pair_receipt_admits_from_completed_workload_handoff();
    });
}

#[test]
fn phase4_spatial_touch_event_ledger_handoff_admits_real_completed_workload_ledger() {
    reduced_pair_support::run_with_large_stack(|| {
        spatial_touch_receipt_admission_support::
            assert_event_ledger_receipt_admits_from_completed_workload_handoff();
    });
}

#[test]
fn phase4_spatial_touch_split_handoff_admits_real_completed_workload_ledger() {
    reduced_pair_support::run_with_large_stack(|| {
        spatial_touch_receipt_admission_support::
            assert_split_receipt_admits_from_completed_workload_handoff();
    });
}

#[test]
fn phase4_spatial_touch_loop_handoff_admits_real_completed_workload_ledger() {
    reduced_pair_support::run_with_large_stack(|| {
        spatial_touch_receipt_admission_support::
            assert_loop_receipt_admits_from_completed_workload_handoff();
    });
}

#[test]
fn phase4_spatial_touch_split_replay_equivalence_uses_completed_workload_ledgers() {
    reduced_pair_support::run_with_large_stack(|| {
        spatial_touch_receipt_admission_support::
            assert_split_replay_preserves_completed_workload_spatial_touch_authority();
    });
}

#[test]
fn phase4_spatial_touch_loop_replay_equivalence_uses_completed_workload_ledgers() {
    reduced_pair_support::run_with_large_stack(|| {
        spatial_touch_receipt_admission_support::
            assert_loop_replay_preserves_completed_workload_spatial_touch_authority();
    });
}

#[test]
fn phase6_spatial_touch_migrated_split_consumer_uses_facade_lookup_authority() {
    reduced_pair_support::run_with_large_stack(|| {
        edge_splitting_public_contract_support::
            assert_split_downstream_migration_uses_spatial_facade_proof_product();
    });
}

#[test]
fn phase10_cross_crate_replay_preserves_kernel_spatial_query_handoff() {
    reduced_pair_support::run_with_large_stack(|| {
        spatial_touch_receipt_admission_support::
            assert_split_replay_preserves_cross_crate_spatial_query_handoff();
    });
}
