#[path = "public_api_planar_boolean_loop_reconstruction_metaboss_closeout_support/mod.rs"]
mod closeout_support;
#[path = "public_api_planar_boolean_common_plane_reduced_operand_pair_support.rs"]
mod reduced_pair_support;

#[test]
fn planar_boolean_loop_reconstruction_metaboss_chain_is_canonical_replayable_role_preserving_and_unforgeable(
) {
    reduced_pair_support::run_with_large_stack(|| {
        closeout_support::assertions::assert_loop_reconstruction_summum_bonum_closeout_certifies_real_production_chain();
    });
}

#[test]
fn planar_boolean_loop_reconstruction_metaboss_replay_closeout_holds() {
    reduced_pair_support::run_with_large_stack(|| {
        closeout_support::assertions::assert_loop_reconstruction_summum_bonum_replay_closeout_holds();
    });
}

#[test]
fn planar_boolean_loop_reconstruction_metaboss_public_contract_fences_hold() {
    reduced_pair_support::run_with_large_stack(|| {
        closeout_support::assertions::assert_loop_reconstruction_summum_bonum_public_contract_fences_hold();
    });
}

#[test]
fn loop_reconstruction_metaboss_rejects_synthetic_loop_ledgers_raw_fragments_and_hand_filled_evidence(
) {
    reduced_pair_support::run_with_large_stack(|| {
        closeout_support::synthetic_rejection::assert_loop_reconstruction_metaboss_rejects_synthetic_loop_ledgers_raw_fragments_and_hand_filled_evidence();
    });
}
