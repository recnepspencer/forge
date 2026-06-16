use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanEventGroupingCounters, PlanarBooleanIntervalEventExtractionCounters,
    PlanarBooleanPointEventExtractionCounters,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanEventLedgerCounters {
    point_events_consumed: usize,
    interval_events_consumed: usize,
    collinear_relations_consumed: usize,
    point_groups_emitted: usize,
    interval_groups_emitted: usize,
    relation_diagnostics_retained: usize,
    duplicate_point_reports_suppressed: usize,
    duplicate_point_group_reports_merged: usize,
    duplicate_interval_group_reports_merged: usize,
    total_grouped_event_count: usize,
    downstream_consumable_artifact_count: usize,
}

impl PlanarBooleanEventLedgerCounters {
    pub(crate) fn new(
        point_events_consumed: usize,
        interval_events_consumed: usize,
        point_counters: PlanarBooleanPointEventExtractionCounters,
        _interval_counters: PlanarBooleanIntervalEventExtractionCounters,
        grouping_counters: PlanarBooleanEventGroupingCounters,
        collinear_relations_consumed: usize,
        relation_diagnostics_retained: usize,
    ) -> Self {
        let point_groups_emitted = grouping_counters.emitted_point_groups();
        let interval_groups_emitted = grouping_counters.emitted_interval_groups();
        Self {
            point_events_consumed,
            interval_events_consumed,
            collinear_relations_consumed,
            point_groups_emitted,
            interval_groups_emitted,
            relation_diagnostics_retained,
            duplicate_point_reports_suppressed: point_counters.duplicate_point_reports_suppressed(),
            duplicate_point_group_reports_merged: grouping_counters
                .duplicate_point_group_reports_merged(),
            duplicate_interval_group_reports_merged: grouping_counters
                .duplicate_interval_group_reports_merged(),
            total_grouped_event_count: point_groups_emitted + interval_groups_emitted,
            downstream_consumable_artifact_count: 1,
        }
    }

    pub fn point_events_consumed(self) -> usize {
        self.point_events_consumed
    }

    pub fn interval_events_consumed(self) -> usize {
        self.interval_events_consumed
    }

    pub fn collinear_relations_consumed(self) -> usize {
        self.collinear_relations_consumed
    }

    pub fn point_groups_emitted(self) -> usize {
        self.point_groups_emitted
    }

    pub fn interval_groups_emitted(self) -> usize {
        self.interval_groups_emitted
    }

    pub fn relation_diagnostics_retained(self) -> usize {
        self.relation_diagnostics_retained
    }

    pub fn duplicate_point_reports_suppressed(self) -> usize {
        self.duplicate_point_reports_suppressed
    }

    pub fn duplicate_point_group_reports_merged(self) -> usize {
        self.duplicate_point_group_reports_merged
    }

    pub fn duplicate_interval_group_reports_merged(self) -> usize {
        self.duplicate_interval_group_reports_merged
    }

    pub fn total_grouped_event_count(self) -> usize {
        self.total_grouped_event_count
    }

    pub fn downstream_consumable_artifact_count(self) -> usize {
        self.downstream_consumable_artifact_count
    }
}
