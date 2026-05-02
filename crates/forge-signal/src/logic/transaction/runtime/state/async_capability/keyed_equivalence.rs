use crate::data::async_node::{
    AsyncCapableNode, AsyncKeyedNodeCapabilityBinding, AsyncKeyedNodeCapabilityEquivalenceReport,
    AsyncNodeCapabilityDeclaration, DeniedAsyncKeyedNodeCapabilityEquivalence,
};
use crate::data::resource::{
    ResourceBoundaryPerformanceEnvelope, ResourceDiagnosticsExpansionBudget,
};

use super::SignalRuntime;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn async_keyed_node_capability_equivalence_report(
        &mut self,
        binding: &AsyncKeyedNodeCapabilityBinding,
        handle: &AsyncCapableNode,
        declaration: &AsyncNodeCapabilityDeclaration,
        budget: ResourceDiagnosticsExpansionBudget,
    ) -> Result<AsyncKeyedNodeCapabilityEquivalenceReport, DeniedAsyncKeyedNodeCapabilityEquivalence>
    {
        let denied_performance =
            ResourceBoundaryPerformanceEnvelope::async_keyed_node_capability_equivalence(
                0, 0, 0, 1,
            );
        if binding.node() != handle.node() {
            return Err(
                DeniedAsyncKeyedNodeCapabilityEquivalence::binding_handle_node_mismatch(
                    binding,
                    handle.node(),
                    denied_performance,
                ),
            );
        }
        if !Self::async_keyed_binding_matches_handle(binding, handle) {
            return Err(
                DeniedAsyncKeyedNodeCapabilityEquivalence::binding_handle_digest_mismatch(
                    binding,
                    handle.node(),
                    denied_performance,
                ),
            );
        }

        let equivalence_report = self
            .async_node_capability_equivalence_report(handle, declaration, budget)
            .map_err(|denial| {
                DeniedAsyncKeyedNodeCapabilityEquivalence::capability_equivalence_denied(
                    binding,
                    handle.node(),
                    denial,
                    denied_performance,
                )
            })?;
        let performance =
            ResourceBoundaryPerformanceEnvelope::async_keyed_node_capability_equivalence(
                u32::from(
                    equivalence_report
                        .historical_parity_report()
                        .branch_restore_report()
                        .is_some(),
                ),
                u32::from(equivalence_report.observation_digest().is_some()),
                1u32.saturating_add(u32::from(equivalence_report.explanation_digest().is_some()))
                    .saturating_add(u32::from(
                        equivalence_report
                            .historical_parity_report()
                            .diagnostics_summary()
                            .is_some(),
                    )),
                0,
            );
        self.telemetry
            .resource
            .async_keyed_node_capability_equivalence_count += 1;
        Ok(AsyncKeyedNodeCapabilityEquivalenceReport::new(
            binding,
            equivalence_report,
            performance,
        ))
    }
}
