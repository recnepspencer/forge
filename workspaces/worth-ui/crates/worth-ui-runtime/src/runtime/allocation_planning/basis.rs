use crate::evidence::{UiAllocationConstraintSet, UiAllocationNeighborhood, UiMeasurementBasis};
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiAllocationPlanningBasis {
    admitted: Option<Rc<crate::graph::UiAllocationConstraintProvenance>>,
    denied_measurement_basis: Option<Rc<UiMeasurementBasis>>,
    denied_allocation_neighborhood: Option<Rc<UiAllocationNeighborhood>>,
    portal_allocation_input: Option<crate::runtime::UiPortalAllocationPlanningBasis>,
}

impl WorthUiAllocationPlanningBasis {
    #[cfg(test)]
    pub(crate) fn new(
        measurement_basis: UiMeasurementBasis,
        allocation_neighborhood: UiAllocationNeighborhood,
        portal_allocation_input: Option<crate::runtime::UiPortalAllocationPlanningBasis>,
    ) -> Self {
        Self {
            admitted: None,
            denied_measurement_basis: Some(Rc::new(measurement_basis)),
            denied_allocation_neighborhood: Some(Rc::new(allocation_neighborhood)),
            portal_allocation_input,
        }
    }
    pub(crate) fn denied(
        measurement_basis: UiMeasurementBasis,
        allocation_neighborhood: UiAllocationNeighborhood,
    ) -> Self {
        Self {
            admitted: None,
            denied_measurement_basis: Some(Rc::new(measurement_basis)),
            denied_allocation_neighborhood: Some(Rc::new(allocation_neighborhood)),
            portal_allocation_input: None,
        }
    }

    pub(crate) fn from_admitted(
        admitted: crate::graph::UiAdmittedAllocationConstraintBasis,
        portal_allocation_input: Option<crate::runtime::UiPortalAllocationPlanningBasis>,
    ) -> Self {
        Self {
            admitted: Some(admitted.into_provenance()),
            denied_measurement_basis: None,
            denied_allocation_neighborhood: None,
            portal_allocation_input,
        }
    }
    pub(crate) fn scroll_authority(&self) -> Option<&crate::graph::UiGraphScrollPlanningAuthority> {
        self.admitted
            .as_ref()
            .and_then(|admitted| admitted.scroll_authority())
    }

    pub fn measurement_basis(&self) -> &UiMeasurementBasis {
        self.admitted.as_ref().map_or_else(
            || {
                &**self
                    .denied_measurement_basis
                    .as_ref()
                    .expect("denied planning basis retains measurement basis")
            },
            |admitted| admitted.measurement_basis(),
        )
    }

    pub fn allocation_neighborhood(&self) -> &UiAllocationNeighborhood {
        self.admitted.as_ref().map_or_else(
            || {
                &**self
                    .denied_allocation_neighborhood
                    .as_ref()
                    .expect("denied planning basis retains neighborhood")
            },
            |admitted| admitted.neighborhood(),
        )
    }

    pub fn allocation_constraint_set(&self) -> Option<&UiAllocationConstraintSet> {
        self.admitted
            .as_ref()
            .map(|admitted| admitted.constraint_set())
    }

    pub fn portal_allocation_input(
        &self,
    ) -> Option<&crate::runtime::UiPortalAllocationPlanningBasis> {
        self.portal_allocation_input.as_ref()
    }

    pub(crate) fn invalidation_authority_parts(
        &self,
    ) -> (Rc<UiMeasurementBasis>, Rc<UiAllocationNeighborhood>) {
        self.admitted.as_ref().map_or_else(
            || {
                (
                    self.denied_measurement_basis
                        .as_ref()
                        .expect("denied basis")
                        .clone(),
                    self.denied_allocation_neighborhood
                        .as_ref()
                        .expect("denied neighborhood")
                        .clone(),
                )
            },
            |admitted| admitted.structural_parts(),
        )
    }
}
