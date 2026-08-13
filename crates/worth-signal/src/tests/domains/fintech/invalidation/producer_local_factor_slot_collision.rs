use crate::data::node::NodeState;
use crate::tests::domains::fintech::certification::invalidation::{
    FinancialNecessityManifest, FreshFinancialRecompute,
};
use crate::tests::domains::fintech::world::{
    compile_financial_world, CurveBucket, FinancialWorldDefinition, FxPair, MarketFactorKey,
    SemanticOutputKey,
};

#[test]
fn producer_local_factor_slot_collision() {
    let base = FinancialWorldDefinition::producer_local_slot_courtroom(41);
    let fx = MarketFactorKey::FxSpot(FxPair::EurUsd);
    let curve = MarketFactorKey::Curve(CurveBucket::UsdOneYear);
    let shocked = base.with_market_factor_delta(fx, 20_000);
    let mut compiled = compile_financial_world(base.clone())
        .unwrap()
        .into_compiled();
    let shared_slot = compiled.factor_slot(fx);
    assert_eq!(shared_slot, compiled.factor_slot(curve));
    assert_ne!(
        compiled.handles().factor(fx).0,
        compiled.handles().factor(curve).0
    );
    let curve_before = compiled
        .node_version(SemanticOutputKey::Factor(curve))
        .unwrap()
        .get(shared_slot);

    compiled.apply_factor_change(shocked.clone(), fx).unwrap();

    assert_eq!(
        compiled.economic_snapshot(),
        &FreshFinancialRecompute::run(&shocked).economic_snapshot()
    );
    assert_eq!(
        compiled.ledger().observed_work(),
        FinancialNecessityManifest::derive(&base, fx).required_work()
    );
    assert!(!compiled
        .ledger()
        .observed_work()
        .contains(&SemanticOutputKey::Factor(curve)));
    assert_eq!(
        compiled
            .node_version(SemanticOutputKey::Factor(curve))
            .unwrap()
            .get(shared_slot),
        curve_before
    );
    assert_eq!(
        compiled
            .node_state(SemanticOutputKey::Factor(curve))
            .unwrap(),
        NodeState::Clean
    );
}
