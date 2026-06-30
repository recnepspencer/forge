use crate::runtime::frame_activation_gate::digest_fold::WorthUiActivationGateDigestFold;
use crate::runtime::{
    WorthUiDurableStateFamilyId, WorthUiDurableStateReconciliationOutcome,
    WorthUiDurableStateReconciliationPlan, WorthUiNodeLifecycleTransition,
};

pub(super) fn reconciliation_basis_digest(plan: &WorthUiDurableStateReconciliationPlan) -> u64 {
    let mut fold = WorthUiActivationGateDigestFold::new(0x7265_636f_6e5f_0011);
    fold.fold_u64(plan.active_artifact_digest());
    fold.fold_u64(plan.candidate_artifact_digest());
    fold.fold_usize(plan.receipts().len());
    for receipt in plan.receipts() {
        fold.fold_text(receipt.identity_basis());
        fold_family_id(&mut fold, receipt.family_id());
        fold_outcome(&mut fold, receipt.outcome());
        if let Some(carry_forward) = receipt.carry_forward() {
            fold.fold_tag(0xcafe);
            fold_transition(&mut fold, carry_forward.transition());
        }
        if let Some(replacement) = receipt.replacement() {
            fold.fold_tag(0xdead);
            fold_transition(&mut fold, replacement.transition());
            fold_outcome(&mut fold, replacement.outcome());
            fold.fold_text(replacement.reason());
        }
    }
    fold.finish()
}

fn fold_family_id(
    fold: &mut WorthUiActivationGateDigestFold,
    family_id: &WorthUiDurableStateFamilyId,
) {
    match family_id {
        WorthUiDurableStateFamilyId::FocusChain => fold.fold_tag(1),
        WorthUiDurableStateFamilyId::ScrollAnchor => fold.fold_tag(2),
        WorthUiDurableStateFamilyId::SelectionRange => fold.fold_tag(3),
        WorthUiDurableStateFamilyId::TextEditBuffer => fold.fold_tag(4),
        WorthUiDurableStateFamilyId::SplitterPosition => fold.fold_tag(5),
        WorthUiDurableStateFamilyId::TabState => fold.fold_tag(6),
        WorthUiDurableStateFamilyId::PanelVisibility => fold.fold_tag(7),
        WorthUiDurableStateFamilyId::Custom(id) => {
            fold.fold_tag(8);
            fold.fold_text(id);
        }
    }
}

fn fold_outcome(
    fold: &mut WorthUiActivationGateDigestFold,
    outcome: WorthUiDurableStateReconciliationOutcome,
) {
    let tag = match outcome {
        WorthUiDurableStateReconciliationOutcome::CarryForward => 1,
        WorthUiDurableStateReconciliationOutcome::Replace => 2,
        WorthUiDurableStateReconciliationOutcome::Drop => 3,
        WorthUiDurableStateReconciliationOutcome::Recreate => 4,
    };
    fold.fold_tag(tag);
}

fn fold_transition(
    fold: &mut WorthUiActivationGateDigestFold,
    transition: WorthUiNodeLifecycleTransition,
) {
    let tag = match transition {
        WorthUiNodeLifecycleTransition::Preserve => 1,
        WorthUiNodeLifecycleTransition::Replace => 2,
        WorthUiNodeLifecycleTransition::Drop => 3,
        WorthUiNodeLifecycleTransition::Create => 4,
        WorthUiNodeLifecycleTransition::Move => 5,
        WorthUiNodeLifecycleTransition::Rebind => 6,
        WorthUiNodeLifecycleTransition::LaneChange => 7,
    };
    fold.fold_tag(tag);
}
