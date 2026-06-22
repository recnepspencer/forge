#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanEdgeSplitRequestCounters {
    split_request_count: usize,
    segment_carrier_count: usize,
    point_event_count: usize,
    interval_event_count: usize,
    event_group_count: usize,
}

impl PlanarBooleanEdgeSplitRequestCounters {
    pub(crate) fn new(
        segment_carrier_count: usize,
        point_event_count: usize,
        interval_event_count: usize,
        event_group_count: usize,
    ) -> Self {
        Self {
            split_request_count: 1,
            segment_carrier_count,
            point_event_count,
            interval_event_count,
            event_group_count,
        }
    }

    pub fn split_request_count(self) -> usize {
        self.split_request_count
    }

    pub fn segment_carrier_count(self) -> usize {
        self.segment_carrier_count
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
}
