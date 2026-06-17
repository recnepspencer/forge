#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanEventGroupingCounters {
    inspected_point_events: usize,
    inspected_interval_events: usize,
    emitted_point_groups: usize,
    emitted_interval_groups: usize,
    duplicate_point_group_reports_merged: usize,
    duplicate_interval_group_reports_merged: usize,
    participating_carrier_identities_retained: usize,
    segment_pair_identities_retained: usize,
}

impl PlanarBooleanEventGroupingCounters {
    pub(crate) fn inspect_point_event(&mut self) {
        self.inspected_point_events += 1;
    }

    pub(crate) fn inspect_interval_event(&mut self) {
        self.inspected_interval_events += 1;
    }

    pub(crate) fn emit_point_group(&mut self, duplicate_reports: usize) {
        self.emitted_point_groups += 1;
        self.duplicate_point_group_reports_merged += duplicate_reports;
    }

    pub(crate) fn emit_interval_group(&mut self, duplicate_reports: usize) {
        self.emitted_interval_groups += 1;
        self.duplicate_interval_group_reports_merged += duplicate_reports;
    }

    pub(crate) fn retain_group_provenance(&mut self, carriers: usize, segment_pairs: usize) {
        self.participating_carrier_identities_retained += carriers;
        self.segment_pair_identities_retained += segment_pairs;
    }

    pub(crate) fn merge(&mut self, other: Self) {
        self.inspected_point_events += other.inspected_point_events;
        self.inspected_interval_events += other.inspected_interval_events;
        self.emitted_point_groups += other.emitted_point_groups;
        self.emitted_interval_groups += other.emitted_interval_groups;
        self.duplicate_point_group_reports_merged += other.duplicate_point_group_reports_merged;
        self.duplicate_interval_group_reports_merged +=
            other.duplicate_interval_group_reports_merged;
        self.participating_carrier_identities_retained +=
            other.participating_carrier_identities_retained;
        self.segment_pair_identities_retained += other.segment_pair_identities_retained;
    }

    pub fn inspected_point_events(self) -> usize {
        self.inspected_point_events
    }

    pub fn inspected_interval_events(self) -> usize {
        self.inspected_interval_events
    }

    pub fn emitted_point_groups(self) -> usize {
        self.emitted_point_groups
    }

    pub fn emitted_interval_groups(self) -> usize {
        self.emitted_interval_groups
    }

    pub fn duplicate_point_group_reports_merged(self) -> usize {
        self.duplicate_point_group_reports_merged
    }

    pub fn duplicate_interval_group_reports_merged(self) -> usize {
        self.duplicate_interval_group_reports_merged
    }

    pub fn participating_carrier_identities_retained(self) -> usize {
        self.participating_carrier_identities_retained
    }

    pub fn segment_pair_identities_retained(self) -> usize {
        self.segment_pair_identities_retained
    }
}
