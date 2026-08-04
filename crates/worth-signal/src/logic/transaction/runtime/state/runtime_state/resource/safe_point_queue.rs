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
        self.resource
            .in_flight_request(handle, &mut self.telemetry.resource)
    }

    pub fn observe_resource_safe_point(
        &mut self,
        binding: &ResourceManagedQueueBinding,
    ) -> Result<ResourceSafePointObservationReport, ResourceSafePointObservationDenial> {
        self.resource
            .observe_safe_point(binding, &mut self.telemetry.resource)
    }

    pub fn bind_resource_managed_queue(
        &mut self,
        admitted: AdmittedResourceRequest,
        queue_capacity: u64,
    ) -> Result<ResourceManagedQueueBinding, ResourceManagedQueueDenial> {
        self.resource
            .bind_managed_queue(admitted, queue_capacity, &mut self.telemetry.resource)
    }

    pub fn enqueue_resource_managed_queue(
        &mut self,
        binding: &ResourceManagedQueueBinding,
        width: u64,
    ) -> Result<ResourceManagedQueueMutationReport, ResourceManagedQueueDenial> {
        self.resource
            .enqueue_managed_queue(binding, width, &mut self.telemetry.resource)
    }

    pub fn dequeue_resource_managed_queue(
        &mut self,
        binding: &ResourceManagedQueueBinding,
        width: u64,
    ) -> Result<ResourceManagedQueueMutationReport, ResourceManagedQueueDenial> {
        self.resource
            .dequeue_managed_queue(binding, width, &mut self.telemetry.resource)
    }
}
