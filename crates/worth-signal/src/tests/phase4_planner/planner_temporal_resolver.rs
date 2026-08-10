use crate::facade::{
    ConditionEvaluationContext, ConditionResolver, SignalError, TemporalCondition,
    TemporalConditionResolver, TemporalDuration,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tier {
    A,
}

#[derive(Default)]
pub(super) struct PlannerTemporalResolver {
    pub(super) temporal_ready: bool,
}

impl TemporalConditionResolver for PlannerTemporalResolver {
    fn resolve_temporal(
        &mut self,
        _condition: &TemporalCondition,
        _ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        Ok(self.temporal_ready)
    }

    fn debounce_ready(
        &mut self,
        _quiet_period: TemporalDuration,
        _ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        Ok(self.temporal_ready)
    }
}

impl ConditionResolver for PlannerTemporalResolver {
    fn resolve_custom(
        &mut self,
        _key: &str,
        _ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        Ok(false)
    }
}
