#[derive(Debug)]
pub(crate) struct UiAdmittedAllocationConstraintBasis {
    provenance: std::rc::Rc<UiAllocationConstraintProvenance>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct UiAllocationConstraintProvenance {
    measurement_basis: std::rc::Rc<crate::evidence::UiMeasurementBasis>,
    neighborhood: std::rc::Rc<crate::evidence::UiAllocationNeighborhood>,
    constraint_set: crate::evidence::UiAllocationConstraintSet,
    scroll_authority: Option<super::UiGraphScrollPlanningAuthority>,
}

impl UiAdmittedAllocationConstraintBasis {
    pub(super) fn seal(
        measurement_basis: &crate::evidence::UiMeasurementBasis,
        neighborhood: &crate::evidence::UiAllocationNeighborhood,
        constraint_set: crate::evidence::UiAllocationConstraintSet,
        scroll_authority: Option<super::UiGraphScrollPlanningAuthority>,
    ) -> Self {
        Self { provenance: std::rc::Rc::new(UiAllocationConstraintProvenance {
            measurement_basis: std::rc::Rc::new(measurement_basis.clone()),
            neighborhood: std::rc::Rc::new(neighborhood.clone()),
            constraint_set,
            scroll_authority,
        }) }
    }
    pub(crate) fn measurement_basis(&self) -> &crate::evidence::UiMeasurementBasis {
        &self.provenance.measurement_basis
    }
    pub(crate) fn neighborhood(&self) -> &crate::evidence::UiAllocationNeighborhood {
        &self.provenance.neighborhood
    }
    pub(crate) fn constraint_set(&self) -> &crate::evidence::UiAllocationConstraintSet {
        &self.provenance.constraint_set
    }
    pub(crate) fn scroll_authority(&self) -> Option<&super::UiGraphScrollPlanningAuthority> {
        self.provenance.scroll_authority.as_ref()
    }
    pub(crate) fn into_provenance(self) -> std::rc::Rc<UiAllocationConstraintProvenance> {
        self.provenance
    }
}

impl UiAllocationConstraintProvenance {
    pub(crate) fn measurement_basis(&self) -> &crate::evidence::UiMeasurementBasis { &self.measurement_basis }
    pub(crate) fn neighborhood(&self) -> &crate::evidence::UiAllocationNeighborhood { &self.neighborhood }
    pub(crate) fn constraint_set(&self) -> &crate::evidence::UiAllocationConstraintSet { &self.constraint_set }
    pub(crate) fn scroll_authority(&self) -> Option<&super::UiGraphScrollPlanningAuthority> { self.scroll_authority.as_ref() }
    pub(crate) fn structural_parts(&self) -> (std::rc::Rc<crate::evidence::UiMeasurementBasis>, std::rc::Rc<crate::evidence::UiAllocationNeighborhood>) {
        (self.measurement_basis.clone(), self.neighborhood.clone())
    }
}
