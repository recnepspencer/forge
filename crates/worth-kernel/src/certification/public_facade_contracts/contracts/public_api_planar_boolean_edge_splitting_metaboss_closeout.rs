#[path = "public_api_planar_boolean_edge_splitting_metaboss_closeout_support/mod.rs"]
mod edge_splitting_metaboss_closeout_support;
use super::edge_splitting_public_contract_support;
use super::edge_splitting_replay_parity_support;
use super::metaboss_support;
use super::reduced_pair_support;

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
