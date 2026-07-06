use crate::{
    CurrentGenerationPhysicalReference, HazardLeaseEpochIndexSnapshot, HazardLeaseOverlap,
};
use forge_store_physical_format::{PhysicalGenerationOwner, PhysicalReclaimRegion};

use super::{ExecutedReachabilityEvidence, ReclaimCounterSnapshot, ReclaimDenial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReclaimEligibilityProof {
    evidence: ExecutedReachabilityEvidence,
    decision: ReclaimDecision,
    counters: ReclaimCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReclaimReachabilityRemovalReceipt {
    evidence: ExecutedReachabilityEvidence,
    counters: ReclaimCounterSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S6ReclaimReachabilityRemovalEvidence {
    region: PhysicalReclaimRegion,
    root_epoch: u64,
    protected_ranges: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S6ReclaimReachabilityRemovalEvidenceDenial {
    EmptyProtectedReachability,
    OwnerNotCoveredByReachability,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReclaimDecision {
    Eligible,
    Blocked(BlockedReclaimReport),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockedReclaimReport {
    overlap: HazardLeaseOverlap,
    candidate_ranges: u64,
}

impl ReclaimEligibilityProof {
    pub fn admit(
        evidence: ExecutedReachabilityEvidence,
        live_hazards: HazardLeaseEpochIndexSnapshot,
    ) -> Result<Self, ReclaimDenial> {
        let scan = live_hazards.first_overlap(evidence.candidates());
        let mut counters = ReclaimCounterSnapshot::from_inputs(
            evidence.candidates().candidate_ranges().len() as u64,
            live_hazards.live_entries() as u64,
            scan.epoch_buckets_touched,
            scan.hazard_entries_touched,
            scan.counters,
        );
        let decision = match scan.first_overlap {
            Some(overlap) => {
                counters = counters.with_blocked_reclaim();
                ReclaimDecision::Blocked(BlockedReclaimReport {
                    overlap,
                    candidate_ranges: evidence.candidates().candidate_ranges().len() as u64,
                })
            }
            None => {
                counters = counters.with_eligible_reclaim();
                ReclaimDecision::Eligible
            }
        };
        Ok(Self {
            evidence,
            decision,
            counters,
        })
    }

    pub fn try_reclaim(&self) -> Result<ReclaimCounterSnapshot, ReclaimDenial> {
        match &self.decision {
            ReclaimDecision::Eligible => Ok(self.counters),
            ReclaimDecision::Blocked(report) => Err(report.denial()),
        }
    }

    pub fn admit_reachability_removal(
        self,
    ) -> Result<ReclaimReachabilityRemovalReceipt, ReclaimDenial> {
        match self.decision {
            ReclaimDecision::Eligible => Ok(ReclaimReachabilityRemovalReceipt {
                evidence: self.evidence,
                counters: self.counters,
            }),
            ReclaimDecision::Blocked(report) => Err(report.denial()),
        }
    }

    pub const fn decision(&self) -> &ReclaimDecision {
        &self.decision
    }

    pub const fn counters(&self) -> ReclaimCounterSnapshot {
        self.counters
    }

    pub const fn evidence(&self) -> &ExecutedReachabilityEvidence {
        &self.evidence
    }

    #[cfg(any(test, feature = "certification-authority"))]
    pub fn for_certification_test() -> Self {
        let evidence = ExecutedReachabilityEvidence::for_certification_test();
        let capacity = crate::HazardLeaseTableCapacity::bounded_slots(1)
            .expect("certification test capacity is non-empty");
        let hazards = crate::HazardLeaseTable::with_capacity(capacity).live_index_snapshot();
        Self::admit(evidence, hazards).expect("certification test reclaim proof is eligible")
    }

    #[cfg(any(test, feature = "certification-authority"))]
    pub fn for_certification_reference(reference: CurrentGenerationPhysicalReference) -> Self {
        let evidence = ExecutedReachabilityEvidence::for_certification_reference(reference);
        let capacity = crate::HazardLeaseTableCapacity::bounded_slots(1)
            .expect("certification test capacity is non-empty");
        let hazards = crate::HazardLeaseTable::with_capacity(capacity).live_index_snapshot();
        Self::admit(evidence, hazards).expect("certification test reclaim proof is eligible")
    }
}

impl ReclaimReachabilityRemovalReceipt {
    pub const fn evidence(&self) -> &ExecutedReachabilityEvidence {
        &self.evidence
    }

    pub fn covers_reclaimed_identity(&self, identity: CurrentGenerationPhysicalReference) -> bool {
        self.evidence.candidates().contains_identity(identity)
    }

    pub fn covers_reclaimed_owner(&self, owner: PhysicalGenerationOwner) -> bool {
        self.evidence.candidates().contains_owner(owner)
    }

    pub fn lower_for_s6_reclaim_policy(
        &self,
        region: PhysicalReclaimRegion,
    ) -> Result<S6ReclaimReachabilityRemovalEvidence, S6ReclaimReachabilityRemovalEvidenceDenial>
    {
        let owner = region.reference().generation_owner();
        if !self.covers_reclaimed_owner(owner) {
            return Err(S6ReclaimReachabilityRemovalEvidenceDenial::OwnerNotCoveredByReachability);
        }
        S6ReclaimReachabilityRemovalEvidence::from_reclaim_reachability_removal_receipt(
            region,
            self.evidence.root_epoch().get(),
            self.evidence.candidates().candidate_ranges().len() as u32,
        )
    }

    pub const fn counters(&self) -> ReclaimCounterSnapshot {
        self.counters
    }
}

impl S6ReclaimReachabilityRemovalEvidence {
    fn from_reclaim_reachability_removal_receipt(
        region: PhysicalReclaimRegion,
        root_epoch: u64,
        protected_ranges: u32,
    ) -> Result<Self, S6ReclaimReachabilityRemovalEvidenceDenial> {
        if protected_ranges == 0 {
            return Err(S6ReclaimReachabilityRemovalEvidenceDenial::EmptyProtectedReachability);
        }
        Ok(Self {
            region,
            root_epoch,
            protected_ranges,
        })
    }

    pub const fn region(self) -> PhysicalReclaimRegion {
        self.region
    }

    pub const fn root_epoch(self) -> u64 {
        self.root_epoch
    }

    pub const fn protected_ranges(self) -> u32 {
        self.protected_ranges
    }
}

impl ReclaimDecision {
    pub const fn blocked_report(&self) -> Option<BlockedReclaimReport> {
        match self {
            Self::Eligible => None,
            Self::Blocked(report) => Some(*report),
        }
    }

    pub const fn is_eligible(&self) -> bool {
        matches!(self, Self::Eligible)
    }
}

impl BlockedReclaimReport {
    const fn denial(self) -> ReclaimDenial {
        ReclaimDenial::BlockedByLiveHazardLease {
            slot: self.overlap.slot(),
            generation: self.overlap.generation(),
            kind: self.overlap.kind(),
            overlapping_ranges: self.overlap.overlapping_ranges(),
        }
    }

    pub const fn slot(self) -> crate::HazardLeaseSlot {
        self.overlap.slot()
    }

    pub const fn generation(self) -> crate::HazardLeaseGeneration {
        self.overlap.generation()
    }

    pub const fn overlapping_ranges(self) -> u64 {
        self.overlap.overlapping_ranges()
    }

    pub const fn candidate_ranges(self) -> u64 {
        self.candidate_ranges
    }
}
