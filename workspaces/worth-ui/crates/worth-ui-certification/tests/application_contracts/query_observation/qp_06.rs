use super::capacity_world::run_capacity_boundary;
use super::mixed_world::{run_mixed_permutation, MixedCause};

const PERMUTATIONS: [[MixedCause; 3]; 6] = [
    [MixedCause::Query, MixedCause::Source, MixedCause::Viewport],
    [MixedCause::Query, MixedCause::Viewport, MixedCause::Source],
    [MixedCause::Source, MixedCause::Query, MixedCause::Viewport],
    [MixedCause::Source, MixedCause::Viewport, MixedCause::Query],
    [MixedCause::Viewport, MixedCause::Query, MixedCause::Source],
    [MixedCause::Viewport, MixedCause::Source, MixedCause::Query],
];

#[test]
fn all_mixed_cause_permutations_publish_one_canonical_successor_after_effecting() {
    let mut expected_cost = None;
    for (index, permutation) in PERMUTATIONS.into_iter().enumerate() {
        let cost = run_mixed_permutation(permutation, index);
        match expected_cost {
            Some(expected) => assert_eq!(cost, expected),
            None => expected_cost = Some(cost),
        }
    }
}

#[test]
fn effecting_queue_stops_the_seventeenth_observation_without_losing_it() {
    run_capacity_boundary();
}
