use super::{
    CompactionCutoverDelta, CompactionInterlockFoundationalEvidence,
    CompactionReadInterlockCounters, CompactionReadInterlockDenial,
};
use crate::PhysicalPublicationReceipt;

#[derive(Debug, Clone)]
pub struct CompactionRewritePublication {
    delta: CompactionCutoverDelta,
    publication: PhysicalPublicationReceipt,
    counters: CompactionReadInterlockCounters,
}

impl CompactionRewritePublication {
    const OWNER_CASE: super::CompactionOwnerCaseDeclaration =
        super::CompactionOwnerCaseDeclaration::declared_by_owner(
            super::CompactionOwnerCaseId::owned("physical.compaction.publish_rewrite"),
            super::CompactionCutoverState::RewriteLowered,
            super::CompactionCutoverState::PublicationCommitted,
        );

    pub const fn cutover_state(&self) -> super::CompactionCutoverState {
        super::CompactionCutoverState::PublicationCommitted
    }

    pub const fn owner_case_observation(&self) -> super::CompactionOwnerCaseObservation {
        super::CompactionOwnerCaseObservation::issued_by_owner(Self::OWNER_CASE)
    }

    pub fn publish_rewrite(
        delta: CompactionCutoverDelta,
        publication: PhysicalPublicationReceipt,
    ) -> Result<Self, CompactionReadInterlockDenial> {
        let delta = delta.bind_publication(&publication)?;
        let counters = delta.plan().counters().with_publication_swap();
        Ok(Self {
            delta,
            publication,
            counters,
        })
    }

    pub const fn delta(&self) -> &super::CompactionCutoverDelta {
        &self.delta
    }

    pub const fn publication(&self) -> &PhysicalPublicationReceipt {
        &self.publication
    }

    pub const fn counters(&self) -> CompactionReadInterlockCounters {
        self.counters
    }

    pub const fn foundational_evidence(&self) -> CompactionInterlockFoundationalEvidence {
        CompactionInterlockFoundationalEvidence::after_store_decision(self.counters)
    }
}

pub(super) fn owner_cases() -> impl Iterator<Item = super::CompactionOwnerCaseDeclaration> {
    std::iter::once(CompactionRewritePublication::OWNER_CASE)
}
