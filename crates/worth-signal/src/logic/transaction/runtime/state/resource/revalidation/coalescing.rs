use super::super::observation::output_continuity::ResourceTerminalVisibilityCause;
use super::super::timeout::plan::ScheduledResourceTimeoutAdmission;
use super::super::ResourceRuntimeState;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;
use crate::data::temporal::ClockTick;
use crate::state::SignalBranchId;

struct RevalidationCoalescingInput {
    intent: ResourceRevalidationIntent,
    descriptor_id: ResourceDescriptorId,
    active_request_id: ResourceRequestId,
    branch_id: SignalBranchId,
    generation_started_tick: ClockTick,
    freshness_decision: ResourceRevalidationFreshnessDecision,
    evidence: ResourceRevalidationEvidence,
    revalidation_decision_digest: ResourcePolicyDigest,
    resolved_timeout: Option<ScheduledResourceTimeoutAdmission>,
}

struct PreparedRevalidationCoalescing {
    node: ResourceNodeId,
    active_request_id: ResourceRequestId,
    active_in_flight: InFlightResourceRequest,
    coalesced_request: AdmittedResourceRequest,
    coalesced_in_flight: InFlightResourceRequest,
    transition: ResourceLifecycleTransition,
    freshness_decision: ResourceRevalidationFreshnessDecision,
    evidence: ResourceRevalidationEvidence,
    revalidation_decision_digest: ResourcePolicyDigest,
    resolved_timeout: Option<ScheduledResourceTimeoutAdmission>,
    temporal_wake_footprint: u32,
    terminal_visibility_classified: bool,
}

impl ResourceRuntimeState {
    pub(in crate::logic::transaction::runtime::state::resource) fn coalesce_revalidation(
        &mut self,
        intent: ResourceRevalidationIntent,
        descriptor_id: ResourceDescriptorId,
        active_request_id: ResourceRequestId,
        branch_id: SignalBranchId,
        generation_started_tick: ClockTick,
        freshness_decision: ResourceRevalidationFreshnessDecision,
        evidence: ResourceRevalidationEvidence,
        revalidation_decision_digest: ResourcePolicyDigest,
        resolved_timeout: Option<ScheduledResourceTimeoutAdmission>,
        mut telemetry: Option<&mut ResourceTelemetry>,
    ) -> ResourceRevalidationReport {
        let input = RevalidationCoalescingInput {
            intent,
            descriptor_id,
            active_request_id,
            branch_id,
            generation_started_tick,
            freshness_decision,
            evidence,
            revalidation_decision_digest,
            resolved_timeout,
        };
        let active_in_flight = self.exact_revalidation_coalescing_target(&input);
        let prepared =
            self.prepare_revalidation_coalescing(input, active_in_flight, telemetry.as_deref_mut());
        let PreparedRevalidationCoalescing {
            node,
            active_request_id,
            active_in_flight,
            coalesced_request,
            coalesced_in_flight,
            transition,
            freshness_decision,
            evidence,
            revalidation_decision_digest,
            resolved_timeout,
            temporal_wake_footprint,
            terminal_visibility_classified,
        } = prepared;
        self.install_revalidation_coalescing(
            coalesced_request,
            coalesced_in_flight,
            telemetry.as_deref_mut(),
        );
        self.transfer_revalidation_timeout_wake(
            active_request_id,
            resolved_timeout,
            telemetry.as_deref_mut(),
        );
        if let Some(telemetry) = telemetry.as_deref_mut() {
            telemetry.resource_revalidation_admission_count += 1;
        }
        let performance = Self::record_boundary_performance_optional(
            telemetry.as_deref_mut(),
            ResourceBoundaryPerformanceEnvelope::revalidation_admission(
                1,
                0,
                1,
                temporal_wake_footprint,
            )
            .with_coalescing_width(1)
            .with_output_continuity_classification_width(u32::from(terminal_visibility_classified)),
        );
        self.build_revalidation_coalescing_report(
            node,
            active_in_flight,
            coalesced_request,
            freshness_decision,
            evidence,
            revalidation_decision_digest,
            transition,
            performance,
        )
    }

    fn exact_revalidation_coalescing_target(
        &self,
        input: &RevalidationCoalescingInput,
    ) -> InFlightResourceRequest {
        let active_in_flight = self
            .in_flight_by_request
            .get(&input.active_request_id)
            .cloned()
            .expect("coalesced revalidation requires an active request");
        let request_intent = match input.intent.transaction_deadline() {
            Some(deadline) => {
                ResourceRequestIntent::with_transaction_deadline(input.intent.node(), deadline)
            }
            None => ResourceRequestIntent::new(input.intent.node()),
        };
        let exact_intent =
            active_in_flight.request_intent_digest() == &request_intent.canonical_digest();
        let exact_freshness = active_in_flight
            .revalidation_freshness_decision()
            .as_ref()
            .is_some_and(|existing| {
                existing.class() == input.freshness_decision.class()
                    && existing.freshness_digest() == input.freshness_decision.freshness_digest()
            });
        assert!(
            active_in_flight.status() == ResourceInFlightStatus::Active
                && active_in_flight.lifecycle() == ResourceLifecycleClass::Pending
                && exact_intent
                && exact_freshness,
            "revalidation coalescing requires exact active intent and freshness equivalence"
        );
        active_in_flight
    }

