use crate::capability::MosaicStateReplacementRule;
use crate::runtime::{
    WorthUiDurableStateFamilyId, WorthUiDurableStateReconciliationDenial,
    WorthUiDurableStateReconciliationOutcome, WorthUiNodeLifecycleTransition,
};

use super::durable_state_reconciliation_test_support::{
    ambiguous_plan_with_inventory, custom_inventory_for_rule, deterministic_reconciliation_inputs,
    drop_create_inputs, inventory_from_foreign_replacement, lane_change_inputs,
    moved_scroll_anchor_inputs, rebind_plan_with_inventory, reversed_inventory_for,
    structural_replacement_inputs,
};

#[test]
fn equivalent_identity_and_state_inputs_preserve_equivalent_state() {
    let (runtime, plan, inventory) = deterministic_reconciliation_inputs();
    let replay_inventory = reversed_inventory_for(&runtime, &plan);

    let first = runtime
        .reconcile_durable_state(&plan, &inventory)
        .expect("reconciliation succeeds");
    let second = runtime
        .reconcile_durable_state(&plan, &replay_inventory)
        .expect("reconciliation replays");

    assert_eq!(first, second);
    assert_eq!(first.counters().reconciled_family_count(), 7);
    assert_eq!(first.counters().reconciled_node_count(), 14);
    assert_eq!(first.counters().receipt_count(), 14);
    assert_eq!(first.counters().carry_forward_count(), 14);
    assert_eq!(
        first.receipts().len(),
        plan.classifications().len() * inventory.families().len()
    );
    for classification in plan.classifications() {
        for family in inventory.families() {
            assert!(
                first
                    .receipt_for(classification.identity_basis(), family.id())
                    .is_some(),
                "missing receipt for {} / {:?}",
                classification.identity_basis(),
                family.id()
            );
        }
    }
}

#[test]
fn ambiguous_or_replaced_identity_drops_state_with_receipt() {
    let (ambiguous_runtime, ambiguous_plan, ambiguous_inventory) = ambiguous_plan_with_inventory();
    let denial = ambiguous_runtime
        .reconcile_durable_state(&ambiguous_plan, &ambiguous_inventory)
        .expect_err("ambiguous plan denies before receipt construction");
    match denial {
        WorthUiDurableStateReconciliationDenial::AmbiguousNodeReplacementPlan { counters } => {
            assert_eq!(counters.rejected_reconciliation_count(), 1);
            assert_eq!(counters.receipt_count(), 0);
        }
        other => panic!("unexpected ambiguous denial: {other:?}"),
    }

    let (runtime, plan, inventory) = structural_replacement_inputs();

    let reconciliation = runtime
        .reconcile_durable_state(&plan, &inventory)
        .expect("reconciliation succeeds");
    let receipt = reconciliation
        .receipt_for(
            "component:affected",
            &WorthUiDurableStateFamilyId::FocusChain,
        )
        .expect("affected receipt exists");

    assert_eq!(
        receipt.outcome(),
        WorthUiDurableStateReconciliationOutcome::Drop
    );
    assert_eq!(
        receipt
            .replacement()
            .expect("replacement evidence")
            .transition(),
        WorthUiNodeLifecycleTransition::Replace
    );
    assert_eq!(reconciliation.counters().drop_count(), 7);
    assert_eq!(reconciliation.counters().carry_forward_count(), 7);
}

#[test]
fn text_input_state_not_preserved_across_incompatible_component_shape() {
    let (runtime, plan, inventory) = structural_replacement_inputs();

    let reconciliation = runtime
        .reconcile_durable_state(&plan, &inventory)
        .expect("reconciliation succeeds");
    let receipt = reconciliation
        .receipt_for(
            "component:affected",
            &WorthUiDurableStateFamilyId::TextEditBuffer,
        )
        .expect("text receipt exists");

    assert_eq!(
        receipt.outcome(),
        WorthUiDurableStateReconciliationOutcome::Drop
    );
    assert_eq!(reconciliation.counters().incompatible_shape_count(), 1);
    assert!(receipt
        .replacement()
        .expect("text replacement evidence")
        .reason()
        .contains("compatible component shape"));
}

#[test]
fn orphan_state_removed_after_node_drop() {
    let (runtime, plan, inventory) = drop_create_inputs();

    let reconciliation = runtime
        .reconcile_durable_state(&plan, &inventory)
        .expect("reconciliation succeeds");

    for family_id in WorthUiDurableStateFamilyId::reserved_platform_families() {
        assert_eq!(
            reconciliation
                .receipt_for("component:dashboard:old", family_id)
                .expect("old node receipt exists")
                .outcome(),
            WorthUiDurableStateReconciliationOutcome::Drop
        );
        assert_eq!(
            reconciliation
                .receipt_for("component:dashboard:new", family_id)
                .expect("new node receipt exists")
                .outcome(),
            WorthUiDurableStateReconciliationOutcome::Recreate
        );
    }
    assert_eq!(reconciliation.counters().orphan_removal_count(), 7);
}

