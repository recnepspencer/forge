use crate::runtime::{
    WorthUiDurableStateCarryForward, WorthUiDurableStateFamilyId,
    WorthUiDurableStateReconciliationOutcome, WorthUiDurableStateReplacement,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDurableStateReconciliationReceipt {
    identity_basis: String,
    family_id: WorthUiDurableStateFamilyId,
    family_contract_digest: u64,
    outcome: WorthUiDurableStateReconciliationOutcome,
    carry_forward: Option<WorthUiDurableStateCarryForward>,
    replacement: Option<WorthUiDurableStateReplacement>,
}

impl WorthUiDurableStateReconciliationReceipt {
    pub(crate) fn from_carry_forward(
        carry_forward: WorthUiDurableStateCarryForward,
        family_contract_digest: u64,
    ) -> Self {
        Self {
            identity_basis: carry_forward.identity_basis().to_owned(),
            family_id: carry_forward.family_id().clone(),
            family_contract_digest,
            outcome: WorthUiDurableStateReconciliationOutcome::CarryForward,
            carry_forward: Some(carry_forward),
            replacement: None,
        }
    }

    pub(crate) fn from_replacement(
        replacement: WorthUiDurableStateReplacement,
        family_contract_digest: u64,
    ) -> Self {
        Self {
            identity_basis: replacement.identity_basis().to_owned(),
            family_id: replacement.family_id().clone(),
            family_contract_digest,
            outcome: replacement.outcome(),
            carry_forward: None,
            replacement: Some(replacement),
        }
    }

    pub fn identity_basis(&self) -> &str {
        &self.identity_basis
    }

    pub fn family_id(&self) -> &WorthUiDurableStateFamilyId {
        &self.family_id
    }

    pub fn family_contract_digest(&self) -> u64 {
        self.family_contract_digest
    }

    pub fn outcome(&self) -> WorthUiDurableStateReconciliationOutcome {
        self.outcome
    }

    pub fn carry_forward(&self) -> Option<&WorthUiDurableStateCarryForward> {
        self.carry_forward.as_ref()
    }

    pub fn replacement(&self) -> Option<&WorthUiDurableStateReplacement> {
        self.replacement.as_ref()
    }
}
