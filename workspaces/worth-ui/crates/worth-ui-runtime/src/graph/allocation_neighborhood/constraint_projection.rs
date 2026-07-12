use crate::evidence::{
    UiAllocationConstraintSet, UiAllocationNeighborhood, UiConstraintPropagationDenial,
    UiMeasurementBasis,
};

#[cfg(test)]
use super::constraint_authority::admit_constraint_set;

impl UiMeasurementBasis {
    pub(crate) fn admit_allocation_constraint_basis(
        &self,
        neighborhood: &UiAllocationNeighborhood,
    ) -> Result<crate::graph::UiAdmittedAllocationConstraintBasis, UiConstraintPropagationDenial>
    {
        super::constraint_authority::admit_constraint_basis(self, neighborhood)
    }
    pub(crate) fn admit_allocation_constraint_basis_with_portal(
        &self,
        neighborhood: &UiAllocationNeighborhood,
        portal: &crate::runtime::UiPortalAllocationPlanningBasis,
    ) -> Result<crate::graph::UiAdmittedAllocationConstraintBasis, UiConstraintPropagationDenial>
    {
        super::constraint_authority::admit_constraint_basis_with_portal(self, neighborhood, portal)
    }
    #[cfg(test)]
    pub(crate) fn admit_allocation_constraint_set(
        &self,
        neighborhood: &UiAllocationNeighborhood,
    ) -> Result<UiAllocationConstraintSet, UiConstraintPropagationDenial> {
        admit_constraint_set(self, neighborhood)
    }

    #[cfg(test)]
    pub(crate) fn admit_allocation_constraint_set_with_portal(
        &self,
        neighborhood: &UiAllocationNeighborhood,
        portal: &crate::runtime::UiPortalAllocationPlanningBasis,
    ) -> Result<UiAllocationConstraintSet, UiConstraintPropagationDenial> {
        super::constraint_authority::admit_constraint_set_with_portal(self, neighborhood, portal)
    }
}
