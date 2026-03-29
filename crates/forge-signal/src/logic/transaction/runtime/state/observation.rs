use crate::data::checkpoint::CheckpointBarrier;
use crate::data::comparator::VersionComparatorPolicy;
use crate::data::handle::NodeId;
use crate::data::tier::TierPolicy;
use crate::diagnostics::policy::SignalRuntimePolicy;
use crate::diagnostics::profile::DiagnosticsTier;

use super::{runtime_state::SignalRuntime, RuntimeHistory, RuntimeMerge};

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn prepare_for_observation(&mut self) {
        self.graph.prepare_for_observation();
    }

    pub fn runtime_policy(&self) -> SignalRuntimePolicy {
        self.graph.runtime_policy()
    }

    pub fn diagnostics(&self) -> crate::diagnostics::Diagnostics<'_> {
        self.observe().diagnostics()
    }

    pub fn history(&mut self) -> RuntimeHistory<'_, D, I, E, Ctx, T> {
        RuntimeHistory::new(self)
    }

    pub fn merge(&mut self) -> RuntimeMerge<'_, D, I, E, Ctx, T> {
        RuntimeMerge::new(self)
    }

    /// Reset the runtime to the named diagnostics tier preset.
    ///
    /// This is a convenience for switching back to a stock posture.
    /// If you already have narrower retention or replay overrides, prefer
    /// `set_runtime_policy(...)` so you do not accidentally throw them away.
    pub fn reset_runtime_policy_to_tier(&mut self, profile: DiagnosticsTier) {
        self.graph.reset_runtime_policy_to_tier(profile);
    }

    #[deprecated(
        note = "use reset_runtime_policy_to_tier(...) for stock preset resets, or set_runtime_policy(...) for full policy control"
    )]
    pub fn set_diagnostics_profile(&mut self, profile: DiagnosticsTier) {
        self.reset_runtime_policy_to_tier(profile);
    }

    /// Apply the full runtime policy bundle.
    ///
    /// This is the canonical owner for runtime posture and diagnostics
    /// richness once you move past the stock presets.
    pub fn set_runtime_policy(&mut self, policy: SignalRuntimePolicy) {
        self.graph.set_runtime_policy(policy);
    }

    /// Adjust the current runtime policy in one place.
    pub fn adjust_runtime_policy<F>(&mut self, adjust: F)
    where
        F: FnOnce(SignalRuntimePolicy) -> SignalRuntimePolicy,
    {
        let next = adjust(self.runtime_policy());
        self.set_runtime_policy(next);
    }

    pub fn set_node_tier(&mut self, node: NodeId, tier: T) {
        self.config.set_node_tier(&self.graph, node, tier);
    }

    pub fn set_tier_policy(&mut self, policy: TierPolicy<T>) {
        self.config.set_tier_policy(policy);
    }

    /// Adjust the policy for one existing tier.
    ///
    /// Returns `true` when the tier existed and was updated.
    pub fn adjust_tier_policy<F>(&mut self, tier: T, adjust: F) -> bool
    where
        F: FnOnce(TierPolicy<T>) -> TierPolicy<T>,
    {
        let Some(current) = self.config.tier_policies().get(tier).cloned() else {
            return false;
        };
        self.set_tier_policy(adjust(current));
        true
    }

    pub fn set_fallback_comparator(&mut self, policy: VersionComparatorPolicy) {
        self.config.set_fallback_comparator(policy);
    }

    /// Adjust the fallback comparator in one place.
    pub fn adjust_fallback_comparator<F>(&mut self, adjust: F)
    where
        F: FnOnce(VersionComparatorPolicy) -> VersionComparatorPolicy,
    {
        let next = adjust(self.config.fallback_comparator().clone());
        self.set_fallback_comparator(next);
    }

    /// Set one domain-specific checkpoint barrier.
    pub fn set_domain_checkpoint_barrier(&mut self, domain: D, barrier: CheckpointBarrier) {
        self.checkpoint.policy_mut().set_barrier(domain, barrier);
    }

    /// Adjust the full checkpoint policy in one place.
    pub fn adjust_checkpoint_policy<F>(&mut self, adjust: F)
    where
        F: FnOnce(&mut crate::data::checkpoint_policy::CheckpointPolicy<D>),
    {
        adjust(self.checkpoint.policy_mut());
    }
}
