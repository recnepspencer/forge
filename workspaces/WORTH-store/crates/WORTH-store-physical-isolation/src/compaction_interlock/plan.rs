use super::{
    CompactionCandidateRangeSet, CompactionProtectedReferenceSet, CompactionReadInterlockCounters,
    CompactionReadInterlockDenial,
};
use crate::{
    CurrentPhysicalRoot, PhysicalReadProtectedFootprintBasis, RootEpoch, StablePhysicalReadReceipt,
};
use worth_store_physical_integrity::CompactionSourceIntegrityClearance;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CompactionSourceEvidencePosture {
    IntegrityClearedStableRead {
        root: CurrentPhysicalRoot,
        footprint: PhysicalReadProtectedFootprintBasis,
        clearance: CompactionSourceIntegrityClearance,
    },
    Quarantined(CompactionSourceIntegrityClearance),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompactionSourceIntegrityEvidence {
    posture: CompactionSourceEvidencePosture,
}

impl CompactionSourceIntegrityEvidence {
    pub fn from_stable_read_receipt_and_integrity_clearance(
        receipt: StablePhysicalReadReceipt,
        clearance: CompactionSourceIntegrityClearance,
    ) -> Result<Self, CompactionReadInterlockDenial> {
        let release = receipt.read_plan_release();
        if release.protected_references_released() == 0 {
            return Err(CompactionReadInterlockDenial::EmptyCandidateRangeSet);
        }
        if !clearance.permits_compaction_movement() {
            return Ok(Self {
                posture: CompactionSourceEvidencePosture::Quarantined(clearance),
            });
        }
        if clearance.inspected_bytes() == 0 {
            return Err(CompactionReadInterlockDenial::SourceEvidenceMismatch);
        }
        Ok(Self {
            posture: CompactionSourceEvidencePosture::IntegrityClearedStableRead {
                root: release.root(),
                footprint: release.footprint_basis(),
                clearance,
            },
        })
    }

    pub const fn from_quarantine_clearance(clearance: CompactionSourceIntegrityClearance) -> Self {
        Self {
            posture: CompactionSourceEvidencePosture::Quarantined(clearance),
        }
    }

    const fn posture(self) -> CompactionSourceEvidencePosture {
        self.posture
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactionReadInterlockPlan {
    protected: CompactionProtectedReferenceSet,
    candidates: CompactionCandidateRangeSet,
    source_epoch: RootEpoch,
    target_epoch: RootEpoch,
    counters: CompactionReadInterlockCounters,
    reclaim_deferred: bool,
}

impl CompactionReadInterlockPlan {
    pub fn admit(
        protected: CompactionProtectedReferenceSet,
        candidates: CompactionCandidateRangeSet,
        source_epoch: RootEpoch,
        target_epoch: RootEpoch,
        integrity: CompactionSourceIntegrityEvidence,
    ) -> Result<Self, CompactionReadInterlockDenial> {
        if candidates.candidate_references() == 0 {
            return Err(CompactionReadInterlockDenial::EmptyCandidateRangeSet);
        }
        match integrity.posture() {
            CompactionSourceEvidencePosture::Quarantined(_) => {
                return Err(CompactionReadInterlockDenial::QuarantinedCandidateRange);
            }
            CompactionSourceEvidencePosture::IntegrityClearedStableRead {
                root,
                footprint,
                clearance,
            } => {
                if !clearance.permits_compaction_movement() {
                    return Err(CompactionReadInterlockDenial::QuarantinedCandidateRange);
                }
                let Some(owner) = clearance.locality_owner() else {
                    return Err(CompactionReadInterlockDenial::SourceEvidenceMismatch);
                };
                if !protected.contains_owner(owner) {
                    return Err(CompactionReadInterlockDenial::SourceEvidenceMismatch);
                }
                if !candidates.is_fully_covered_by_owner(owner) {
                    return Err(CompactionReadInterlockDenial::SourceEvidenceMismatch);
                }
                if root != protected.root() || footprint != protected.footprint_basis() {
                    return Err(CompactionReadInterlockDenial::SourceEvidenceMismatch);
                }
            }
        }
        if protected.root().epoch() != source_epoch {
            return Err(CompactionReadInterlockDenial::StaleCompactionSourceEpoch {
                expected: protected.root().epoch(),
                observed: source_epoch,
            });
        }
        if target_epoch.get() <= source_epoch.get() {
            return Err(CompactionReadInterlockDenial::StaleEpochReuse {
                source_epoch,
                reused_epoch: target_epoch,
            });
        }
        let counters = candidates.intersect_protected(&protected);
        let reclaim_deferred = counters.overlapping_ranges() > 0;
        Ok(Self {
            protected,
            candidates,
            source_epoch,
            target_epoch,
            counters,
            reclaim_deferred,
        })
    }

    pub fn deny_in_place_overwrite(
        self,
    ) -> (
        CompactionReadInterlockDenial,
        CompactionReadInterlockCounters,
    ) {
        (
            CompactionReadInterlockDenial::InPlaceOverwriteOfProtectedStructure,
            self.counters.with_denied_in_place_overwrite(),
        )
    }

    pub const fn protected(&self) -> &CompactionProtectedReferenceSet {
        &self.protected
    }

    pub const fn candidates(&self) -> &CompactionCandidateRangeSet {
        &self.candidates
    }

    pub const fn source_epoch(&self) -> RootEpoch {
        self.source_epoch
    }

    pub const fn target_epoch(&self) -> RootEpoch {
        self.target_epoch
    }

    pub const fn counters(&self) -> CompactionReadInterlockCounters {
        self.counters
    }

    pub const fn reclaim_deferred(&self) -> bool {
        self.reclaim_deferred
    }

    #[cfg(any(test, feature = "certification-authority"))]
    pub fn for_certification_test() -> Self {
        let read_plan = crate::stable_physical_read_plan_for_certification_test(64);
        let protected = CompactionProtectedReferenceSet::from_read_plan(&read_plan);
        let candidates =
            CompactionCandidateRangeSet::from_protected_set_for_certification_test(&protected);
        let source_epoch = protected.root().epoch();
        let target_epoch = crate::RootEpoch::from_admitted_physical_basis(source_epoch.get() + 1);
        let counters = candidates.intersect_protected(&protected);
        Self {
            protected,
            candidates,
            source_epoch,
            target_epoch,
            counters,
            reclaim_deferred: counters.overlapping_ranges() > 0,
        }
    }
}

#[cfg(any(test, feature = "certification-authority"))]
pub fn compaction_read_interlock_plan_for_certification_test() -> CompactionReadInterlockPlan {
    CompactionReadInterlockPlan::for_certification_test()
}
