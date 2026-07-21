use crate::runtime::{
    WorthUiDurableStateFamilyHook, WorthUiDurableStateInventory,
    WorthUiDurableStateReplacementPolicy, WorthUiIdentityMatchNodeKind,
    WorthUiNodeLifecycleTransition, WorthUiNodeReplacementClassification,
    WorthUiNodeReplacementCounters, WorthUiNodeReplacementPlan, WorthUiStatePersistencePosture,
};

use super::durable_state_inventory_test_support::{
    custom_state_family_hook, deterministic_replacement_plan, platform_inventory,
    reversed_platform_inventory,
};
use super::identity_match_graph_test_support::{
    artifact_from_nodes, component_node, component_node_with_descriptor, identity_match_app,
    runtime_and_narrowing, splitter_surface_node, surface_node,
};
use super::node_replacement_classification_test_support::{
    lane_affecting_impact_for, lane_narrowing_for, narrowing_for, narrowing_for_identity,
    structural_impact_for, structural_impact_for_identity,
};

pub(crate) fn deterministic_reconciliation_inputs() -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    WorthUiNodeReplacementPlan,
    WorthUiDurableStateInventory,
) {
    let (runtime, plan) = deterministic_replacement_plan();
    let inventory = platform_inventory(&runtime)
        .build_for_replacement(&plan)
        .expect("platform inventory builds");
    (runtime, plan, inventory)
}

pub(super) fn reversed_inventory_for(
    runtime: &crate::runtime::WorthUiRuntime,
    plan: &WorthUiNodeReplacementPlan,
) -> WorthUiDurableStateInventory {
    reversed_platform_inventory(runtime)
        .build_for_replacement(plan)
        .expect("reversed platform inventory builds")
}

pub(super) fn structural_replacement_inputs() -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    WorthUiNodeReplacementPlan,
    WorthUiDurableStateInventory,
) {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![
            component_node("component:affected", 0),
            surface_node("surface:unaffected", "workspace.surface.main", 1),
        ],
    )]);
    let candidate = active.clone();
    let (runtime, admitted, identity_narrowing) = runtime_and_narrowing(&app, active, candidate);
    let identity_report = runtime
        .build_identity_match_graph(&identity_narrowing, &admitted)
        .expect("identity graph builds");
    let impact = structural_impact_for_identity(&identity_report, "component:affected");
    let narrowing = narrowing_for_identity(&identity_report, "component:affected");
    let plan = runtime
        .classify_node_replacements(&impact, &narrowing, &identity_report)
        .expect("structural replacement plan builds");
    let inventory = platform_inventory(&runtime)
        .build_for_replacement(&plan)
        .expect("inventory builds");
    (runtime, plan, inventory)
}

pub(super) fn drop_create_inputs() -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    WorthUiNodeReplacementPlan,
    WorthUiDurableStateInventory,
) {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![component_node_with_descriptor(
            "component:dashboard:old",
            "workspace.component.dashboard",
            0,
        )],
    )]);
    let candidate = artifact_from_nodes([(
        "app/main.wui",
        vec![component_node_with_descriptor(
            "component:dashboard:new",
            "workspace.component.dashboard",
            0,
        )],
    )]);
    let (runtime, admitted, identity_narrowing) = runtime_and_narrowing(&app, active, candidate);
    let identity_report = runtime
        .build_identity_match_graph(&identity_narrowing, &admitted)
        .expect("identity graph builds");
    let impact = structural_impact_for(&identity_report);
    let narrowing = narrowing_for(&identity_report);
    let plan = runtime
        .classify_node_replacements(&impact, &narrowing, &identity_report)
        .expect("drop/create plan builds");
    let inventory = platform_inventory(&runtime)
        .build_for_replacement(&plan)
        .expect("inventory builds");
    (runtime, plan, inventory)
}

