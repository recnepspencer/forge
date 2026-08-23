use std::collections::BTreeSet;

use crate::data::aspect::AspectMask;
use crate::data::error::SignalError;
use crate::facade::NodeState;
use crate::tests::domains::fintech::aspects::{ALERT, PRICE};
use crate::tests::domains::fintech::certification::invalidation::{
    FinancialNecessityManifest, FreshFinancialRecompute,
};
use crate::tests::domains::fintech::world::{
    compile_financial_world, FinancialConsumerRole, FinancialReproductionTuple,
    FinancialWorldDefinition, FxPair, InstrumentId, MarketFactorKey, SemanticOutputKey,
};

const SCENARIO_SEED: u64 = 41;
const FX_SPOT_SHOCK: i64 = 20_000;

#[derive(Debug)]
struct QuoteToRiskScenarioResult {
    reproduction: FinancialReproductionTuple,
    necessary_work: BTreeSet<SemanticOutputKey>,
    observed_work: BTreeSet<SemanticOutputKey>,
    matched_evaluations: u64,
    unmatched_evaluations: u64,
    matched_expected_revision: u64,
    matched_actual_revision: u64,
    matched_dirty_aspects: AspectMask,
    unmatched_dirty_aspects: AspectMask,
    matched_state: NodeState,
    structural_visits: u64,
}

impl QuoteToRiskScenarioResult {
    fn assert_incremental_matches_oracles(&self) {
        assert_eq!(
            self.observed_work, self.necessary_work,
            "incremental semantic work must equal the independent financial necessity manifest"
        );
        assert_eq!(
            self.matched_actual_revision, self.matched_expected_revision,
            "the RISK-matched consumer must publish the projected fresh financial result"
        );
        assert_eq!(self.matched_evaluations, 1);
        assert_eq!(self.unmatched_evaluations, 0);
    }
}

fn run_quote_to_risk_scenario() -> Result<QuoteToRiskScenarioResult, SignalError> {
    let base_definition = FinancialWorldDefinition::deterministic(SCENARIO_SEED);
    let baseline = compile_financial_world(base_definition.clone())?;
    let reproduction = baseline.reproduction();
    let factor = MarketFactorKey::FxSpot(FxPair::EurUsd);
    let manifest = FinancialNecessityManifest::derive(&base_definition, factor);
    let shocked_definition = base_definition.with_market_factor_delta(factor, FX_SPOT_SHOCK);
    let fresh_before = FreshFinancialRecompute::run(&base_definition);
    let fresh_after = FreshFinancialRecompute::run(&shocked_definition);
    let fx = InstrumentId("EURUSD-1Y-FWD");
    assert_ne!(fresh_before.result(fx), fresh_after.result(fx));

    let mut compiled = baseline.into_compiled();
    let visits_before = compiled.metrics().invalidation.invalidation_nodes_visited;
    compiled.apply_factor_change(shocked_definition, factor)?;
    let structural_visits = compiled
        .metrics()
        .invalidation
        .invalidation_nodes_visited
        .saturating_sub(visits_before);
    let matched_key = SemanticOutputKey::Consumer(FinancialConsumerRole::RiskMatched);
    let unmatched_key = SemanticOutputKey::Consumer(FinancialConsumerRole::RiskUnmatched);
    let matched = compiled.handles().node_for(matched_key);
    let unmatched = compiled.handles().node_for(unmatched_key);
    let matched_expected_revision = compiled.projection().output(matched_key).revision;
    let matched_actual_revision = compiled.node_version(matched_key)?.get(ALERT);

    Ok(QuoteToRiskScenarioResult {
        reproduction,
        necessary_work: manifest.required_work(),
        observed_work: compiled.ledger().observed_work(),
        matched_evaluations: compiled.ledger().count(matched_key),
        unmatched_evaluations: compiled.ledger().count(unmatched_key),
        matched_expected_revision,
        matched_actual_revision,
        matched_dirty_aspects: compiled.graph().node_dirty_aspects(matched)?,
        unmatched_dirty_aspects: compiled.graph().node_dirty_aspects(unmatched)?,
        matched_state: compiled.node_state(matched_key)?,
        structural_visits,
    })
}

#[test]
fn quote_to_risk_aspect_translation_matches_fresh_truth_and_necessity() {
    let result =
        run_quote_to_risk_scenario().expect("financial quote-to-risk scenario should execute");
    assert_eq!(result.reproduction.seed, SCENARIO_SEED);
    assert_eq!(result.structural_visits, 0);
    assert_eq!(result.matched_state, NodeState::Clean);
    assert!(result.matched_dirty_aspects.is_empty());
    assert!(result
        .observed_work
        .contains(&SemanticOutputKey::Risk(InstrumentId("EURUSD-1Y-FWD"))));
    assert!(!result
        .unmatched_dirty_aspects
        .intersects(AspectMask::from_aspect(PRICE)));
    result.assert_incremental_matches_oracles();
}
