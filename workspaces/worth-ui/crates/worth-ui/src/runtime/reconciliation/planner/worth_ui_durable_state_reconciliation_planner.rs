use crate::runtime::{
    WorthUiDurableStateCarryForward, WorthUiDurableStateFamily, WorthUiDurableStateFamilyId,
    WorthUiDurableStateInventory, WorthUiDurableStateReconciliationCounters,
    WorthUiDurableStateReconciliationDenial, WorthUiDurableStateReconciliationOutcome,
    WorthUiDurableStateReconciliationPlan, WorthUiDurableStateReconciliationReceipt,
    WorthUiDurableStateReplacement, WorthUiDurableStateReplacementPolicy,
    WorthUiNodeLifecycleTransition, WorthUiNodeReplacementClassification,
    WorthUiNodeReplacementPlan,
};
use crate::runtime::{
    WorthUiFocusChainReconciliation, WorthUiPanelVisibilityReconciliation,
    WorthUiScrollAnchorReconciliation, WorthUiSelectionRangeReconciliation,
    WorthUiSplitterPositionReconciliation, WorthUiTabStateReconciliation,
    WorthUiTextEditStateReconciliation,
};

pub(crate) struct WorthUiDurableStateReconciliationPlanner;

impl WorthUiDurableStateReconciliationPlanner {
    pub(crate) fn reconcile(
        node_plan: &WorthUiNodeReplacementPlan,
        inventory: &WorthUiDurableStateInventory,
    ) -> Result<WorthUiDurableStateReconciliationPlan, WorthUiDurableStateReconciliationDenial>
    {
        let mut counters = WorthUiDurableStateReconciliationCounters::default();
        reject_ambiguous_node_plan(node_plan, counters)?;
        reject_inventory_digest_mismatch(node_plan, inventory, counters)?;
        reject_missing_platform_families(inventory, &mut counters)?;

        let mut receipts = Vec::new();
        for family in inventory.families() {
            counters.record_family();
            for classification in node_plan.classifications() {
                counters.record_node();
                let receipt =
                    reconcile_classification_family(classification, family, &mut counters);
                counters.record_receipt(receipt.outcome());
                receipts.push(receipt);
            }
        }

        Ok(WorthUiDurableStateReconciliationPlan::new(
            node_plan.active_artifact_digest(),
            node_plan.candidate_artifact_digest(),
            receipts,
            counters,
        ))
    }
}

fn reject_ambiguous_node_plan(
    node_plan: &WorthUiNodeReplacementPlan,
    mut counters: WorthUiDurableStateReconciliationCounters,
) -> Result<(), WorthUiDurableStateReconciliationDenial> {
    if node_plan.is_unambiguous() {
        Ok(())
    } else {
        counters.record_rejected_reconciliation();
        Err(WorthUiDurableStateReconciliationDenial::AmbiguousNodeReplacementPlan { counters })
    }
}

fn reject_inventory_digest_mismatch(
    node_plan: &WorthUiNodeReplacementPlan,
    inventory: &WorthUiDurableStateInventory,
    mut counters: WorthUiDurableStateReconciliationCounters,
) -> Result<(), WorthUiDurableStateReconciliationDenial> {
    if node_plan.active_artifact_digest() == inventory.active_artifact_digest()
        && node_plan.candidate_artifact_digest() == inventory.candidate_artifact_digest()
    {
        Ok(())
    } else {
        counters.record_rejected_reconciliation();
        Err(
            WorthUiDurableStateReconciliationDenial::InventoryDigestMismatch {
                plan_active_artifact_digest: node_plan.active_artifact_digest(),
                inventory_active_artifact_digest: inventory.active_artifact_digest(),
                plan_candidate_artifact_digest: node_plan.candidate_artifact_digest(),
                inventory_candidate_artifact_digest: inventory.candidate_artifact_digest(),
                counters,
            },
        )
    }
}

