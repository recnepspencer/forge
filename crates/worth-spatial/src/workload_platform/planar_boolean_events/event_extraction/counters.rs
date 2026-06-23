#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanEventExtractionCounters {
    inspected_carriers: usize,
    inspected_segment_pairs: usize,
    denied_micro_events: usize,
    policy_exits: usize,
}

impl PlanarBooleanEventExtractionCounters {
    #[cfg(test)]
    pub(crate) fn inspect_carriers(mut self, count: usize) -> Self {
        self.inspected_carriers += count;
        self
    }

    pub(crate) fn inspect_segment_pairs(mut self, count: usize) -> Self {
        self.inspected_segment_pairs += count;
        self
    }

    pub(crate) fn deny_micro_event(mut self) -> Self {
        self.denied_micro_events += 1;
        self
    }

    pub(crate) fn policy_exit(mut self) -> Self {
        self.policy_exits += 1;
        self
    }

    pub fn inspected_carriers(&self) -> usize {
        self.inspected_carriers
    }

    pub fn inspected_segment_pairs(&self) -> usize {
        self.inspected_segment_pairs
    }

    pub fn denied_micro_events(&self) -> usize {
        self.denied_micro_events
    }

    pub fn policy_exits(&self) -> usize {
        self.policy_exits
    }
}
