use super::{
    FinancialCertificationPolicy, FinancialNecessityEvidence, FinancialScenarioCertificationClaim,
    FinancialScenarioCompletion, FreshFinancialRecompute,
};
use crate::tests::domains::fintech::invalidation::run_comparator_scenario;
use crate::tests::domains::fintech::world::{
    compile_financial_world, FinancialConsumerRole, FinancialOutputEquivalencePolicy,
    FinancialScenarioIdentity, FinancialWorldDefinition, FxPair, InstrumentId, MarketFactorKey,
    SemanticOutputKey,
};

#[test]
fn claim_construction_rejects_runtime_oracle_and_manifest_drift() {
    let base = FinancialWorldDefinition::deterministic(41);
    let factor = MarketFactorKey::FxSpot(FxPair::EurUsd);
    let shocked = base.with_market_factor_delta(factor, 20_000);
    let baseline = compile_financial_world(base.clone()).unwrap();
    let reproduction = baseline.reproduction();
    let mut compiled = baseline.into_compiled();
    let instrument = InstrumentId("EURUSD-1Y-FWD");
    let evidence = compiled
        .apply_quote_translation_change(shocked.clone(), factor, instrument)
        .unwrap();
    let completion =
        FinancialScenarioCompletion::quote_translation(&compiled, &evidence, factor, instrument)
            .unwrap();
    let necessity = FinancialNecessityEvidence::for_mutation(&base, factor);
    let revision_key = SemanticOutputKey::Consumer(FinancialConsumerRole::RiskMatched);

    assert!(FinancialScenarioCertificationClaim::verify(
        &compiled,
        &FreshFinancialRecompute::run(&FinancialWorldDefinition::deterministic(99)),
        &necessity,
        reproduction,
        FinancialScenarioIdentity::QuoteToRiskAspectTranslation,
        FinancialCertificationPolicy::Exact,
        revision_key,
        completion.clone(),
    )
    .is_err());

    compiled
        .forge_node_version_for_test(
            SemanticOutputKey::Risk(InstrumentId("EURUSD-1Y-FWD")),
            crate::data::aspect::AspectVersion::zero(),
        )
        .unwrap();
    assert!(FinancialScenarioCertificationClaim::verify(
        &compiled,
        &FreshFinancialRecompute::run(&shocked),
        &necessity,
        reproduction,
        FinancialScenarioIdentity::QuoteToRiskAspectTranslation,
        FinancialCertificationPolicy::Exact,
        revision_key,
        completion,
    )
    .is_err());
}

#[test]
fn claim_construction_rejects_nonzero_dependency_revision_drift() {
    let base = FinancialWorldDefinition::deterministic(41);
    let factor = MarketFactorKey::FxSpot(FxPair::EurUsd);
    let shocked = base.with_market_factor_delta(factor, 20_000);
    let baseline = compile_financial_world(base.clone()).unwrap();
    let reproduction = baseline.reproduction();
    let mut compiled = baseline.into_compiled();
    let instrument = InstrumentId("EURUSD-1Y-FWD");
    let evidence = compiled
        .apply_quote_translation_change(shocked.clone(), factor, instrument)
        .unwrap();
    let completion =
        FinancialScenarioCompletion::quote_translation(&compiled, &evidence, factor, instrument)
            .unwrap();
    let revision_key = SemanticOutputKey::Consumer(FinancialConsumerRole::RiskMatched);
    compiled
        .advance_dependency_revision_for_test(revision_key)
        .unwrap();

    assert!(FinancialScenarioCertificationClaim::verify(
        &compiled,
        &FreshFinancialRecompute::run(&shocked),
        &FinancialNecessityEvidence::for_mutation(&base, factor),
        reproduction,
        FinancialScenarioIdentity::QuoteToRiskAspectTranslation,
        FinancialCertificationPolicy::Exact,
        revision_key,
        completion,
    )
    .is_err());
}

#[test]
fn claim_construction_rejects_preverify_producer_epsilon_drift() {
    let small = run_comparator_scenario(Some(5), 2_000, 2);
    let large = run_comparator_scenario(Some(5), 20_000, 6);
    let base = FinancialWorldDefinition::comparator_courtroom(41)
        .with_factor_output_tolerance(large.factor, 5);
    let revision_delta = large.final_revision.abs_diff(large.baseline_revision);
    let mut reproduction = large.reproduction;
    reproduction.policy.producer_output_equivalence =
        FinancialOutputEquivalencePolicy::Tolerance { epsilon: 999 };

    assert!(FinancialScenarioCertificationClaim::verify(
        &large.compiled,
        &FreshFinancialRecompute::run(large.compiled.definition()),
        &FinancialNecessityEvidence::for_revision_delta(&base, large.factor, revision_delta),
        reproduction,
        FinancialScenarioIdentity::ToleranceSuppressedRepricing,
        FinancialCertificationPolicy::ProducerTolerance,
        SemanticOutputKey::Risk(InstrumentId("EURUSD-1Y-FWD")),
        FinancialScenarioCompletion::tolerance(&small, &large).unwrap(),
    )
    .is_err());
}
