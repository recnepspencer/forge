use crate::{
    authority::AuthoritativeExportBundle, live_query::continuation::ContinuationStrategy,
};
use forge_relational::facade::history::CommitId;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone8CertificationSummary {
    pub truth_matches_control_lane: bool,
    pub restore_truth_parity: bool,
    pub control_lane_matches_authoritative_truth: bool,
    pub admitted_lane_stayed_narrow: bool,
    pub no_gap_batches_observed: bool,
    pub no_duplicate_batches_observed: bool,
    pub no_illegal_acknowledgments: bool,
    pub no_hidden_control_lane_fallback: bool,
    pub no_failure_markers: bool,
}

#[derive(Debug, Clone)]
pub struct Milestone8CertificationRequest<'a> {
    control_export: &'a AuthoritativeExportBundle,
    basis: &'a crate::StableBasisHandle,
    continuation_strategy: ContinuationStrategy,
    continuation_results: &'a [crate::ContinuationBatchResult],
    final_frontier_commit_id: CommitId,
    control_strategy: ContinuationStrategy,
    control_continuation_results: &'a [crate::ContinuationBatchResult],
    control_final_frontier_commit_id: CommitId,
    failure_markers: &'a [String],
}

impl<'a> Milestone8CertificationRequest<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        control_export: &'a AuthoritativeExportBundle,
        basis: &'a crate::StableBasisHandle,
        continuation_strategy: ContinuationStrategy,
        continuation_results: &'a [crate::ContinuationBatchResult],
        final_frontier_commit_id: CommitId,
        control_strategy: ContinuationStrategy,
        control_continuation_results: &'a [crate::ContinuationBatchResult],
        control_final_frontier_commit_id: CommitId,
        failure_markers: &'a [String],
    ) -> Self {
        Self {
            control_export,
            basis,
            continuation_strategy,
            continuation_results,
            final_frontier_commit_id,
            control_strategy,
            control_continuation_results,
            control_final_frontier_commit_id,
            failure_markers,
        }
    }

    pub fn control_export(&self) -> &'a AuthoritativeExportBundle { self.control_export }
    pub fn basis(&self) -> &'a crate::StableBasisHandle { self.basis }
    pub fn continuation_strategy(&self) -> ContinuationStrategy { self.continuation_strategy }
    pub fn continuation_results(&self) -> &'a [crate::ContinuationBatchResult] { self.continuation_results }
    pub fn final_frontier_commit_id(&self) -> CommitId { self.final_frontier_commit_id }
    pub fn control_strategy(&self) -> ContinuationStrategy { self.control_strategy }
    pub fn control_continuation_results(&self) -> &'a [crate::ContinuationBatchResult] { self.control_continuation_results }
    pub fn control_final_frontier_commit_id(&self) -> CommitId { self.control_final_frontier_commit_id }
    pub fn failure_markers(&self) -> &'a [String] { self.failure_markers }
}
