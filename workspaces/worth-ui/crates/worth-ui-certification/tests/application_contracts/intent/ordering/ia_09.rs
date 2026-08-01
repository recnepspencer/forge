use super::{
    model::{permutations, CANONICAL_OBSERVATIONS},
    world::run,
};

#[test]
fn all_twenty_four_cause_orders_preserve_one_canonical_effecting_verdict() {
    prove_all_twenty_four_cause_orders_preserve_one_canonical_effecting_verdict();
}

pub(in crate::intent) fn prove_all_twenty_four_cause_orders_preserve_one_canonical_effecting_verdict(
) {
    let permutations = permutations();
    assert_eq!(permutations.len(), 24);
    assert_eq!(
        permutations
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        24
    );
    let mut positions = [0; 4];
    let mut expected_cost = None;
    for (run_index, order) in permutations.into_iter().enumerate() {
        let verdict = run(order, run_index);
        positions[verdict.interaction_position] += 1;
        assert_eq!(verdict.admitted_order, order);
        assert_eq!(verdict.families, CANONICAL_OBSERVATIONS);
        assert_eq!(verdict.cause_publications, 1);
        assert_eq!(verdict.queued_observations, 3);
        assert!(verdict.target_preserved);
        assert_eq!(verdict.provider_counts, [1, 1, 1, 0, 1, 0, 1]);
        match expected_cost {
            Some(expected) => assert_eq!(verdict.cost, expected),
            None => expected_cost = Some(verdict.cost),
        }
    }
    assert_eq!(positions, [6, 6, 6, 6]);
}
