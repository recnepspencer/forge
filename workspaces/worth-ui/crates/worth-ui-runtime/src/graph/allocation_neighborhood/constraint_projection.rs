use crate::evidence::{
    UiAllocationConstraintSet, UiAllocationNeighborhood, UiConstraintPropagationDenial,
    UiMeasurementBasis,
};

use super::constraint_authority::admit_constraint_set;

impl UiMeasurementBasis {
    pub(crate) fn admit_allocation_constraint_set(
        &self,
        neighborhood: &UiAllocationNeighborhood,
    ) -> Result<UiAllocationConstraintSet, UiConstraintPropagationDenial> {
        admit_constraint_set(self, neighborhood)
    }
}
