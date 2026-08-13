use std::collections::BTreeSet;

use crate::tests::domains::fintech::certification::invalidation::{
    FinancialNecessityManifest, FreshFinancialRecompute,
};
use crate::tests::domains::fintech::world::{
    compile_financial_world, CompiledFinancialWorld, FinancialComparatorProfile,
    FinancialConsumerRole, FinancialOutputEquivalencePolicy, FinancialReproductionTuple,
    FinancialScenarioIdentity, FinancialWorldDefinition, FxPair, InstrumentId, MarketFactorKey,
    SemanticOutputKey,
};

pub(in crate::tests::domains::fintech) struct ComparatorScenarioOutcome {
    pub(in crate::tests::domains::fintech) reproduction: FinancialReproductionTuple,
    pub(in crate::tests::domains::fintech) required_work: BTreeSet<SemanticOutputKey>,
    pub(in crate::tests::domains::fintech) observed_work: BTreeSet<SemanticOutputKey>,
    pub(in crate::tests::domains::fintech) baseline_revision: u64,
    pub(in crate::tests::domains::fintech) final_revision: u64,
    pub(in crate::tests::domains::fintech) factor: MarketFactorKey,
    pub(in crate::tests::domains::fintech) compiled: CompiledFinancialWorld,
}

pub(in crate::tests::domains::fintech) fn run_comparator_scenario(
    factor_output_tolerance: Option<u64>,
    economic_delta: i64,
    mutation_steps: u64,
) -> ComparatorScenarioOutcome {
    assert!(mutation_steps > 0);
    let factor = MarketFactorKey::FxSpot(FxPair::EurUsd);
    let instrument = InstrumentId("EURUSD-1Y-FWD");
    let mut definition = FinancialWorldDefinition::comparator_courtroom(41);
    if let Some(epsilon) = factor_output_tolerance {
        definition = definition.with_factor_output_tolerance(factor, epsilon);
    }
    let fresh_before = FreshFinancialRecompute::run(&definition);
    let baseline = compile_financial_world(definition.clone())
        .expect("financial comparator courtroom must compile and seal");
    let mut reproduction = baseline.reproduction();
    reproduction.scenario = if factor_output_tolerance.is_some() {
        FinancialScenarioIdentity::ToleranceSuppressedRepricing
    } else {
        FinancialScenarioIdentity::HeterogeneousConsumerComparators
    };
    reproduction.policy.consumer_comparators =
        FinancialComparatorProfile::ExactToleranceAndInstalledTolerance;
    reproduction.policy.producer_output_equivalence = factor_output_tolerance
        .map_or(FinancialOutputEquivalencePolicy::Exact, |epsilon| {
            FinancialOutputEquivalencePolicy::Tolerance { epsilon }
        });
    reproduction.mutation_step = mutation_steps
        .try_into()
        .expect("financial reproduction step count must fit u32");
    reproduction.economic_delta = economic_delta;
    let mut compiled = baseline.into_compiled();
    let risk_key = SemanticOutputKey::Risk(instrument);
    let baseline_revision = compiled.projected_revision(risk_key);

    let mut final_definition = definition.clone();
    for step in 1..=mutation_steps {
        let cumulative_delta = economic_delta * step as i64 / mutation_steps as i64;
        final_definition = definition.with_market_factor_delta(factor, cumulative_delta);
        compiled
            .apply_factor_change_with_runtime_comparators(final_definition.clone(), factor)
            .expect("compiled financial mutation must evaluate through runtime comparators");
    }

    let fresh_after = FreshFinancialRecompute::run(&final_definition);
    assert_ne!(
        fresh_before.result(instrument),
        fresh_after.result(instrument),
        "economic mutation twin must change independently recomputed financial truth"
    );
    assert_eq!(
        compiled.economic_snapshot(),
        &fresh_after.economic_snapshot(),
        "mutated compiled financial truth must equal independent fresh recompute"
    );
    let final_revision = compiled.projected_revision(risk_key);
    let revision_delta = final_revision.abs_diff(baseline_revision);
    let required_work =
        FinancialNecessityManifest::derive_for_revision_delta(&definition, factor, revision_delta)
            .required_work();
    let observed_work = compiled.ledger().observed_work();

    ComparatorScenarioOutcome {
        reproduction,
        required_work,
        observed_work,
        baseline_revision,
        final_revision,
        factor,
        compiled,
    }
}

pub(super) const fn exact_consumer() -> SemanticOutputKey {
    SemanticOutputKey::Consumer(FinancialConsumerRole::RiskMatched)
}

pub(super) const fn tolerance_consumer() -> SemanticOutputKey {
    SemanticOutputKey::Consumer(FinancialConsumerRole::RiskTolerance)
}

pub(super) const fn installed_consumer() -> SemanticOutputKey {
    SemanticOutputKey::Consumer(FinancialConsumerRole::RiskInstalled)
}