fn reject_missing_platform_families(
    inventory: &WorthUiDurableStateInventory,
    counters: &mut WorthUiDurableStateReconciliationCounters,
) -> Result<(), WorthUiDurableStateReconciliationDenial> {
    for family_id in WorthUiDurableStateFamilyId::reserved_platform_families() {
        if inventory.family(family_id).is_none() {
            counters.record_rejected_reconciliation();
            return Err(
                WorthUiDurableStateReconciliationDenial::MissingInventoryFamily {
                    family_id: family_id.clone(),
                    counters: *counters,
                },
            );
        }
    }
    Ok(())
}

fn reconcile_classification_family(
    classification: &WorthUiNodeReplacementClassification,
    family: &WorthUiDurableStateFamily,
    counters: &mut WorthUiDurableStateReconciliationCounters,
) -> WorthUiDurableStateReconciliationReceipt {
    if classification.unrestored_durable_state_carry_permitted()
        && family_allows_carry_for_transition(family, classification.transition())
    {
        return carry_receipt(classification, family);
    }

    replacement_receipt(classification, family, counters)
}

fn family_allows_carry_for_transition(
    family: &WorthUiDurableStateFamily,
    transition: WorthUiNodeLifecycleTransition,
) -> bool {
    match family.id() {
        WorthUiDurableStateFamilyId::FocusChain => {
            WorthUiFocusChainReconciliation::allows_carry_for_transition(transition)
        }
        WorthUiDurableStateFamilyId::ScrollAnchor => {
            WorthUiScrollAnchorReconciliation::allows_carry_for_transition(transition)
        }
        WorthUiDurableStateFamilyId::SelectionRange => {
            WorthUiSelectionRangeReconciliation::allows_carry_for_transition(transition)
        }
        WorthUiDurableStateFamilyId::TextEditBuffer => {
            WorthUiTextEditStateReconciliation::allows_carry_for_transition(transition)
        }
        WorthUiDurableStateFamilyId::SplitterPosition => {
            WorthUiSplitterPositionReconciliation::allows_carry_for_transition(transition)
        }
        WorthUiDurableStateFamilyId::TabState => {
            WorthUiTabStateReconciliation::allows_carry_for_transition(transition)
        }
        WorthUiDurableStateFamilyId::PanelVisibility => {
            WorthUiPanelVisibilityReconciliation::allows_carry_for_transition(transition)
        }
        WorthUiDurableStateFamilyId::Custom(_) => custom_family_allows_carry(family, transition),
    }
}

fn custom_family_allows_carry(
    family: &WorthUiDurableStateFamily,
    transition: WorthUiNodeLifecycleTransition,
) -> bool {
    match family.replacement_policy() {
        WorthUiDurableStateReplacementPolicy::PreserveWhenNodeCarriesState => matches!(
            transition,
            WorthUiNodeLifecycleTransition::Preserve
                | WorthUiNodeLifecycleTransition::Move
                | WorthUiNodeLifecycleTransition::Rebind
        ),
        WorthUiDurableStateReplacementPolicy::ReconcileOnLaneChange => matches!(
            transition,
            WorthUiNodeLifecycleTransition::Preserve
                | WorthUiNodeLifecycleTransition::Move
                | WorthUiNodeLifecycleTransition::Rebind
        ),
        WorthUiDurableStateReplacementPolicy::DropOnReplacement
        | WorthUiDurableStateReplacementPolicy::ReplaceOnReplacement => {
            matches!(transition, WorthUiNodeLifecycleTransition::Preserve)
        }
    }
}

fn carry_receipt(
    classification: &WorthUiNodeReplacementClassification,
    family: &WorthUiDurableStateFamily,
) -> WorthUiDurableStateReconciliationReceipt {
    WorthUiDurableStateReconciliationReceipt::from_carry_forward(
        WorthUiDurableStateCarryForward::new(
            classification.identity_basis().to_owned(),
            family.id().clone(),
            classification.transition(),
        ),
    )
}

fn replacement_receipt(
    classification: &WorthUiNodeReplacementClassification,
    family: &WorthUiDurableStateFamily,
    counters: &mut WorthUiDurableStateReconciliationCounters,
) -> WorthUiDurableStateReconciliationReceipt {
    let (outcome, reason) = replacement_outcome(classification, family, counters);
    WorthUiDurableStateReconciliationReceipt::from_replacement(WorthUiDurableStateReplacement::new(
        classification.identity_basis().to_owned(),
        family.id().clone(),
        classification.transition(),
        outcome,
        reason,
    ))
}

