use super::super::observation::output_continuity::ResourceTerminalVisibilityCause;
use super::super::ResourceRuntimeState;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;
use crate::data::temporal::ClockTick;
use crate::state::SignalBranchId;

pub(super) struct RequestIntentCoalescingInput {
    node: ResourceNodeId,
    descriptor_id: ResourceDescriptorId,
    request_intent_digest: ResourceRequestIntentDigest,
    branch_id: SignalBranchId,
    generation_started_tick: ClockTick,
}

impl RequestIntentCoalescingInput {
    pub(super) fn new(
        node: ResourceNodeId,
        descriptor_id: ResourceDescriptorId,
        request_intent_digest: ResourceRequestIntentDigest,
        branch_id: SignalBranchId,
        generation_started_tick: ClockTick,
    ) -> Self {
        Self {
            node,
            descriptor_id,
            request_intent_digest,
            branch_id,
            generation_started_tick,
        }
    }
}

struct RequestIntentCoalescingCandidate {
    active_in_flight: InFlightResourceRequest,
    supersession_plan: ResourceSupersessionDecisionPlan,
}

struct PreparedRequestIntentCoalescing {
    coalesced_request: AdmittedResourceRequest,
    coalesced_in_flight: InFlightResourceRequest,
    transition: ResourceLifecycleTransition,
    supersession_ordinal: ResourceSupersessionOrdinal,
    terminal_visibility_classified: bool,
}

impl ResourceRuntimeState {
    pub(in crate::logic::transaction::runtime::state::resource::request) fn try_coalesce_equivalent_request_intent(
        &mut self,
        input: RequestIntentCoalescingInput,
        mut telemetry: Option<&mut ResourceTelemetry>,
    ) -> Option<ResourceRequestAdmissionReport> {
        let candidate = self.request_intent_coalescing_candidate(&input)?;
        if let Some(telemetry) = telemetry.as_deref_mut() {
            telemetry.resource_supersession_policy_decision_count += 1;
            telemetry.resource_intent_equivalence_coalescing_count += 1;
        }

        let prepared = self.prepare_request_intent_coalescing(
            &input,
            &candidate.active_in_flight,
            telemetry.as_deref_mut(),
        );
        let coalesced_request = prepared.coalesced_request;
        let coalesced_in_flight = prepared.coalesced_in_flight;
        let transition = prepared.transition;
        let supersession_ordinal = prepared.supersession_ordinal;
        let terminal_visibility_classified = prepared.terminal_visibility_classified;
        self.install_request_intent_coalescing(
            coalesced_request,
            coalesced_in_flight,
            telemetry.as_deref_mut(),
        );

        let density_strategy =
            ResourceDensityStrategy::request_pressure(self.in_flight_by_request.len() as u32);
        let performance = Self::record_boundary_performance_optional(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::request_admission(1, 0, 1)
                .with_density_strategy(density_strategy)
                .with_output_continuity_classification_width(u32::from(
                    terminal_visibility_classified,
                )),
        );
        Some(self.build_request_intent_coalescing_report(
            &input,
            candidate,
            coalesced_request,
            supersession_ordinal,
            transition,
            performance,
        ))
    }

    fn request_intent_coalescing_candidate(
        &self,
        input: &RequestIntentCoalescingInput,
    ) -> Option<RequestIntentCoalescingCandidate> {
        let active_request_id = self.active_request_by_node.get(&input.node).copied()?;
        let active_in_flight = self.in_flight_by_request.get(&active_request_id)?.clone();
        if active_in_flight.status() != ResourceInFlightStatus::Active
            || active_in_flight.lifecycle() != ResourceLifecycleClass::Pending
            || active_in_flight.request_intent_digest() != &input.request_intent_digest
        {
            return None;
        }
        let supersession_plan = self
            .descriptors
            .get(&input.descriptor_id)?
            .supersession_decision_plan()
            .clone();
        supersession_plan
            .permits_intent_equivalence_coalescing()
            .then_some(RequestIntentCoalescingCandidate {
                active_in_flight,
                supersession_plan,
            })
    }

