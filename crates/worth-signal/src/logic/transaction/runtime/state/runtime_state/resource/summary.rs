use crate::data::resource::{
    DeniedResourcePolicyRestoreCompatibility, LoweredResourceDescriptor,
    ResourceBranchRestoreReport, ResourceNodeDeclaration, ResourceNodeId,
    ResourcePolicyCompatibilityReport, ResourcePolicyRestoreCompatibilityProof,
    ResourceRuntimeSummary, ResourceRuntimeSummaryReadReport,
};

use super::super::SignalRuntime;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn resource_runtime_summary(&self) -> ResourceRuntimeSummary {
        self.resource.summary()
    }

    pub fn resource_runtime_summary_read_report(&mut self) -> ResourceRuntimeSummaryReadReport {
        let capture_telemetry = self.graph.captures_observation_surface(
            crate::logic::transaction::SignalObservationSurface::OptionalTelemetry,
        );
        self.resource
            .summary_read_report_optional(capture_telemetry.then_some(&mut self.telemetry.resource))
    }

    pub fn resource_descriptor_for_node(
        &self,
        node: ResourceNodeId,
    ) -> Option<&LoweredResourceDescriptor> {
        self.resource.descriptor_for_node(node)
    }

    pub fn classify_resource_policy_compatibility(
        &mut self,
        declaration: &ResourceNodeDeclaration,
    ) -> Result<ResourcePolicyCompatibilityReport, crate::data::error::SignalError> {
        let capture_telemetry = self.graph.captures_observation_surface(
            crate::logic::transaction::SignalObservationSurface::OptionalTelemetry,
        );
        self.resource.classify_policy_compatibility_optional(
            declaration,
            capture_telemetry.then_some(&mut self.telemetry.resource),
        )
    }

    pub fn admit_resource_policy_restore_compatibility(
        &mut self,
        declaration: &ResourceNodeDeclaration,
    ) -> Result<
        Result<ResourcePolicyRestoreCompatibilityProof, DeniedResourcePolicyRestoreCompatibility>,
        crate::data::error::SignalError,
    > {
        let capture_telemetry = self.graph.captures_observation_surface(
            crate::logic::transaction::SignalObservationSurface::OptionalTelemetry,
        );
        self.resource.admit_policy_restore_compatibility_optional(
            declaration,
            capture_telemetry.then_some(&mut self.telemetry.resource),
        )
    }

    pub fn latest_resource_branch_restore_report(&self) -> Option<ResourceBranchRestoreReport> {
        self.resource.latest_branch_restore_report()
    }
}
