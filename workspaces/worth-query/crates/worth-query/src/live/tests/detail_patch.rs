use super::{aspect_key, field_key};
use crate::live::*;
#[test]
fn detail_live_outcome_emits_patch_for_projected_field_change() {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let change = BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
        "identity",
        "id",
        Some("user-1"),
        Some("user-2"),
    ));

    let outcome = live
        .detail_live_outcome(&change)
        .expect("projected detail change should produce a patch");

    match outcome {
        DetailLiveOutcome::Patch(patch) => {
            assert_eq!(patch.field_deltas().len(), 1);
            let delta = &patch.field_deltas()[0];
            assert_eq!(delta.field().native_aspect_key(), &aspect_key("identity"));
            assert_eq!(delta.field().native_field_key(), &field_key("id"));
            assert_eq!(delta.old_value(), Some("user-1"));
            assert_eq!(delta.new_value(), Some("user-2"));
            assert!(!patch.digest().as_str().is_empty());
        }
        DetailLiveOutcome::Refresh(fallback) => {
            panic!("expected patch, got refresh fallback: {fallback:?}");
        }
        DetailLiveOutcome::Suppressed(reason) => {
            panic!("expected patch, got suppression: {reason:?}");
        }
    }
}

#[test]
fn detail_live_outcome_suppresses_irrelevant_change() {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let change = BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
        "profile",
        "display_name",
        Some("Esther"),
        Some("Ess"),
    ));

    let outcome = live
        .detail_live_outcome(&change)
        .expect("irrelevant detail change should suppress");

    assert_eq!(
        outcome.suppression_decision(),
        SuppressionDecision::Suppress(SuppressionReason::IrrelevantChange(
            IrrelevantChangeClass::NoProjectedFieldOverlap
        ))
    );
}
