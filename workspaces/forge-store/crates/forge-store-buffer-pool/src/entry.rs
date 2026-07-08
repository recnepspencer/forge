use crate::{
    AdmittedBufferPoolEntry, BufferPoolAdmission, BufferPoolBudget, BufferPoolEntryDenial,
    BufferPoolEntryDenialKind, S2PhysicalEntryFacts,
};
use forge_store_contracts::S2PhysicalSubstrateReadinessSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S2PhysicalResidencyEntry {
    facts: S2PhysicalEntryFacts,
}

impl S2PhysicalResidencyEntry {
    pub fn from_physical_substrate_snapshot(
        snapshot: S2PhysicalSubstrateReadinessSnapshot,
    ) -> Result<Self, BufferPoolEntryDenial> {
        if !snapshot.is_sealed() {
            return Err(BufferPoolEntryDenial::new(
                BufferPoolEntryDenialKind::UnsealedReadiness,
            ));
        }
        Ok(Self {
            facts: S2PhysicalEntryFacts::from_snapshot(snapshot),
        })
    }

    pub const fn with_budget(self, budget: BufferPoolBudget) -> S2PhysicalResidencyEntryBuilder {
        S2PhysicalResidencyEntryBuilder {
            entry: Some(self),
            budget: Some(budget),
        }
    }

    pub const fn facts(&self) -> S2PhysicalEntryFacts {
        self.facts
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S2PhysicalResidencyEntryBuilder {
    entry: Option<S2PhysicalResidencyEntry>,
    budget: Option<BufferPoolBudget>,
}

impl S2PhysicalResidencyEntryBuilder {
    pub const fn admit(self) -> Result<AdmittedBufferPoolEntry, BufferPoolEntryDenial> {
        let Some(entry) = self.entry else {
            return Err(BufferPoolEntryDenial::new(
                BufferPoolEntryDenialKind::MissingReadiness,
            ));
        };
        let Some(budget) = self.budget else {
            return Err(BufferPoolEntryDenial::new(
                BufferPoolEntryDenialKind::MissingBudget,
            ));
        };
        Ok(AdmittedBufferPoolEntry::new(BufferPoolAdmission::new(
            budget,
            entry.facts,
        )))
    }
}
