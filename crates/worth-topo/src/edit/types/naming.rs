use serde::{Deserialize, Serialize};

use super::{WorthTopologyEditFamily, WorthTopologyEditNamingScope};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthTopologyEditNamingOutcome {
    Preserved,
    Ambiguous,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTopologyEditNamingRow {
    pub family: WorthTopologyEditFamily,
    pub scope: WorthTopologyEditNamingScope,
    pub outcome: WorthTopologyEditNamingOutcome,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTopologyEditNamingReport {
    pub rows: Vec<WorthTopologyEditNamingRow>,
}

impl WorthTopologyEditNamingReport {
    pub fn rejected(mut self, reason: impl Into<String>) -> Self {
        let reason = reason.into();
        for row in &mut self.rows {
            row.outcome = WorthTopologyEditNamingOutcome::Rejected;
            row.reason = reason.clone();
        }
        self
    }
}
