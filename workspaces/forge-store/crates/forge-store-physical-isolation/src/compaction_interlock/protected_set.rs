use crate::{
    CurrentPhysicalRoot, PhysicalReadProtectedFootprintBasis, ProtectedReferenceRangeSet,
    StablePhysicalReadPlan,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactionProtectedReferenceSet {
    root: CurrentPhysicalRoot,
    footprint_basis: PhysicalReadProtectedFootprintBasis,
    ranges: ProtectedReferenceRangeSet,
    owners: Vec<forge_store_physical_format::PhysicalGenerationOwner>,
}

impl CompactionProtectedReferenceSet {
    pub fn from_read_plan(plan: &StablePhysicalReadPlan) -> Self {
        let owners = plan
            .footprint()
            .protected()
            .references()
            .iter()
            .map(|reference| reference.owner())
            .collect();
        Self {
            root: plan.root(),
            footprint_basis: plan.footprint().declared_footprint_basis(),
            ranges: plan.footprint().protected().ranges().clone(),
            owners,
        }
    }

    pub const fn root(&self) -> CurrentPhysicalRoot {
        self.root
    }

    pub const fn footprint_basis(&self) -> PhysicalReadProtectedFootprintBasis {
        self.footprint_basis
    }

    pub const fn ranges(&self) -> &ProtectedReferenceRangeSet {
        &self.ranges
    }

    pub fn contains_owner(
        &self,
        owner: forge_store_physical_format::PhysicalGenerationOwner,
    ) -> bool {
        self.owners.contains(&owner)
    }

    #[cfg(any(test, feature = "certification-authority"))]
    pub(super) fn first_owner(
        &self,
    ) -> Option<forge_store_physical_format::PhysicalGenerationOwner> {
        self.owners.first().copied()
    }
}
