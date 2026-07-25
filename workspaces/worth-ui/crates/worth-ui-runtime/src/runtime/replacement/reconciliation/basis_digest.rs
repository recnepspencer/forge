use crate::runtime::{
    WorthUiDurableResizeInputDisposition, WorthUiDurableStateFamilyId,
    WorthUiDurableStateReconciliationOutcome, WorthUiDurableStateReconciliationReceipt,
    WorthUiNodeLifecycleTransition,
};

pub(crate) fn reconciliation_basis_digest(
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    receipts: &[WorthUiDurableStateReconciliationReceipt],
    durable_resize_dispositions: &[WorthUiDurableResizeInputDisposition],
) -> u64 {
    let mut fold = WorthUiReconciliationDigestFold::new(0x7265_636f_6e5f_0011);
    fold.fold_u64(active_artifact_digest);
    fold.fold_u64(candidate_artifact_digest);
    fold.fold_usize(receipts.len());
    for receipt in receipts {
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
    fold.fold_usize(durable_resize_dispositions.len());
    for disposition in durable_resize_dispositions {
        fold.fold_u64(disposition.identity_digest());
    }
    fold.finish()
}

fn fold_family_id(
    fold: &mut WorthUiReconciliationDigestFold,
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
    fold: &mut WorthUiReconciliationDigestFold,
    outcome: WorthUiDurableStateReconciliationOutcome,
) {
    fold.fold_tag(match outcome {
        WorthUiDurableStateReconciliationOutcome::CarryForward => 1,
        WorthUiDurableStateReconciliationOutcome::Replace => 2,
        WorthUiDurableStateReconciliationOutcome::Drop => 3,
        WorthUiDurableStateReconciliationOutcome::Recreate => 4,
    });
}

fn fold_transition(
    fold: &mut WorthUiReconciliationDigestFold,
    transition: WorthUiNodeLifecycleTransition,
) {
    fold.fold_tag(match transition {
        WorthUiNodeLifecycleTransition::Preserve => 1,
        WorthUiNodeLifecycleTransition::Replace => 2,
        WorthUiNodeLifecycleTransition::Drop => 3,
        WorthUiNodeLifecycleTransition::Create => 4,
        WorthUiNodeLifecycleTransition::Move => 5,
        WorthUiNodeLifecycleTransition::Rebind => 6,
        WorthUiNodeLifecycleTransition::LaneChange => 7,
    });
}

struct WorthUiReconciliationDigestFold {
    value: u64,
}

impl WorthUiReconciliationDigestFold {
    fn new(seed: u64) -> Self {
        Self { value: seed }
    }

    fn fold_u64(&mut self, value: u64) {
        self.value ^= value.wrapping_mul(0x9e37_79b9_7f4a_7c15);
        self.value = self.value.rotate_left(13);
    }

    fn fold_usize(&mut self, value: usize) {
        self.fold_u64(value as u64);
    }

    fn fold_tag(&mut self, tag: u64) {
        self.fold_u64(tag);
    }

    fn fold_text(&mut self, text: &str) {
        self.fold_usize(text.len());
        for byte in text.as_bytes() {
            self.value ^= u64::from(*byte);
            self.value = self.value.rotate_left(5);
            self.value = self.value.wrapping_mul(0x100_0000_01b3);
        }
    }

    fn finish(self) -> u64 {
        self.value ^ 0xa47f_2b19_63d5_81ceu64
    }
}
