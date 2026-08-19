use super::ResourceRuntimeState;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;

impl ResourceRuntimeState {
    pub fn bind_managed_queue(
        &mut self,
        admitted: AdmittedResourceRequest,
        queue_capacity: u64,
        telemetry: Option<&mut ResourceTelemetry>,
    ) -> Result<ResourceManagedQueueBinding, ResourceManagedQueueDenial> {
        let state = ResourceManagedQueueState::new(queue_capacity).map_err(|class| {
            ResourceManagedQueueDenial::new(
                admitted.handle().request_id(),
                class,
                ResourceManagedQueueCounters::none(),
            )
        })?;
        if let Some(telemetry) = telemetry {
            telemetry.resource_hot_in_flight_lookup_count += 1;
        }
        let request_id = admitted.handle().request_id();
        let request = self
            .in_flight_by_request
            .get_mut(&request_id)
            .filter(|request| {
                request.handle() == admitted.handle() && request.attempt() == admitted.attempt()
            })
            .ok_or_else(|| {
                ResourceManagedQueueDenial::new(
                    request_id,
                    ResourceManagedQueueDenialClass::RequestUnavailable,
                    ResourceManagedQueueCounters::exact_lookup(0),
                )
            })?;
        if request.status() != ResourceInFlightStatus::Active
            || request.lifecycle() != ResourceLifecycleClass::Pending
        {
            return Err(ResourceManagedQueueDenial::new(
                request_id,
                ResourceManagedQueueDenialClass::RequestNotActive,
                ResourceManagedQueueCounters::exact_lookup(0),
            ));
        }
        if request.managed_queue().is_some() {
            return Err(ResourceManagedQueueDenial::new(
                request_id,
                ResourceManagedQueueDenialClass::QueueAlreadyBound,
                ResourceManagedQueueCounters::exact_lookup(0),
            ));
        }
        request.bind_managed_queue(state);
        Ok(ResourceManagedQueueBinding::new(
            admitted.handle(),
            admitted.attempt(),
            queue_capacity,
        ))
    }
    pub(in crate::logic::transaction::runtime::state) fn bound_managed_queue_count(&self) -> u32 {
        u32::try_from(
            self.in_flight_by_request
                .values()
                .filter(|request| request.managed_queue().is_some())
                .count(),
        )
        .unwrap_or(u32::MAX)
    }
    pub fn enqueue_managed_queue(
        &mut self,
        binding: &ResourceManagedQueueBinding,
        width: u64,
        telemetry: Option<&mut ResourceTelemetry>,
    ) -> Result<ResourceManagedQueueMutationReport, ResourceManagedQueueDenial> {
        self.mutate_managed_queue(
            binding,
            width,
            ResourceManagedQueueMutationKind::Enqueued,
            telemetry,
        )
    }
    pub fn dequeue_managed_queue(
        &mut self,
        binding: &ResourceManagedQueueBinding,
        width: u64,
        telemetry: Option<&mut ResourceTelemetry>,
    ) -> Result<ResourceManagedQueueMutationReport, ResourceManagedQueueDenial> {
        self.mutate_managed_queue(
            binding,
            width,
            ResourceManagedQueueMutationKind::Dequeued,
            telemetry,
        )
    }
    fn mutate_managed_queue(
        &mut self,
        binding: &ResourceManagedQueueBinding,
        width: u64,
        kind: ResourceManagedQueueMutationKind,
        telemetry: Option<&mut ResourceTelemetry>,
    ) -> Result<ResourceManagedQueueMutationReport, ResourceManagedQueueDenial> {
        if let Some(telemetry) = telemetry {
            telemetry.resource_hot_in_flight_lookup_count += 1;
        }
        let request_id = binding.request().request_id();
        let request = self
            .in_flight_by_request
            .get_mut(&request_id)
            .filter(|request| {
                request.handle() == binding.request() && request.attempt() == binding.attempt()
            })
            .ok_or_else(|| {
                ResourceManagedQueueDenial::new(
                    request_id,
                    ResourceManagedQueueDenialClass::RequestUnavailable,
                    ResourceManagedQueueCounters::exact_lookup(0),
                )
            })?;
        let request_is_active = request.status() == ResourceInFlightStatus::Active
            && request.lifecycle() == ResourceLifecycleClass::Pending;
        if kind == ResourceManagedQueueMutationKind::Enqueued && !request_is_active {
            return Err(ResourceManagedQueueDenial::new(
                request_id,
                ResourceManagedQueueDenialClass::RequestNotActive,
                ResourceManagedQueueCounters::exact_lookup(0),
            ));
        }
        let queue = request
            .managed_queue_mut()
            .filter(|queue| queue.queue_capacity() == binding.queue_capacity())
            .ok_or_else(|| {
                ResourceManagedQueueDenial::new(
                    request_id,
                    ResourceManagedQueueDenialClass::BindingMismatch,
                    ResourceManagedQueueCounters::exact_lookup(0),
                )
            })?;
        let mutation = match kind {
            ResourceManagedQueueMutationKind::Enqueued => queue.enqueue(width),
            ResourceManagedQueueMutationKind::Dequeued => queue.dequeue(width),
        };
        mutation.map_err(|class| {
            ResourceManagedQueueDenial::new(
                request_id,
                class,
                ResourceManagedQueueCounters::exact_lookup(0),
            )
        })?;
        Ok(ResourceManagedQueueMutationReport::new(
            binding.request(),
            kind,
            queue.pressure(),
            ResourceManagedQueueCounters::exact_lookup(1),
        ))
    }
}
