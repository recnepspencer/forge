use super::{
    WorthQueryGraphObligationDenialProjection, WorthQueryGraphObligationExecutionInput,
    WorthQueryGraphObligationExecutionResultEnvelope, WorthQueryGraphObligationExecutionResultRow,
    WorthQueryGraphObligationExecutionStatus, WorthQueryGraphObligationKind,
    WorthQueryGraphObligationOperatingWorldSelector, WorthQueryGraphObligationRegistration,
    WorthQueryGraphObligationRuleIdentity, WorthQueryGraphObligationStateLoadCounters,
    WorthQueryGraphObligationSupportLane, WorthQueryGraphObligationSupportPosture,
    WorthQueryGraphObligationVerdict, WorthQueryGraphTouchSelector,
};

#[test]
fn execution_statuses_do_not_collapse_into_public_verdicts() {
    for status in [
        WorthQueryGraphObligationExecutionStatus::Unsupported,
        WorthQueryGraphObligationExecutionStatus::DiagnosticOnly,
        WorthQueryGraphObligationExecutionStatus::DeferredToBackstop,
        WorthQueryGraphObligationExecutionStatus::BudgetExceeded,
        WorthQueryGraphObligationExecutionStatus::ExecutorError,
    ] {
        let row = WorthQueryGraphObligationExecutionResultRow::status_only(input("status"), status);

        assert_eq!(row.status(), status);
        assert!(
            row.verdict().is_none(),
            "{status:?} must stay execution status, not a verdict"
        );
    }
}

#[test]
fn reduction_orders_blocks_advice_and_duplicate_rule_observations_canonically() {
    let first = WorthQueryGraphObligationExecutionResultEnvelope::new(vec![
        executed_block("loop", "first block"),
        executed_advise("near-boundary"),
        executed_block("loop", "duplicate block"),
    ]);
    let replay = WorthQueryGraphObligationExecutionResultEnvelope::new(vec![
        executed_advise("near-boundary"),
        executed_block("loop", "duplicate block"),
        executed_block("loop", "first block"),
    ]);
    let reduced = first.reduce();
    let replay_reduced = replay.reduce();

    assert_eq!(
        reduced.reduction_digest(),
        replay_reduced.reduction_digest()
    );
    assert_eq!(reduced.blocking_count(), 2);
    assert_eq!(reduced.advisory_count(), 1);
    assert_eq!(reduced.duplicate_rule_observation_count(), 1);
    assert!(reduced.blocks_if_required());
    assert!(WorthQueryGraphObligationDenialProjection::from_reduction(&reduced).is_some());
}

fn executed_block(name: &str, context: &str) -> WorthQueryGraphObligationExecutionResultRow {
    WorthQueryGraphObligationExecutionResultRow::executed(
        input(name),
        WorthQueryGraphObligationVerdict::block(context).unwrap(),
        WorthQueryGraphObligationStateLoadCounters::new(1, 2, 3),
    )
}

fn executed_advise(name: &str) -> WorthQueryGraphObligationExecutionResultRow {
    WorthQueryGraphObligationExecutionResultRow::executed(
        input(name),
        WorthQueryGraphObligationVerdict::advise("advisory context").unwrap(),
        WorthQueryGraphObligationStateLoadCounters::none(),
    )
}

fn input(name: &str) -> WorthQueryGraphObligationExecutionInput {
    WorthQueryGraphObligationExecutionInput::from_selected_registration(
        "selection.digest",
        registration(name),
    )
}

fn registration(name: &str) -> WorthQueryGraphObligationRegistration {
    WorthQueryGraphObligationRegistration::new(
        WorthQueryGraphObligationKind::BlockingInvariant,
        WorthQueryGraphObligationRuleIdentity::new("test.execution", name, "v1").unwrap(),
        WorthQueryGraphTouchSelector::relation_kind("topology.edge").unwrap(),
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    )
    .with_support_posture(WorthQueryGraphObligationSupportPosture::supported(
        WorthQueryGraphObligationSupportLane::GraphComposition,
    ))
}
