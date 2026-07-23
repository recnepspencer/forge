use crate::runtime::tests::query_binding_comparison_test_support::{
    query_artifact_with_bindings, standard_query_app,
};
use crate::runtime::tests::replacement_impact_test_support::{admitted_candidate, launch_runtime};
use crate::runtime::{WorthUiNodeLifecycleTransition, WorthUiReplacementImpact};

#[test]
fn query_row_addition_and_removal_use_canonical_structural_transitions() {
    let app = standard_query_app();
    let first = "workspace.view_binding.selection";
    let second = "workspace.view_binding.detail";
    let runtime = launch_runtime(&app, query_artifact_with_bindings(&app, &[first]));
    let addition = admitted_candidate(
        &app,
        &runtime,
        query_artifact_with_bindings(&app, &[first, second]),
    );
    let addition_comparison = runtime
        .compare_admitted_replacement(&addition)
        .expect("Query addition compares");
    let addition_impact = runtime
        .classify_replacement_impact(&addition_comparison, &addition)
        .expect("Query-only node count change is structurally explained");
    assert!(matches!(
        addition_impact.impact(),
        WorthUiReplacementImpact::StructuralReplacement(scope) if scope.is_structural()
    ));
    let narrowing = runtime
        .narrow_replacement_impact(&addition_impact, &addition)
        .expect("Query addition narrows");
    let identity = runtime
        .build_identity_match_graph(&narrowing, &addition)
        .expect("Query identities match canonically");
    let addition_plan = runtime
        .classify_node_replacements(&addition_impact, &narrowing, &identity)
        .expect("Query addition classifies");
    assert_eq!(
        addition_plan.transition_for_identity(second),
        Some(WorthUiNodeLifecycleTransition::Create)
    );

    let runtime = launch_runtime(&app, query_artifact_with_bindings(&app, &[first, second]));
    let removal = admitted_candidate(
        &app,
        &runtime,
        query_artifact_with_bindings(&app, &[second]),
    );
    let comparison = runtime
        .compare_admitted_replacement(&removal)
        .expect("Query removal compares");
    let impact = runtime
        .classify_replacement_impact(&comparison, &removal)
        .expect("Query-only removal is structurally explained");
    let narrowing = runtime
        .narrow_replacement_impact(&impact, &removal)
        .expect("Query removal narrows");
    let identity = runtime
        .build_identity_match_graph(&narrowing, &removal)
        .expect("Query removal identities match canonically");
    let removal_plan = runtime
        .classify_node_replacements(&impact, &narrowing, &identity)
        .expect("Query removal classifies");
    assert_eq!(
        removal_plan.transition_for_identity(first),
        Some(WorthUiNodeLifecycleTransition::Drop)
    );
}
