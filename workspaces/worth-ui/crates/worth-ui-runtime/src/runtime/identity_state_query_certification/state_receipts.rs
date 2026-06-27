use crate::runtime::{
    WorthUiDurableStateFamilyId, WorthUiDurableStateReconciliationOutcome,
    WorthUiDurableStateReconciliationReceipt, WorthUiNodeLifecycleTransition,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiStateCarryForwardReceipt {
    identity_basis: String,
    family_id: WorthUiDurableStateFamilyId,
    transition: WorthUiNodeLifecycleTransition,
    source_receipt: WorthUiDurableStateReconciliationReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiStateLifecycleReceipt {
    identity_basis: String,
    family_id: WorthUiDurableStateFamilyId,
    outcome: WorthUiDurableStateReconciliationOutcome,
    transition: WorthUiNodeLifecycleTransition,
    source_receipt: WorthUiDurableStateReconciliationReceipt,
}

impl WorthUiStateCarryForwardReceipt {
    pub(crate) fn from_source(source_receipt: WorthUiDurableStateReconciliationReceipt) -> Self {
        let carry_forward = source_receipt
            .carry_forward()
            .expect("carry-forward source receipt");
        Self {
            identity_basis: carry_forward.identity_basis().to_owned(),
            family_id: carry_forward.family_id().clone(),
            transition: carry_forward.transition(),
            source_receipt,
        }
    }

    pub fn identity_basis(&self) -> &str {
        &self.identity_basis
    }

    pub fn family_id(&self) -> &WorthUiDurableStateFamilyId {
        &self.family_id
    }

    pub fn transition(&self) -> WorthUiNodeLifecycleTransition {
        self.transition
    }

    pub fn source_receipt(&self) -> &WorthUiDurableStateReconciliationReceipt {
        &self.source_receipt
    }
}

impl WorthUiStateLifecycleReceipt {
    pub(crate) fn from_source(source_receipt: WorthUiDurableStateReconciliationReceipt) -> Self {
        let transition = source_receipt
            .carry_forward()
            .map(|receipt| receipt.transition())
            .or_else(|| {
                source_receipt
                    .replacement()
                    .map(|receipt| receipt.transition())
            })
            .expect("state reconciliation receipt transition");
        Self {
            identity_basis: source_receipt.identity_basis().to_owned(),
            family_id: source_receipt.family_id().clone(),
            outcome: source_receipt.outcome(),
            transition,
            source_receipt,
        }
    }

    pub fn identity_basis(&self) -> &str {
        &self.identity_basis
    }

    pub fn family_id(&self) -> &WorthUiDurableStateFamilyId {
        &self.family_id
    }

    pub fn outcome(&self) -> WorthUiDurableStateReconciliationOutcome {
        self.outcome
    }

    pub fn transition(&self) -> WorthUiNodeLifecycleTransition {
        self.transition
    }

    pub fn source_receipt(&self) -> &WorthUiDurableStateReconciliationReceipt {
        &self.source_receipt
    }
}
