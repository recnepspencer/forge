use crate::data::resource::{
    ResourceCancellationReason, ResourceCancellationReport, ResourceRejectionReason,
    ResourceRejectionReport, ResourceRequestHandle, ResourceTimeoutHeartbeatExtensionReport,
    ResourceTimeoutReport,
};

use crate::data::temporal::{
    ReadyTemporalWake, TemporalCondition, TemporalWakeOwner, TemporalWakeRetirementReason,
};

use super::super::SignalRuntime;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn cancel_resource_request(
        &mut self,
        handle: ResourceRequestHandle,
        reason: ResourceCancellationReason,
    ) -> Result<ResourceCancellationReport, crate::data::error::SignalError> {
        for wake_id in self
            .resource
            .active_timeout_wakes_for_cancellation_footprint(handle)
        {
            self.retire_temporal_wake(wake_id, TemporalWakeRetirementReason::Cancelled)?;
        }
        let telemetry = self
            .graph
            .captures_observation_surface(
                crate::logic::transaction::SignalObservationSurface::OptionalTelemetry,
            )
            .then_some(&mut self.telemetry.resource);
        let report = self
            .resource
            .cancel_resource_request(handle, reason, telemetry);
        Ok(report)
    }

    pub fn reject_resource_request(
        &mut self,
        handle: ResourceRequestHandle,
        reason: ResourceRejectionReason,
    ) -> Result<ResourceRejectionReport, crate::data::error::SignalError> {
        if let Some(wake_id) = self.resource.active_timeout_wake_for_handle(handle) {
            self.retire_temporal_wake(wake_id, TemporalWakeRetirementReason::Consumed)?;
        }
        let telemetry = self
            .graph
            .captures_observation_surface(
                crate::logic::transaction::SignalObservationSurface::OptionalTelemetry,
            )
            .then_some(&mut self.telemetry.resource);
        let report = self
            .resource
            .reject_resource_request(handle, reason, telemetry);
        Ok(report)
    }

    pub fn admit_resource_timeout(
        &mut self,
        handle: ResourceRequestHandle,
        ready_wake: ReadyTemporalWake,
    ) -> Result<ResourceTimeoutReport, crate::data::error::SignalError> {
        let wake_id = ready_wake.id();
        if self
            .resource
            .active_timeout_wake_for_handle(handle)
            .is_some_and(|active| active == wake_id)
        {
            self.retire_temporal_wake(wake_id, TemporalWakeRetirementReason::Consumed)?;
        }
        let telemetry = self
            .graph
            .captures_observation_surface(
                crate::logic::transaction::SignalObservationSurface::OptionalTelemetry,
            )
            .then_some(&mut self.telemetry.resource);
        let report = self
            .resource
            .admit_resource_timeout(handle, ready_wake, telemetry);
        Ok(report)
    }

    pub fn extend_resource_timeout_heartbeat(
        &mut self,
        handle: ResourceRequestHandle,
    ) -> Result<ResourceTimeoutHeartbeatExtensionReport, crate::data::error::SignalError> {
        let capture_telemetry = self.graph.captures_observation_surface(
            crate::logic::transaction::SignalObservationSurface::OptionalTelemetry,
        );
        let (node, previous_timeout_wake_id, extension_duration) =
            match self.resource.timeout_heartbeat_extension_candidate(
                handle,
                capture_telemetry.then_some(&mut self.telemetry.resource),
            ) {
                Ok(candidate) => candidate,
                Err(class) => {
                    return Ok(self.resource.deny_timeout_heartbeat_extension_for_report(
                        handle.request_id(),
                        class,
                        capture_telemetry.then_some(&mut self.telemetry.resource),
                    ))
                }
            };
        let current_tick = self.clock_basis().current_tick();
        let due_tick = crate::data::temporal::ClockTick::new(
            current_tick.get().saturating_add(extension_duration.get()),
        );
        let extended_timeout_wake = self.schedule_owned_temporal_wake(
            TemporalWakeOwner::ResourceNode(node.node()),
            TemporalCondition::after(extension_duration.get())?,
            due_tick,
        )?;
        let report = self.resource.extend_timeout_heartbeat(
            handle,
            previous_timeout_wake_id,
            extended_timeout_wake.clone(),
            capture_telemetry.then_some(&mut self.telemetry.resource),
        );
        if report.extended_heartbeat().is_some() {
            self.retire_temporal_wake(
                previous_timeout_wake_id,
                TemporalWakeRetirementReason::Superseded,
            )?;
            return Ok(report);
        }
        let _ = self.retire_temporal_wake(
            extended_timeout_wake.id(),
            TemporalWakeRetirementReason::Disposed,
        );
        Ok(report)
    }
}
