#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanSplitPointAdmissionCounters {
    inspected_point_candidates: usize,
    admitted_point_candidates: usize,
    endpoint_candidates: usize,
    interior_candidates: usize,
    rejected_out_of_domain_points: usize,
}

impl PlanarBooleanSplitPointAdmissionCounters {
    pub(crate) fn new(
        inspected_point_candidates: usize,
        admitted_point_candidates: usize,
        endpoint_candidates: usize,
        interior_candidates: usize,
        rejected_out_of_domain_points: usize,
    ) -> Self {
        Self {
            inspected_point_candidates,
            admitted_point_candidates,
            endpoint_candidates,
            interior_candidates,
            rejected_out_of_domain_points,
        }
    }

    pub fn inspected_point_candidates(self) -> usize {
        self.inspected_point_candidates
    }

    pub fn admitted_point_candidates(self) -> usize {
        self.admitted_point_candidates
    }

    pub fn endpoint_candidates(self) -> usize {
        self.endpoint_candidates
    }

    pub fn interior_candidates(self) -> usize {
        self.interior_candidates
    }

    pub fn rejected_out_of_domain_points(self) -> usize {
        self.rejected_out_of_domain_points
    }
}
