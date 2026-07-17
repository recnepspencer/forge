use std::cell::Cell;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiInspectionFacadeObservation {
    total_query_count: u64,
    unsupported_query_count: u64,
    support_report_count: u64,
    rich_artifact_materialization_count: u64,
    authored_lookup_count: u64,
    graph_node_evidence_index_rebuild_count: u64,
    graph_aspect_evidence_index_rebuild_count: u64,
    log_emission_count: u64,
}

impl UiInspectionFacadeObservation {
    fn from_state(state: &WorthUiInspectionObservationState) -> Self {
        Self {
            total_query_count: state.total_query_count.get(),
            unsupported_query_count: state.unsupported_query_count.get(),
            support_report_count: state.support_report_count.get(),
            rich_artifact_materialization_count: state.rich_artifact_materialization_count.get(),
            authored_lookup_count: state.authored_lookup_count.get(),
            graph_node_evidence_index_rebuild_count: state
                .graph_node_evidence_index_rebuild_count
                .get(),
            graph_aspect_evidence_index_rebuild_count: state
                .graph_aspect_evidence_index_rebuild_count
                .get(),
            log_emission_count: state.log_emission_count.get(),
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

    pub fn authored_lookup_count(self) -> u64 {
        self.authored_lookup_count
    }

    pub fn graph_node_evidence_index_rebuild_count(self) -> u64 {
        self.graph_node_evidence_index_rebuild_count
    }

    pub fn graph_aspect_evidence_index_rebuild_count(self) -> u64 {
        self.graph_aspect_evidence_index_rebuild_count
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
    authored_lookup_count: Cell<u64>,
    graph_node_evidence_index_rebuild_count: Cell<u64>,
    graph_aspect_evidence_index_rebuild_count: Cell<u64>,
    log_emission_count: Cell<u64>,
}

impl WorthUiInspectionObservationState {
    pub(crate) const fn new() -> Self {
        Self {
            total_query_count: Cell::new(0),
            unsupported_query_count: Cell::new(0),
            support_report_count: Cell::new(0),
            rich_artifact_materialization_count: Cell::new(0),
            authored_lookup_count: Cell::new(0),
            graph_node_evidence_index_rebuild_count: Cell::new(0),
            graph_aspect_evidence_index_rebuild_count: Cell::new(0),
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
        UiInspectionFacadeObservation::from_state(self)
    }

    pub(crate) fn record_rich_artifact_materialization(&self) {
        self.rich_artifact_materialization_count
            .set(self.rich_artifact_materialization_count.get() + 1);
    }

    pub(crate) fn record_authored_lookup(&self) {
        self.authored_lookup_count
            .set(self.authored_lookup_count.get() + 1);
    }

    pub(crate) fn record_graph_node_evidence_index_rebuild(&self) {
        self.graph_node_evidence_index_rebuild_count
            .set(self.graph_node_evidence_index_rebuild_count.get() + 1);
    }

    pub(crate) fn record_graph_aspect_evidence_index_rebuild(&self) {
        self.graph_aspect_evidence_index_rebuild_count
            .set(self.graph_aspect_evidence_index_rebuild_count.get() + 1);
    }
}
