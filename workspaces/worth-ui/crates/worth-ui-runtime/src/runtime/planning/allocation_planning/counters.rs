#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiAllocationPlanningCounters {
    planning_attempt_count: usize,
    candidate_projection_read_count: usize,
    measurement_basis_read_count: usize,
    denial_count: usize,
}

impl WorthUiAllocationPlanningCounters {
    pub(crate) fn record_planning_attempt(&mut self) {
        self.planning_attempt_count += 1;
    }

    pub(crate) fn record_candidate_projection_read(&mut self) {
        self.candidate_projection_read_count += 1;
    }

    pub(crate) fn record_measurement_basis_read(&mut self) {
        self.measurement_basis_read_count += 1;
    }

    pub(crate) fn record_denial(&mut self) {
        self.denial_count += 1;
    }

    pub fn planning_attempt_count(self) -> usize {
        self.planning_attempt_count
    }

    pub fn candidate_projection_read_count(self) -> usize {
        self.candidate_projection_read_count
    }

    pub fn measurement_basis_read_count(self) -> usize {
        self.measurement_basis_read_count
    }

    pub fn denial_count(self) -> usize {
        self.denial_count
    }
}
