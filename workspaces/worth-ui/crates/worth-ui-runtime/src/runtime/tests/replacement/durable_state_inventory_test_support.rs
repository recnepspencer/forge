use crate::runtime::{
    WorthUiDurableStateFamily, WorthUiDurableStateFamilyHook, WorthUiDurableStateFamilyId,
    WorthUiDurableStateReplacementPolicy, WorthUiStateOwnerIdentity,
    WorthUiStatePersistencePosture,
};

use super::identity_match_graph_test_support::{
    artifact_from_nodes, component_node, identity_match_app, runtime_and_narrowing,
    splitter_surface_node,
};
use super::node_replacement_classification_test_support::{narrowing_for, no_op_impact_for};

pub(super) fn deterministic_replacement_plan() -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    crate::runtime::WorthUiNodeReplacementPlan,
) {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![
            component_node("component:dashboard", 0),
            splitter_surface_node(
                "surface:main",
                "workspace.surface.main",
                "workspace.sizing.splitter.main",
                1,
            ),
        ],
    )]);
    let candidate = active.clone();

    let (runtime, admitted, identity_narrowing) = runtime_and_narrowing(&app, active, candidate);
    let identity_report = runtime
        .build_identity_match_graph(&identity_narrowing, &admitted)
        .expect("identity graph builds");
    let impact = no_op_impact_for(&identity_report);
    let narrowing = narrowing_for(&identity_report);
    let plan = runtime
        .classify_node_replacements(&impact, &narrowing, &identity_report)
        .expect("replacement plan builds");
    (runtime, plan)
}

pub(super) fn platform_inventory(
    runtime: &crate::runtime::WorthUiRuntime,
) -> crate::runtime::WorthUiDurableStateInventoryBuilder {
    runtime
        .durable_state_inventory()
        .register_platform_family(WorthUiDurableStateFamily::focus_chain())
        .register_platform_family(WorthUiDurableStateFamily::scroll_anchor())
        .register_platform_family(WorthUiDurableStateFamily::selection_range())
        .register_platform_family(WorthUiDurableStateFamily::text_edit_buffer())
        .register_platform_family(WorthUiDurableStateFamily::splitter_position())
        .register_platform_family(WorthUiDurableStateFamily::tab_state())
        .register_platform_family(WorthUiDurableStateFamily::panel_visibility())
}

pub(super) fn reversed_platform_inventory(
    runtime: &crate::runtime::WorthUiRuntime,
) -> crate::runtime::WorthUiDurableStateInventoryBuilder {
    runtime
        .durable_state_inventory()
        .register_platform_family(WorthUiDurableStateFamily::panel_visibility())
        .register_platform_family(WorthUiDurableStateFamily::tab_state())
        .register_platform_family(WorthUiDurableStateFamily::splitter_position())
        .register_platform_family(WorthUiDurableStateFamily::text_edit_buffer())
        .register_platform_family(WorthUiDurableStateFamily::selection_range())
        .register_platform_family(WorthUiDurableStateFamily::scroll_anchor())
        .register_platform_family(WorthUiDurableStateFamily::focus_chain())
}

pub(super) fn ownerless_hook(family_id: &str) -> WorthUiDurableStateFamilyHook {
    WorthUiDurableStateFamilyHook::custom(WorthUiDurableStateFamilyId::custom(family_id))
        .with_replacement_policy(WorthUiDurableStateReplacementPolicy::ReconcileOnLaneChange)
        .with_persistence_posture(WorthUiStatePersistencePosture::SessionRecorded)
}

pub(super) fn domain_truth_hook() -> WorthUiDurableStateFamilyHook {
    WorthUiDurableStateFamilyHook::custom(WorthUiDurableStateFamilyId::custom(
        "workspace.domain.document-status",
    ))
    .with_owner_identity(WorthUiStateOwnerIdentity::domain_truth(
        "workspace.domain.document",
    ))
    .with_replacement_policy(WorthUiDurableStateReplacementPolicy::PreserveWhenNodeCarriesState)
    .with_persistence_posture(WorthUiStatePersistencePosture::WorkspaceRecordedForLater)
}

pub(super) fn policyless_inspector_tabs_hook() -> WorthUiDurableStateFamilyHook {
    WorthUiDurableStateFamilyHook::custom(WorthUiDurableStateFamilyId::custom(
        "workspace.custom.inspector-tabs",
    ))
    .with_owner_identity(WorthUiStateOwnerIdentity::custom_hook(
        "workspace.inspector.tabs",
    ))
    .with_persistence_posture(WorthUiStatePersistencePosture::SessionRecorded)
}

pub(super) fn duplicate_panel_cache_hook(owner_identity: &str) -> WorthUiDurableStateFamilyHook {
    WorthUiDurableStateFamilyHook::custom(WorthUiDurableStateFamilyId::custom(
        "workspace.custom.panel-cache",
    ))
    .with_owner_identity(WorthUiStateOwnerIdentity::custom_hook(owner_identity))
    .with_replacement_policy(WorthUiDurableStateReplacementPolicy::PreserveWhenNodeCarriesState)
    .with_persistence_posture(WorthUiStatePersistencePosture::RuntimeOnly)
}

pub(super) fn reserved_focus_hook() -> WorthUiDurableStateFamilyHook {
    WorthUiDurableStateFamilyHook::custom(WorthUiDurableStateFamilyId::FocusChain)
        .with_owner_identity(WorthUiStateOwnerIdentity::custom_hook(
            "workspace.custom.focus",
        ))
        .with_replacement_policy(WorthUiDurableStateReplacementPolicy::PreserveWhenNodeCarriesState)
        .with_persistence_posture(WorthUiStatePersistencePosture::RuntimeOnly)
}

pub(super) fn inspector_tabs_hook() -> WorthUiDurableStateFamilyHook {
    custom_state_family_hook(
        "workspace.custom.inspector-tabs",
        "workspace.inspector.tabs",
        WorthUiDurableStateReplacementPolicy::ReconcileOnLaneChange,
        WorthUiStatePersistencePosture::SessionRecorded,
    )
}

pub(super) fn custom_state_family_hook(
    family_id: &str,
    owner_identity: &str,
    replacement_policy: WorthUiDurableStateReplacementPolicy,
    persistence_posture: WorthUiStatePersistencePosture,
) -> WorthUiDurableStateFamilyHook {
    WorthUiDurableStateFamilyHook::custom(WorthUiDurableStateFamilyId::custom(family_id))
        .with_owner_identity(WorthUiStateOwnerIdentity::custom_hook(owner_identity))
        .with_replacement_policy(replacement_policy)
        .with_persistence_posture(persistence_posture)
}
