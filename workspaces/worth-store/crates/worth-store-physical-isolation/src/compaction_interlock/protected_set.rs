use crate::{
    CurrentGenerationPhysicalReference, CurrentPhysicalRoot, PhysicalReadProtectedFootprintBasis,
    ProtectedReferenceRangeSet, StablePhysicalReadPlan,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactionProtectedReferenceSet {
    root: CurrentPhysicalRoot,
    footprint_basis: PhysicalReadProtectedFootprintBasis,
    ranges: ProtectedReferenceRangeSet,
    references: Vec<CurrentGenerationPhysicalReference>,
    owners: Vec<worth_store_physical_format::PhysicalGenerationOwner>,
}

impl CompactionProtectedReferenceSet {
    pub fn from_read_plan(plan: &StablePhysicalReadPlan) -> Self {
        let references = plan
            .footprint()
            .protected()
            .references()
            .iter()
            .map(|reference| reference.current_generation())
            .collect::<Vec<_>>();
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
            references,
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

    pub fn references(&self) -> &[CurrentGenerationPhysicalReference] {
        &self.references
    }

    pub fn contains_owner(
        &self,
        owner: worth_store_physical_format::PhysicalGenerationOwner,
    ) -> bool {
        self.owners.contains(&owner)
    }
}
