use crate::data::async_node::{
    AsyncCapableNode, AsyncNodeHistoricalParityReport, DeniedAsyncNodeHistoricalParity,
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
    pub fn async_node_historical_parity_report(
        &mut self,
        handle: &AsyncCapableNode,
        budget: ResourceDiagnosticsExpansionBudget,
    ) -> Result<AsyncNodeHistoricalParityReport, DeniedAsyncNodeHistoricalParity> {
        let branch_restore_report = self.latest_resource_branch_restore_report();
        let branch_restore_width = u32::from(branch_restore_report.is_some());
        let bundle = self.current_async_capability_bundle(handle, branch_restore_width)?;
        let replay_reconstruction = self.reconstruct_resource_replay_summary();
        let observation_batch_report = self.latest_resource_observation_batch_report();
        let (explanation_artifact, explanation_availability) = self
            .graph()
            .materialize_explanation_artifact(handle.node())
            .expect("historical parity should materialize explanation availability for live nodes");
        let explanation_summary = explanation_artifact
            .as_ref()
            .map(|artifact| artifact.diagnostics_summary(self.runtime_policy().tier));
        let diagnostics_result = self.try_resource_diagnostics_summary(budget);
        let diagnostics_allocations = 1u32
            .saturating_add(u32::from(observation_batch_report.is_some()))
            .saturating_add(u32::from(explanation_summary.is_some()));
        let performance = ResourceBoundaryPerformanceEnvelope::async_node_historical_parity(
            branch_restore_width,
            diagnostics_allocations,
            0,
        );
        self.telemetry.resource.async_node_historical_parity_count += 1;

        let (diagnostics_summary, diagnostics_denial) = match diagnostics_result {
            Ok(summary) => (Some(summary), None),
            Err(denial) => (None, Some(denial)),
        };

        Ok(AsyncNodeHistoricalParityReport::new(
            handle.node(),
            bundle.registry_digest().clone(),
            bundle.bundle_digest().clone(),
            bundle.payload_contract_digest().clone(),
            branch_restore_report,
            replay_reconstruction,
            observation_batch_report,
            explanation_summary,
            explanation_availability,
            diagnostics_summary,
            diagnostics_denial,
            performance,
        ))
    }
}
