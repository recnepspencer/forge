use crate::runtime::WorthUiCandidateAuthoringLane;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiReloadLatencyCounters {
    iteration_count: usize,
    file_candidate_count: usize,
    rust_candidate_count: usize,
    prepared_pending_cutover_count: usize,
    denied_candidate_count: usize,
    preservation_count: usize,
    foundational_receipt_count: usize,
    forged_receipt_reuse_denial_count: usize,
}

impl WorthUiReloadLatencyCounters {
    pub(crate) fn record_iteration(&mut self) {
        self.iteration_count += 1;
    }

    pub(crate) fn record_candidate_lane(&mut self, lane: WorthUiCandidateAuthoringLane) {
        match lane {
            WorthUiCandidateAuthoringLane::FileAuthored => self.file_candidate_count += 1,
            WorthUiCandidateAuthoringLane::RustAuthored => self.rust_candidate_count += 1,
        }
    }

    pub(crate) fn record_prepared_pending_cutover(&mut self) {
        self.prepared_pending_cutover_count += 1;
        self.preservation_count += 1;
    }

    pub(crate) fn record_denied_preservation(&mut self) {
        self.denied_candidate_count += 1;
        self.preservation_count += 1;
    }

    pub(crate) fn record_foundational_receipts(&mut self, count: usize) {
        self.foundational_receipt_count += count;
    }

    pub(crate) fn record_forged_receipt_reuse_denial(&mut self) {
        self.forged_receipt_reuse_denial_count += 1;
    }

    pub fn iteration_count(self) -> usize {
        self.iteration_count
    }

    pub fn file_candidate_count(self) -> usize {
        self.file_candidate_count
    }

    pub fn rust_candidate_count(self) -> usize {
        self.rust_candidate_count
    }

    pub fn prepared_pending_cutover_count(self) -> usize {
        self.prepared_pending_cutover_count
    }

    pub fn denied_candidate_count(self) -> usize {
        self.denied_candidate_count
    }

    pub fn preservation_count(self) -> usize {
        self.preservation_count
    }

    pub fn foundational_receipt_count(self) -> usize {
        self.foundational_receipt_count
    }

    pub fn forged_receipt_reuse_denial_count(self) -> usize {
        self.forged_receipt_reuse_denial_count
    }
}
