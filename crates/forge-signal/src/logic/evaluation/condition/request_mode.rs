use serde::{Deserialize, Serialize};

/// Request mode for one evaluation call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvaluationRequestMode {
    /// Standard evaluation behavior.
    Default,
    /// Force evaluation for nodes gated behind `OnDemand`.
    ForceOnDemand,
}
