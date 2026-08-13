use crate::facade::{
    ConditionEvaluationContext, ConditionResolver, SignalError, TemporalCondition,
    TemporalConditionResolver, TemporalDuration,
};

#[derive(Default)]
pub(super) struct TestConditionResolver {
    pub(super) debounce_ready: bool,
    pub(super) temporal_ready: bool,
    pub(super) custom_result: bool,
    pub(super) temporal_calls: u64,
    pub(super) custom_calls: u64,
}

impl TemporalConditionResolver for TestConditionResolver {
    fn resolve_temporal(
        &mut self,
        condition: &TemporalCondition,
        _ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        self.temporal_calls += 1;
        Ok(match condition {
            TemporalCondition::Debounce(_) => self.debounce_ready,
            _ => self.temporal_ready,
        })
    }

    fn debounce_ready(
        &mut self,
        _quiet_period: TemporalDuration,
        _ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        self.temporal_calls += 1;
        Ok(self.debounce_ready)
    }
}

impl ConditionResolver for TestConditionResolver {
    fn resolve_custom(
        &mut self,
        _key: &str,
        _ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        self.custom_calls += 1;
        Ok(self.custom_result)
    }
}
