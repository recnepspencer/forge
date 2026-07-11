use super::{
    CompactionInterlockFoundationalEvidence, CompactionReadInterlockCounters,
    CompactionReadInterlockDenial, LsmCompactionCutoverDelta,
};
use crate::PhysicalPublicationReceipt;

#[derive(Debug, Clone)]
pub struct CompactionRewritePublication {
    delta: LsmCompactionCutoverDelta,
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

    pub fn publish_with_lsm(
        delta: LsmCompactionCutoverDelta,
        publication: PhysicalPublicationReceipt,
    ) -> Result<Self, CompactionReadInterlockDenial> {
        let delta = delta.bind_publication(&publication)?;
        let counters = delta.delta().plan().counters().with_publication_swap();
        Ok(Self {
            delta,
            publication,
            counters,
        })
    }

    pub const fn delta(&self) -> &super::CompactionCutoverDelta {
        self.delta.delta()
    }

    pub const fn lsm_compaction_receipt(
        &self,
    ) -> &forge_store_wal::layout_access::baseline_lsm_counter_observation::BaselineLsmCompactionPublicationReceipt{
        self.delta.receipt()
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
    lsm_receipt: forge_store_wal::layout_access::baseline_lsm_counter_observation::BaselineLsmCompactionPublicationReceipt,
) -> Result<CompactionRewritePublication, CompactionReadInterlockDenial> {
    CompactionRewritePublication::publish_with_lsm(
        LsmCompactionCutoverDelta::admit(delta, lsm_receipt).into_result()?,
        publication,
    )
}
