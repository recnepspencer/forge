use crate::capability::{
    MosaicStatePersistencePolicy, MosaicStateReplacementRule, MosaicStateSlotKind,
};
use crate::runtime::{
    WorthUiDurableStateFamilyId, WorthUiDurableStateReplacementPolicy,
    WorthUiTransientInteractionState,
};

use super::durable_state_inventory_test_support::{
    admitted_state_inventory, deterministic_replacement_plan, platform_inventory, state_slot,
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
    assert_eq!(first.counters().registered_application_family_count(), 0);
    assert_eq!(first.counters().replacement_classification_count(), 2);
}

#[test]
fn admitted_state_family_registration_order_is_canonicalized() {
    let (_, plan) = deterministic_replacement_plan();
    let alpha = state_slot(
        "workspace.state.alpha",
        MosaicStateSlotKind::region_visibility(),
        MosaicStatePersistencePolicy::ephemeral_during_runtime(),
        MosaicStateReplacementRule::discard_when_owner_changes(),
    );
    let beta = state_slot(
        "workspace.state.beta",
        MosaicStateSlotKind::focused_region(),
        MosaicStatePersistencePolicy::restore_across_hot_reload(),
        MosaicStateReplacementRule::preserve_when_owner_matches(),
    );

    let alpha_then_beta = admitted_state_inventory([alpha.clone(), beta.clone()])
        .build_for_replacement(&plan)
        .expect("admitted inventory builds");
    let beta_then_alpha = admitted_state_inventory([beta, alpha])
        .build_for_replacement(&plan)
        .expect("admitted inventory reorders deterministically");

    assert_eq!(alpha_then_beta, beta_then_alpha);
    assert_eq!(alpha_then_beta.families().len(), 9);
    assert_eq!(
        alpha_then_beta
            .counters()
            .registered_application_family_count(),
        2
    );
}

#[test]
fn accepted_state_slot_contract_lowers_to_replacement_policy() {
    let (_, plan) = deterministic_replacement_plan();
    let inventory = admitted_state_inventory([state_slot(
        "workspace.state.inspector_tabs",
        MosaicStateSlotKind::active_stack_item(),
        MosaicStatePersistencePolicy::persist_across_runtime_restart(),
        MosaicStateReplacementRule::remap_when_runtime_supplies_alias(),
    )])
    .build_for_replacement(&plan)
    .expect("accepted state slot lowers");

    let family_id = WorthUiDurableStateFamilyId::custom("workspace.state.inspector_tabs");
    let family = inventory
        .families()
        .iter()
        .find(|family| family.id() == &family_id)
        .expect("admitted state family registered");
    assert_eq!(
        family.replacement_policy(),
        WorthUiDurableStateReplacementPolicy::ReplaceOnReplacement
    );
    assert_ne!(family.contract_digest(), 0);
}

#[test]
fn transient_interaction_state_is_a_sealed_drop_policy() {
    let (runtime, plan) = deterministic_replacement_plan();
    let inventory = platform_inventory(&runtime)
        .build_for_replacement(&plan)
        .expect("inventory builds");

    assert!(WorthUiTransientInteractionState::all()
        .iter()
        .all(|state| state.drops_by_default()));
    assert_eq!(
        inventory.counters().transient_drop_policy_count(),
        WorthUiTransientInteractionState::all().len()
    );
}
