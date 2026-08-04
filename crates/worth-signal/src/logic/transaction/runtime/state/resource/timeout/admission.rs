use super::super::observation::output_continuity::ResourceTerminalVisibilityCause;
use super::super::ResourceRuntimeState;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;
use crate::data::temporal::ReadyTemporalWake;

struct PreparedTimeoutAdmission {
    request_id: ResourceRequestId,
    node: ResourceNodeId,
    lifecycle_ordinal: ResourceLifecycleOrdinal,
    timed_out: TimedOutResourceRequest,
    lifecycle: ResourceLifecycleSummary,
    transition: ResourceLifecycleTransition,
    terminal_visibility_classified: bool,
}

impl ResourceRuntimeState {
    pub fn admit_resource_timeout(
        &mut self,
        handle: ResourceRequestHandle,
        ready_wake: ReadyTemporalWake,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceTimeoutReport {
        telemetry.resource_hot_in_flight_lookup_count += 1;
        let in_flight = match self.validate_timeout_admission(handle, &ready_wake) {
            Ok(in_flight) => in_flight,
            Err(class) => return self.deny_timeout(handle.request_id(), class, telemetry),
        };
        let prepared = self.prepare_timeout_admission(in_flight, handle, ready_wake, telemetry);
        self.apply_timeout_admission(&prepared, telemetry);
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::timeout_admission(1, 0, 1)
                .with_output_continuity_classification_width(u32::from(
                    prepared.terminal_visibility_classified,
                )),
        );
        ResourceTimeoutReport::admitted(
            prepared.timed_out,
            prepared.lifecycle,
            prepared.transition,
            performance,
        )
    }

    fn validate_timeout_admission(
        &self,
        handle: ResourceRequestHandle,
        ready_wake: &ReadyTemporalWake,
    ) -> Result<InFlightResourceRequest, ResourceTimeoutDenialClass> {
        let request_id = handle.request_id();
        let Some(in_flight) = self.in_flight_by_request.get(&request_id).cloned() else {
            return Err(ResourceTimeoutDenialClass::UnknownOrStaleRequest);
        };
        if in_flight.handle() != handle {
            return Err(ResourceTimeoutDenialClass::UnknownOrStaleRequest);
        }
        if in_flight.status() != ResourceInFlightStatus::Active
            || in_flight.lifecycle() != ResourceLifecycleClass::Pending
        {
            return Err(ResourceTimeoutDenialClass::NonActiveRequest);
        }
        let Some(timeout_wake_id) = in_flight.timeout_wake_id() else {
            return Err(ResourceTimeoutDenialClass::MissingTimeoutWake);
        };
        if timeout_wake_id != ready_wake.id() {
            return Err(ResourceTimeoutDenialClass::WakeMismatch);
        }
        if in_flight.timeout_duration().is_none() {
            return Err(ResourceTimeoutDenialClass::MissingTimeoutWake);
        }
        Ok(in_flight)
    }

    fn prepare_timeout_admission(
        &mut self,
        in_flight: InFlightResourceRequest,
        handle: ResourceRequestHandle,
        ready_wake: ReadyTemporalWake,
        telemetry: &mut ResourceTelemetry,
    ) -> PreparedTimeoutAdmission {
        let (output_continuity, terminal_visibility_classified) = self
            .classify_terminal_output_continuity_for_node(
                in_flight.node(),
                in_flight.descriptor_id(),
                ResourceTerminalVisibilityCause::Timeout,
                telemetry,
            );
        let lifecycle_ordinal = self.issue_lifecycle_ordinal();
        let timeout_ordinal = self.issue_timeout_ordinal();
        let transition = ResourceLifecycleTransition::new(
            in_flight.node(),
            ResourceLifecycleClass::Pending,
            ResourceLifecycleClass::TimedOut,
            ResourceLifecycleTransitionKind::RequestTimedOut,
            lifecycle_ordinal,
            output_continuity,
        );
        let timed_out = TimedOutResourceRequest::new(
            handle,
            timeout_ordinal,
            ready_wake,
            in_flight
                .timeout_duration()
                .expect("validated timeout admission retains duration"),
            in_flight.timeout_outcome_class(),
            in_flight.timeout_deadline_authority(),
            in_flight.timeout_decision_digest().clone(),
            transition,
        );
        let lifecycle = ResourceLifecycleSummary::new(
            in_flight.node(),
            ResourceLifecycleClass::TimedOut,
            output_continuity,
            lifecycle_ordinal,
        );
        PreparedTimeoutAdmission {
            request_id: handle.request_id(),
            node: in_flight.node(),
            lifecycle_ordinal,
            timed_out,
            lifecycle,
            transition,
            terminal_visibility_classified,
        }
    }

    fn apply_timeout_admission(
        &mut self,
        prepared: &PreparedTimeoutAdmission,
        telemetry: &mut ResourceTelemetry,
    ) {
        self.in_flight_by_request
            .get_mut(&prepared.request_id)
            .expect("in-flight request was just resolved for timeout")
            .timeout(prepared.lifecycle_ordinal);
        self.mark_terminal_in_flight(prepared.request_id);
        if self
            .active_request_by_node
            .get(&prepared.node)
            .is_some_and(|active| *active == prepared.request_id)
        {
            self.active_request_by_node.remove(&prepared.node);
        }
        self.lifecycle_by_node
            .insert(prepared.node, prepared.lifecycle);
        self.clear_latest_denied_completion_for_node(prepared.node);
        telemetry.resource_timeout_admission_count += 1;
        telemetry.resource_in_flight_request_count = self.in_flight_by_request.len() as u64;
        telemetry.resource_timeout_temporal_wake_footprint = telemetry
            .resource_timeout_temporal_wake_footprint
            .saturating_add(1);
    }
}