pub(super) fn lane_change_inputs() -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    WorthUiNodeReplacementPlan,
    WorthUiDurableStateInventory,
) {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![splitter_surface_node(
            "surface:stable",
            "workspace.surface.main",
            "workspace.sizing.splitter.main",
            0,
        )],
    )]);
    let candidate = active.clone();
    let (runtime, admitted, identity_narrowing) = runtime_and_narrowing(&app, active, candidate);
    let identity_report = runtime
        .build_identity_match_graph(&identity_narrowing, &admitted)
        .expect("identity graph builds");
    let impact = lane_affecting_impact_for(&identity_report);
    let narrowing = lane_narrowing_for(&identity_report);
    let plan = runtime
        .classify_node_replacements(&impact, &narrowing, &identity_report)
        .expect("lane change plan builds");
    let inventory = platform_inventory(&runtime)
        .build_for_replacement(&plan)
        .expect("inventory builds");
    (runtime, plan, inventory)
}

pub(super) fn moved_scroll_anchor_inputs() -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    WorthUiNodeReplacementPlan,
    WorthUiDurableStateInventory,
) {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![surface_node("surface:stable", "workspace.surface.main", 0)],
    )]);
    let candidate = artifact_from_nodes([(
        "app/panels.wui",
        vec![surface_node("surface:stable", "workspace.surface.main", 0)],
    )]);
    let (runtime, admitted, identity_narrowing) = runtime_and_narrowing(&app, active, candidate);
    let identity_report = runtime
        .build_identity_match_graph(&identity_narrowing, &admitted)
        .expect("identity graph builds");
    let impact = structural_impact_for(&identity_report);
    let narrowing = narrowing_for(&identity_report);
    let plan = runtime
        .classify_node_replacements(&impact, &narrowing, &identity_report)
        .expect("moved node plan builds");
    let inventory = platform_inventory(&runtime)
        .build_for_replacement(&plan)
        .expect("inventory builds");
    (runtime, plan, inventory)
}

pub(super) fn rebind_plan_with_inventory() -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    WorthUiNodeReplacementPlan,
    WorthUiDurableStateInventory,
) {
    let (runtime, base_plan) = deterministic_replacement_plan();
    let plan = plan_from_single_transition(
        base_plan.active_artifact_digest(),
        base_plan.candidate_artifact_digest(),
        "binding:query-results",
        WorthUiNodeLifecycleTransition::Rebind,
        Some(WorthUiIdentityMatchNodeKind::Binding),
        Some(WorthUiIdentityMatchNodeKind::Binding),
    );
    let inventory = platform_inventory(&runtime)
        .build_for_replacement(&plan)
        .expect("inventory builds");
    (runtime, plan, inventory)
}

pub(crate) fn splitter_replace_inputs() -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    WorthUiNodeReplacementPlan,
    WorthUiDurableStateInventory,
) {
    let (runtime, base_plan) = deterministic_replacement_plan();
    let plan = plan_from_single_transition(
        base_plan.active_artifact_digest(),
        base_plan.candidate_artifact_digest(),
        "surface:main",
        WorthUiNodeLifecycleTransition::Replace,
        Some(WorthUiIdentityMatchNodeKind::Surface),
        Some(WorthUiIdentityMatchNodeKind::Surface),
    );
    let inventory = platform_inventory(&runtime)
        .build_for_replacement(&plan)
        .expect("inventory builds");
    (runtime, plan, inventory)
}

pub(super) fn ambiguous_plan_with_inventory() -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    WorthUiNodeReplacementPlan,
    WorthUiDurableStateInventory,
) {
    let (runtime, base_plan) = deterministic_replacement_plan();
    let mut counters = WorthUiNodeReplacementCounters::default();
    counters.record_active_node_classified();
    counters.record_candidate_node_classified();
    counters.record_transition(WorthUiNodeLifecycleTransition::Preserve);
    counters.record_ambiguous_node();
    let plan = WorthUiNodeReplacementPlan::new(
        base_plan.active_artifact_digest(),
        base_plan.candidate_artifact_digest(),
        vec![WorthUiNodeReplacementClassification::new(
            crate::runtime::replacement::node_classification::WorthUiNodeReplacementClassificationInput {
                identity_basis: "component:ambiguous".to_owned(),
                authored_provenance_digest: None,
                transition: WorthUiNodeLifecycleTransition::Preserve,
                active_kind: Some(WorthUiIdentityMatchNodeKind::Component),
                candidate_kind: Some(WorthUiIdentityMatchNodeKind::Component),
                active_durable_state_eligible: true,
                candidate_durable_state_eligible: true,
                active_resize_contract_id: None,
                candidate_resize_contract_id: None,
                active_resize_permission: None,
                candidate_resize_permission: None,
                active_resize_shape_digest: None,
                candidate_resize_shape_digest: None,
            },
        )],
        counters,
    );
    let inventory = platform_inventory(&runtime)
        .build_for_replacement(&base_plan)
        .expect("inventory builds from unambiguous base plan");
    (runtime, plan, inventory)
}

