#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanSharedAreaAdmissionCounters {
    admitted_shared_area_outcomes: usize,
    admitted_mixed_boundary_area_outcomes: usize,
    denied_admissions: usize,
}

impl PlanarBooleanSharedAreaAdmissionCounters {
    pub(crate) fn admitted_shared_area_outcome(&mut self) {
        self.admitted_shared_area_outcomes += 1;
    }

    pub(crate) fn admitted_mixed_boundary_area_outcome(&mut self) {
        self.admitted_mixed_boundary_area_outcomes += 1;
    }

    pub(crate) fn denied_admission(&mut self) {
        self.denied_admissions += 1;
    }
}
