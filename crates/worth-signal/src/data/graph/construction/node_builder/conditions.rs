use crate::data::aspect::AspectMask;
use crate::data::error::SignalError;
use crate::data::node::EvaluationCondition;
use crate::data::temporal::{ClockTick, IntervalCondition, TemporalCondition};

use super::NodeBuilder;

impl NodeBuilder<'_> {
    /// Set the node evaluation condition directly.
    pub fn condition(mut self, condition: EvaluationCondition) -> Self {
        self.config.condition = condition;
        self
    }

    /// Always evaluate the node when dirty.
    pub fn always(self) -> Self {
        self.condition(EvaluationCondition::Always)
    }

    /// Evaluate the node only on explicit request.
    pub fn on_demand(self) -> Self {
        self.condition(EvaluationCondition::OnDemand)
    }

    /// Evaluate the node only after the relative delay has elapsed.
    pub fn after(self, milliseconds: u64) -> Result<Self, SignalError> {
        Ok(
            self.condition(EvaluationCondition::Temporal(TemporalCondition::after(
                milliseconds,
            )?)),
        )
    }

    /// Evaluate the node only at or after the explicit runtime tick.
    pub fn at_or_after(self, tick_ms: u64) -> Self {
        self.condition(EvaluationCondition::Temporal(
            TemporalCondition::at_or_after(ClockTick::new(tick_ms)),
        ))
    }

    /// Evaluate the node only after the quiet period has elapsed.
    pub fn debounce(self, milliseconds: u64) -> Result<Self, SignalError> {
        Ok(
            self.condition(EvaluationCondition::Temporal(TemporalCondition::debounce(
                milliseconds,
            )?)),
        )
    }

    /// Evaluate the node at most once per throttle window.
    pub fn throttle(self, milliseconds: u64) -> Result<Self, SignalError> {
        Ok(
            self.condition(EvaluationCondition::Temporal(TemporalCondition::throttle(
                milliseconds,
            )?)),
        )
    }

    /// Treat the node as stale once the freshness window has elapsed.
    pub fn stale_after(self, milliseconds: u64) -> Result<Self, SignalError> {
        Ok(self.condition(EvaluationCondition::Temporal(
            TemporalCondition::stale_after(milliseconds)?,
        )))
    }

    /// Evaluate the node on a recurring interval with default scheduling semantics.
    pub fn interval(self, period_ms: u64) -> Result<Self, SignalError> {
        Ok(
            self.condition(EvaluationCondition::Temporal(TemporalCondition::interval(
                IntervalCondition::try_new(period_ms)?,
            ))),
        )
    }

    /// Evaluate the node on a recurring interval with explicit scheduling semantics.
    pub fn interval_with(self, interval: IntervalCondition) -> Self {
        self.condition(EvaluationCondition::Temporal(TemporalCondition::interval(
            interval,
        )))
    }

    /// Evaluate the node only when the matching aspects are touched.
    pub fn aspect_filter(self, mask: impl Into<AspectMask>) -> Self {
        self.condition(EvaluationCondition::AspectFilter(mask.into()))
    }

    /// Evaluate the node only when the upstream delta crosses the threshold.
    pub fn delta_threshold(self, threshold: f64) -> Self {
        self.condition(EvaluationCondition::DeltaThreshold(threshold))
    }

    /// Defer the condition decision to a host-provided resolver.
    pub fn custom_condition(self, key: impl Into<String>) -> Self {
        self.condition(EvaluationCondition::Custom(key.into()))
    }
}
