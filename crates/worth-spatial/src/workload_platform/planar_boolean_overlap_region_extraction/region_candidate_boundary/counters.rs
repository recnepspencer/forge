#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionCandidateBoundaryCounters {
    promoted_candidates: usize,
    denied_candidates: usize,
    admitted_overlap_regions: usize,
    boundary_only_outcomes: usize,
    examined_shared_area_outcomes: usize,
}

impl PlanarBooleanOverlapRegionCandidateBoundaryCounters {
    pub(crate) fn promoted_candidate(&mut self) {
        self.promoted_candidates += 1;
    }

    pub(crate) fn denied_candidate(&mut self) {
        self.denied_candidates += 1;
    }

    pub(crate) fn admitted_overlap_region(&mut self) {
        self.admitted_overlap_regions += 1;
    }

    pub(crate) fn boundary_only_outcome(&mut self) {
        self.boundary_only_outcomes += 1;
    }

    pub(crate) fn examined_shared_area_outcome(&mut self) {
        self.examined_shared_area_outcomes += 1;
    }
}