fn replacement_outcome(
    classification: &WorthUiNodeReplacementClassification,
    family: &WorthUiDurableStateFamily,
    counters: &mut WorthUiDurableStateReconciliationCounters,
) -> (WorthUiDurableStateReconciliationOutcome, &'static str) {
    if matches!(
        classification.transition(),
        WorthUiNodeLifecycleTransition::Drop
    ) {
        counters.record_orphan_removal();
        return (
            WorthUiDurableStateReconciliationOutcome::Drop,
            "active node dropped; durable state is removed",
        );
    }

    match family.id() {
        WorthUiDurableStateFamilyId::TextEditBuffer => {
            WorthUiTextEditStateReconciliation::replacement_outcome(
                classification.transition(),
                counters,
            )
        }
        WorthUiDurableStateFamilyId::SelectionRange => {
            if let Some(outcome) = WorthUiSelectionRangeReconciliation::replacement_outcome(
                classification.transition(),
                counters,
            ) {
                outcome
            } else {
                platform_replacement_outcome(classification, family)
            }
        }
        WorthUiDurableStateFamilyId::Custom(_) => {
            custom_replacement_outcome(classification, family)
        }
        _ => platform_replacement_outcome(classification, family),
    }
}

fn custom_replacement_outcome(
    classification: &WorthUiNodeReplacementClassification,
    family: &WorthUiDurableStateFamily,
) -> (WorthUiDurableStateReconciliationOutcome, &'static str) {
    if matches!(
        classification.transition(),
        WorthUiNodeLifecycleTransition::Create
    ) {
        return (
            WorthUiDurableStateReconciliationOutcome::Recreate,
            "custom family initializes state for created node",
        );
    }

    match family.replacement_policy() {
        WorthUiDurableStateReplacementPolicy::PreserveWhenNodeCarriesState
        | WorthUiDurableStateReplacementPolicy::DropOnReplacement => (
            WorthUiDurableStateReconciliationOutcome::Drop,
            "custom family policy drops state for this transition",
        ),
        WorthUiDurableStateReplacementPolicy::ReplaceOnReplacement => (
            WorthUiDurableStateReconciliationOutcome::Replace,
            "custom family policy replaces state for this transition",
        ),
        WorthUiDurableStateReplacementPolicy::ReconcileOnLaneChange => {
            if matches!(
                classification.transition(),
                WorthUiNodeLifecycleTransition::LaneChange
            ) {
                (
                    WorthUiDurableStateReconciliationOutcome::Recreate,
                    "custom family policy recreates state after lane change",
                )
            } else {
                (
                    WorthUiDurableStateReconciliationOutcome::Replace,
                    "custom family policy replaces state for non-carry transition",
                )
            }
        }
    }
}

fn platform_replacement_outcome(
    classification: &WorthUiNodeReplacementClassification,
    family: &WorthUiDurableStateFamily,
) -> (WorthUiDurableStateReconciliationOutcome, &'static str) {
    match (family.replacement_policy(), classification.transition()) {
        (_, WorthUiNodeLifecycleTransition::Create) => (
            WorthUiDurableStateReconciliationOutcome::Recreate,
            "candidate node created; durable state is initialized",
        ),
        (
            WorthUiDurableStateReplacementPolicy::ReplaceOnReplacement,
            WorthUiNodeLifecycleTransition::Replace | WorthUiNodeLifecycleTransition::LaneChange,
        ) => (
            WorthUiDurableStateReconciliationOutcome::Replace,
            "family policy replaces state for changed node",
        ),
        (
            WorthUiDurableStateReplacementPolicy::ReconcileOnLaneChange,
            WorthUiNodeLifecycleTransition::LaneChange,
        ) => (
            WorthUiDurableStateReconciliationOutcome::Recreate,
            "family policy recreates state across lane change",
        ),
        _ => (
            WorthUiDurableStateReconciliationOutcome::Drop,
            "node transition does not permit durable state carry-forward",
        ),
    }
}
