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
    WorthUiNodeReplacementPlan, WorthUiQueryLiveRebindPlan, WorthUiRuntime,
    WorthUiRuntimeImpactNarrowing,
};

pub(super) fn ui_local_drift_rebind_plan() -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    WorthUiQueryLiveRebindPlan,
) {
    let active_app = standard_query_app();
    let candidate_app = denial_presentation_drift_query_app();
    let active = query_artifact(&active_app, "workspace.view_binding.selection");
    let candidate = query_artifact(&candidate_app, "workspace.view_binding.selection");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&active_app, active, candidate);
    let rebind_plan = query_rebind_plan(&runtime, &plan, &narrowing, &admitted);
    (runtime, rebind_plan)
}

pub(super) fn single_active_state_lifecycle_inputs() -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
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
            crate::runtime::replacement::node_classification::WorthUiNodeReplacementClassificationInput {
                identity_basis: "binding:ambiguous-query-state".to_owned(),
                authored_provenance_digest: None,
                transition: WorthUiNodeLifecycleTransition::Preserve,
                active_kind: Some(WorthUiIdentityMatchNodeKind::Binding),
                candidate_kind: Some(WorthUiIdentityMatchNodeKind::Binding),
                active_durable_state_eligible: true,
                candidate_durable_state_eligible: true,
                active_has_restorable_splitter_state: false,
                candidate_has_restorable_splitter_state: false,
                active_resize_contract_id: None,
                candidate_resize_contract_id: None,
                active_resize_permission: None,
                candidate_resize_permission: None,
                active_resize_shape_digest: None,
                candidate_resize_shape_digest: None,
            },
        )],
        counters,
    )
}

fn query_rebind_plan(
    runtime: &WorthUiRuntime,
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