#[test]
fn scroll_anchor_survives_reorder_only_with_stable_target_identity() {
    let (runtime, moved_plan, moved_inventory) = moved_scroll_anchor_inputs();
    let moved_reconciliation = runtime
        .reconcile_durable_state(&moved_plan, &moved_inventory)
        .expect("moved state reconciles");

    let (lane_runtime, lane_plan, lane_inventory) = lane_change_inputs();
    let lane_reconciliation = lane_runtime
        .reconcile_durable_state(&lane_plan, &lane_inventory)
        .expect("lane state reconciles");

    assert_eq!(
        moved_reconciliation
            .receipt_for("surface:stable", &WorthUiDurableStateFamilyId::ScrollAnchor)
            .expect("moved scroll receipt")
            .outcome(),
        WorthUiDurableStateReconciliationOutcome::CarryForward
    );
    assert_eq!(
        lane_reconciliation
            .receipt_for("surface:stable", &WorthUiDurableStateFamilyId::ScrollAnchor)
            .expect("lane scroll receipt")
            .outcome(),
        WorthUiDurableStateReconciliationOutcome::Recreate
    );
}

#[test]
fn selection_range_rejected_when_backing_collection_identity_changes() {
    let (runtime, plan, inventory) = rebind_plan_with_inventory();

    let reconciliation = runtime
        .reconcile_durable_state(&plan, &inventory)
        .expect("reconciliation succeeds with selection drop evidence");
    let receipt = reconciliation
        .receipt_for(
            "binding:query-results",
            &WorthUiDurableStateFamilyId::SelectionRange,
        )
        .expect("selection receipt exists");

    assert_eq!(
        receipt.outcome(),
        WorthUiDurableStateReconciliationOutcome::Drop
    );
    assert_eq!(reconciliation.counters().query_posture_required_count(), 1);
}

#[test]
fn admitted_state_slot_cannot_escape_declared_replacement_rule() {
    let (runtime, plan, _) = structural_replacement_inputs();
    let custom_family = WorthUiDurableStateFamilyId::custom("workspace.state.reconcile_cache");

    let drop_inventory = custom_inventory_for_rule(
        &plan,
        MosaicStateReplacementRule::discard_when_owner_changes(),
    );
    let drop_reconciliation = runtime
        .reconcile_durable_state(&plan, &drop_inventory)
        .expect("custom drop reconciliation succeeds");
    assert_eq!(
        drop_reconciliation
            .receipt_for("component:affected", &custom_family)
            .expect("custom drop receipt")
            .outcome(),
        WorthUiDurableStateReconciliationOutcome::Drop
    );

    let replace_inventory = custom_inventory_for_rule(
        &plan,
        MosaicStateReplacementRule::remap_when_runtime_supplies_alias(),
    );
    let replace_reconciliation = runtime
        .reconcile_durable_state(&plan, &replace_inventory)
        .expect("custom replace reconciliation succeeds");
    assert_eq!(
        replace_reconciliation
            .receipt_for("component:affected", &custom_family)
            .expect("custom replace receipt")
            .outcome(),
        WorthUiDurableStateReconciliationOutcome::Replace
    );
}

#[test]
fn admitted_family_contract_change_changes_reconciliation_authority_basis() {
    let (runtime, plan, _) = structural_replacement_inputs();
    let preserve_inventory = custom_inventory_for_rule(
        &plan,
        MosaicStateReplacementRule::preserve_when_owner_matches(),
    );
    let discard_inventory = custom_inventory_for_rule(
        &plan,
        MosaicStateReplacementRule::discard_when_owner_changes(),
    );

    let preserve = runtime
        .reconcile_durable_state(&plan, &preserve_inventory)
        .expect("preserve contract reconciles");
    let discard = runtime
        .reconcile_durable_state(&plan, &discard_inventory)
        .expect("discard contract reconciles");
    let family_id = WorthUiDurableStateFamilyId::custom("workspace.state.reconcile_cache");
    let preserve_receipt = preserve
        .receipt_for("component:affected", &family_id)
        .expect("preserve receipt exists");
    let discard_receipt = discard
        .receipt_for("component:affected", &family_id)
        .expect("discard receipt exists");

    assert_ne!(
        preserve_receipt.family_contract_digest(),
        discard_receipt.family_contract_digest()
    );
    assert_ne!(preserve.basis_digest(), discard.basis_digest());
}

#[test]
fn inventory_digest_mismatch_denies_before_reconciliation() {
    let (runtime, plan, _) = deterministic_reconciliation_inputs();
    let foreign_inventory = inventory_from_foreign_replacement();

    let denial = runtime
        .reconcile_durable_state(&plan, &foreign_inventory)
        .expect_err("foreign inventory denies");

    match denial {
        WorthUiDurableStateReconciliationDenial::InventoryDigestMismatch {
            plan_active_artifact_digest,
            inventory_active_artifact_digest,
            plan_candidate_artifact_digest,
            inventory_candidate_artifact_digest,
            counters,
        } => {
            assert!(
                plan_active_artifact_digest != inventory_active_artifact_digest
                    || plan_candidate_artifact_digest != inventory_candidate_artifact_digest
            );
            assert_eq!(counters.rejected_reconciliation_count(), 1);
            assert_eq!(counters.receipt_count(), 0);
        }
        other => panic!("unexpected denial: {other:?}"),
    }
}
