use crate::data::async_node::{
    AsyncCapableNode, AsyncNodeHierarchyHistoricalParityReport,
    DeniedAsyncNodeHierarchyHistoricalParity,
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
    pub fn async_node_hierarchy_historical_parity_report(
        &mut self,
        handle: &AsyncCapableNode,
        budget: ResourceDiagnosticsExpansionBudget,
    ) -> Result<AsyncNodeHierarchyHistoricalParityReport, DeniedAsyncNodeHierarchyHistoricalParity>
    {
        let denied_performance =
            ResourceBoundaryPerformanceEnvelope::async_node_hierarchy_historical_parity(
                1, 0, 0, 0, 1,
            );
        let hierarchy_replay_summary = self
            .async_node_hierarchy_replay_summary(handle.node())
            .expect("historical parity root should produce hierarchy replay summary");
        if hierarchy_replay_summary.hierarchy_depth() == 0 {
            return Err(
                DeniedAsyncNodeHierarchyHistoricalParity::not_hierarchical_root(
                    handle.node(),
                    denied_performance,
                ),
            );
        }
        let historical_parity_report = self
            .async_node_historical_parity_report(handle, budget)
            .map_err(|denial| {
                DeniedAsyncNodeHierarchyHistoricalParity::historical_parity_denied(
                    handle.node(),
                    denial,
                    denied_performance,
                )
            })?;

        let performance =
            ResourceBoundaryPerformanceEnvelope::async_node_hierarchy_historical_parity(
                hierarchy_replay_summary.hierarchy_nodes().len() as u32,
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
            telemetry.async_node_hierarchy_historical_parity_count += 1
        });
        Ok(AsyncNodeHierarchyHistoricalParityReport::new(
            handle.node(),
            hierarchy_replay_summary,
            historical_parity_report,
            performance,
        ))
    }
}
