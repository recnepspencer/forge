use super::{
    foundational_lowering::PhysicalIsolationEntryFoundationalEvidence,
    proof_progression::PhysicalIsolationEntryProofProgression, PhysicalIsolationEntryIdentity,
};

#[derive(Debug, Clone)]
pub struct PhysicalIsolationEntryEvidence {
    foundational: PhysicalIsolationEntryFoundationalEvidence,
    proof_progression: PhysicalIsolationEntryProofProgression,
}

impl PhysicalIsolationEntryEvidence {
    pub(crate) fn from_entry_identity(identity: &PhysicalIsolationEntryIdentity) -> Self {
        Self {
            foundational: PhysicalIsolationEntryFoundationalEvidence::lower(identity),
            proof_progression: PhysicalIsolationEntryProofProgression::from_identity(
                identity.clone(),
            ),
        }
    }

    pub const fn foundational(&self) -> &PhysicalIsolationEntryFoundationalEvidence {
        &self.foundational
    }

    pub const fn proof_progression(&self) -> &PhysicalIsolationEntryProofProgression {
        &self.proof_progression
    }

    pub const fn is_store_physical_stability_authority(&self) -> bool {
        false
    }
}
