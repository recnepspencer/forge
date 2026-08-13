use crate::tests::domains::fintech::world::{
    FinancialComparatorProfile, FinancialOutputEquivalencePolicy, FinancialScenarioIdentity,
    SemanticOutputKey,
};

use super::comparator_world::{
    exact_consumer, installed_consumer, run_comparator_scenario, tolerance_consumer,
};

#[test]
fn heterogeneous_consumer_comparators() {
    let small = run_comparator_scenario(None, 2_000, 2);
    assert_eq!(small.reproduction.seed, 41);
    assert_eq!(
        small.reproduction.scenario,
        FinancialScenarioIdentity::HeterogeneousConsumerComparators
    );
    assert_eq!(small.reproduction.mutation_step, 2);
    assert_eq!(small.reproduction.economic_delta, 2_000);
    assert_eq!(
        small.reproduction.policy.consumer_comparators,
        FinancialComparatorProfile::ExactToleranceAndInstalledTolerance
    );
    assert_eq!(
        small.reproduction.policy.producer_output_equivalence,
        FinancialOutputEquivalencePolicy::Exact
    );
    assert_eq!(small.final_revision - small.baseline_revision, 2);
    assert_eq!(small.observed_work, small.required_work);
    assert!(small.observed_work.contains(&exact_consumer()));
    assert!(!small.observed_work.contains(&tolerance_consumer()));
    assert!(!small.observed_work.contains(&installed_consumer()));

    let large = run_comparator_scenario(None, 20_000, 6);
    assert_eq!(large.reproduction.mutation_step, 6);
    assert_eq!(large.reproduction.economic_delta, 20_000);
    assert_eq!(large.final_revision - large.baseline_revision, 6);
    assert_eq!(large.observed_work, large.required_work);
    for consumer in [exact_consumer(), tolerance_consumer(), installed_consumer()] {
        assert!(large.observed_work.contains(&consumer));
        assert_eq!(large.compiled.ledger().count(consumer), 1);
    }
    assert!(large
        .observed_work
        .contains(&SemanticOutputKey::Factor(large.factor)));
}
