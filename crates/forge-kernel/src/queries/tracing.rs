//! Decision and tracing query traits.
//!
//! DOMAIN: Named interfaces for recording traced decisions and scoping
//! operations within named spans during feature execution.

use forge_core::{DecisionKind, DecisionLog, DecisionTier};
use crate::core::ModelingContext;

/// Decision recording access.
///
/// Used by: boolean steps (log tolerance decisions), pipeline executor
/// (collect decisions into envelope).
pub trait DecisionQuery {
    /// Record a tolerance decision.
    fn log_decision(
        &mut self,
        kind: DecisionKind,
        tier: DecisionTier,
        location: [f64; 3],
        margin: f64,
        threshold: f64,
    );

    /// Get the current decision log.
    fn get_decision_log(&self) -> &DecisionLog;

    /// Execute a closure within a named tracing span.
    fn scope<F, R>(&mut self, name: &'static str, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R;
}

impl DecisionQuery for ModelingContext {
    fn log_decision(
        &mut self,
        kind: DecisionKind,
        tier: DecisionTier,
        location: [f64; 3],
        margin: f64,
        threshold: f64,
    ) {
        ModelingContext::log_decision(self, kind, tier, location, margin, threshold);
    }

    fn get_decision_log(&self) -> &DecisionLog {
        ModelingContext::get_decision_log(self)
    }

    fn scope<F, R>(&mut self, name: &'static str, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        ModelingContext::scope(self, name, f)
    }
}

/// Error budget checking.
///
/// Used by: boolean pipeline (accumulated error tracking across phases).
pub trait BudgetQuery {
    /// Check whether the accumulated error budget has been exceeded.
    /// Returns a warning if it has.
    fn check_budget(&self, accumulated: f64) -> Option<forge_core::KernelWarning>;
}

impl BudgetQuery for ModelingContext {
    fn check_budget(&self, accumulated: f64) -> Option<forge_core::KernelWarning> {
        ModelingContext::check_budget(self, accumulated)
    }
}
