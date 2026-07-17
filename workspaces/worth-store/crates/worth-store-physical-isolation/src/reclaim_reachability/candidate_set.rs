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
    owners: Vec<worth_store_physical_format::PhysicalGenerationOwner>,
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
        let mut owners = Vec::new();
        owners
            .try_reserve_exact(footprint.references().len())
            .map_err(|_| ReclaimDenial::AllocationFailed)?;
        owners.extend(
            footprint
                .references()
                .iter()
                .map(|reference| reference.owner()),
        );
        owners.sort_unstable();
        owners.dedup();
        Ok(Self {
            root_epoch: root.epoch(),
            basis: footprint.declared_footprint_basis(),
            ranges: footprint.ranges().clone(),
            owners,
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

    pub(crate) fn candidate_owners(
        &self,
    ) -> &[worth_store_physical_format::PhysicalGenerationOwner] {
        &self.owners
    }

    pub fn contains_identity(&self, identity: CurrentGenerationPhysicalReference) -> bool {
        self.ranges.contains_owner(identity.owner())
    }

    pub fn contains_owner(
        &self,
        owner: worth_store_physical_format::PhysicalGenerationOwner,
    ) -> bool {
        self.ranges.contains_owner(owner)
    }

    #[cfg(any(test, feature = "certification-authority"))]
    pub(crate) fn for_certification_test() -> Self {
        Self {
            root_epoch: crate::epoch::root_epoch_from_entry_seed(17),
            basis: PhysicalReadProtectedFootprintBasis::for_certification_test(1),
            ranges: ProtectedReferenceRangeSet::for_certification_test(),
            owners: Vec::new(),
        }
    }

    #[cfg(any(test, feature = "certification-authority"))]
    pub(crate) fn for_certification_reference(
        reference: CurrentGenerationPhysicalReference,
    ) -> Self {
        let protected = [crate::ProtectedPhysicalReference::from_current_generation(
            reference,
        )];
        Self {
            root_epoch: crate::epoch::root_epoch_from_entry_seed(17),
            basis: PhysicalReadProtectedFootprintBasis::for_certification_test(1),
            ranges: ProtectedReferenceRangeSet::from_references(&protected, Vec::new()),
            owners: vec![reference.owner()],
        }
    }
}
