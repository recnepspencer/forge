use serde::{Deserialize, Serialize};

use crate::history::data::BranchId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeIntent {
    ReconcileIntoTarget,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergePlanningRequest {
    pub target_branch: BranchId,
    pub source_branch: BranchId,
    pub merge_intent: MergeIntent,
}

impl MergePlanningRequest {
    pub fn new(
        target_branch: BranchId,
        source_branch: BranchId,
        merge_intent: MergeIntent,
    ) -> Self {
        Self {
            target_branch,
            source_branch,
            merge_intent,
        }
    }
}
