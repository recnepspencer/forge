use worth_ui::facade::{
    WorthUiHeaderFrameRebindStatus, WorthUiPageHostRebindStatus,
    WorthUiRebindPhaseExecutionReceipt, WorthUiRebindPhaseLane, WorthUiRebindPhaseSelectionStatus,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationPhaseExecutionEvidence {
    replay_digest: u64,
    phase_row_count: usize,
    inspected_projection_count: usize,
    dependency_intersection_count: usize,
    skipped_phase_count: usize,
    rebuild_attempt_count: usize,
    preserved_projection_count: usize,
    rebuilt_projection_count: usize,
    header_rebind_status: WorthUiHeaderFrameRebindStatus,
    page_host_rebind_status: WorthUiPageHostRebindStatus,
    rows: Vec<ValidationPhaseExecutionRowEvidence>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationPhaseExecutionRowEvidence {
    lane: WorthUiRebindPhaseLane,
    status: WorthUiRebindPhaseSelectionStatus,
    dependency_intersection_count: usize,
}

impl ValidationPhaseExecutionEvidence {
    pub fn from_receipt(receipt: &WorthUiRebindPhaseExecutionReceipt) -> Self {
        let counters = receipt.counters();
        Self {
            replay_digest: receipt.replay_digest(),
            phase_row_count: counters.phase_row_count(),
            inspected_projection_count: counters.inspected_projection_count(),
            dependency_intersection_count: counters.dependency_intersection_count(),
            skipped_phase_count: counters.skipped_phase_count(),
            rebuild_attempt_count: counters.rebuild_attempt_count(),
            preserved_projection_count: counters.preserved_projection_count(),
            rebuilt_projection_count: counters.rebuilt_projection_count(),
            header_rebind_status: receipt.header_rebind().status(),
            page_host_rebind_status: receipt.page_host_rebind().status(),
            rows: receipt
                .rows()
                .iter()
                .map(ValidationPhaseExecutionRowEvidence::from_row)
                .collect(),
        }
    }

    pub fn replay_digest(&self) -> u64 {
        self.replay_digest
    }

    pub fn phase_row_count(&self) -> usize {
        self.phase_row_count
    }

    pub fn inspected_projection_count(&self) -> usize {
        self.inspected_projection_count
    }

    pub fn dependency_intersection_count(&self) -> usize {
        self.dependency_intersection_count
    }

    pub fn skipped_phase_count(&self) -> usize {
        self.skipped_phase_count
    }

    pub fn rebuild_attempt_count(&self) -> usize {
        self.rebuild_attempt_count
    }

    pub fn preserved_projection_count(&self) -> usize {
        self.preserved_projection_count
    }

    pub fn rebuilt_projection_count(&self) -> usize {
        self.rebuilt_projection_count
    }

    pub fn header_rebind_status(&self) -> WorthUiHeaderFrameRebindStatus {
        self.header_rebind_status
    }

    pub fn page_host_rebind_status(&self) -> WorthUiPageHostRebindStatus {
        self.page_host_rebind_status
    }

    pub fn rows(&self) -> &[ValidationPhaseExecutionRowEvidence] {
        &self.rows
    }
}

impl ValidationPhaseExecutionRowEvidence {
    fn from_row(row: &worth_ui::facade::WorthUiRebindPhaseSelectionRow) -> Self {
        Self {
            lane: row.lane(),
            status: row.status(),
            dependency_intersection_count: row.dependency_intersection_count(),
        }
    }

    pub fn lane(&self) -> WorthUiRebindPhaseLane {
        self.lane
    }

    pub fn status(&self) -> WorthUiRebindPhaseSelectionStatus {
        self.status
    }

    pub fn dependency_intersection_count(&self) -> usize {
        self.dependency_intersection_count
    }
}
