use crate::data::resource::{
    AdmittedResourceRequest, InFlightResourceRequest, ResourceManagedQueueBinding,
    ResourceManagedQueueDenial, ResourceManagedQueueMutationReport, ResourceRequestHandle,
    ResourceSafePointObservationDenial, ResourceSafePointObservationReport,
};

use super::super::SignalRuntime;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn in_flight_resource_request(
        &mut self,
        handle: ResourceRequestHandle,
    ) -> Option<&InFlightResourceRequest> {
        let capture_telemetry = self.graph.captures_observation_surface(
            crate::logic::transaction::SignalObservationSurface::OptionalTelemetry,
        );
        self.resource.in_flight_request_optional(
            handle,
            capture_telemetry.then_some(&mut self.telemetry.resource),
        )
    }

    pub fn observe_resource_safe_point(
        &mut self,
        binding: &ResourceManagedQueueBinding,
    ) -> Result<ResourceSafePointObservationReport, ResourceSafePointObservationDenial> {
        let capture_telemetry = self.graph.captures_observation_surface(
            crate::logic::transaction::SignalObservationSurface::OptionalTelemetry,
        );
        self.resource.observe_safe_point(
            binding,
            capture_telemetry.then_some(&mut self.telemetry.resource),
        )
    }

    pub fn bind_resource_managed_queue(
        &mut self,
        admitted: AdmittedResourceRequest,
        queue_capacity: u64,
    ) -> Result<ResourceManagedQueueBinding, ResourceManagedQueueDenial> {
        let capture_telemetry = self.graph.captures_observation_surface(
            crate::logic::transaction::SignalObservationSurface::OptionalTelemetry,
        );
        self.resource.bind_managed_queue(
            admitted,
            queue_capacity,
            capture_telemetry.then_some(&mut self.telemetry.resource),
        )
    }

    pub fn enqueue_resource_managed_queue(
        &mut self,
        binding: &ResourceManagedQueueBinding,
        width: u64,
    ) -> Result<ResourceManagedQueueMutationReport, ResourceManagedQueueDenial> {
        let capture_telemetry = self.graph.captures_observation_surface(
            crate::logic::transaction::SignalObservationSurface::OptionalTelemetry,
        );
        self.resource.enqueue_managed_queue(
            binding,
            width,
            capture_telemetry.then_some(&mut self.telemetry.resource),
        )
    }

    pub fn dequeue_resource_managed_queue(
        &mut self,
        binding: &ResourceManagedQueueBinding,
        width: u64,
    ) -> Result<ResourceManagedQueueMutationReport, ResourceManagedQueueDenial> {
        let capture_telemetry = self.graph.captures_observation_surface(
            crate::logic::transaction::SignalObservationSurface::OptionalTelemetry,
        );
        self.resource.dequeue_managed_queue(
            binding,
            width,
            capture_telemetry.then_some(&mut self.telemetry.resource),
        )
    }
}
