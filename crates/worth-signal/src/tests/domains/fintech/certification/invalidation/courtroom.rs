use crate::data::error::SignalError;
use crate::facade::DiagnosticsTier;
use crate::tests::domains::fintech::invalidation::run_comparator_scenario;
use crate::tests::domains::fintech::world::{
    compile_financial_world, CurveBucket, FinancialConsumerRole, FinancialReproductionTuple,
    FinancialScenarioIdentity, FinancialWorldDefinition, FxPair, InstrumentId, MarketFactorKey,
    SemanticOutputKey,
};

use super::{
    FinancialAspectCausalityCertificationRun, FinancialCertificationPolicy,
    FinancialNecessityEvidence, FinancialScenarioCertificationClaim, FinancialScenarioCompletion,
    FreshFinancialRecompute,
};

pub(in crate::tests::domains::fintech) fn run_financial_causality_courtroom(
) -> Result<FinancialAspectCausalityCertificationRun, SignalError> {
    FinancialAspectCausalityCertificationRun::seal(build_financial_causality_claims()?)
}

pub(super) fn build_financial_causality_claims(
) -> Result<Vec<FinancialScenarioCertificationClaim>, SignalError> {
    Ok(vec![
        quote_claim()?,
        heterogeneous_claim()?,
        tolerance_claim()?,
        producer_slot_claim()?,
        partition_claim()?,
        gated_claim()?,
        rewire_claim()?,
        branch_claim()?,
    ])
}

fn quote_claim() -> Result<FinancialScenarioCertificationClaim, SignalError> {
    let base = FinancialWorldDefinition::deterministic(41);
    let factor = MarketFactorKey::FxSpot(FxPair::EurUsd);
    let shocked = base.with_market_factor_delta(factor, 20_000);
    let baseline = compile_financial_world(base.clone())?;
    let mut reproduction = baseline.reproduction();
    stamp(
        &mut reproduction,
        FinancialScenarioIdentity::QuoteToRiskAspectTranslation,
        1,
        20_000,
    );
    let mut compiled = baseline.into_compiled();
    let instrument = InstrumentId("EURUSD-1Y-FWD");
    let translation =
        compiled.apply_quote_translation_change(shocked.clone(), factor, instrument)?;
    verified_claim(
        &compiled,
        reproduction,
        FinancialScenarioIdentity::QuoteToRiskAspectTranslation,
        FinancialCertificationPolicy::Exact,
        SemanticOutputKey::Consumer(FinancialConsumerRole::RiskMatched),
        &FreshFinancialRecompute::run(&shocked),
        &FinancialNecessityEvidence::for_mutation(&base, factor),
        FinancialScenarioCompletion::quote_translation(
            &compiled,
            &translation,
            factor,
            instrument,
        )?,
    )
}

fn heterogeneous_claim() -> Result<FinancialScenarioCertificationClaim, SignalError> {
    let outcome = run_comparator_scenario(None, 20_000, 6);
    let base = FinancialWorldDefinition::comparator_courtroom(41);
    let revision_delta = outcome.final_revision.abs_diff(outcome.baseline_revision);
    verified_claim(
        &outcome.compiled,
        outcome.reproduction,
        FinancialScenarioIdentity::HeterogeneousConsumerComparators,
        FinancialCertificationPolicy::HeterogeneousComparators,
        SemanticOutputKey::Risk(InstrumentId("EURUSD-1Y-FWD")),
        &FreshFinancialRecompute::run(outcome.compiled.definition()),
        &FinancialNecessityEvidence::for_revision_delta(&base, outcome.factor, revision_delta),
        FinancialScenarioCompletion::heterogeneous(&outcome)?,
    )
}

fn tolerance_claim() -> Result<FinancialScenarioCertificationClaim, SignalError> {
    let small = run_comparator_scenario(Some(5), 2_000, 2);
    let large = run_comparator_scenario(Some(5), 20_000, 6);
    let base = FinancialWorldDefinition::comparator_courtroom(41)
        .with_factor_output_tolerance(large.factor, 5);
    let revision_delta = large.final_revision.abs_diff(large.baseline_revision);
    verified_claim(
        &large.compiled,
        large.reproduction,
        FinancialScenarioIdentity::ToleranceSuppressedRepricing,
        FinancialCertificationPolicy::ProducerTolerance,
        SemanticOutputKey::Risk(InstrumentId("EURUSD-1Y-FWD")),
        &FreshFinancialRecompute::run(large.compiled.definition()),
        &FinancialNecessityEvidence::for_revision_delta(&base, large.factor, revision_delta),
        FinancialScenarioCompletion::tolerance(&small, &large)?,
    )
}

