use serde::{Deserialize, Serialize};

use super::{TopologyMutationFamily, TopologyMutationNamingScope};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TopologyMutationNamingOutcome {
    Preserved,
    Ambiguous,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyMutationNamingRow {
    pub family: TopologyMutationFamily,
    pub scope: TopologyMutationNamingScope,
    pub outcome: TopologyMutationNamingOutcome,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyMutationNamingReport {
    pub rows: Vec<TopologyMutationNamingRow>,
}

impl TopologyMutationNamingReport {
    pub fn rejected(mut self, reason: impl Into<String>) -> Self {
        let reason = reason.into();
        for row in &mut self.rows {
            row.outcome = TopologyMutationNamingOutcome::Rejected;
            row.reason = reason.clone();
        }
        self
    }
}
