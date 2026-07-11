use super::{
    CompactionInterlockFoundationalEvidence, CompactionReadInterlockCounters,
    CompactionCutoverDelta, CompactionReadInterlockDenial,
};
use crate::PhysicalPublicationReceipt;

#[derive(Debug, Clone)]
pub struct CompactionRewritePublication {
    delta: CompactionCutoverDelta,
    publication: PhysicalPublicationReceipt,
    counters: CompactionReadInterlockCounters,
}

impl CompactionRewritePublication {
    pub const fn cutover_state(&self) -> super::CompactionCutoverState {
        super::CompactionCutoverState::PublicationCommitted
    }

    pub const fn cutover_transition(&self) -> super::CompactionCutoverTransition {
        super::CompactionCutoverTransitionKind::PublishRewrite.transition()
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

#[cfg(any(test, feature = "certification-authority"))]
pub fn publish_compaction_rewrite_for_certification(
    delta: super::CompactionCutoverDelta,
    publication: PhysicalPublicationReceipt,
) -> Result<CompactionRewritePublication, CompactionReadInterlockDenial> {
    CompactionRewritePublication::publish_rewrite(delta, publication)
}
