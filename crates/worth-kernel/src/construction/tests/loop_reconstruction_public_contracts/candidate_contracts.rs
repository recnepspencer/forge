use super::support::{
    assert_honest_promotion_partition,
    assert_loop_reconstruction_continuation_contract_preserves_real_neighborhoods_and_ordering,
    completed_loop_products, run_with_large_stack, ReplayBranch,
};

#[test]
fn loop_reconstruction_candidate_contract_preserves_real_promotion_boundary() {
    run_with_large_stack(|| {
        let label = "phase7.4 public loop candidate contract";
        let original_products = completed_loop_products(label, ReplayBranch::Original);
        let replayed_products = completed_loop_products(label, ReplayBranch::Replayed);

        assert_eq!(
            original_products
                .walk_candidate_assembly()
                .closed_walk_candidates()
                .rows(),
            replayed_products
                .walk_candidate_assembly()
                .closed_walk_candidates()
                .rows()
        );
        assert_eq!(
            original_products
                .walk_candidate_assembly()
                .fragment_consumption_proof(),
            replayed_products
                .walk_candidate_assembly()
                .fragment_consumption_proof()
        );
        assert_eq!(
            original_products.walk_outcomes().rows(),
            replayed_products.walk_outcomes().rows()
        );
        assert_eq!(
            original_products
                .candidate_boundary()
                .loop_candidates()
                .rows(),
            replayed_products
                .candidate_boundary()
                .loop_candidates()
                .rows()
        );
        assert_eq!(
            original_products
                .candidate_boundary()
                .denied_loop_candidates()
                .rows(),
            replayed_products
                .candidate_boundary()
                .denied_loop_candidates()
                .rows()
        );

        assert_honest_promotion_partition(&original_products);
        assert_honest_promotion_partition(&replayed_products);
    });
}

#[test]
fn loop_reconstruction_continuation_contract_preserves_real_neighborhoods_and_ordering() {
    run_with_large_stack(|| {
        assert_loop_reconstruction_continuation_contract_preserves_real_neighborhoods_and_ordering(
        );
    });
}
