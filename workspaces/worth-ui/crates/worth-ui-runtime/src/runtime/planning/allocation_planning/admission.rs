use crate::evidence::UiMeasurementBasis;
use crate::runtime::planning::allocation_planning::WorthUiAllocationPlanningProjection;
use crate::runtime::WorthUiPendingActivation;

#[derive(Clone, Debug)]
pub(crate) struct WorthUiAllocationPlanningAdmission {
    constraint_basis: crate::graph::UiAdmittedAllocationConstraintBasis,
    portal_allocation_input: Option<crate::runtime::UiPortalAllocationPlanningBasis>,
    projection: WorthUiAllocationPlanningProjection,
}

impl WorthUiAllocationPlanningAdmission {
    pub(crate) fn constraint_basis(&self) -> &crate::graph::UiAdmittedAllocationConstraintBasis {
        &self.constraint_basis
    }

    pub(crate) fn from_pending_activation(
        pending_activation: &WorthUiPendingActivation,
        constraint_basis: crate::graph::UiAdmittedAllocationConstraintBasis,
    ) -> Self {
        Self {
            constraint_basis,
            portal_allocation_input: None,
            projection: pending_activation.allocation_planning_projection().clone(),
        }
    }

    pub(crate) fn from_projection(
        projection: WorthUiAllocationPlanningProjection,
        constraint_basis: crate::graph::UiAdmittedAllocationConstraintBasis,
        portal_allocation_input: Option<crate::runtime::UiPortalAllocationPlanningBasis>,
    ) -> Self {
        Self {
            constraint_basis,
            portal_allocation_input,
            projection,
        }
    }

    pub(crate) fn measurement_basis(&self) -> &UiMeasurementBasis {
        self.constraint_basis.measurement_basis()
    }

    pub(crate) fn portal_allocation_input(
        &self,
    ) -> Option<&crate::runtime::UiPortalAllocationPlanningBasis> {
        self.portal_allocation_input.as_ref()
    }

    pub(crate) fn into_projection(self) -> WorthUiAllocationPlanningProjection {
        self.projection
    }
}
