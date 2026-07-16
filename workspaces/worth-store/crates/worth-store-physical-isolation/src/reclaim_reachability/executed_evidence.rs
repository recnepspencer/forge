use crate::{PhysicalReadProtectedFootprintBasis, ReleasedOldReachability, RootEpoch};

use super::{ReclaimCandidateSet, ReclaimDenial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutedReachabilityEvidence {
    root_epoch: RootEpoch,
    footprint_basis: PhysicalReadProtectedFootprintBasis,
    candidates: ReclaimCandidateSet,
}

impl ExecutedReachabilityEvidence {
    pub fn from_released_old_reachability(
        released: ReleasedOldReachability,
        candidates: ReclaimCandidateSet,
    ) -> Result<Self, ReclaimDenial> {
        let root_epoch = released.release_receipt().root_epoch();
        let footprint_basis = released.footprint_basis();
        if footprint_basis != candidates.footprint_basis() {
            return Err(ReclaimDenial::CandidateDoesNotMatchExecutedReachability {
                executed: footprint_basis,
                candidate: candidates.footprint_basis(),
            });
        }
        if !root_epoch.has_same_epoch_value(candidates.root_epoch()) {
            return Err(
                ReclaimDenial::CandidateRootDoesNotMatchExecutedReachability {
                    executed: root_epoch,
                    candidate: candidates.root_epoch(),
                },
            );
        }
        Ok(Self {
            root_epoch,
            footprint_basis,
            candidates,
        })
    }

    pub const fn root_epoch(&self) -> RootEpoch {
        self.root_epoch
    }

    pub const fn footprint_basis(&self) -> PhysicalReadProtectedFootprintBasis {
        self.footprint_basis
    }

    pub const fn candidates(&self) -> &ReclaimCandidateSet {
        &self.candidates
    }

    #[cfg(any(test, feature = "certification-authority"))]
    pub(crate) fn for_certification_test() -> Self {
        let candidates = ReclaimCandidateSet::for_certification_test();
        Self {
            root_epoch: candidates.root_epoch(),
            footprint_basis: candidates.footprint_basis(),
            candidates,
        }
    }

    #[cfg(any(test, feature = "certification-authority"))]
    pub fn for_certification_reference(
        reference: crate::CurrentGenerationPhysicalReference,
    ) -> Self {
        let candidates = ReclaimCandidateSet::for_certification_reference(reference);
        Self {
            root_epoch: candidates.root_epoch(),
            footprint_basis: candidates.footprint_basis(),
            candidates,
        }
    }
}
