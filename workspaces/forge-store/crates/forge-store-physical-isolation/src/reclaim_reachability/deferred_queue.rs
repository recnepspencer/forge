use super::{ReclaimCounterSnapshot, ReclaimDenial, ReclaimEligibilityProof};

#[derive(Debug, Clone)]
pub struct DeferredReclaimQueue {
    blocked: Vec<ReclaimEligibilityProof>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeferredReclaimReceipt {
    deferred_entries: u64,
    counters: ReclaimCounterSnapshot,
}

impl DeferredReclaimQueue {
    pub fn from_blocked(proof: ReclaimEligibilityProof) -> Result<Self, ReclaimDenial> {
        match proof.try_reclaim() {
            Ok(_) => Ok(Self {
                blocked: Vec::new(),
            }),
            Err(ReclaimDenial::BlockedByLiveHazardLease { .. }) => Ok(Self {
                blocked: vec![proof],
            }),
            Err(denial) => Err(denial),
        }
    }

    pub fn defer_blocked(proof: ReclaimEligibilityProof) -> Self {
        Self {
            blocked: vec![proof],
        }
    }

    pub fn drain_when_eligible(
        self,
        proof: ReclaimEligibilityProof,
    ) -> Result<DeferredReclaimReceipt, ReclaimDenial> {
        let counters = proof.try_reclaim()?;
        Ok(DeferredReclaimReceipt {
            deferred_entries: self.blocked.len() as u64,
            counters,
        })
    }
}

impl DeferredReclaimReceipt {
    pub const fn deferred_entries(self) -> u64 {
        self.deferred_entries
    }

    pub const fn counters(self) -> ReclaimCounterSnapshot {
        self.counters
    }
}
