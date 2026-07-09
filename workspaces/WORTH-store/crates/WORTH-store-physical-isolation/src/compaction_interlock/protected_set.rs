use crate::{
    CurrentPhysicalRoot, PhysicalReadProtectedFootprintBasis, ProtectedReferenceRangeSet,
    StablePhysicalReadPlan,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactionProtectedReferenceSet {
    root: CurrentPhysicalRoot,
    footprint_basis: PhysicalReadProtectedFootprintBasis,
    ranges: ProtectedReferenceRangeSet,
}

impl CompactionProtectedReferenceSet {
    pub fn from_read_plan(plan: &StablePhysicalReadPlan) -> Self {
        Self {
            root: plan.root(),
            footprint_basis: plan.footprint().declared_footprint_basis(),
            ranges: plan.footprint().protected().ranges().clone(),
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
        owner: worth_store_physical_format::PhysicalGenerationOwner,
    ) -> bool {
        self.ranges.contains_owner(owner)
    }
}
