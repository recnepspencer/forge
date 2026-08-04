use super::super::ResourceRuntimeState;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;
use crate::data::temporal::{ScheduledTemporalWake, TemporalWakeId};

struct ResourceTimeoutHeartbeatExtensionCandidate {
    node: ResourceNodeId,
    handle: ResourceRequestHandle,
    active_timeout_wake_id: TemporalWakeId,
    extension_duration: crate::data::temporal::TemporalDuration,
    decision_digest: ResourcePolicyDigest,
}

impl ResourceRuntimeState {
    pub fn extend_timeout_heartbeat(
        &mut self,
        handle: ResourceRequestHandle,
        previous_timeout_wake_id: TemporalWakeId,
        extended_timeout_wake: ScheduledTemporalWake,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceTimeoutHeartbeatExtensionReport {
        telemetry.resource_hot_in_flight_lookup_count += 1;
        let candidate = match self
            .classify_timeout_heartbeat_extension(handle, Some(previous_timeout_wake_id))
        {
            Ok(candidate) => candidate,
            Err(class) => {
                return self.deny_timeout_heartbeat_extension(handle.request_id(), class, telemetry)
            }
        };
        let extended = self.apply_timeout_heartbeat_extension(
            candidate,
            previous_timeout_wake_id,
            extended_timeout_wake,
            telemetry,
        );
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::timeout_heartbeat_extension(1, 0, 1),
        );
        ResourceTimeoutHeartbeatExtensionReport::admitted(extended, performance)
    }

    fn classify_timeout_heartbeat_extension(
        &self,
        handle: ResourceRequestHandle,
        expected_previous_timeout_wake_id: Option<TemporalWakeId>,
    ) -> Result<
        ResourceTimeoutHeartbeatExtensionCandidate,
        ResourceTimeoutHeartbeatExtensionDenialClass,
    > {
        let request_id = handle.request_id();
        let Some(in_flight) = self.in_flight_by_request.get(&request_id) else {
            return Err(ResourceTimeoutHeartbeatExtensionDenialClass::UnknownOrStaleRequest);
        };
        if in_flight.handle() != handle {
            return Err(ResourceTimeoutHeartbeatExtensionDenialClass::UnknownOrStaleRequest);
        }
        if in_flight.status() != ResourceInFlightStatus::Active
            || in_flight.lifecycle() != ResourceLifecycleClass::Pending
        {
            return Err(ResourceTimeoutHeartbeatExtensionDenialClass::NonActiveRequest);
        }
        let Some(active_timeout_wake_id) = in_flight.timeout_wake_id() else {
            return Err(ResourceTimeoutHeartbeatExtensionDenialClass::MissingTimeoutWake);
        };
        if expected_previous_timeout_wake_id
            .is_some_and(|expected| expected != active_timeout_wake_id)
        {
            return Err(ResourceTimeoutHeartbeatExtensionDenialClass::MissingTimeoutWake);
        }
        let Some(descriptor) = self.descriptors.get(&in_flight.descriptor_id()) else {
            return Err(ResourceTimeoutHeartbeatExtensionDenialClass::UnknownOrStaleRequest);
        };
        let timeout_plan = descriptor.timeout_decision_plan();
        let Some(extension_duration) = timeout_plan.heartbeat_extension() else {
            return Err(
                ResourceTimeoutHeartbeatExtensionDenialClass::PolicyDoesNotAllowHeartbeatExtension,
            );
        };
        Ok(ResourceTimeoutHeartbeatExtensionCandidate {
            node: in_flight.node(),
            handle,
            active_timeout_wake_id,
            extension_duration,
            decision_digest: timeout_plan.decision_digest().clone(),
        })
    }

    fn apply_timeout_heartbeat_extension(
        &mut self,
        candidate: ResourceTimeoutHeartbeatExtensionCandidate,
        previous_timeout_wake_id: TemporalWakeId,
        extended_timeout_wake: ScheduledTemporalWake,
        telemetry: &mut ResourceTelemetry,
    ) -> ExtendedResourceTimeoutHeartbeat {
        let in_flight = self
            .in_flight_by_request
            .get_mut(&candidate.handle.request_id())
            .expect("admitted timeout heartbeat request must remain in flight");
        debug_assert_eq!(in_flight.handle(), candidate.handle);
        debug_assert_eq!(
            in_flight.timeout_wake_id(),
            Some(candidate.active_timeout_wake_id)
        );
        in_flight.attach_timeout_wake(extended_timeout_wake.id());
        telemetry.resource_progress_heartbeat_extension_count += 1;
        telemetry.resource_timeout_temporal_wake_footprint = telemetry
            .resource_timeout_temporal_wake_footprint
            .saturating_add(1);
        ExtendedResourceTimeoutHeartbeat::new(
            candidate.handle,
            previous_timeout_wake_id,
            extended_timeout_wake,
            candidate.extension_duration,
            candidate.decision_digest,
        )
    }
    pub fn timeout_heartbeat_extension_candidate(
        &self,
        handle: ResourceRequestHandle,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<
        (
            ResourceNodeId,
            TemporalWakeId,
            crate::data::temporal::TemporalDuration,
        ),
        ResourceTimeoutHeartbeatExtensionDenialClass,
    > {
        telemetry.resource_hot_in_flight_lookup_count += 1;
        let candidate = self.classify_timeout_heartbeat_extension(handle, None)?;
        Ok((
            candidate.node,
            candidate.active_timeout_wake_id,
            candidate.extension_duration,
        ))
    }
}
