#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanRawEdgeSplitScheduleCounters {
    source_edge_schedules: usize,
    point_entries: usize,
    interval_entries: usize,
    t_junction_entries: usize,
    shared_endpoint_noop_entries: usize,
    endpoint_noop_entries: usize,
    source_event_groups: usize,
}

impl PlanarBooleanRawEdgeSplitScheduleCounters {
    pub(crate) fn new(
        source_edge_schedules: usize,
        point_entries: usize,
        interval_entries: usize,
        t_junction_entries: usize,
        shared_endpoint_noop_entries: usize,
        endpoint_noop_entries: usize,
        source_event_groups: usize,
    ) -> Self {
        Self {
            source_edge_schedules,
            point_entries,
            interval_entries,
            t_junction_entries,
            shared_endpoint_noop_entries,
            endpoint_noop_entries,
            source_event_groups,
        }
    }

    pub fn source_edge_schedules(self) -> usize {
        self.source_edge_schedules
    }

    pub fn point_entries(self) -> usize {
        self.point_entries
    }

    pub fn interval_entries(self) -> usize {
        self.interval_entries
    }

    pub fn t_junction_entries(self) -> usize {
        self.t_junction_entries
    }

    pub fn shared_endpoint_noop_entries(self) -> usize {
        self.shared_endpoint_noop_entries
    }

    pub fn endpoint_noop_entries(self) -> usize {
        self.endpoint_noop_entries
    }

    pub fn source_event_groups(self) -> usize {
        self.source_event_groups
    }
}
