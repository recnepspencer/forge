use super::WorkloadEvidenceStageCounters;
use crate::workload_platform::planar_boolean_events::PlanarBooleanEventLedgerCounters;

impl WorkloadEvidenceStageCounters {
    pub fn boolean_event_ledger(counters: PlanarBooleanEventLedgerCounters) -> Self {
        Self {
            boolean_event_ledger_count: 1,
            boolean_event_ledger_point_event_count: counters.point_events_consumed(),
            boolean_event_ledger_interval_event_count: counters.interval_events_consumed(),
            boolean_event_ledger_group_count: counters.total_grouped_event_count(),
            boolean_event_ledger_relation_diagnostic_count: counters
                .relation_diagnostics_retained(),
            ..Self::default()
        }
    }

    pub fn boolean_event_ledger_count(self) -> usize {
        self.boolean_event_ledger_count
    }

    pub fn boolean_event_ledger_point_event_count(self) -> usize {
        self.boolean_event_ledger_point_event_count
    }

    pub fn boolean_event_ledger_interval_event_count(self) -> usize {
        self.boolean_event_ledger_interval_event_count
    }

    pub fn boolean_event_ledger_group_count(self) -> usize {
        self.boolean_event_ledger_group_count
    }

    pub fn boolean_event_ledger_relation_diagnostic_count(self) -> usize {
        self.boolean_event_ledger_relation_diagnostic_count
    }
}
