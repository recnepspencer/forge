use super::super::observation::output_continuity::ResourceTerminalVisibilityCause;
use super::super::ResourceRuntimeState;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;

#[derive(Debug, Clone)]
pub(in crate::logic::transaction::runtime::state::resource) struct AppliedResourceCancellation {
    pub(in crate::logic::transaction::runtime::state::resource) cancelled: CancelledResourceRequest,
    pub(in crate::logic::transaction::runtime::state::resource) lifecycle: ResourceLifecycleSummary,
    pub(in crate::logic::transaction::runtime::state::resource) transition:
        ResourceLifecycleTransition,
    pub(in crate::logic::transaction::runtime::state::resource) propagated_dependents:
        Vec<CancelledResourceRequest>,
}

struct CancellationPolicyBasis {
    cancellation_digest: ResourcePolicyDigest,
    requests_host_advisory: bool,
    grace_period: Option<crate::data::temporal::TemporalDuration>,
    declared_dependent_cancellation_nodes: Vec<ResourceNodeId>,
}

struct PreparedCancellationApplication {
    handle: ResourceRequestHandle,
    node: ResourceNodeId,
    reason: ResourceCancellationReason,
    cancellation_ordinal: ResourceCancellationOrdinal,
    policy: CancellationPolicyBasis,
    lifecycle: ResourceLifecycleSummary,
    transition: ResourceLifecycleTransition,
    lifecycle_ordinal: ResourceLifecycleOrdinal,
}

impl ResourceRuntimeState {
    pub(in crate::logic::transaction::runtime::state::resource) fn apply_resource_cancellation(
        &mut self,
        request_id: ResourceRequestId,
        reason: ResourceCancellationReason,
        visited: &mut std::collections::BTreeSet<ResourceRequestId>,
        mut telemetry: Option<&mut ResourceTelemetry>,
    ) -> Option<AppliedResourceCancellation> {
        if !visited.insert(request_id) {
            return None;
        }
        let in_flight = self.in_flight_by_request.get(&request_id)?.clone();
        if in_flight.status() != ResourceInFlightStatus::Active
            || in_flight.lifecycle() != ResourceLifecycleClass::Pending
        {
            return None;
        }

        let policy = self.cancellation_policy_basis(&in_flight);
        let prepared = self.prepare_cancellation_application(
            in_flight.clone(),
            reason,
            policy,
            telemetry.as_deref_mut(),
        );
        self.apply_cancellation_transition(&prepared, telemetry.as_deref_mut());
        let host_advisory =
            self.build_host_cancellation_advisory(&prepared, telemetry.as_deref_mut());
        let grace_window =
            self.build_cancellation_grace_window(&prepared, telemetry.as_deref_mut());
        let cancelled = CancelledResourceRequest::new(
            prepared.handle,
            prepared.cancellation_ordinal,
            prepared.reason,
            prepared.policy.cancellation_digest.clone(),
            host_advisory,
            grace_window,
            prepared.transition,
        );
        let propagated_dependents = self.propagate_dependent_cancellations(
            &prepared.policy.declared_dependent_cancellation_nodes,
            visited,
            telemetry.as_deref_mut(),
        );
        Some(AppliedResourceCancellation {
            cancelled,
            lifecycle: prepared.lifecycle,
            transition: prepared.transition,
            propagated_dependents,
        })
    }

    fn cancellation_policy_basis(
        &self,
        in_flight: &InFlightResourceRequest,
    ) -> CancellationPolicyBasis {
        let descriptor = self
            .descriptors
            .get(&in_flight.descriptor_id())
            .expect("in-flight cancellation must retain a declared descriptor");
        let plan = descriptor.cancellation_decision_plan();
        CancellationPolicyBasis {
            cancellation_digest: plan.decision_digest().clone(),
            requests_host_advisory: plan.requests_host_advisory(),
            grace_period: plan.grace_period(),
            declared_dependent_cancellation_nodes: plan
                .declared_dependent_cancellation_nodes()
                .to_vec(),
        }
    }

