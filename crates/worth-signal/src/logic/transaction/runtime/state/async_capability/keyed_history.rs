use crate::data::async_node::{
    AsyncCapableNode, AsyncKeyedNodeCapabilityBinding, AsyncKeyedNodeHistoricalParityReport,
    DeniedAsyncKeyedNodeHistoricalParity,
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
    pub fn async_keyed_node_historical_parity_report(
        &mut self,
        binding: &AsyncKeyedNodeCapabilityBinding,
        handle: &AsyncCapableNode,
        budget: ResourceDiagnosticsExpansionBudget,
    ) -> Result<AsyncKeyedNodeHistoricalParityReport, DeniedAsyncKeyedNodeHistoricalParity> {
        let denied_performance =
            ResourceBoundaryPerformanceEnvelope::async_keyed_node_historical_parity(0, 0, 0, 1);
        if binding.node() != handle.node() {
            return Err(
                DeniedAsyncKeyedNodeHistoricalParity::binding_handle_node_mismatch(
                    binding,
                    handle.node(),
                    denied_performance,
                ),
            );
        }
        if !Self::async_keyed_binding_matches_handle(binding, handle) {
            return Err(
                DeniedAsyncKeyedNodeHistoricalParity::binding_handle_digest_mismatch(
                    binding,
                    handle.node(),
                    denied_performance,
                ),
            );
        }

        let historical_parity_report = self
            .async_node_historical_parity_report(handle, budget)
            .map_err(|denial| {
                DeniedAsyncKeyedNodeHistoricalParity::historical_parity_denied(
                    binding,
                    handle.node(),
                    denial,
                    denied_performance,
                )
            })?;
        let performance = ResourceBoundaryPerformanceEnvelope::async_keyed_node_historical_parity(
            u32::from(historical_parity_report.branch_restore_report().is_some()),
            u32::from(
                historical_parity_report
                    .observation_batch_report()
                    .is_some(),
            ),
            1u32.saturating_add(u32::from(
                historical_parity_report.explanation_summary().is_some(),
            ))
            .saturating_add(u32::from(
                historical_parity_report.diagnostics_summary().is_some(),
            )),
            0,
        );
        self.with_resource_telemetry(|telemetry| {
            telemetry.async_keyed_node_historical_parity_count += 1
        });
        Ok(AsyncKeyedNodeHistoricalParityReport::new(
            binding,
            historical_parity_report,
            performance,
        ))
    }
}
