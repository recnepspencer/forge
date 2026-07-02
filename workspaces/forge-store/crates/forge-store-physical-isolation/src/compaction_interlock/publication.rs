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
    pub fn publish(
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

    pub const fn delta(&self) -> &CompactionCutoverDelta {
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
