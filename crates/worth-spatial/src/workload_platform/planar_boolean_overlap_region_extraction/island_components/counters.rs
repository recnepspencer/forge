#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanOverlapIslandComponentCounters {
    admitted_candidates: usize,
    admitted_islands: usize,
    admitted_boundary_contact_components: usize,
    admitted_area_overlap_components: usize,
    denied_partitions: usize,
}

impl PlanarBooleanOverlapIslandComponentCounters {
    pub(crate) fn admitted_candidate(&mut self) {
        self.admitted_candidates += 1;
    }

    pub(crate) fn admitted_island(&mut self) {
        self.admitted_islands += 1;
    }

    pub(crate) fn admitted_boundary_contact_component(&mut self) {
        self.admitted_boundary_contact_components += 1;
    }

    pub(crate) fn admitted_area_overlap_component(&mut self) {
        self.admitted_area_overlap_components += 1;
    }

    pub(crate) fn denied_partition(&mut self) {
        self.denied_partitions += 1;
    }
}
