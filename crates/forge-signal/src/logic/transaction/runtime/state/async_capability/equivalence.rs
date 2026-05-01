use crate::data::async_node::{
    AsyncCapableNode, AsyncNodeCapabilityDeclaration, AsyncNodeCapabilityEquivalenceReport,
    DeniedAsyncNodeCapabilityEquivalence,
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
    pub fn async_node_capability_equivalence_report(
        &mut self,
        handle: &AsyncCapableNode,
        declaration: &AsyncNodeCapabilityDeclaration,
        budget: ResourceDiagnosticsExpansionBudget,
    ) -> Result<AsyncNodeCapabilityEquivalenceReport, DeniedAsyncNodeCapabilityEquivalence> {
        let mismatch_performance =
            ResourceBoundaryPerformanceEnvelope::async_node_capability_equivalence(0, 0, 0, 1);
        if handle.node() != declaration.node() {
            return Err(
                DeniedAsyncNodeCapabilityEquivalence::handle_declaration_node_mismatch(
                    handle.node(),
                    declaration.node(),
                    mismatch_performance,
                ),
            );
        }

        let alias_lowering_proof = self
            .prove_async_node_capability_alias_lowering(declaration)
            .map_err(|_| {
                DeniedAsyncNodeCapabilityEquivalence::handle_declaration_node_mismatch(
                    handle.node(),
                    declaration.node(),
                    mismatch_performance,
                )
            })?;
        if handle.registry_digest() != alias_lowering_proof.capability_registry_digest()
            || handle.bundle_digest() != alias_lowering_proof.capability_bundle_digest()
            || handle.payload_contract_digest()
                != alias_lowering_proof.capability_payload_contract_digest()
        {
            return Err(
                DeniedAsyncNodeCapabilityEquivalence::handle_declaration_digest_mismatch(
                    handle.node(),
                    declaration.node(),
                    mismatch_performance,
                ),
            );
        }
        let historical_parity_report = self
            .async_node_historical_parity_report(handle, budget)
            .map_err(|denial| {
                let performance =
                    ResourceBoundaryPerformanceEnvelope::async_node_capability_equivalence(
                        0, 0, 0, 1,
                    );
                DeniedAsyncNodeCapabilityEquivalence::historical_parity_denied(
                    handle.node(),
                    declaration.node(),
                    denial,
                    performance,
                )
            })?;
        let branch_restore_width =
            u32::from(historical_parity_report.branch_restore_report().is_some());
        let observation_width = u32::from(
            historical_parity_report
                .observation_batch_report()
                .is_some(),
        );
        let diagnostics_allocations = 1u32
            .saturating_add(observation_width)
            .saturating_add(u32::from(
                historical_parity_report.diagnostics_summary().is_some(),
            ))
            .saturating_add(u32::from(
                historical_parity_report.explanation_summary().is_some(),
            ));
        let performance = ResourceBoundaryPerformanceEnvelope::async_node_capability_equivalence(
            branch_restore_width,
            observation_width,
            diagnostics_allocations,
            0,
        );
        self.telemetry
            .resource
            .async_node_capability_equivalence_count += 1;

        Ok(AsyncNodeCapabilityEquivalenceReport::new(
            declaration,
            alias_lowering_proof,
            historical_parity_report,
            performance,
        ))
    }
}
