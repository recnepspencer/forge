#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanPostAdmissionNormalizationCounters {
    examined_admitted_regions: usize,
    examined_boundary_only_outcomes: usize,
    admitted_canonical_rows: usize,
    denied_canonical_rows: usize,
}

impl PlanarBooleanPostAdmissionNormalizationCounters {
    pub(crate) fn examined_admitted_region(&mut self) {
        self.examined_admitted_regions += 1;
    }

    pub(crate) fn examined_boundary_only_outcome(&mut self) {
        self.examined_boundary_only_outcomes += 1;
    }

    pub(crate) fn admitted_canonical_row(&mut self) {
        self.admitted_canonical_rows += 1;
    }

    pub(crate) fn denied_canonical_row(&mut self) {
        self.denied_canonical_rows += 1;
    }
}
