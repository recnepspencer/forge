use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeSideQuestContractRow {
    pub contract_name: String,
    pub status: String,
    pub reason: String,
    pub row_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeSideQuestBlockerRow {
    pub blocker_name: String,
    pub status: String,
    pub reason: String,
    pub row_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeSideQuestCloseoutReport {
    pub domain_read_request_count: usize,
    pub domain_read_parity_count: usize,
    pub replay_checked_count: usize,
    pub replay_verified_count: usize,
    pub branch_local_checked_count: usize,
    pub branch_local_verified_count: usize,
    pub contract_rows: Vec<MilestoneThreeSideQuestContractRow>,
    pub blocker_rows: Vec<MilestoneThreeSideQuestBlockerRow>,
    pub phase_three_ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeReturnGateBlockerRow {
    pub blocker_name: String,
    pub reason: String,
    pub row_digest: String,
}
