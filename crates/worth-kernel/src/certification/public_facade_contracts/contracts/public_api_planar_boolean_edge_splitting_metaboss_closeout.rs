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
#[path = "public_api_planar_boolean_edge_splitting_metaboss_closeout_support/mod.rs"]
mod edge_splitting_metaboss_closeout_support;
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

use edge_splitting_metaboss_closeout_support::{
    assert_edge_split_summum_bonum_closeout_certifies_real_production_chain,
    assert_edge_split_summum_bonum_public_contract_fences_hold,
};

#[test]
fn planar_boolean_edge_splitting_metaboss_chain_is_canonical_replayable_name_preserving_and_unforgeable(
) {
    reduced_pair_support::run_with_large_stack(|| {
        assert_edge_split_summum_bonum_closeout_certifies_real_production_chain();
    });
}

#[test]
fn edge_split_metaboss_replay_orientation_and_checkpoint_parity_hold() {
    reduced_pair_support::run_with_large_stack(|| {
        edge_splitting_metaboss_closeout_support::
            assert_edge_split_summum_bonum_replay_closeout_holds();
    });
}

#[test]
fn edge_split_metaboss_proves_candidate_index_product_rows_and_culled_pair_counts() {
    reduced_pair_support::run_with_large_stack(|| {
        edge_splitting_metaboss_closeout_support::
            assert_edge_split_summum_bonum_candidate_index_closeout_holds();
    });
}

#[test]
fn edge_split_metaboss_rejects_synthetic_split_ledgers_raw_events_and_hand_filled_evidence() {
    reduced_pair_support::run_with_large_stack(|| {
        assert_edge_split_summum_bonum_public_contract_fences_hold();
    });
}

#[test]
fn edge_split_metaboss_rejects_cross_product_candidate_discovery_as_production_proof() {
    reduced_pair_support::run_with_large_stack(|| {
        edge_splitting_metaboss_closeout_support::
            assert_edge_split_summum_bonum_rejects_cross_product_discovery();
    });
}

#[test]
fn edge_split_metaboss_localizes_every_denial_to_phase_source_edge_and_event() {
    reduced_pair_support::run_with_large_stack(|| {
        edge_splitting_metaboss_closeout_support::
            assert_edge_split_summum_bonum_decision_localization_holds();
    });
}
