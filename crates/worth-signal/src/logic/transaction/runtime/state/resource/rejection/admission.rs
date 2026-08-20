use super::super::observation::output_continuity::ResourceTerminalVisibilityCause;
use super::super::ResourceRuntimeState;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;

struct PreparedRejectionAdmission {
    handle: ResourceRequestHandle,
    node: ResourceNodeId,
    reason: ResourceRejectionReason,
    rejection_ordinal: ResourceRejectionOrdinal,
    rejection_digest: ResourcePolicyDigest,
    lifecycle: ResourceLifecycleSummary,
    transition: ResourceLifecycleTransition,
    lifecycle_ordinal: ResourceLifecycleOrdinal,
    terminal_visibility_classified: bool,
}

impl ResourceRuntimeState {
    pub fn reject_resource_request(
        &mut self,
        handle: ResourceRequestHandle,
        reason: ResourceRejectionReason,
        mut telemetry: Option<&mut ResourceTelemetry>,
    ) -> ResourceRejectionReport {
        let in_flight = match self.validate_rejection_admission(handle) {
            Ok(in_flight) => in_flight,
            Err(class) => {
                return self.deny_rejection(handle.request_id(), class, telemetry.as_deref_mut());
            }
        };
        let prepared =
            self.prepare_rejection_admission(in_flight, handle, reason, telemetry.as_deref_mut());
        self.apply_rejection_admission(&prepared, telemetry.as_deref_mut());
        let performance = Self::record_boundary_performance_optional(
            telemetry.as_deref_mut(),
            ResourceBoundaryPerformanceEnvelope::rejection_admission(1, 0)
                .with_output_continuity_classification_width(u32::from(
                    prepared.terminal_visibility_classified,
                )),
        );
        ResourceRejectionReport::admitted(
            RejectedResourceRequest::new(
                prepared.handle,
                prepared.node,
                prepared.rejection_ordinal,
                prepared.reason,
                prepared.rejection_digest,
                prepared.transition,
            ),
            prepared.lifecycle,
            prepared.transition,
            performance,
        )
    }

    fn validate_rejection_admission(
        &self,
        handle: ResourceRequestHandle,
    ) -> Result<InFlightResourceRequest, ResourceRejectionDenialClass> {
        let Some(in_flight) = self.in_flight_by_request.get(&handle.request_id()).cloned() else {
            return Err(ResourceRejectionDenialClass::UnknownOrStaleRequest);
        };
        if in_flight.handle() != handle {
            return Err(ResourceRejectionDenialClass::UnknownOrStaleRequest);
        }
        if in_flight.status() != ResourceInFlightStatus::Active
            || in_flight.lifecycle() != ResourceLifecycleClass::Pending
        {
            return Err(ResourceRejectionDenialClass::NonActiveRequest);
        }
        Ok(in_flight)
    }

    fn prepare_rejection_admission(
        &mut self,
        in_flight: InFlightResourceRequest,
        handle: ResourceRequestHandle,
        reason: ResourceRejectionReason,
        telemetry: Option<&mut ResourceTelemetry>,
    ) -> PreparedRejectionAdmission {
        let rejection_digest = ResourcePolicyDigest::new(format!(
            "resource-rejection:{}:{}",
            handle.request_id().get(),
            match reason {
                ResourceRejectionReason::HostFailure => "host-failure",
                ResourceRejectionReason::SemanticFailure => "semantic-failure",
            }
        ));
        let lifecycle_ordinal = self.issue_lifecycle_ordinal();
        let rejection_ordinal = self.issue_rejection_ordinal();
        let (output_continuity, terminal_visibility_classified) = self
            .classify_terminal_output_continuity_for_node_optional(
                in_flight.node(),
                in_flight.descriptor_id(),
                ResourceTerminalVisibilityCause::Rejection,
                telemetry,
            );
        let transition = ResourceLifecycleTransition::new(
            in_flight.node(),
            ResourceLifecycleClass::Pending,
            ResourceLifecycleClass::Rejected,
            ResourceLifecycleTransitionKind::RequestRejected,
            lifecycle_ordinal,
            output_continuity,
        );
        let lifecycle = ResourceLifecycleSummary::new(
            in_flight.node(),
            ResourceLifecycleClass::Rejected,
            output_continuity,
            lifecycle_ordinal,
        );
        PreparedRejectionAdmission {
            handle,
            node: in_flight.node(),
            reason,
            rejection_ordinal,
            rejection_digest,
            lifecycle,
            transition,
            lifecycle_ordinal,
            terminal_visibility_classified,
        }
    }

    fn apply_rejection_admission(
        &mut self,
        prepared: &PreparedRejectionAdmission,
        telemetry: Option<&mut ResourceTelemetry>,
    ) {
        self.in_flight_by_request
            .get_mut(&prepared.handle.request_id())
            .expect("in-flight request was just resolved for rejection")
            .reject(prepared.lifecycle_ordinal);
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
        if let Some(telemetry) = telemetry {
            telemetry.resource_rejection_admission_count += 1;
            match prepared.reason {
                ResourceRejectionReason::HostFailure => {
                    telemetry.resource_host_failure_rejection_count += 1
                }
                ResourceRejectionReason::SemanticFailure => {
                    telemetry.resource_semantic_rejection_count += 1
                }
            }
            telemetry.resource_in_flight_request_count = self.in_flight_by_request.len() as u64;
        }
    }
}
