use worth_ui::facade::{
    WorthUiPageHostRebindReceipt, WorthUiPageHostRebindStatus, WorthUiProjectionFamily,
    WorthUiProjectionRebindRowReceipt, WorthUiProjectionRebindStatus,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationPageHostRebindEvidence {
    status: WorthUiPageHostRebindStatus,
    previous_frame_digest: u64,
    rebound_frame_digest: u64,
    inspected_projection_count: usize,
    dependency_intersection_count: usize,
    rebuild_attempt_count: usize,
    preserved_frame_count: usize,
    denied_frame_count: usize,
    rebuilt_frame_count: usize,
    rows: Vec<ValidationPageHostProjectionRowEvidence>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationPageHostProjectionRowEvidence {
    projection_identity: String,
    projection_family: WorthUiProjectionFamily,
    status: WorthUiProjectionRebindStatus,
    rebuild_attempted: bool,
    previous_frame_digest: u64,
    rebound_frame_digest: u64,
}

impl ValidationPageHostRebindEvidence {
    pub fn from_receipt(receipt: &WorthUiPageHostRebindReceipt) -> Self {
        let counters = receipt.projection_rebind_batch().counters();
        Self {
            status: receipt.status(),
            previous_frame_digest: receipt.previous_frame_digest(),
            rebound_frame_digest: receipt.rebound_frame_digest(),
            inspected_projection_count: counters.inspected_projection_count(),
            dependency_intersection_count: counters.dependency_intersection_count(),
            rebuild_attempt_count: counters.rebuild_attempt_count(),
            preserved_frame_count: counters.preserved_frame_count(),
            denied_frame_count: counters.denied_frame_count(),
            rebuilt_frame_count: counters.rebuilt_frame_count(),
            rows: receipt
                .projection_rebind_batch()
                .rows()
                .iter()
                .map(ValidationPageHostProjectionRowEvidence::from_row)
                .collect(),
        }
    }

    pub fn status(&self) -> WorthUiPageHostRebindStatus {
        self.status
    }

    pub fn previous_frame_digest(&self) -> u64 {
        self.previous_frame_digest
    }

    pub fn rebound_frame_digest(&self) -> u64 {
        self.rebound_frame_digest
    }

    pub fn inspected_projection_count(&self) -> usize {
        self.inspected_projection_count
    }

    pub fn dependency_intersection_count(&self) -> usize {
        self.dependency_intersection_count
    }

    pub fn rebuild_attempt_count(&self) -> usize {
        self.rebuild_attempt_count
    }

    pub fn preserved_frame_count(&self) -> usize {
        self.preserved_frame_count
    }

    pub fn denied_frame_count(&self) -> usize {
        self.denied_frame_count
    }

    pub fn rebuilt_frame_count(&self) -> usize {
        self.rebuilt_frame_count
    }

    pub fn rows(&self) -> &[ValidationPageHostProjectionRowEvidence] {
        &self.rows
    }
}

impl ValidationPageHostProjectionRowEvidence {
    fn from_row(row: &WorthUiProjectionRebindRowReceipt) -> Self {
        Self {
            projection_identity: row.projection_identity().as_str().to_owned(),
            projection_family: row.projection_family(),
            status: row.status(),
            rebuild_attempted: row.rebuild_attempted(),
            previous_frame_digest: row.previous_frame_digest(),
            rebound_frame_digest: row.rebound_frame_digest(),
        }
    }

    pub fn projection_identity(&self) -> &str {
        &self.projection_identity
    }

    pub fn projection_family(&self) -> WorthUiProjectionFamily {
        self.projection_family
    }

    pub fn status(&self) -> WorthUiProjectionRebindStatus {
        self.status
    }

    pub fn rebuild_attempted(&self) -> bool {
        self.rebuild_attempted
    }

    pub fn previous_frame_digest(&self) -> u64 {
        self.previous_frame_digest
    }

    pub fn rebound_frame_digest(&self) -> u64 {
        self.rebound_frame_digest
    }
}