pub(super) fn custom_lane_change_inventory(
    runtime: &crate::runtime::WorthUiRuntime,
    plan: &WorthUiNodeReplacementPlan,
) -> WorthUiDurableStateInventory {
    custom_inventory_for_policy(
        runtime,
        plan,
        WorthUiDurableStateReplacementPolicy::ReconcileOnLaneChange,
    )
}

pub(super) fn custom_inventory_for_policy(
    runtime: &crate::runtime::WorthUiRuntime,
    plan: &WorthUiNodeReplacementPlan,
    replacement_policy: WorthUiDurableStateReplacementPolicy,
) -> WorthUiDurableStateInventory {
    platform_inventory(runtime)
        .register_family_hook(custom_reconcile_hook(replacement_policy))
        .build_for_replacement(plan)
        .expect("custom inventory builds")
}

pub(super) fn stale_inventory_for(
    inventory: &WorthUiDurableStateInventory,
) -> WorthUiDurableStateInventory {
    WorthUiDurableStateInventory::new(
        inventory.active_artifact_digest(),
        inventory.candidate_artifact_digest() + 1,
        inventory.families().to_vec(),
        Vec::new(),
        inventory.counters(),
    )
}

pub(super) fn stale_active_inventory_for(
    inventory: &WorthUiDurableStateInventory,
) -> WorthUiDurableStateInventory {
    WorthUiDurableStateInventory::new(
        inventory.active_artifact_digest() + 1,
        inventory.candidate_artifact_digest(),
        inventory.families().to_vec(),
        Vec::new(),
        inventory.counters(),
    )
}

pub(super) fn inventory_missing_scroll_family(
    inventory: &WorthUiDurableStateInventory,
) -> WorthUiDurableStateInventory {
    WorthUiDurableStateInventory::new(
        inventory.active_artifact_digest(),
        inventory.candidate_artifact_digest(),
        inventory
            .families()
            .iter()
            .filter(|family| {
                family.id() != &crate::runtime::WorthUiDurableStateFamilyId::ScrollAnchor
            })
            .cloned()
            .collect(),
        Vec::new(),
        inventory.counters(),
    )
}

fn plan_from_single_transition(
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    identity_basis: &str,
    transition: WorthUiNodeLifecycleTransition,
    active_kind: Option<WorthUiIdentityMatchNodeKind>,
    candidate_kind: Option<WorthUiIdentityMatchNodeKind>,
) -> WorthUiNodeReplacementPlan {
    let mut counters = WorthUiNodeReplacementCounters::default();
    counters.record_active_node_classified();
    counters.record_candidate_node_classified();
    counters.record_transition(transition);
    WorthUiNodeReplacementPlan::new(
        active_artifact_digest,
        candidate_artifact_digest,
        vec![WorthUiNodeReplacementClassification::new(
            crate::runtime::replacement::node_classification::WorthUiNodeReplacementClassificationInput {
                identity_basis: identity_basis.to_owned(),
                authored_provenance_digest: None,
                transition,
                active_kind,
                candidate_kind,
                active_durable_state_eligible: true,
                candidate_durable_state_eligible: true,
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

fn custom_reconcile_hook(
    replacement_policy: WorthUiDurableStateReplacementPolicy,
) -> WorthUiDurableStateFamilyHook {
    custom_state_family_hook(
        "workspace.custom.reconcile-cache",
        "workspace.custom.reconcile-cache",
        replacement_policy,
        WorthUiStatePersistencePosture::RuntimeOnly,
    )
}
