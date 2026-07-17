use worth_ui_runtime::facade::runtime_handoff::UiAllocationCounterName;
use worth_ui_test_support::{runtime_origin_fixture, WorthUiTouchOriginFixtureVariant};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiAllocationCloseoutCertificationReport {
    committed_receipts: u16,
    maximum_committed_receipts: u16,
    neighborhood_set_cardinality: u16,
    maximum_neighborhood_set_cardinality: u16,
    root_widen_attempts: u16,
    local_inspection_complete: bool,
    deterministic_replay: bool,
    execution_consumption_admitted: bool,
    every_counter_within_bound: bool,
}

pub fn certify_allocation_closeout() -> UiAllocationCloseoutCertificationReport {
    let first = runtime_origin_fixture(WorthUiTouchOriginFixtureVariant::Baseline);
    let replay = runtime_origin_fixture(WorthUiTouchOriginFixtureVariant::Baseline);
    let counters = first
        .allocation_receipt
        .report()
        .counters()
        .expect("production catalog commit carries allocation counters");
    let committed = counters.value(UiAllocationCounterName::CommittedReceipts);
    let neighborhoods = counters.value(UiAllocationCounterName::NeighborhoodSelections);
    let root_widen = counters.value(UiAllocationCounterName::RootWidenAttempts);
    let local = first.allocation_inspection.local_explanation();
    let local_inspection_complete = !local.stream_families().is_empty()
        && !local.invalidation_families().is_empty()
        && local.invalidation_evidence_ref().identity() != 0
        && local.selection().evidence_ref().identity() != 0
        && local.reuse_evidence_ref().identity() != 0
        && local.geometry().evidence_ref().identity() != 0;
    UiAllocationCloseoutCertificationReport {
        committed_receipts: committed.observed(),
        maximum_committed_receipts: committed.maximum(),
        neighborhood_set_cardinality: neighborhoods.observed(),
        maximum_neighborhood_set_cardinality: neighborhoods.maximum(),
        root_widen_attempts: root_widen.observed(),
        local_inspection_complete,
        deterministic_replay: local == replay.allocation_inspection.local_explanation(),
        execution_consumption_admitted: first.allocation_receipt.lowering_input().is_ok(),
        every_counter_within_bound: counters
            .values()
            .iter()
            .all(|value| value.is_within_bound()),
    }
}

impl UiAllocationCloseoutCertificationReport {
    pub const fn committed_receipts(self) -> u16 {
        self.committed_receipts
    }
    pub const fn maximum_committed_receipts(self) -> u16 {
        self.maximum_committed_receipts
    }
    pub const fn neighborhood_set_cardinality(self) -> u16 {
        self.neighborhood_set_cardinality
    }
    pub const fn maximum_neighborhood_set_cardinality(self) -> u16 {
        self.maximum_neighborhood_set_cardinality
    }
    pub const fn root_widen_attempts(self) -> u16 {
        self.root_widen_attempts
    }
    pub const fn local_inspection_complete(self) -> bool {
        self.local_inspection_complete
    }
    pub const fn deterministic_replay(self) -> bool {
        self.deterministic_replay
    }
    pub const fn execution_consumption_admitted(self) -> bool {
        self.execution_consumption_admitted
    }
    pub const fn every_counter_within_bound(self) -> bool {
        self.every_counter_within_bound
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn production_path_closes_all_allocation_oracles() {
        let report = super::certify_allocation_closeout();
        assert_eq!(report.committed_receipts(), 1);
        assert_eq!(report.maximum_committed_receipts(), 1);
        assert_eq!(report.neighborhood_set_cardinality(), 1);
        assert_eq!(report.maximum_neighborhood_set_cardinality(), 1);
        assert_eq!(report.root_widen_attempts(), 0);
        assert!(report.local_inspection_complete());
        assert!(report.deterministic_replay());
        assert!(report.execution_consumption_admitted());
        assert!(report.every_counter_within_bound());
    }
}
