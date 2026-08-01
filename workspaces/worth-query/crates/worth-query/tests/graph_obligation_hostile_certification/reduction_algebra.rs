use worth_query::facade::runtime::{
    WorthQueryGraphObligationDenialProjection, WorthQueryGraphObligationExecutionResultEnvelope,
    WorthQueryGraphObligationExecutionResultRow, WorthQueryGraphObligationIndex,
    WorthQueryGraphObligationMaterializedDispatch, WorthQueryGraphObligationRegistrationCatalog,
    WorthQueryGraphObligationStateLoadCounters, WorthQueryGraphObligationVerdict,
};

use super::support::{committed_world, execution_input, graph_mutation_touch, registrations};
use worth_query::facade::runtime::{
    WorthQueryGraphObligationKind, WorthQueryGraphObligationOperatingWorldSelector,
    WorthQueryGraphObligationRegistration, WorthQueryGraphObligationRuleIdentity,
    WorthQueryGraphObligationSupportLane, WorthQueryGraphObligationSupportPosture,
    WorthQueryGraphTouchSelector,
};

#[test]
fn reduction_digest_is_stable_for_equivalent_reordered_observations() {
    let first = WorthQueryGraphObligationExecutionResultEnvelope::new(vec![
        block_row("blocking-rule", "first block"),
        advise_row("advisory-rule"),
        block_row("blocking-rule", "duplicate block"),
    ]);
    let replay = WorthQueryGraphObligationExecutionResultEnvelope::new(vec![
        advise_row("advisory-rule"),
        block_row("blocking-rule", "duplicate block"),
        block_row("blocking-rule", "first block"),
    ]);

    let reduced = first.reduce();
    let replay_reduced = replay.reduce();

    assert_eq!(
        reduced.reduction_digest(),
        replay_reduced.reduction_digest()
    );
    assert_eq!(reduced.blocking_count(), 2);
    assert_eq!(reduced.advisory_count(), 1);
    assert_eq!(reduced.allow_count(), 0);
    assert_eq!(reduced.duplicate_rule_observation_count(), 1);
    assert!(reduced.blocks_if_required());
    assert!(WorthQueryGraphObligationDenialProjection::from_reduction(&reduced).is_some());
}

#[test]
fn reduction_digest_changes_when_verdict_semantics_change() {
    let blocking = WorthQueryGraphObligationExecutionResultEnvelope::new(vec![block_row(
        "semantic-rule",
        "blocking context",
    )]);
    let advisory =
        WorthQueryGraphObligationExecutionResultEnvelope::new(vec![advise_row("semantic-rule")]);

    assert_ne!(
        blocking.reduce().reduction_digest(),
        advisory.reduce().reduction_digest()
    );
}

#[test]
fn reduction_accepts_real_materialized_dispatch_rows() {
    let catalog = WorthQueryGraphObligationRegistrationCatalog::from_registrations(registrations())
        .expect("registration catalog");
    let selection = WorthQueryGraphObligationIndex::from_catalog(&catalog)
        .select_for_touch(&graph_mutation_touch(), &committed_world());
    let envelope = WorthQueryGraphObligationMaterializedDispatch::from_selection(selection)
        .selected_result_envelope();

    let reduced = envelope.reduce();
    let projection = WorthQueryGraphObligationDenialProjection::from_reduction(&reduced)
        .expect("materialized dispatch should expose blocking rows");

    assert_eq!(
        envelope.rows().len(),
        WorthQueryGraphObligationKind::ALL.len()
    );
    assert_eq!(reduced.blocking_count(), 5);
    assert_eq!(reduced.advisory_count(), 1);
    assert_eq!(reduced.allow_count(), 0);
    assert!(reduced.blocks_if_required());
    assert_eq!(projection.blocking_count(), reduced.blocking_count());
}

fn block_row(rule_name: &str, context: &str) -> WorthQueryGraphObligationExecutionResultRow {
    WorthQueryGraphObligationExecutionResultRow::executed(
        execution_input(registration(rule_name)),
        WorthQueryGraphObligationVerdict::block(context).unwrap(),
        WorthQueryGraphObligationStateLoadCounters::new(1, 2, 3),
    )
}

fn advise_row(rule_name: &str) -> WorthQueryGraphObligationExecutionResultRow {
    WorthQueryGraphObligationExecutionResultRow::executed(
        execution_input(registration(rule_name)),
        WorthQueryGraphObligationVerdict::advise("advisory context").unwrap(),
        WorthQueryGraphObligationStateLoadCounters::none(),
    )
}

fn registration(rule_name: &str) -> WorthQueryGraphObligationRegistration {
    WorthQueryGraphObligationRegistration::new(
        WorthQueryGraphObligationKind::BlockingInvariant,
        WorthQueryGraphObligationRuleIdentity::new("phase-20.reduction", rule_name, "v1").unwrap(),
        WorthQueryGraphTouchSelector::collection(rule_name).unwrap(),
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    )
    .with_support_posture(WorthQueryGraphObligationSupportPosture::supported(
        WorthQueryGraphObligationSupportLane::GraphComposition,
    ))
}