    fn prepare_cancellation_application(
        &mut self,
        in_flight: InFlightResourceRequest,
        reason: ResourceCancellationReason,
        policy: CancellationPolicyBasis,
        mut telemetry: Option<&mut ResourceTelemetry>,
    ) -> PreparedCancellationApplication {
        let lifecycle_ordinal = self.issue_lifecycle_ordinal();
        let cancellation_ordinal = self.issue_cancellation_ordinal();
        let (output_continuity, _) = self.classify_terminal_output_continuity_for_node_optional(
            in_flight.node(),
            in_flight.descriptor_id(),
            ResourceTerminalVisibilityCause::Cancellation,
            telemetry.as_deref_mut(),
        );
        let transition = ResourceLifecycleTransition::new(
            in_flight.node(),
            ResourceLifecycleClass::Pending,
            ResourceLifecycleClass::Cancelled,
            ResourceLifecycleTransitionKind::RequestCancelled,
            lifecycle_ordinal,
            output_continuity,
        );
        let lifecycle = ResourceLifecycleSummary::new(
            in_flight.node(),
            ResourceLifecycleClass::Cancelled,
            output_continuity,
            lifecycle_ordinal,
        );
        PreparedCancellationApplication {
            handle: in_flight.handle(),
            node: in_flight.node(),
            reason,
            cancellation_ordinal,
            policy,
            lifecycle,
            transition,
            lifecycle_ordinal,
        }
    }

    fn apply_cancellation_transition(
        &mut self,
        prepared: &PreparedCancellationApplication,
        telemetry: Option<&mut ResourceTelemetry>,
    ) {
        self.in_flight_by_request
            .get_mut(&prepared.handle.request_id())
            .expect("in-flight request was just resolved for cancellation")
            .cancel(prepared.lifecycle_ordinal);
        self.mark_terminal_in_flight(prepared.handle.request_id());
        if self
            .active_request_by_node
            .get(&prepared.node)
            .is_some_and(|active| *active == prepared.handle.request_id())
        {
            self.active_request_by_node.remove(&prepared.node);
        }
        self.lifecycle_by_node
            .insert(prepared.node, prepared.lifecycle);
        self.clear_latest_denied_completion_for_node(prepared.node);
        self.retry_budget_ledger
            .clear_request_generation(prepared.handle.generation());
        if let Some(telemetry) = telemetry {
            telemetry.resource_cancellation_policy_decision_count += 1;
            telemetry.resource_runtime_hard_cancellation_count += 1;
            telemetry.resource_cancellation_count += 1;
        }
    }

    fn build_host_cancellation_advisory(
        &self,
        prepared: &PreparedCancellationApplication,
        telemetry: Option<&mut ResourceTelemetry>,
    ) -> Option<ResourceHostCancellationAdvisory> {
        prepared.policy.requests_host_advisory.then(|| {
            if let Some(telemetry) = telemetry {
                telemetry.resource_host_cancellation_advisory_count += 1;
            }
            ResourceHostCancellationAdvisory::requested(prepared.policy.cancellation_digest.clone())
        })
    }

    fn build_cancellation_grace_window(
        &self,
        prepared: &PreparedCancellationApplication,
        telemetry: Option<&mut ResourceTelemetry>,
    ) -> Option<ResourceCancellationGraceWindow> {
        prepared.policy.grace_period.map(|duration| {
            if let Some(telemetry) = telemetry {
                telemetry.resource_cancellation_grace_period_count += 1;
            }
            ResourceCancellationGraceWindow::new(duration)
        })
    }

    fn propagate_dependent_cancellations(
        &mut self,
        dependent_nodes: &[ResourceNodeId],
        visited: &mut std::collections::BTreeSet<ResourceRequestId>,
        mut telemetry: Option<&mut ResourceTelemetry>,
    ) -> Vec<CancelledResourceRequest> {
        let mut propagated = Vec::new();
        for dependent_node in dependent_nodes {
            let Some(dependent_request_id) =
                self.active_request_by_node.get(dependent_node).copied()
            else {
                continue;
            };
            let Some(dependent_cancellation) = self.apply_resource_cancellation(
                dependent_request_id,
                ResourceCancellationReason::RuntimePolicy,
                visited,
                telemetry.as_deref_mut(),
            ) else {
                continue;
            };
            if let Some(telemetry) = telemetry.as_deref_mut() {
                telemetry.resource_dependent_cancellation_propagation_count += 1;
            }
            propagated.push(dependent_cancellation.cancelled.clone());
            propagated.extend(dependent_cancellation.propagated_dependents);
        }
        propagated
    }
}
