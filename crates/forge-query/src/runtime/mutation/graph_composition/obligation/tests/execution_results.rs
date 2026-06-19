use super::{
    ForgeQueryGraphObligationDenialProjection, ForgeQueryGraphObligationExecutionInput,
    ForgeQueryGraphObligationExecutionResultEnvelope, ForgeQueryGraphObligationExecutionResultRow,
    ForgeQueryGraphObligationExecutionStatus, ForgeQueryGraphObligationKind,
    ForgeQueryGraphObligationOperatingWorldSelector, ForgeQueryGraphObligationRegistration,
    ForgeQueryGraphObligationRuleIdentity, ForgeQueryGraphObligationStateLoadCounters,
    ForgeQueryGraphObligationSupportLane, ForgeQueryGraphObligationSupportPosture,
    ForgeQueryGraphObligationVerdict, ForgeQueryGraphTouchSelector,
};

#[test]
fn execution_statuses_do_not_collapse_into_public_verdicts() {
    for status in [
        ForgeQueryGraphObligationExecutionStatus::Unsupported,
        ForgeQueryGraphObligationExecutionStatus::DiagnosticOnly,
        ForgeQueryGraphObligationExecutionStatus::DeferredToBackstop,
        ForgeQueryGraphObligationExecutionStatus::BudgetExceeded,
        ForgeQueryGraphObligationExecutionStatus::ExecutorError,
    ] {
        let row = ForgeQueryGraphObligationExecutionResultRow::status_only(input("status"), status);

        assert_eq!(row.status(), status);
        assert!(
            row.verdict().is_none(),
            "{status:?} must stay execution status, not a verdict"
        );
    }
}

#[test]
fn reduction_orders_blocks_advice_and_duplicate_rule_observations_canonically() {
    let first = ForgeQueryGraphObligationExecutionResultEnvelope::new(vec![
        executed_block("loop", "first block"),
        executed_advise("near-boundary"),
        executed_block("loop", "duplicate block"),
    ]);
    let replay = ForgeQueryGraphObligationExecutionResultEnvelope::new(vec![
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
    assert!(ForgeQueryGraphObligationDenialProjection::from_reduction(&reduced).is_some());
}

fn executed_block(name: &str, context: &str) -> ForgeQueryGraphObligationExecutionResultRow {
    ForgeQueryGraphObligationExecutionResultRow::executed(
        input(name),
        ForgeQueryGraphObligationVerdict::block(context).unwrap(),
        ForgeQueryGraphObligationStateLoadCounters::new(1, 2, 3),
    )
}

fn executed_advise(name: &str) -> ForgeQueryGraphObligationExecutionResultRow {
    ForgeQueryGraphObligationExecutionResultRow::executed(
        input(name),
        ForgeQueryGraphObligationVerdict::advise("advisory context").unwrap(),
        ForgeQueryGraphObligationStateLoadCounters::none(),
    )
}

fn input(name: &str) -> ForgeQueryGraphObligationExecutionInput {
    ForgeQueryGraphObligationExecutionInput::from_selected_registration(
        "selection.digest",
        registration(name),
    )
}

fn registration(name: &str) -> ForgeQueryGraphObligationRegistration {
    ForgeQueryGraphObligationRegistration::new(
        ForgeQueryGraphObligationKind::BlockingInvariant,
        ForgeQueryGraphObligationRuleIdentity::new("test.execution", name, "v1").unwrap(),
        ForgeQueryGraphTouchSelector::relation_kind("topology.edge").unwrap(),
        ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    )
    .with_support_posture(ForgeQueryGraphObligationSupportPosture::supported(
        ForgeQueryGraphObligationSupportLane::GraphComposition,
    ))
}
