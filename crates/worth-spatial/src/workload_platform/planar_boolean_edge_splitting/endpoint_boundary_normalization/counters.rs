#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanEndpointBoundaryNormalizationCounters {
    normalized_schedules: usize,
    inspected_point_cuts: usize,
    fragment_point_cuts: usize,
    endpoint_noop_decisions: usize,
    shared_endpoint_decisions: usize,
    t_junction_boundary_decisions: usize,
    retained_interval_entries: usize,
}

impl PlanarBooleanEndpointBoundaryNormalizationCounters {
    pub(crate) fn new(
        normalized_schedules: usize,
        inspected_point_cuts: usize,
        fragment_point_cuts: usize,
        endpoint_noop_decisions: usize,
        shared_endpoint_decisions: usize,
        t_junction_boundary_decisions: usize,
        retained_interval_entries: usize,
    ) -> Self {
        Self {
            normalized_schedules,
            inspected_point_cuts,
            fragment_point_cuts,
            endpoint_noop_decisions,
            shared_endpoint_decisions,
            t_junction_boundary_decisions,
            retained_interval_entries,
        }
    }

    pub fn normalized_schedules(self) -> usize {
        self.normalized_schedules
    }
    pub fn inspected_point_cuts(self) -> usize {
        self.inspected_point_cuts
    }
    pub fn fragment_point_cuts(self) -> usize {
        self.fragment_point_cuts
    }
    pub fn endpoint_noop_decisions(self) -> usize {
        self.endpoint_noop_decisions
    }
    pub fn shared_endpoint_decisions(self) -> usize {
        self.shared_endpoint_decisions
    }
    pub fn t_junction_boundary_decisions(self) -> usize {
        self.t_junction_boundary_decisions
    }
    pub fn retained_interval_entries(self) -> usize {
        self.retained_interval_entries
    }
}
