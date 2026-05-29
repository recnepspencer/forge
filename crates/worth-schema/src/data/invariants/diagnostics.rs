use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DiagnosticsInvariantGroup {
    DecisionTraceCoverage,
    InterpretationCoverage,
}

impl DiagnosticsInvariantGroup {
    pub const ALL: [Self; 2] = [Self::DecisionTraceCoverage, Self::InterpretationCoverage];
}
