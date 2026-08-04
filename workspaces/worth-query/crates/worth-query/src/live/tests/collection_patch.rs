use super::field_key;
use crate::live::*;
#[test]
fn ordered_collection_live_outcome_emits_reorder_patch() {
    let preflight = crate::harness::fixtures::execution_preflights::
            ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");
    let change = BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
        "profile",
        "display_name",
        Some("Avery"),
        Some("Zoey"),
    ));

    let outcome = live
        .ordered_collection_live_outcome(&change)
        .expect("ordering-key change should produce a collection patch");

    match outcome {
        OrderedCollectionLiveOutcome::Patch(patch) => {
            match patch.kind() {
                OrderedCollectionPatchKind::Reordered(ordering) => {
                    assert_eq!(ordering.ordering_field_deltas().len(), 1);
                    assert_eq!(
                        ordering.ordering_field_deltas()[0]
                            .field()
                            .native_field_key(),
                        &field_key("display_name")
                    );
                }
                other => panic!("expected reorder patch, got {other:?}"),
            }
            assert!(!patch.digest().as_str().is_empty());
        }
        OrderedCollectionLiveOutcome::Refresh(fallback) => {
            panic!("expected patch, got refresh fallback: {fallback:?}");
        }
        OrderedCollectionLiveOutcome::Suppressed(reason) => {
            panic!("expected patch, got suppression: {reason:?}");
        }
    }
}

#[test]
fn ordered_collection_live_outcome_suppresses_irrelevant_relation_change() {
    let preflight = crate::harness::fixtures::execution_preflights::
            ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");
    let change =
        BridgeChangeSummary::default().with_relation_delta(BridgeRelationDelta::new("manager"));

    let outcome = live
        .ordered_collection_live_outcome(&change)
        .expect("relation-only change should suppress");

    assert_eq!(
        outcome.suppression_decision(),
        SuppressionDecision::Suppress(SuppressionReason::IrrelevantChange(
            IrrelevantChangeClass::NoProjectedFieldOverlap
        ))
    );
}

#[test]
fn ordered_collection_live_outcome_suppresses_noop_membership_transition() {
    let preflight = crate::harness::fixtures::execution_preflights::
            ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");
    let change = BridgeChangeSummary::default().with_membership_transition(true, true);

    let outcome = live
        .ordered_collection_live_outcome(&change)
        .expect("no-op membership transition should suppress");

    assert_eq!(
        outcome.suppression_decision(),
        SuppressionDecision::Suppress(SuppressionReason::IrrelevantChange(
            IrrelevantChangeClass::NoProjectedFieldOverlap
        ))
    );
}

#[test]
fn noop_membership_transition_is_irrelevant_before_patch_construction() {
    let preflight = crate::harness::fixtures::execution_preflights::
            ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");
    let change = BridgeChangeSummary::default().with_membership_transition(true, true);

    let relevance = live.classify_change(&change);

    assert_eq!(
        relevance,
        ChangeRelevance::Irrelevant(IrrelevantChangeClass::NoProjectedFieldOverlap)
    );
}
