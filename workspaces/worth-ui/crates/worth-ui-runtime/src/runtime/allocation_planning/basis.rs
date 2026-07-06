use crate::evidence::{UiAllocationConstraintSet, UiAllocationNeighborhood, UiMeasurementBasis};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiAllocationPlanningBasis {
    measurement_basis: UiMeasurementBasis,
    allocation_neighborhood: UiAllocationNeighborhood,
    allocation_constraint_set: Option<UiAllocationConstraintSet>,
}

impl WorthUiAllocationPlanningBasis {
    pub(crate) fn new(
        measurement_basis: UiMeasurementBasis,
        allocation_neighborhood: UiAllocationNeighborhood,
        allocation_constraint_set: Option<UiAllocationConstraintSet>,
    ) -> Self {
        Self {
            measurement_basis,
            allocation_neighborhood,
            allocation_constraint_set,
        }
    }

    pub fn measurement_basis(&self) -> &UiMeasurementBasis {
        &self.measurement_basis
    }

    pub fn allocation_neighborhood(&self) -> &UiAllocationNeighborhood {
        &self.allocation_neighborhood
    }

    pub fn allocation_constraint_set(&self) -> Option<&UiAllocationConstraintSet> {
        self.allocation_constraint_set.as_ref()
    }
}
