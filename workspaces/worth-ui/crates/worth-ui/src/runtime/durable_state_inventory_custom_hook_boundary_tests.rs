use crate::runtime::{
    WorthUiDurableStateFamilyHook, WorthUiDurableStateFamilyId, WorthUiDurableStateInventoryDenial,
    WorthUiDurableStateReplacementPolicy, WorthUiStateOwnerIdentity,
    WorthUiStatePersistencePosture,
};

use super::durable_state_inventory_test_support::{
    deterministic_replacement_plan, inspector_tabs_hook, platform_inventory,
};

#[test]
fn custom_state_family_without_explicit_id_rejected() {
    let (runtime, plan) = deterministic_replacement_plan();
    let hook = WorthUiDurableStateFamilyHook::custom(WorthUiDurableStateFamilyId::custom("  "))
        .with_owner_identity(WorthUiStateOwnerIdentity::custom_hook(
            "workspace.custom.blank-id",
        ))
        .with_replacement_policy(WorthUiDurableStateReplacementPolicy::DropOnReplacement)
        .with_persistence_posture(WorthUiStatePersistencePosture::RuntimeOnly);

    let denial = platform_inventory(&runtime)
        .register_family_hook(hook)
        .build_for_replacement(&plan)
        .expect_err("custom state family id must name a bounded family");

    assert!(matches!(
        denial,
        WorthUiDurableStateInventoryDenial::InvalidCustomStateFamilyId { .. }
    ));
}

#[test]
fn custom_state_family_without_explicit_owner_basis_rejected() {
    let (runtime, plan) = deterministic_replacement_plan();
    let hook = WorthUiDurableStateFamilyHook::custom(WorthUiDurableStateFamilyId::custom(
        "workspace.custom.empty-owner",
    ))
    .with_owner_identity(WorthUiStateOwnerIdentity::custom_hook("   "))
    .with_replacement_policy(WorthUiDurableStateReplacementPolicy::DropOnReplacement)
    .with_persistence_posture(WorthUiStatePersistencePosture::RuntimeOnly);

    let denial = platform_inventory(&runtime)
        .register_family_hook(hook)
        .build_for_replacement(&plan)
        .expect_err("owner identity cannot be blank");

    assert!(matches!(
        denial,
        WorthUiDurableStateInventoryDenial::InvalidOwnerIdentity { .. }
    ));
}

#[test]
fn custom_state_family_cannot_claim_platform_owner_identity() {
    let (runtime, plan) = deterministic_replacement_plan();
    let hook = WorthUiDurableStateFamilyHook::custom(WorthUiDurableStateFamilyId::custom(
        "workspace.custom.fake-platform-owner",
    ))
    .with_owner_identity(WorthUiStateOwnerIdentity::platform_shell())
    .with_replacement_policy(WorthUiDurableStateReplacementPolicy::DropOnReplacement)
    .with_persistence_posture(WorthUiStatePersistencePosture::RuntimeOnly);

    let denial = platform_inventory(&runtime)
        .register_family_hook(hook)
        .build_for_replacement(&plan)
        .expect_err("custom hooks cannot claim platform owner authority");

    assert!(matches!(
        denial,
        WorthUiDurableStateInventoryDenial::ReservedPlatformOwnerIdentity { .. }
    ));
}

#[test]
fn bounded_custom_state_family_registered_with_complete_platform_inventory() {
    let (runtime, plan) = deterministic_replacement_plan();
    let hook = inspector_tabs_hook();

    let inventory = platform_inventory(&runtime)
        .register_family_hook(hook)
        .build_for_replacement(&plan)
        .expect("bounded custom family admits");

    let family = inventory
        .family(&WorthUiDurableStateFamilyId::custom(
            "workspace.custom.inspector-tabs",
        ))
        .expect("custom family registered");
    assert_eq!(
        family.owner_identity().identity_basis(),
        "workspace.inspector.tabs"
    );
    assert_eq!(
        family.replacement_policy(),
        WorthUiDurableStateReplacementPolicy::ReconcileOnLaneChange
    );
    assert_eq!(inventory.counters().registered_hook_family_count(), 1);
}
