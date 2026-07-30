use super::{BlobCompactionCounterSnapshot, BlobCompactionRewritePlan};
use crate::ChunkTreeRoot;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobCompactionResidue {
    pre_compaction_root: ChunkTreeRoot,
    counters: BlobCompactionCounterSnapshot,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BlobCompactionRestartOutcome {
    ResumeAdmittedRewrite(Box<BlobCompactionRewritePlan>),
    RollBackToPreCompactionPlacement { root: ChunkTreeRoot },
    ResidueLocalized(BlobCompactionResidue),
}

impl BlobCompactionRestartOutcome {
    pub fn resume(plan: BlobCompactionRewritePlan) -> Self {
        Self::ResumeAdmittedRewrite(Box::new(plan))
    }

    pub fn roll_back(plan: &BlobCompactionRewritePlan) -> Self {
        Self::RollBackToPreCompactionPlacement {
            root: plan.old_root().clone(),
        }
    }

    pub fn localize_residue(plan: &BlobCompactionRewritePlan) -> Self {
        Self::ResidueLocalized(BlobCompactionResidue {
            pre_compaction_root: plan.old_root().clone(),
            counters: plan.counters().record_residue_localized(),
        })
    }
}

impl BlobCompactionResidue {
    pub const fn pre_compaction_root(&self) -> &ChunkTreeRoot {
        &self.pre_compaction_root
    }

    pub const fn counters(&self) -> BlobCompactionCounterSnapshot {
        self.counters
    }
}
