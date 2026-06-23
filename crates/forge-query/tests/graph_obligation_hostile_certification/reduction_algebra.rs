use forge_query::facade::runtime::{
    ForgeQueryGraphObligationDenialProjection, ForgeQueryGraphObligationExecutionResultEnvelope,
    ForgeQueryGraphObligationExecutionResultRow, ForgeQueryGraphObligationIndex,
    ForgeQueryGraphObligationMaterializedDispatch, ForgeQueryGraphObligationRegistrationCatalog,
    ForgeQueryGraphObligationStateLoadCounters, ForgeQueryGraphObligationVerdict,
};

use super::support::{committed_world, execution_input, graph_mutation_touch, registrations};
use forge_query::facade::runtime::{
    ForgeQueryGraphObligationKind, ForgeQueryGraphObligationOperatingWorldSelector,
    ForgeQueryGraphObligationRegistration, ForgeQueryGraphObligationRuleIdentity,
    ForgeQueryGraphObligationSupportLane, ForgeQueryGraphObligationSupportPosture,
    ForgeQueryGraphTouchSelector,
};

#[test]
fn reduction_digest_is_stable_for_equivalent_reordered_observations() {
    let first = ForgeQueryGraphObligationExecutionResultEnvelope::new(vec![
        block_row("blocking-rule", "first block"),
        advise_row("advisory-rule"),
        block_row("blocking-rule", "duplicate block"),
    ]);
    let replay = ForgeQueryGraphObligationExecutionResultEnvelope::new(vec![
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
    assert!(ForgeQueryGraphObligationDenialProjection::from_reduction(&reduced).is_some());
}

#[test]
fn reduction_digest_changes_when_verdict_semantics_change() {
    let blocking = ForgeQueryGraphObligationExecutionResultEnvelope::new(vec![block_row(
        "semantic-rule",
        "blocking context",
    )]);
    let advisory =
        ForgeQueryGraphObligationExecutionResultEnvelope::new(vec![advise_row("semantic-rule")]);

    assert_ne!(
        blocking.reduce().reduction_digest(),
        advisory.reduce().reduction_digest()
    );
}

#[test]
fn reduction_accepts_real_materialized_dispatch_rows() {
    let catalog = ForgeQueryGraphObligationRegistrationCatalog::from_registrations(registrations())
        .expect("registration catalog");
    let selection = ForgeQueryGraphObligationIndex::from_catalog(&catalog)
        .select_for_touch(&graph_mutation_touch(), &committed_world());
    let envelope = ForgeQueryGraphObligationMaterializedDispatch::from_selection(selection)
        .selected_result_envelope();

    let reduced = envelope.reduce();
    let projection = ForgeQueryGraphObligationDenialProjection::from_reduction(&reduced)
        .expect("materialized dispatch should expose blocking rows");

    assert_eq!(
        envelope.rows().len(),
        ForgeQueryGraphObligationKind::ALL.len()
    );
    assert_eq!(reduced.blocking_count(), 2);
    assert_eq!(reduced.advisory_count(), 1);
    assert_eq!(reduced.allow_count(), 3);
    assert!(reduced.blocks_if_required());
    assert_eq!(projection.blocking_count(), reduced.blocking_count());
}

fn block_row(rule_name: &str, context: &str) -> ForgeQueryGraphObligationExecutionResultRow {
    ForgeQueryGraphObligationExecutionResultRow::executed(
        execution_input(registration(rule_name)),
        ForgeQueryGraphObligationVerdict::block(context).unwrap(),
        ForgeQueryGraphObligationStateLoadCounters::new(1, 2, 3),
    )
}

fn advise_row(rule_name: &str) -> ForgeQueryGraphObligationExecutionResultRow {
    ForgeQueryGraphObligationExecutionResultRow::executed(
        execution_input(registration(rule_name)),
        ForgeQueryGraphObligationVerdict::advise("advisory context").unwrap(),
        ForgeQueryGraphObligationStateLoadCounters::none(),
    )
}

fn registration(rule_name: &str) -> ForgeQueryGraphObligationRegistration {
    ForgeQueryGraphObligationRegistration::new(
        ForgeQueryGraphObligationKind::BlockingInvariant,
        ForgeQueryGraphObligationRuleIdentity::new("phase-20.reduction", rule_name, "v1").unwrap(),
        ForgeQueryGraphTouchSelector::collection(rule_name).unwrap(),
        ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    )
    .with_support_posture(ForgeQueryGraphObligationSupportPosture::supported(
        ForgeQueryGraphObligationSupportLane::GraphComposition,
    ))
}