fn producer_slot_claim() -> Result<FinancialScenarioCertificationClaim, SignalError> {
    let base = FinancialWorldDefinition::producer_local_slot_courtroom(41);
    let factor = MarketFactorKey::FxSpot(FxPair::EurUsd);
    let curve = MarketFactorKey::Curve(CurveBucket::UsdOneYear);
    let shocked = base.with_market_factor_delta(factor, 20_000);
    let baseline = compile_financial_world(base.clone())?;
    let mut reproduction = baseline.reproduction();
    stamp(
        &mut reproduction,
        FinancialScenarioIdentity::ProducerLocalFactorSlotCollision,
        1,
        20_000,
    );
    let mut compiled = baseline.into_compiled();
    compiled.apply_factor_change(shocked.clone(), factor)?;
    verified_claim(
        &compiled,
        reproduction,
        FinancialScenarioIdentity::ProducerLocalFactorSlotCollision,
        FinancialCertificationPolicy::ProducerLocalSlots,
        SemanticOutputKey::Risk(InstrumentId("EURUSD-1Y-FWD")),
        &FreshFinancialRecompute::run(&shocked),
        &FinancialNecessityEvidence::for_mutation(&base, factor),
        FinancialScenarioCompletion::producer_slots(&compiled, factor, curve)?,
    )
}

fn partition_claim() -> Result<FinancialScenarioCertificationClaim, SignalError> {
    let base = FinancialWorldDefinition::partition_courtroom(41);
    let usd = MarketFactorKey::Curve(CurveBucket::UsdOneYear);
    let eur = MarketFactorKey::Curve(CurveBucket::EurOneYear);
    let after_usd = base.with_market_factor_delta(usd, 4);
    let after_both = after_usd.with_market_factor_delta(eur, 7);
    let baseline = compile_financial_world(base.clone())?;
    let mut reproduction = baseline.reproduction();
    stamp(
        &mut reproduction,
        FinancialScenarioIdentity::PartitionedCurveBucketBump,
        2,
        11,
    );
    let mut compiled = baseline.into_compiled();
    let evidence = compiled.apply_factor_change_sequence(
        &[(after_usd, usd), (after_both.clone(), eur)],
        InstrumentId("EURUSD-1Y-FWD"),
    )?;
    verified_claim(
        &compiled,
        reproduction,
        FinancialScenarioIdentity::PartitionedCurveBucketBump,
        FinancialCertificationPolicy::ExactPartitionLocality,
        SemanticOutputKey::Risk(InstrumentId("EURUSD-1Y-FWD")),
        &FreshFinancialRecompute::run(&after_both),
        &FinancialNecessityEvidence::for_mutations(&base, [usd, eur]),
        FinancialScenarioCompletion::partition(&evidence)?,
    )
}

fn gated_claim() -> Result<FinancialScenarioCertificationClaim, SignalError> {
    let base = FinancialWorldDefinition::gated_courtroom(41);
    let factor = MarketFactorKey::FxSpot(FxPair::EurUsd);
    let instrument = InstrumentId("EURUSD-1Y-FWD");
    let consumer = SemanticOutputKey::Consumer(FinancialConsumerRole::RiskThreshold);
    let mut small = compile_financial_world(base.clone())?.into_compiled();
    let small_final = base.with_market_factor_delta(factor, 2_000);
    small.apply_gated_factor_sequence(
        &[
            (base.with_market_factor_delta(factor, 1_000), factor),
            (small_final, factor),
        ],
        instrument,
        FinancialConsumerRole::RiskThreshold,
    )?;
    let baseline = compile_financial_world(base.clone())?;
    let mut reproduction = baseline.reproduction();
    stamp(
        &mut reproduction,
        FinancialScenarioIdentity::GatedRepricingRelease,
        3,
        3_000,
    );
    let mut large = baseline.into_compiled();
    let large_final = base.with_market_factor_delta(factor, 3_000);
    large.apply_gated_factor_sequence(
        &[
            (base.with_market_factor_delta(factor, 1_000), factor),
            (base.with_market_factor_delta(factor, 2_000), factor),
            (large_final.clone(), factor),
        ],
        instrument,
        FinancialConsumerRole::RiskThreshold,
    )?;
    verified_claim(
        &large,
        reproduction,
        FinancialScenarioIdentity::GatedRepricingRelease,
        FinancialCertificationPolicy::DeltaThreshold,
        consumer,
        &FreshFinancialRecompute::run(&large_final),
        &FinancialNecessityEvidence::for_revision_delta(&base, factor, 3),
        FinancialScenarioCompletion::gated(&small, &large)?,
    )
}

