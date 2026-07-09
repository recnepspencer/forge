//! Domain-free checkpoint evaluator contract for Tier-0 scheduling.

use crate::data::error::SignalError;

use crate::data::dirty_set::DomainImpact;

/// Refresh adapter invoked by checkpoint runtime.
pub trait CheckpointEvaluator {
    type Domain: Copy + Ord;
    type Impact: Copy + Ord;
    type Context;

    /// Refresh one domain for the provided dirty impact.
    fn refresh(
        &mut self,
        domain: Self::Domain,
        impact: DomainImpact<Self::Impact>,
        ctx: &mut Self::Context,
    ) -> Result<(), SignalError>;
}
