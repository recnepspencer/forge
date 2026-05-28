use super::super::replay_branch_breadth_row::MilestoneThreeReplayBranchBreadthRow;

impl MilestoneThreeReplayBranchBreadthRow {
    pub fn required_scenario_count(&self) -> usize {
        self.required_scenario_count
    }

    pub fn replay_checked_scenario_count(&self) -> usize {
        self.replay_checked_scenario_count
    }

    pub fn replay_step_count(&self) -> usize {
        self.replay_step_count
    }

    pub fn replay_comparison_step_count(&self) -> usize {
        self.replay_comparison_step_count
    }

    pub fn replay_mismatch_count(&self) -> usize {
        self.replay_mismatch_count
    }

    pub fn branch_local_row_count(&self) -> usize {
        self.branch_local_row_count
    }

    pub fn accepted_branch_local_row_count(&self) -> usize {
        self.accepted_branch_local_row_count
    }

    pub fn required_accepted_branch_local_count(&self) -> usize {
        self.required_accepted_branch_local_count
    }

    pub fn rejected_branch_local_row_count(&self) -> usize {
        self.rejected_branch_local_row_count
    }

    pub fn required_rejected_branch_local_count(&self) -> usize {
        self.required_rejected_branch_local_count
    }

    pub fn branch_truth_digest_count(&self) -> usize {
        self.branch_truth_digest_count
    }

    pub fn unchanged_rejected_branch_count(&self) -> usize {
        self.unchanged_rejected_branch_count
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}




