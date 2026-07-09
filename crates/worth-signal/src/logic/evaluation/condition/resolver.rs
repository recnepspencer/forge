use crate::data::error::SignalError;
use crate::data::temporal::{ClockTick, IntervalCondition, TemporalCondition, TemporalDuration};

use super::ConditionEvaluationContext;

/// Host callback contract for first-class temporal policies.
pub trait TemporalConditionResolver {
    /// Resolve readiness for a first-class temporal condition.
    fn resolve_temporal(
        &mut self,
        condition: &TemporalCondition,
        ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        match condition {
            TemporalCondition::After(condition) => self.after_ready(condition.delay(), ctx),
            TemporalCondition::AtOrAfter(condition) => {
                self.at_or_after_ready(condition.tick(), ctx)
            }
            TemporalCondition::Debounce(condition) => {
                self.debounce_ready(condition.quiet_period(), ctx)
            }
            TemporalCondition::Throttle(condition) => self.throttle_ready(condition.window(), ctx),
            TemporalCondition::StaleAfter(condition) => {
                self.stale_after_ready(condition.stale_after(), ctx)
            }
            TemporalCondition::Interval(interval) => self.interval_ready(interval, ctx),
        }
    }

    /// Resolve readiness for an `After` temporal condition.
    fn after_ready(
        &mut self,
        delay: TemporalDuration,
        _ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        Err(SignalError::invalid_input(format!(
            "After({}ms) requires a temporal condition resolver",
            delay.get()
        )))
    }

    /// Resolve readiness for an `AtOrAfter` temporal condition.
    fn at_or_after_ready(
        &mut self,
        tick: ClockTick,
        _ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        Err(SignalError::invalid_input(format!(
            "AtOrAfter({}ms) requires a temporal condition resolver",
            tick.get()
        )))
    }

    /// Resolve readiness for a debounced node.
    fn debounce_ready(
        &mut self,
        quiet_period: TemporalDuration,
        _ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError>;

    /// Resolve readiness for a throttled node.
    fn throttle_ready(
        &mut self,
        window: TemporalDuration,
        _ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        Err(SignalError::invalid_input(format!(
            "Throttle({}ms) requires a temporal condition resolver",
            window.get()
        )))
    }

    /// Resolve readiness for a stale-after node.
    fn stale_after_ready(
        &mut self,
        stale_after: TemporalDuration,
        _ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        Err(SignalError::invalid_input(format!(
            "StaleAfter({}ms) requires a temporal condition resolver",
            stale_after.get()
        )))
    }

    /// Resolve readiness for an interval node.
    fn interval_ready(
        &mut self,
        interval: &IntervalCondition,
        _ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        Err(SignalError::invalid_input(format!(
            "Interval({}ms) requires a temporal condition resolver",
            interval.period_ms()
        )))
    }
}

impl<T: TemporalConditionResolver + ?Sized> TemporalConditionResolver for &mut T {
    fn resolve_temporal(
        &mut self,
        condition: &TemporalCondition,
        ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        (**self).resolve_temporal(condition, ctx)
    }

    fn after_ready(
        &mut self,
        delay: TemporalDuration,
        ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        (**self).after_ready(delay, ctx)
    }

    fn at_or_after_ready(
        &mut self,
        tick: ClockTick,
        ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        (**self).at_or_after_ready(tick, ctx)
    }

    fn debounce_ready(
        &mut self,
        quiet_period: TemporalDuration,
        ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        (**self).debounce_ready(quiet_period, ctx)
    }

    fn throttle_ready(
        &mut self,
        window: TemporalDuration,
        ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        (**self).throttle_ready(window, ctx)
    }

    fn stale_after_ready(
        &mut self,
        stale_after: TemporalDuration,
        ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        (**self).stale_after_ready(stale_after, ctx)
    }

    fn interval_ready(
        &mut self,
        interval: &IntervalCondition,
        ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        (**self).interval_ready(interval, ctx)
    }
}

/// Host callback contract for non-built-in custom conditions.
pub trait ConditionResolver: TemporalConditionResolver {
    /// Resolve a named custom condition.
    fn resolve_custom(
        &mut self,
        key: &str,
        ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError>;
}

impl<T: ConditionResolver + ?Sized> ConditionResolver for &mut T {
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

impl TemporalConditionResolver for DefaultConditionResolver {
    fn debounce_ready(
        &mut self,
        quiet_period: TemporalDuration,
        _ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        Err(SignalError::invalid_input(format!(
            "Debounce({}ms) requires a temporal condition resolver",
            quiet_period.get()
        )))
    }
}

impl ConditionResolver for DefaultConditionResolver {
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
