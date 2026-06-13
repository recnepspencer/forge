use crate::runtime::{
    WorthUiDurableStateFamily, WorthUiDurableStateFamilyId, WorthUiDurableStateInventoryDenial,
    WorthUiDurableStateReplacementPolicy, WorthUiStatePersistencePosture,
    WorthUiTransientInteractionPolicy, WorthUiTransientInteractionState,
};

use super::durable_state_inventory_test_support::{
    custom_state_family_hook, deterministic_replacement_plan, domain_truth_hook,
    duplicate_panel_cache_hook, ownerless_hook, platform_inventory, policyless_inspector_tabs_hook,
    reserved_focus_hook, reversed_platform_inventory,
};

#[test]
fn durable_state_inventory_replay_is_deterministic() {
    let (runtime, plan) = deterministic_replacement_plan();

    let first = platform_inventory(&runtime)
        .build_for_replacement(&plan)
        .expect("inventory builds");
    let second = platform_inventory(&runtime)
        .build_for_replacement(&plan)
        .expect("inventory replays");

    assert_eq!(first, second);
    assert_eq!(
        first.active_artifact_digest(),
        plan.active_artifact_digest()
    );
    assert_eq!(
        first.candidate_artifact_digest(),
        plan.candidate_artifact_digest()
    );
    assert_eq!(first.families().len(), 7);
    assert_eq!(first.counters().registered_platform_family_count(), 7);
    assert_eq!(first.counters().replacement_classification_count(), 2);
    let focus_family = first
        .family(&WorthUiDurableStateFamilyId::FocusChain)
        .expect("focus family registered");
    let scroll_family = first
        .family(&WorthUiDurableStateFamilyId::ScrollAnchor)
        .expect("scroll family registered");
    assert!(focus_family.is_durable());
    assert_ne!(
        focus_family.owner_identity().identity_basis(),
        scroll_family.owner_identity().identity_basis()
    );
}

#[test]
fn platform_state_family_registration_order_is_canonicalized() {
    let (runtime, plan) = deterministic_replacement_plan();

    let forward = platform_inventory(&runtime)
        .build_for_replacement(&plan)
        .expect("forward platform inventory builds");
    let reversed = reversed_platform_inventory(&runtime)
        .build_for_replacement(&plan)
        .expect("reversed platform inventory builds");

    assert_eq!(forward, reversed);
}

#[test]
fn custom_state_family_registration_order_is_canonicalized() {
    let (runtime, plan) = deterministic_replacement_plan();
    let alpha = custom_state_family_hook(
        "workspace.custom.alpha",
        "workspace.custom.owner.alpha",
        WorthUiDurableStateReplacementPolicy::DropOnReplacement,
        WorthUiStatePersistencePosture::RuntimeOnly,
    );
    let beta = custom_state_family_hook(
        "workspace.custom.beta",
        "workspace.custom.owner.beta",
        WorthUiDurableStateReplacementPolicy::ReconcileOnLaneChange,
        WorthUiStatePersistencePosture::SessionRecorded,
    );

    let alpha_then_beta = platform_inventory(&runtime)
        .register_family_hook(alpha.clone())
        .register_family_hook(beta.clone())
        .build_for_replacement(&plan)
        .expect("custom inventory builds");
    let beta_then_alpha = platform_inventory(&runtime)
        .register_family_hook(beta)
        .register_family_hook(alpha)
        .build_for_replacement(&plan)
        .expect("custom inventory reorders deterministically");

    assert_eq!(alpha_then_beta, beta_then_alpha);
}

#[test]
fn state_family_without_owner_identity_rejected() {
    let (runtime, plan) = deterministic_replacement_plan();
    let hook = ownerless_hook("workspace.custom.panel-cache");

    let denial = runtime
        .durable_state_inventory()
        .register_platform_family(WorthUiDurableStateFamily::focus_chain())
        .register_family_hook(hook)
        .build_for_replacement(&plan)
        .expect_err("platform inventory must complete before hook validation");

    match denial {
        WorthUiDurableStateInventoryDenial::MissingPlatformStateFamily {
            family_id,
            counters,
        } => {
            assert_eq!(family_id, WorthUiDurableStateFamilyId::ScrollAnchor);
            assert_eq!(counters.registered_platform_family_count(), 1);
            assert_eq!(counters.rejected_family_count(), 1);
        }
        other => panic!("unexpected denial: {other:?}"),
    }

    let hook = ownerless_hook("workspace.custom.panel-cache");

    let denial = platform_inventory(&runtime)
        .register_family_hook(hook)
        .build_for_replacement(&plan)
        .expect_err("owner identity is mandatory once platform inventory is complete");

    match denial {
        WorthUiDurableStateInventoryDenial::MissingOwnerIdentity {
            family_id,
            counters,
        } => {
            assert_eq!(
                family_id,
                WorthUiDurableStateFamilyId::custom("workspace.custom.panel-cache")
            );
            assert_eq!(counters.rejected_family_count(), 1);
            assert_eq!(counters.registered_hook_family_count(), 0);
        }
        other => panic!("unexpected denial: {other:?}"),
    }
}

#[test]
fn domain_truth_state_cannot_enter_durable_ui_state_inventory() {
    let (runtime, plan) = deterministic_replacement_plan();
    let hook = domain_truth_hook();

    let denial = runtime
        .durable_state_inventory()
        .register_platform_family(WorthUiDurableStateFamily::focus_chain())
        .register_family_hook(hook)
        .build_for_replacement(&plan)
        .expect_err("platform inventory must complete before domain truth hook validation");

    assert!(matches!(
        denial,
        WorthUiDurableStateInventoryDenial::MissingPlatformStateFamily { .. }
    ));

    let hook = domain_truth_hook();

    let denial = platform_inventory(&runtime)
        .register_family_hook(hook)
        .build_for_replacement(&plan)
        .expect_err("domain truth is not UI durable state once platform inventory is complete");

    assert!(matches!(
        denial,
        WorthUiDurableStateInventoryDenial::DomainTruthStateFamily { .. }
    ));
}

