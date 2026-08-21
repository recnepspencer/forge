use super::ResourceRuntimeState;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;

impl ResourceRuntimeState {
    pub fn observe_safe_point(
        &mut self,
        binding: &ResourceManagedQueueBinding,
        telemetry: Option<&mut ResourceTelemetry>,
    ) -> Result<ResourceSafePointObservationReport, ResourceSafePointObservationDenial> {
        let counters = ResourceSafePointObservationCounters::exact_request_and_pressure();
        let (request, status, lifecycle_ordinal, pressure, timeout_wake_id) = {
            let request = self
                .in_flight_request_optional(binding.request(), telemetry)
                .ok_or_else(|| {
                    ResourceSafePointObservationDenial::request_unavailable(
                        binding.request().request_id(),
                        counters,
                    )
                })?;
            let pressure = request
                .managed_queue()
                .filter(|queue| {
                    request.attempt() == binding.attempt()
                        && queue.queue_capacity() == binding.queue_capacity()
                })
                .map(ResourceManagedQueueState::pressure)
                .ok_or_else(|| {
                    ResourceSafePointObservationDenial::queue_unavailable(
                        binding.request().request_id(),
                        counters,
                    )
                })?;
            (
                request.handle(),
                request.status(),
                request.lifecycle_ordinal(),
                pressure,
                request.timeout_wake_id(),
            )
        };
        let ordinal = self.next_safe_point_observation_ordinal;
        self.next_safe_point_observation_ordinal = self.next_safe_point_observation_ordinal.next();
        Ok(ResourceSafePointObservationReport::new(
            ordinal,
            ResourceSafePointObservationEvidence {
                request,
                status,
                lifecycle_ordinal,
                pressure,
                timeout_wake_id,
            },
            counters,
        ))
    }
}
