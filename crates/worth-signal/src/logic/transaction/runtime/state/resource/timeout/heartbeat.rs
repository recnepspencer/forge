use super::super::ResourceRuntimeState;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;
use crate::data::temporal::{ScheduledTemporalWake, TemporalWakeId};

impl ResourceRuntimeState {
    pub fn extend_timeout_heartbeat(
        &mut self,
        handle: ResourceRequestHandle,
        previous_timeout_wake_id: TemporalWakeId,
        extended_timeout_wake: ScheduledTemporalWake,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceTimeoutHeartbeatExtensionReport {
        telemetry.resource_hot_in_flight_lookup_count += 1;
        let request_id = handle.request_id();
        let Some(in_flight) = self.in_flight_by_request.get_mut(&request_id) else {
            return self.deny_timeout_heartbeat_extension(
                request_id,
                ResourceTimeoutHeartbeatExtensionDenialClass::UnknownOrStaleRequest,
                telemetry,
            );
        };
        if in_flight.handle() != handle {
            return self.deny_timeout_heartbeat_extension(
                request_id,
                ResourceTimeoutHeartbeatExtensionDenialClass::UnknownOrStaleRequest,
                telemetry,
            );
        }
        if in_flight.status() != ResourceInFlightStatus::Active
            || in_flight.lifecycle() != ResourceLifecycleClass::Pending
        {
            return self.deny_timeout_heartbeat_extension(
                request_id,
                ResourceTimeoutHeartbeatExtensionDenialClass::NonActiveRequest,
                telemetry,
            );
        }
        let Some(active_timeout_wake) = in_flight.timeout_wake_id() else {
            return self.deny_timeout_heartbeat_extension(
                request_id,
                ResourceTimeoutHeartbeatExtensionDenialClass::MissingTimeoutWake,
                telemetry,
            );
        };
        if active_timeout_wake != previous_timeout_wake_id {
            return self.deny_timeout_heartbeat_extension(
                request_id,
                ResourceTimeoutHeartbeatExtensionDenialClass::MissingTimeoutWake,
                telemetry,
            );
        }
        let Some(descriptor) = self.descriptors.get(&in_flight.descriptor_id()) else {
            return self.deny_timeout_heartbeat_extension(
                request_id,
                ResourceTimeoutHeartbeatExtensionDenialClass::UnknownOrStaleRequest,
                telemetry,
            );
        };
        let timeout_plan = descriptor.timeout_decision_plan();
        let Some(extension_duration) = timeout_plan.heartbeat_extension() else {
            return self.deny_timeout_heartbeat_extension(
                request_id,
                ResourceTimeoutHeartbeatExtensionDenialClass::PolicyDoesNotAllowHeartbeatExtension,
                telemetry,
            );
        };

        in_flight.attach_timeout_wake(extended_timeout_wake.id());
        telemetry.resource_progress_heartbeat_extension_count += 1;
        telemetry.resource_timeout_temporal_wake_footprint = telemetry
            .resource_timeout_temporal_wake_footprint
            .saturating_add(1);
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::timeout_heartbeat_extension(1, 0, 1),
        );
        ResourceTimeoutHeartbeatExtensionReport::admitted(
            ExtendedResourceTimeoutHeartbeat::new(
                handle,
                previous_timeout_wake_id,
                extended_timeout_wake,
                extension_duration,
                timeout_plan.decision_digest().clone(),
            ),
            performance,
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
        let Some(active_timeout_wake) = in_flight.timeout_wake_id() else {
            return Err(ResourceTimeoutHeartbeatExtensionDenialClass::MissingTimeoutWake);
        };
        let Some(descriptor) = self.descriptors.get(&in_flight.descriptor_id()) else {
            return Err(ResourceTimeoutHeartbeatExtensionDenialClass::UnknownOrStaleRequest);
        };
        let Some(extension_duration) = descriptor.timeout_decision_plan().heartbeat_extension()
        else {
            return Err(
                ResourceTimeoutHeartbeatExtensionDenialClass::PolicyDoesNotAllowHeartbeatExtension,
            );
        };
        Ok((in_flight.node(), active_timeout_wake, extension_duration))
    }
}
