#[path = "public_api_planar_boolean_loop_reconstruction_candidate_contract_support/mod.rs"]
mod candidate_contract_support;
#[path = "public_api_planar_boolean_loop_reconstruction_continuation_contract_support/mod.rs"]
mod continuation_contract_support;
use super::edge_splitting_replay_parity_support;
use super::metaboss_support;
use super::reduced_pair_support;

const _: fn() =
    continuation_contract_support::assert_loop_reconstruction_continuation_contract_preserves_real_neighborhoods_and_ordering;

#[test]
fn loop_reconstruction_candidate_contract_preserves_real_promotion_boundary() {
    reduced_pair_support::run_with_large_stack(|| {
        candidate_contract_support::assert_loop_reconstruction_candidate_contract_preserves_real_promotion_boundary();
    });
}
