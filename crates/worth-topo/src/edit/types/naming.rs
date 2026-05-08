use serde::{Deserialize, Serialize};

use super::{TopologyEditFamily, TopologyEditNamingScope};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TopologyEditNamingOutcome {
    Preserved,
    Ambiguous,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyEditNamingRow {
    pub family: TopologyEditFamily,
    pub scope: TopologyEditNamingScope,
    pub outcome: TopologyEditNamingOutcome,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyEditNamingReport {
    pub rows: Vec<TopologyEditNamingRow>,
}

impl TopologyEditNamingReport {
    pub fn rejected(mut self, reason: impl Into<String>) -> Self {
        let reason = reason.into();
        for row in &mut self.rows {
            row.outcome = TopologyEditNamingOutcome::Rejected;
            row.reason = reason.clone();
        }
        self
    }
}