#[test]
fn durable_state_family_without_replacement_policy_cannot_be_registered() {
    let (runtime, plan) = deterministic_replacement_plan();
    let hook = policyless_inspector_tabs_hook();

    let denial = runtime
        .durable_state_inventory()
        .register_platform_family(WorthUiDurableStateFamily::focus_chain())
        .register_family_hook(hook)
        .build_for_replacement(&plan)
        .expect_err("platform inventory must complete before hook policy validation");

    assert!(matches!(
        denial,
        WorthUiDurableStateInventoryDenial::MissingPlatformStateFamily { .. }
    ));

    let hook = policyless_inspector_tabs_hook();

    let denial = platform_inventory(&runtime)
        .register_family_hook(hook)
        .build_for_replacement(&plan)
        .expect_err("replacement policy is mandatory once platform inventory is complete");

    assert!(matches!(
        denial,
        WorthUiDurableStateInventoryDenial::MissingReplacementPolicy { .. }
    ));
}

#[test]
fn transient_interaction_state_defaults_to_drop() {
    let (runtime, plan) = deterministic_replacement_plan();

    let inventory = platform_inventory(&runtime)
        .build_for_replacement(&plan)
        .expect("inventory builds");

    for state in WorthUiTransientInteractionState::all() {
        assert!(state.drops_by_default());
        assert_eq!(
            inventory.transient(*state),
            WorthUiTransientInteractionPolicy::Drop
        );
    }
    assert_eq!(
        inventory.counters().transient_drop_policy_count(),
        WorthUiTransientInteractionState::all().len()
    );
}

#[test]
fn duplicate_durable_state_family_claim_rejected_before_reconciliation() {
    let (runtime, plan) = deterministic_replacement_plan();
    let first_hook = duplicate_panel_cache_hook("workspace.custom.panel-cache.primary");
    let second_hook = duplicate_panel_cache_hook("workspace.custom.panel-cache.secondary");

    let denial = runtime
        .durable_state_inventory()
        .register_platform_family(WorthUiDurableStateFamily::focus_chain())
        .register_family_hook(first_hook)
        .register_family_hook(second_hook)
        .build_for_replacement(&plan)
        .expect_err("platform inventory must complete before custom duplicate validation");

    assert!(matches!(
        denial,
        WorthUiDurableStateInventoryDenial::MissingPlatformStateFamily { .. }
    ));

    let first_hook = duplicate_panel_cache_hook("workspace.custom.panel-cache.primary");
    let second_hook = duplicate_panel_cache_hook("workspace.custom.panel-cache.secondary");

    let denial = platform_inventory(&runtime)
        .register_family_hook(first_hook)
        .register_family_hook(second_hook)
        .build_for_replacement(&plan)
        .expect_err("duplicate family is ambiguous once platform inventory is complete");

    match denial {
        WorthUiDurableStateInventoryDenial::DuplicateStateFamily {
            family_id,
            counters,
        } => {
            assert_eq!(
                family_id,
                WorthUiDurableStateFamilyId::custom("workspace.custom.panel-cache")
            );
            assert_eq!(counters.duplicate_family_count(), 1);
            assert_eq!(counters.rejected_family_count(), 1);
        }
        other => panic!("unexpected denial: {other:?}"),
    }
}

#[test]
fn hook_cannot_claim_reserved_platform_state_family() {
    let (runtime, plan) = deterministic_replacement_plan();
    let hook = reserved_focus_hook();

    let denial = runtime
        .durable_state_inventory()
        .register_platform_family(WorthUiDurableStateFamily::focus_chain())
        .register_family_hook(hook)
        .build_for_replacement(&plan)
        .expect_err("platform inventory must complete before hook family validation");

    assert!(matches!(
        denial,
        WorthUiDurableStateInventoryDenial::MissingPlatformStateFamily { .. }
    ));

    let hook = reserved_focus_hook();

    let denial = platform_inventory(&runtime)
        .register_family_hook(hook)
        .build_for_replacement(&plan)
        .expect_err(
            "hooks cannot claim platform-owned families once platform inventory is complete",
        );

    assert!(matches!(
        denial,
        WorthUiDurableStateInventoryDenial::ReservedPlatformStateFamily {
            family_id: WorthUiDurableStateFamilyId::FocusChain,
            ..
        }
    ));
}

#[test]
fn incomplete_platform_state_inventory_rejected_before_reconciliation() {
    let (runtime, plan) = deterministic_replacement_plan();

    let denial = runtime
        .durable_state_inventory()
        .register_platform_family(WorthUiDurableStateFamily::focus_chain())
        .register_platform_family(WorthUiDurableStateFamily::scroll_anchor())
        .build_for_replacement(&plan)
        .expect_err("partial platform inventory cannot feed reconciliation");

    match denial {
        WorthUiDurableStateInventoryDenial::MissingPlatformStateFamily {
            family_id,
            counters,
        } => {
            assert_eq!(family_id, WorthUiDurableStateFamilyId::SelectionRange);
            assert_eq!(counters.registered_platform_family_count(), 2);
            assert_eq!(counters.rejected_family_count(), 1);
        }
        other => panic!("unexpected denial: {other:?}"),
    }
}
