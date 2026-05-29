use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeReplayBranchBreadthRow {
    pub(crate) required_scenario_count: usize,
    pub(crate) replay_checked_scenario_count: usize,
    pub(crate) replay_step_count: usize,
    pub(crate) replay_comparison_step_count: usize,
    pub(crate) replay_mismatch_count: usize,
    pub(crate) branch_local_row_count: usize,
    pub(crate) accepted_branch_local_row_count: usize,
    pub(crate) required_accepted_branch_local_count: usize,
    pub(crate) rejected_branch_local_row_count: usize,
    pub(crate) required_rejected_branch_local_count: usize,
    pub(crate) branch_truth_digest_count: usize,
    pub(crate) unchanged_rejected_branch_count: usize,
    pub(crate) row_digest: String,
}
