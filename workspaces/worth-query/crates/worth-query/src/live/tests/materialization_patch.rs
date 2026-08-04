use crate::live::*;
#[test]
fn bounded_materialization_live_outcome_emits_scope_patch() {
    let preflight = crate::harness::fixtures::execution_preflights::ordered_collection_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");
    let change = BridgeChangeSummary::default()
        .with_relation_delta(BridgeRelationDelta::new("manager"))
        .with_materialization_scope_transition(false, true)
        .with_field_delta(BridgeFieldDelta::new(
            "profile",
            "display_name",
            Some("Old Manager"),
            Some("New Manager"),
        ));

    let outcome = live
        .bounded_materialization_live_outcome(&change)
        .expect("scope transition should produce a bounded materialization patch");

    match outcome {
        BoundedMaterializationLiveOutcome::Patch(patch) => {
            match patch.kind() {
                BoundedMaterializationPatchKind::Scope(scope) => {
                    assert_eq!(scope, &MaterializationScopeChange::EnteredScope);
                }
                other => panic!("expected scope patch, got {other:?}"),
            }
            assert_eq!(patch.relation_deltas(), &["manager".to_string()]);
            assert_eq!(patch.projected_field_deltas().len(), 1);
        }
        BoundedMaterializationLiveOutcome::Refresh(fallback) => {
            panic!("expected patch, got refresh fallback: {fallback:?}");
        }
        BoundedMaterializationLiveOutcome::Suppressed(reason) => {
            panic!("expected patch, got suppression: {reason:?}");
        }
    }
}
