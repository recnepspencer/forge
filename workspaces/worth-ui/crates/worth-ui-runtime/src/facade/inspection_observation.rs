use std::cell::Cell;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiInspectionFacadeObservation {
    total_query_count: u64,
    unsupported_query_count: u64,
    support_report_count: u64,
    rich_artifact_materialization_count: u64,
    log_emission_count: u64,
}

impl UiInspectionFacadeObservation {
    pub const fn new(
        total_query_count: u64,
        unsupported_query_count: u64,
        support_report_count: u64,
        rich_artifact_materialization_count: u64,
        log_emission_count: u64,
    ) -> Self {
        Self {
            total_query_count,
            unsupported_query_count,
            support_report_count,
            rich_artifact_materialization_count,
            log_emission_count,
        }
    }

    pub fn total_query_count(self) -> u64 {
        self.total_query_count
    }

    pub fn unsupported_query_count(self) -> u64 {
        self.unsupported_query_count
    }

    pub fn support_report_count(self) -> u64 {
        self.support_report_count
    }

    pub fn rich_artifact_materialization_count(self) -> u64 {
        self.rich_artifact_materialization_count
    }

    pub fn log_emission_count(self) -> u64 {
        self.log_emission_count
    }
}

pub(crate) struct WorthUiInspectionObservationState {
    total_query_count: Cell<u64>,
    unsupported_query_count: Cell<u64>,
    support_report_count: Cell<u64>,
    rich_artifact_materialization_count: Cell<u64>,
    log_emission_count: Cell<u64>,
}

impl WorthUiInspectionObservationState {
    pub(crate) const fn new() -> Self {
        Self {
            total_query_count: Cell::new(0),
            unsupported_query_count: Cell::new(0),
            support_report_count: Cell::new(0),
            rich_artifact_materialization_count: Cell::new(0),
            log_emission_count: Cell::new(0),
        }
    }

    pub(crate) fn record_query(&self) {
        self.total_query_count.set(self.total_query_count.get() + 1);
    }

    pub(crate) fn record_unsupported_query(&self) {
        self.unsupported_query_count
            .set(self.unsupported_query_count.get() + 1);
    }

    pub(crate) fn record_support_report(&self) {
        self.support_report_count
            .set(self.support_report_count.get() + 1);
    }

    pub(crate) fn snapshot(&self) -> UiInspectionFacadeObservation {
        UiInspectionFacadeObservation::new(
            self.total_query_count.get(),
            self.unsupported_query_count.get(),
            self.support_report_count.get(),
            self.rich_artifact_materialization_count.get(),
            self.log_emission_count.get(),
        )
    }
}
