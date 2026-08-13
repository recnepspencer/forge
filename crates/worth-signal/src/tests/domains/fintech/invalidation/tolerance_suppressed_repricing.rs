use crate::facade::{NodeState, OutputChange};
use crate::tests::domains::fintech::world::{
    FinancialOutputEquivalencePolicy, FinancialScenarioIdentity, SemanticOutputKey,
};

use super::comparator_world::{exact_consumer, run_comparator_scenario};

#[test]
fn tolerance_suppressed_repricing() {
    let small = run_comparator_scenario(Some(5), 2_000, 2);
    assert_eq!(
        small.reproduction.scenario,
        FinancialScenarioIdentity::ToleranceSuppressedRepricing
    );
    assert_eq!(small.reproduction.mutation_step, 2);
    assert_eq!(small.reproduction.economic_delta, 2_000);
    assert_eq!(
        small.reproduction.policy.producer_output_equivalence,
        FinancialOutputEquivalencePolicy::Tolerance { epsilon: 5 }
    );
    assert_eq!(
        small.observed_work, small.required_work,
        "suppressed compiled repricing must match the mutation-sensitive oracle"
    );
    assert_eq!(
        small.observed_work,
        std::collections::BTreeSet::from([SemanticOutputKey::Factor(small.factor)])
    );
    let source = small.compiled.handles().factor(small.factor).0;
    let trace = small.compiled.graph().observe().explain(source).unwrap();
    assert_eq!(trace.output_change, Some(OutputChange::Unchanged));
    assert!(trace.propagation_suppressed);
    assert_eq!(
        small.compiled.node_state(exact_consumer()).unwrap(),
        NodeState::Clean
    );

    let large = run_comparator_scenario(Some(5), 20_000, 6);
    assert_eq!(large.reproduction.mutation_step, 6);
    assert_eq!(large.reproduction.economic_delta, 20_000);
    assert_eq!(large.observed_work, large.required_work);
    assert!(large.observed_work.contains(&exact_consumer()));
    let source = large.compiled.handles().factor(large.factor).0;
    let trace = large.compiled.graph().observe().explain(source).unwrap();
    assert_ne!(trace.output_change, Some(OutputChange::Unchanged));
    assert!(!trace.propagation_suppressed);
}
