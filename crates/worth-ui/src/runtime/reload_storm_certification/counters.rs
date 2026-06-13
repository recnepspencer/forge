use crate::runtime::{WorthUiCandidateAuthoringLane, WorthUiFileRustReplacementParityCounters};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiReloadLatencyCounters {
    iteration_count: usize,
    file_candidate_count: usize,
    rust_candidate_count: usize,
    valid_candidate_count: usize,
    denied_candidate_count: usize,
    no_op_candidate_count: usize,
    activated_candidate_count: usize,
    preservation_count: usize,
    candidate_admission_count: usize,
    artifact_comparison_count: usize,
    plan_lowering_count: usize,
    lane_admission_count: usize,
    plan_swap_count: usize,
    source_reparse_on_swap_count: usize,
    registry_rebuild_on_swap_count: usize,
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

    pub(crate) fn record_denied_preservation(&mut self) {
        self.denied_candidate_count += 1;
        self.preservation_count += 1;
    }

    pub(crate) fn record_candidate_screening(&mut self) {
        self.candidate_admission_count += 1;
        self.artifact_comparison_count += 1;
    }

    pub(crate) fn record_no_op(&mut self) {
        self.valid_candidate_count += 1;
        self.no_op_candidate_count += 1;
    }

    pub(crate) fn record_activated_pipeline(
        &mut self,
        counters: WorthUiFileRustReplacementParityCounters,
    ) {
        self.valid_candidate_count += 1;
        self.activated_candidate_count += 1;
        self.candidate_admission_count += counters.candidate_admission_count();
        self.artifact_comparison_count += counters.artifact_comparison_count();
        self.plan_lowering_count += counters.plan_lowering_count();
        self.lane_admission_count += counters.lane_admission_count();
        self.plan_swap_count += counters.plan_swap_count();
        self.source_reparse_on_swap_count += counters.source_reparse_on_swap_count();
        self.registry_rebuild_on_swap_count += counters.registry_rebuild_on_swap_count();
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
    pub fn valid_candidate_count(self) -> usize {
        self.valid_candidate_count
    }
    pub fn denied_candidate_count(self) -> usize {
        self.denied_candidate_count
    }
    pub fn no_op_candidate_count(self) -> usize {
        self.no_op_candidate_count
    }
    pub fn activated_candidate_count(self) -> usize {
        self.activated_candidate_count
    }
    pub fn preservation_count(self) -> usize {
        self.preservation_count
    }
    pub fn candidate_admission_count(self) -> usize {
        self.candidate_admission_count
    }
    pub fn artifact_comparison_count(self) -> usize {
        self.artifact_comparison_count
    }
    pub fn plan_lowering_count(self) -> usize {
        self.plan_lowering_count
    }
    pub fn lane_admission_count(self) -> usize {
        self.lane_admission_count
    }
    pub fn plan_swap_count(self) -> usize {
        self.plan_swap_count
    }
    pub fn source_reparse_on_swap_count(self) -> usize {
        self.source_reparse_on_swap_count
    }
    pub fn registry_rebuild_on_swap_count(self) -> usize {
        self.registry_rebuild_on_swap_count
    }
    pub fn foundational_receipt_count(self) -> usize {
        self.foundational_receipt_count
    }
    pub fn forged_receipt_reuse_denial_count(self) -> usize {
        self.forged_receipt_reuse_denial_count
    }
}
