use serde::{Deserialize, Serialize};

use crate::data::aspect::AspectMask;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;

/// Request mode for one evaluation call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvaluationRequestMode {
    /// Standard evaluation behavior.
    Default,
    /// Force evaluation for nodes gated behind `OnDemand`.
    ForceOnDemand,
}

/// Runtime context passed to condition resolution hooks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConditionEvaluationContext {
    pub node: NodeId,
    pub request_mode: EvaluationRequestMode,
    pub dirty_aspects: AspectMask,
    pub max_dependency_delta: u64,
}

/// Host callback contract for runtime evaluation conditions.
pub trait ConditionResolver {
    /// Resolve readiness for a debounced node.
    fn debounce_ready(
        &mut self,
        quiet_period_ms: u64,
        ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError>;

    /// Resolve a named custom condition.
    fn resolve_custom(
        &mut self,
        key: &str,
        ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError>;
}

impl<T: ConditionResolver + ?Sized> ConditionResolver for &mut T {
    fn debounce_ready(
        &mut self,
        quiet_period_ms: u64,
        ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        (**self).debounce_ready(quiet_period_ms, ctx)
    }

    fn resolve_custom(
        &mut self,
        key: &str,
        ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        (**self).resolve_custom(key, ctx)
    }
}

/// Default resolver used when callers do not provide condition hooks.
pub struct DefaultConditionResolver;

impl ConditionResolver for DefaultConditionResolver {
    fn debounce_ready(
        &mut self,
        quiet_period_ms: u64,
        _ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        Err(SignalError::invalid_input(format!(
            "Debounce({quiet_period_ms}ms) requires a condition resolver"
        )))
    }

    fn resolve_custom(
        &mut self,
        key: &str,
        _ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        Err(SignalError::invalid_input(format!(
            "Custom condition '{key}' requires a condition resolver"
        )))
    }
}
