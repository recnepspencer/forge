use crate::{
    CompactProtectedReferenceSet, CurrentGenerationPhysicalReference, CurrentPhysicalRoot,
    PhysicalReadProtectedFootprintBasis, ProtectedReferenceRange, ProtectedReferenceRangeSet,
    ReleasedOldReachability, RootEpoch,
};

use super::ReclaimDenial;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReclaimCandidateSet {
    root_epoch: RootEpoch,
    basis: PhysicalReadProtectedFootprintBasis,
    ranges: ProtectedReferenceRangeSet,
}

impl ReclaimCandidateSet {
    pub fn from_released_old_reachability(
        released: ReleasedOldReachability,
        footprint: &CompactProtectedReferenceSet,
    ) -> Result<Self, ReclaimDenial> {
        Self::from_protected_footprint(released.release_receipt().root(), footprint)
    }

    pub(crate) fn from_protected_footprint(
        root: CurrentPhysicalRoot,
        footprint: &CompactProtectedReferenceSet,
    ) -> Result<Self, ReclaimDenial> {
        if footprint.references().is_empty() || footprint.ranges().ranges().is_empty() {
            return Err(ReclaimDenial::MissingCandidateReachability);
        }
        Ok(Self {
            root_epoch: root.epoch(),
            basis: footprint.declared_footprint_basis(),
            ranges: footprint.ranges().clone(),
        })
    }

    pub(crate) const fn ranges(&self) -> &ProtectedReferenceRangeSet {
        &self.ranges
    }

    pub const fn footprint_basis(&self) -> PhysicalReadProtectedFootprintBasis {
        self.basis
    }

    pub const fn root_epoch(&self) -> RootEpoch {
        self.root_epoch
    }

    pub fn candidate_ranges(&self) -> &[ProtectedReferenceRange] {
        self.ranges.ranges()
    }

    pub fn contains_identity(&self, identity: CurrentGenerationPhysicalReference) -> bool {
        self.ranges.contains_owner(identity.owner())
    }

    pub fn contains_owner(
        &self,
        owner: forge_store_physical_format::PhysicalGenerationOwner,
    ) -> bool {
        self.ranges.contains_owner(owner)
    }
}
