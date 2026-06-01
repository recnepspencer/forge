#[cfg(test)]
use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::history::data::BranchId;

#[cfg(test)]
use super::{LineageDecisionLog, LineageDecisionRecord};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg(test)]
pub(crate) struct LineageRejectionArtifact {
    branch_id: BranchId,
    decision_log: LineageDecisionLog,
}

#[cfg(test)]
impl LineageRejectionArtifact {
    pub(crate) fn single_rejected_promotion(decision: LineageDecisionRecord) -> Self {
        Self {
            branch_id: decision.branch_id().clone(),
            decision_log: LineageDecisionLog::single(decision),
        }
    }

    pub(crate) fn decision_log(&self) -> &LineageDecisionLog {
        &self.decision_log
    }
}