    fn prepare_revalidation_coalescing(
        &mut self,
        input: RevalidationCoalescingInput,
        active_in_flight: InFlightResourceRequest,
        mut telemetry: Option<&mut ResourceTelemetry>,
    ) -> PreparedRevalidationCoalescing {
        let node = input.intent.node();
        let request_intent = match input.intent.transaction_deadline() {
            Some(deadline) => ResourceRequestIntent::with_transaction_deadline(node, deadline),
            None => ResourceRequestIntent::new(node),
        };
        let request_id = self.issue_request_id();
        let coalesced_request = AdmittedResourceRequest::new(
            request_id,
            self.issue_generation(),
            ResourceBranchEpoch::new(input.branch_id, self.restore_epoch),
            ResourceAttemptId::ZERO,
        );
        let lifecycle_ordinal = self.issue_lifecycle_ordinal();
        let (output_continuity, terminal_visibility_classified) = self
            .classify_terminal_output_continuity_for_node_optional(
                node,
                input.descriptor_id,
                ResourceTerminalVisibilityCause::Supersession,
                telemetry.as_deref_mut(),
            );
        let transition = ResourceLifecycleTransition::new(
            node,
            ResourceLifecycleClass::Pending,
            ResourceLifecycleClass::Superseded,
            ResourceLifecycleTransitionKind::RequestSuperseded,
            lifecycle_ordinal,
            output_continuity,
        );
        let mut coalesced_in_flight = InFlightResourceRequest::new(
            coalesced_request.handle(),
            node,
            input.descriptor_id,
            coalesced_request.handle().generation(),
            ResourceAttemptId::ZERO,
            request_intent.canonical_digest(),
            input.generation_started_tick,
            lifecycle_ordinal,
            active_in_flight.timeout_duration(),
            active_in_flight.timeout_due_tick(),
            active_in_flight.timeout_outcome_class(),
            active_in_flight.timeout_deadline_authority(),
            active_in_flight.timeout_decision_digest().clone(),
        );
        coalesced_in_flight.attach_revalidation_freshness(&input.freshness_decision);
        coalesced_in_flight.supersede(lifecycle_ordinal, active_in_flight.handle());
        PreparedRevalidationCoalescing {
            node,
            active_request_id: input.active_request_id,
            active_in_flight,
            coalesced_request,
            coalesced_in_flight,
            transition,
            freshness_decision: input.freshness_decision,
            evidence: input.evidence,
            revalidation_decision_digest: input.revalidation_decision_digest,
            temporal_wake_footprint: u32::from(input.resolved_timeout.is_some()),
            resolved_timeout: input.resolved_timeout,
            terminal_visibility_classified,
        }
    }

    fn install_revalidation_coalescing(
        &mut self,
        coalesced_request: AdmittedResourceRequest,
        coalesced_in_flight: InFlightResourceRequest,
        telemetry: Option<&mut ResourceTelemetry>,
    ) {
        self.in_flight_by_request
            .insert(coalesced_request.handle().request_id(), coalesced_in_flight);
        self.mark_terminal_in_flight(coalesced_request.handle().request_id());
        if let Some(telemetry) = telemetry {
            telemetry.resource_request_admission_count += 1;
            telemetry.resource_revalidation_coalesced_count += 1;
            telemetry.resource_in_flight_request_count = self.in_flight_by_request.len() as u64;
            telemetry.resource_in_flight_frontier_width = telemetry
                .resource_in_flight_frontier_width
                .max(self.active_request_by_node.len() as u64);
        }
    }

    fn transfer_revalidation_timeout_wake(
        &mut self,
        active_request_id: ResourceRequestId,
        resolved_timeout: Option<ScheduledResourceTimeoutAdmission>,
        telemetry: Option<&mut ResourceTelemetry>,
    ) {
        if let Some(timeout) = resolved_timeout {
            self.in_flight_by_request
                .get_mut(&active_request_id)
                .expect("coalesced revalidation winner must remain active")
                .attach_timeout_wake(timeout.wake_id);
            if let Some(telemetry) = telemetry {
                telemetry.resource_timeout_temporal_wake_footprint = telemetry
                    .resource_timeout_temporal_wake_footprint
                    .saturating_add(1);
            }
        }
    }

    fn build_revalidation_coalescing_report(
        &self,
        node: ResourceNodeId,
        active_in_flight: InFlightResourceRequest,
        coalesced_request: AdmittedResourceRequest,
        freshness_decision: ResourceRevalidationFreshnessDecision,
        evidence: ResourceRevalidationEvidence,
        revalidation_decision_digest: ResourcePolicyDigest,
        transition: ResourceLifecycleTransition,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> ResourceRevalidationReport {
        let lifecycle = self
            .lifecycle_by_node
            .get(&node)
            .copied()
            .unwrap_or_else(|| {
                ResourceLifecycleSummary::new(
                    node,
                    ResourceLifecycleClass::Pending,
                    ResourceOutputContinuity::NoPriorOutput,
                    active_in_flight.lifecycle_ordinal(),
                )
            });
        let admitted_request = AdmittedResourceRequest::new(
            active_in_flight.handle().request_id(),
            active_in_flight.generation(),
            active_in_flight.handle().branch_epoch(),
            active_in_flight.attempt(),
        );
        ResourceRevalidationReport::admitted(
            AdmittedResourceRevalidation::new(
                admitted_request,
                freshness_decision.clone(),
                evidence,
                Some(ResourceRevalidationCoalescing::new(
                    active_in_flight.handle(),
                    coalesced_request,
                    freshness_decision,
                    transition,
                )),
                None,
                revalidation_decision_digest,
            ),
            lifecycle,
            transition,
            performance,
        )
    }
}
