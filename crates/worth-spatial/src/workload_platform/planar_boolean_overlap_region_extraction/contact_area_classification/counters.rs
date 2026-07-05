#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanBoundaryContactClassificationCounters {
    admitted_shared_boundary_contact_outcomes: usize,
    admitted_pure_boundary_only_outcomes: usize,
    denied_classifications: usize,
}

impl PlanarBooleanBoundaryContactClassificationCounters {
    pub(crate) fn admitted_shared_boundary_contact_outcome(&mut self) {
        self.admitted_shared_boundary_contact_outcomes += 1;
    }

    pub(crate) fn admitted_pure_boundary_only_outcome(&mut self) {
        self.admitted_pure_boundary_only_outcomes += 1;
    }

    pub(crate) fn denied_classification(&mut self) {
        self.denied_classifications += 1;
    }
}