    fn prepare_request_intent_coalescing(
        &mut self,
        input: &RequestIntentCoalescingInput,
        active_in_flight: &InFlightResourceRequest,
        telemetry: Option<&mut ResourceTelemetry>,
    ) -> PreparedRequestIntentCoalescing {
        let request_id = self.issue_request_id();
        let generation = self.issue_generation();
        let branch_epoch = ResourceBranchEpoch::new(input.branch_id, self.restore_epoch);
        let coalesced_request = AdmittedResourceRequest::new(
            request_id,
            generation,
            branch_epoch,
            ResourceAttemptId::ZERO,
        );
        let lifecycle_ordinal = self.issue_lifecycle_ordinal();
        let supersession_ordinal = self.issue_supersession_ordinal();
        let (output_continuity, terminal_visibility_classified) = self
            .classify_terminal_output_continuity_for_node_optional(
                input.node,
                input.descriptor_id,
                ResourceTerminalVisibilityCause::Supersession,
                telemetry,
            );
        let transition = ResourceLifecycleTransition::new(
            input.node,
            ResourceLifecycleClass::Pending,
            ResourceLifecycleClass::Superseded,
            ResourceLifecycleTransitionKind::RequestSuperseded,
            lifecycle_ordinal,
            output_continuity,
        );
        let mut coalesced_in_flight = InFlightResourceRequest::new(
            coalesced_request.handle(),
            input.node,
            input.descriptor_id,
            generation,
            ResourceAttemptId::ZERO,
            input.request_intent_digest.clone(),
            input.generation_started_tick,
            lifecycle_ordinal,
            active_in_flight.timeout_duration(),
            active_in_flight.timeout_due_tick(),
            active_in_flight.timeout_outcome_class(),
            active_in_flight.timeout_deadline_authority(),
            active_in_flight.timeout_decision_digest().clone(),
        );
        coalesced_in_flight.supersede(lifecycle_ordinal, active_in_flight.handle());
        PreparedRequestIntentCoalescing {
            coalesced_request,
            coalesced_in_flight,
            transition,
            supersession_ordinal,
            terminal_visibility_classified,
        }
    }

    fn install_request_intent_coalescing(
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
            telemetry.resource_in_flight_request_count = self.in_flight_by_request.len() as u64;
            telemetry.resource_in_flight_frontier_width = telemetry
                .resource_in_flight_frontier_width
                .max(self.active_request_by_node.len() as u64);
        }
    }

    fn build_request_intent_coalescing_report(
        &self,
        input: &RequestIntentCoalescingInput,
        candidate: RequestIntentCoalescingCandidate,
        coalesced_request: AdmittedResourceRequest,
        supersession_ordinal: ResourceSupersessionOrdinal,
        transition: ResourceLifecycleTransition,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> ResourceRequestAdmissionReport {
        let lifecycle = self
            .lifecycle_by_node
            .get(&input.node)
            .copied()
            .unwrap_or_else(|| {
                ResourceLifecycleSummary::new(
                    input.node,
                    ResourceLifecycleClass::Pending,
                    ResourceOutputContinuity::NoPriorOutput,
                    candidate.active_in_flight.lifecycle_ordinal(),
                )
            });
        let admitted_request = AdmittedResourceRequest::new(
            candidate.active_in_flight.handle().request_id(),
            candidate.active_in_flight.generation(),
            candidate.active_in_flight.handle().branch_epoch(),
            candidate.active_in_flight.attempt(),
        );
        ResourceRequestAdmissionReport::new(
            admitted_request,
            lifecycle,
            transition,
            None,
            Some(ResourceIntentEquivalenceCoalescing::new(
                supersession_ordinal,
                candidate.active_in_flight.handle(),
                coalesced_request,
                input.request_intent_digest.clone(),
                candidate.supersession_plan.decision_digest().clone(),
                transition,
            )),
            performance,
        )
    }
}
