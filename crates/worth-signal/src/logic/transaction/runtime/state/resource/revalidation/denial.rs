use super::super::ResourceRuntimeState;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;

impl ResourceRuntimeState {
    pub fn deny_forced_revalidation_for_report(
        &mut self,
        node: ResourceNodeId,
        handle: ResourceRequestHandle,
        class: ResourceRevalidationDenialClass,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRevalidationReport {
        self.deny_revalidation(
            ResourceRevalidationIntent::with_expected_active(node, handle),
            class,
            telemetry,
        )
    }
    pub fn deny_resource_revalidation_for_report(
        &mut self,
        node: ResourceNodeId,
        class: ResourceRevalidationDenialClass,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRevalidationReport {
        self.deny_revalidation(ResourceRevalidationIntent::new(node), class, telemetry)
    }
    pub(in crate::logic::transaction::runtime::state::resource) fn deny_revalidation(
        &mut self,
        intent: ResourceRevalidationIntent,
        class: ResourceRevalidationDenialClass,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRevalidationReport {
        telemetry.resource_revalidation_denial_count += 1;
        match class {
            ResourceRevalidationDenialClass::UndeclaredResourceNode => {
                telemetry.resource_undeclared_owner_denial_count += 1
            }
            ResourceRevalidationDenialClass::ActiveRequestRequiresExpectedHandle => {
                telemetry.resource_revalidation_active_requires_expected_denial_count += 1
            }
            ResourceRevalidationDenialClass::ExpectedActiveRequestMismatch => {
                telemetry.resource_revalidation_expected_mismatch_denial_count += 1
            }
            ResourceRevalidationDenialClass::ForcedRevalidationPolicyDisabled => {
                telemetry.resource_forced_revalidation_policy_denial_count += 1
            }
            ResourceRevalidationDenialClass::ActiveHandleProofMismatch => {
                telemetry.resource_revalidation_active_handle_proof_mismatch_denial_count += 1
            }
            ResourceRevalidationDenialClass::DependencyChangeRevalidationPolicyDisabled => {
                telemetry.resource_revalidation_dependency_change_policy_denial_count += 1
            }
            ResourceRevalidationDenialClass::DependencyChangeProofMismatch => {
                telemetry.resource_revalidation_dependency_change_proof_mismatch_denial_count += 1
            }
            ResourceRevalidationDenialClass::ObserverDemandRevalidationPolicyDisabled => {
                telemetry.resource_revalidation_observer_demand_policy_denial_count += 1
            }
            ResourceRevalidationDenialClass::ObserverDemandProofMismatch => {
                telemetry.resource_revalidation_observer_demand_proof_mismatch_denial_count += 1
            }
            ResourceRevalidationDenialClass::TerminalStateRevalidationPolicyDisabled => {
                telemetry.resource_revalidation_terminal_state_policy_denial_count += 1
            }
            ResourceRevalidationDenialClass::TerminalStateProofMismatch => {
                telemetry.resource_revalidation_terminal_state_proof_mismatch_denial_count += 1
            }
            ResourceRevalidationDenialClass::FulfilledLifecycleRevalidationPolicyDisabled => {
                telemetry.resource_revalidation_fulfilled_lifecycle_policy_denial_count += 1
            }
            ResourceRevalidationDenialClass::FulfilledLifecycleProofMismatch => {
                telemetry.resource_revalidation_fulfilled_lifecycle_proof_mismatch_denial_count += 1
            }
            ResourceRevalidationDenialClass::StaleAfterRevalidationPolicyDisabled => {
                telemetry.resource_revalidation_stale_after_policy_denial_count += 1
            }
            ResourceRevalidationDenialClass::StaleAfterWakeMismatch => {
                telemetry.resource_revalidation_stale_after_wake_mismatch_denial_count += 1
            }
            ResourceRevalidationDenialClass::StaleAfterRequiresFulfilledLifecycle => {
                telemetry.resource_revalidation_stale_after_fulfilled_only_denial_count += 1
            }
        }
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::revalidation_admission(0, 1, 0, 0),
        );
        ResourceRevalidationReport::denied(
            DeniedResourceRevalidation::new(
                intent.node(),
                intent
                    .expected_active()
                    .map(ResourceRequestHandle::request_id),
                class,
            ),
            performance,
        )
    }
}
