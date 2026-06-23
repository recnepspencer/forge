#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanEdgeSplitScopeAdmissionCounters {
    scope_admission_count: usize,
    split_request_count: usize,
    source_carrier_count: usize,
    point_event_count: usize,
    interval_event_count: usize,
    event_group_count: usize,
    policy_outcome_count: usize,
}

impl PlanarBooleanEdgeSplitScopeAdmissionCounters {
    pub(crate) fn new(
        source_carrier_count: usize,
        point_event_count: usize,
        interval_event_count: usize,
        event_group_count: usize,
        policy_outcome_count: usize,
    ) -> Self {
        Self {
            scope_admission_count: 1,
            split_request_count: 1,
            source_carrier_count,
            point_event_count,
            interval_event_count,
            event_group_count,
            policy_outcome_count,
        }
    }

    pub fn scope_admission_count(self) -> usize {
        self.scope_admission_count
    }

    pub fn split_request_count(self) -> usize {
        self.split_request_count
    }

    pub fn source_carrier_count(self) -> usize {
        self.source_carrier_count
    }

    pub fn point_event_count(self) -> usize {
        self.point_event_count
    }

    pub fn interval_event_count(self) -> usize {
        self.interval_event_count
    }

    pub fn event_group_count(self) -> usize {
        self.event_group_count
    }

    pub fn policy_outcome_count(self) -> usize {
        self.policy_outcome_count
    }
}
