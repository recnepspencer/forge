use core::marker::PhantomData;
use std::sync::Arc;

use super::settlement::UiIntentAdmissionLease;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentAdmissionSlotIdentity {
    slot: u8,
    generation: u64,
}

pub(crate) struct UiAdmittedIntentIdentity {
    slot: UiIntentAdmissionSlotIdentity,
    lineage: super::super::UiIntentAttemptLineage,
    definition: crate::capability::UiIntentId,
    declaration: crate::declaration::UiIntentDeclarationIdentity,
}

#[must_use]
pub struct UiAdmittedIntent<I: crate::capability::UiIntent> {
    slot_identity: UiIntentAdmissionSlotIdentity,
    lineage: super::super::UiIntentAttemptLineage,
    definition: crate::capability::UiIntentId,
    declaration: crate::declaration::UiIntentDeclarationIdentity,
    cost: super::UiIntentAdmissionCost,
    lease: Arc<UiIntentAdmissionLease>,
    intent: PhantomData<fn() -> I>,
}

impl UiIntentAdmissionSlotIdentity {
    pub(crate) const fn new(slot: u8, generation: u64) -> Self {
        Self { slot, generation }
    }

    pub const fn slot(self) -> u8 {
        self.slot
    }

    pub const fn generation(self) -> u64 {
        self.generation
    }
}

impl<I: crate::capability::UiIntent> UiAdmittedIntent<I> {
    pub(crate) fn new(
        identity: UiAdmittedIntentIdentity,
        cost: super::UiIntentAdmissionCost,
        lease: Arc<UiIntentAdmissionLease>,
    ) -> Self {
        Self {
            slot_identity: identity.slot,
            lineage: identity.lineage,
            definition: identity.definition,
            declaration: identity.declaration,
            cost,
            lease,
            intent: PhantomData,
        }
    }

    pub const fn slot_identity(&self) -> UiIntentAdmissionSlotIdentity {
        self.slot_identity
    }

    pub const fn lineage(&self) -> super::super::UiIntentAttemptLineage {
        self.lineage
    }

    pub const fn definition_id(&self) -> crate::capability::UiIntentId {
        self.definition
    }

    pub fn declaration_identity(&self) -> &str {
        self.declaration.as_str()
    }

    pub const fn cost(&self) -> super::UiIntentAdmissionCost {
        self.cost
    }

    pub(crate) fn into_parts(self) -> (UiIntentAdmissionSlotIdentity, Arc<UiIntentAdmissionLease>) {
        (self.slot_identity, self.lease)
    }
}

impl UiAdmittedIntentIdentity {
    pub(crate) const fn new(
        slot: UiIntentAdmissionSlotIdentity,
        lineage: super::super::UiIntentAttemptLineage,
        definition: crate::capability::UiIntentId,
        declaration: crate::declaration::UiIntentDeclarationIdentity,
    ) -> Self {
        Self {
            slot,
            lineage,
            definition,
            declaration,
        }
    }
}