fn rewire_claim() -> Result<FinancialScenarioCertificationClaim, SignalError> {
    let instrument = InstrumentId("EURUSD-1Y-FWD");
    let base = FinancialWorldDefinition::deterministic(41);
    let old_factor = MarketFactorKey::Curve(CurveBucket::UsdOneYear);
    let cause = base.with_market_factor_delta(old_factor, 100);
    let final_definition =
        cause.with_fx_forward_domestic_curve(instrument, CurveBucket::UsdTwoYear);
    let baseline = compile_financial_world(base.clone())?;
    let mut reproduction = baseline.reproduction();
    stamp(
        &mut reproduction,
        FinancialScenarioIdentity::InstrumentDependencyRewire,
        2,
        100,
    );
    let mut compiled = baseline.into_compiled();
    let evidence = compiled.apply_instrument_dependency_rewire(
        cause.clone(),
        old_factor,
        final_definition.clone(),
        instrument,
    )?;
    verified_claim(
        &compiled,
        reproduction,
        FinancialScenarioIdentity::InstrumentDependencyRewire,
        FinancialCertificationPolicy::DependencyRewire,
        SemanticOutputKey::Valuation(instrument),
        &FreshFinancialRecompute::run(&final_definition),
        &FinancialNecessityEvidence::for_dependency_rewire(
            &base,
            &cause,
            &final_definition,
            old_factor,
            instrument,
        ),
        FinancialScenarioCompletion::rewire(&evidence)?,
    )
}

fn branch_claim() -> Result<FinancialScenarioCertificationClaim, SignalError> {
    let base = FinancialWorldDefinition::deterministic(41);
    let analysis_factor = MarketFactorKey::FxSpot(FxPair::EurUsd);
    let main_factor = MarketFactorKey::Curve(CurveBucket::UsdOneYear);
    let analysis = base.with_market_factor_delta(analysis_factor, 20_000);
    let main = base.with_market_factor_delta(main_factor, 100);
    let baseline = compile_financial_world(base.clone())?;
    let mut reproduction = baseline.reproduction();
    stamp(
        &mut reproduction,
        FinancialScenarioIdentity::BranchShockRestoreReplay,
        2,
        20_000,
    );
    let mut compiled = baseline.into_compiled();
    let development = compiled.exercise_branch_restore_replay(
        base.clone(),
        analysis.clone(),
        analysis_factor,
        main,
        main_factor,
        InstrumentId("EURUSD-1Y-FWD"),
        DiagnosticsTier::Development,
    )?;
    let mut operational_world = compile_financial_world(base.clone())?.into_compiled();
    let operational = operational_world.exercise_branch_restore_replay(
        base.clone(),
        analysis.clone(),
        analysis_factor,
        base.with_market_factor_delta(main_factor, 100),
        main_factor,
        InstrumentId("EURUSD-1Y-FWD"),
        DiagnosticsTier::Operational,
    )?;
    let lifecycle = development.certify_tier_pair(operational)?;
    verified_claim(
        &compiled,
        reproduction,
        FinancialScenarioIdentity::BranchShockRestoreReplay,
        FinancialCertificationPolicy::BranchRestoreReplay,
        SemanticOutputKey::Risk(InstrumentId("EURUSD-1Y-FWD")),
        &FreshFinancialRecompute::run(&analysis),
        &FinancialNecessityEvidence::for_mutation(&base, analysis_factor),
        FinancialScenarioCompletion::branch(lifecycle),
    )
}

fn verified_claim(
    compiled: &crate::tests::domains::fintech::world::CompiledFinancialWorld,
    reproduction: FinancialReproductionTuple,
    scenario: FinancialScenarioIdentity,
    policy: FinancialCertificationPolicy,
    revision_key: SemanticOutputKey,
    fresh: &FreshFinancialRecompute,
    necessity: &FinancialNecessityEvidence,
    completion: FinancialScenarioCompletion,
) -> Result<FinancialScenarioCertificationClaim, SignalError> {
    FinancialScenarioCertificationClaim::verify(
        compiled,
        fresh,
        necessity,
        reproduction,
        scenario,
        policy,
        revision_key,
        completion,
    )
}

fn stamp(
    reproduction: &mut FinancialReproductionTuple,
    scenario: FinancialScenarioIdentity,
    mutation_step: u32,
    economic_delta: i64,
) {
    reproduction.scenario = scenario;
    reproduction.mutation_step = mutation_step;
    reproduction.economic_delta = economic_delta;
}
