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

    pub fn diagnostics(&self) -> crate::diagnostics::RuntimeDiagnostics<'_> {
        self.observe().diagnostics()
    }

    pub fn history(&mut self) -> RuntimeHistory<'_, D, I, E, Ctx, T> {
        RuntimeHistory::new(self)
    }

    pub fn merge(&mut self) -> RuntimeMerge<'_, D, I, E, Ctx, T> {
        RuntimeMerge::new(self)
    }

    pub fn set_diagnostics_profile(&mut self, profile: DiagnosticsTier) {
        self.graph.set_diagnostics_profile(profile);
    }

    pub fn set_runtime_policy(&mut self, policy: SignalRuntimePolicy) {
        self.graph.set_runtime_policy(policy);
    }

    pub fn set_node_tier(&mut self, node: NodeId, tier: T) {
        self.config.set_node_tier(&self.graph, node, tier);
    }

    pub fn set_tier_policy(&mut self, policy: TierPolicy<T>) {
        self.config.set_tier_policy(policy);
    }

    pub fn set_fallback_comparator(&mut self, policy: VersionComparatorPolicy) {
        self.config.set_fallback_comparator(policy);
    }
}
