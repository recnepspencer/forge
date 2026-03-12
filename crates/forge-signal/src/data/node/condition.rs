use serde::{Deserialize, Serialize};

use crate::data::aspect::AspectMask;
use crate::data::comparator::VersionComparatorPolicy;

use super::contract::NodeContract;

/// Evaluation condition descriptor for a node.
///
/// This is a policy declaration. Runtime gating integration is tier/runtime
/// specific and intentionally decoupled from node storage.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum EvaluationCondition {
    /// Always evaluate when dirty.
    #[default]
    Always,
    /// Evaluate only when dirtying touches one of these aspects.
    AspectFilter(AspectMask),
    /// Evaluate only when change magnitude exceeds this threshold.
    DeltaThreshold(f64),
    /// Evaluate only when explicitly requested.
    OnDemand,
    /// Evaluate only after this quiet period (milliseconds) with no updates.
    Debounce(u64),
    /// Named custom condition handled by embedding runtime.
    Custom(String),
}

/// Per-node evaluation configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeEvaluationConfig {
    /// Declarative contract for this node's read/write and context behavior.
    #[serde(default)]
    pub contract: NodeContract,
    /// Condition used to gate node evaluation.
    pub condition: EvaluationCondition,
    /// Comparator policy used to decide whether dependency version changes
    /// are meaningful for this node. `None` means inherit from tier policy.
    #[serde(default)]
    pub comparator: Option<VersionComparatorPolicy>,
    /// Whether this node reports partition-aware output changes.
    #[serde(default)]
    pub partitioned_output: bool,
}

impl Default for NodeEvaluationConfig {
    fn default() -> Self {
        Self {
            contract: NodeContract::default(),
            condition: EvaluationCondition::Always,
            comparator: None,
            partitioned_output: false,
        }
    }
}
