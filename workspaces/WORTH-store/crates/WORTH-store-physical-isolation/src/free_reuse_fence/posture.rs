use crate::{ReclaimCounterSnapshot, ReclaimReachabilityRemovalReceipt};

use super::{AllocatorPublicationReceipt, FreeReuseFenceDenial, GenerationAdvanceReceipt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrashStableReclaimReuseFence {
    reachability_removal: ReclaimReachabilityRemovalReceipt,
    generation_advance: GenerationAdvanceReceipt,
    allocator_publication: AllocatorPublicationReceipt,
}

impl CrashStableReclaimReuseFence {
    pub fn admit_after_reclaim(
        reachability_removal: ReclaimReachabilityRemovalReceipt,
        generation_advance: GenerationAdvanceReceipt,
        allocator_publication: AllocatorPublicationReceipt,
    ) -> Result<Self, FreeReuseFenceDenial> {
        if !reachability_removal.covers_reclaimed_identity(generation_advance.old_identity()) {
            return Err(FreeReuseFenceDenial::ReclaimRemovalDoesNotCoverReusedIdentity);
        }
        Ok(Self {
            reachability_removal,
            generation_advance,
            allocator_publication,
        })
    }

    pub const fn reachability_removal(&self) -> &ReclaimReachabilityRemovalReceipt {
        &self.reachability_removal
    }

    pub const fn reclaim_counters(&self) -> ReclaimCounterSnapshot {
        self.reachability_removal.counters()
    }

    pub const fn generation_advance(&self) -> GenerationAdvanceReceipt {
        self.generation_advance
    }

    pub const fn allocator_publication(&self) -> AllocatorPublicationReceipt {
        self.allocator_publication
    }
}
