use forge_store_contracts::DurableArtifactFamilyId;

use crate::{
    RecoveryLayoutReadmissionClass, RecoveryLayoutReadmissionIdentity,
    RecoveryLayoutReadmissionWitness,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryReadmissionLayoutReport {
    family_id: DurableArtifactFamilyId,
    class: RecoveryLayoutReadmissionClass,
    identity: RecoveryLayoutReadmissionIdentity,
}

impl RecoveryReadmissionLayoutReport {
    pub fn from_witness(witness: &RecoveryLayoutReadmissionWitness) -> Self {
        Self {
            family_id: witness.family_id(),
            class: witness.class(),
            identity: witness.identity().clone(),
        }
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }
    pub const fn class(&self) -> RecoveryLayoutReadmissionClass {
        self.class
    }
    pub const fn identity(&self) -> &RecoveryLayoutReadmissionIdentity {
        &self.identity
    }
}
