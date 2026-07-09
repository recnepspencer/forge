use crate::data::aspect::AspectMask;
use crate::data::handle::NodeId;
use crate::data::node::ContextRequirement;

use super::EvaluationRequestMode;

/// Runtime context passed to condition resolution hooks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConditionEvaluationContext {
    pub node: NodeId,
    pub request_mode: EvaluationRequestMode,
    pub dirty_aspects: AspectMask,
    pub max_dependency_delta: u64,
    pub required_context: ContextRequirement,
}
