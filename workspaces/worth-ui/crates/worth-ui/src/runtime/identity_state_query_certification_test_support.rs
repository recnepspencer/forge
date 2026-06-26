use super::query_binding_comparison_test_support::{
    denial_presentation_drift_query_app, phase11_pipeline, query_artifact, standard_query_app,
};
use super::{
    durable_state_inventory_test_support::platform_inventory,
    identity_match_graph_test_support::{
        artifact_from_nodes, component_node, identity_match_app, runtime_and_narrowing,
    },
    node_replacement_classification_test_support::{
        narrowing_for_identity, structural_impact_for_identity,
    },
};
use crate::runtime::{
    WorthUiDurableStateInventory, WorthUiIdentityMatchNodeKind, WorthUiNodeLifecycleTransition,
    WorthUiNodeReplacementClassification, WorthUiNodeReplacementCounters,
    WorthUiNodeReplacementPlan, WorthUiQueryLiveRebindPlan, WorthUiRuntimeHost,
    WorthUiRuntimeImpactNarrowing,
};

pub(super) fn ui_local_drift_rebind_plan() -> (WorthUiRuntimeHost, WorthUiQueryLiveRebindPlan) {
    let active_app = standard_query_app();
    let candidate_app = denial_presentation_drift_query_app();
    let active = query_artifact(&active_app, "workspace.view_binding.selection");
    let candidate = query_artifact(&candidate_app, "workspace.view_binding.selection");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&active_app, active, candidate);
    let rebind_plan = query_rebind_plan(&runtime, &plan, &narrowing, &admitted);
    (runtime, rebind_plan)
}

pub(super) fn preserved_query_rebind_plan() -> (WorthUiRuntimeHost, WorthUiQueryLiveRebindPlan) {
    let app = standard_query_app();
    let active = query_artifact(&app, "workspace.view_binding.selection");
    let candidate = query_artifact(&app, "workspace.view_binding.selection");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&app, active, candidate);
    let rebind_plan = query_rebind_plan(&runtime, &plan, &narrowing, &admitted);
    (runtime, rebind_plan)
}

pub(super) fn single_active_state_lifecycle_inputs() -> (
    WorthUiRuntimeHost,
    WorthUiNodeReplacementPlan,
    WorthUiDurableStateInventory,
) {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![
            component_node("component:affected", 0),
            component_node("component:dropped", 1),
        ],
    )]);
    let candidate = artifact_from_nodes([(
        "app/main.wui",
        vec![
            component_node("component:affected", 0),
            component_node("component:created", 1),
        ],
    )]);
    let (runtime, admitted, identity_narrowing) = runtime_and_narrowing(&app, active, candidate);
    let identity_report = runtime
        .build_identity_match_graph(&identity_narrowing, &admitted)
        .expect("identity graph builds");
    let impact = structural_impact_for_identity(&identity_report, "component:affected");
    let narrowing = narrowing_for_identity(&identity_report, "component:affected");
    let plan = runtime
        .classify_node_replacements(&impact, &narrowing, &identity_report)
        .expect("mixed lifecycle plan builds");
    let inventory = platform_inventory(&runtime)
        .build_for_replacement(&plan)
        .expect("inventory builds");
    (runtime, plan, inventory)
}

pub(super) fn query_runtime_state_and_rebind_inputs() -> (
    WorthUiRuntimeHost,
    WorthUiNodeReplacementPlan,
    WorthUiDurableStateInventory,
    WorthUiQueryLiveRebindPlan,
) {
    let active_app = standard_query_app();
    let candidate_app = denial_presentation_drift_query_app();
    let active = query_artifact(&active_app, "workspace.view_binding.selection");
    let candidate = query_artifact(&candidate_app, "workspace.view_binding.selection");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&active_app, active, candidate);
    let inventory = platform_inventory(&runtime)
        .build_for_replacement(&plan)
        .expect("query runtime inventory builds");
    let rebind_plan = query_rebind_plan(&runtime, &plan, &narrowing, &admitted);
    (runtime, plan, inventory, rebind_plan)
}

pub(super) fn ambiguous_plan_for_same_active(
    base_plan: &WorthUiNodeReplacementPlan,
) -> WorthUiNodeReplacementPlan {
    let mut counters = WorthUiNodeReplacementCounters::default();
    counters.record_active_node_classified();
    counters.record_candidate_node_classified();
    counters.record_transition(WorthUiNodeLifecycleTransition::Preserve);
    counters.record_ambiguous_node();
    WorthUiNodeReplacementPlan::new(
        base_plan.active_artifact_digest(),
        base_plan.candidate_artifact_digest(),
        vec![WorthUiNodeReplacementClassification::new(
            "binding:ambiguous-query-state".to_owned(),
            WorthUiNodeLifecycleTransition::Preserve,
            Some(WorthUiIdentityMatchNodeKind::Binding),
            Some(WorthUiIdentityMatchNodeKind::Binding),
            true,
            true,
        )],
        counters,
    )
}

fn query_rebind_plan(
    runtime: &WorthUiRuntimeHost,
    plan: &WorthUiNodeReplacementPlan,
    narrowing: &WorthUiRuntimeImpactNarrowing,
    admitted: &crate::runtime::WorthUiAdmittedReplacementCandidate,
) -> WorthUiQueryLiveRebindPlan {
    let comparison = runtime
        .compare_query_bindings(plan, narrowing, admitted)
        .expect("comparison succeeds");
    runtime
        .plan_query_live_rebinds(&comparison, plan, narrowing, admitted)
        .expect("query rebind planning succeeds")
}
