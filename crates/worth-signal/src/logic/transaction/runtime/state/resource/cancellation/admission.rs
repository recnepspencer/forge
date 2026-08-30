use super::super::ResourceRuntimeState;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;
use crate::data::temporal::TemporalWakeId;
use std::collections::BTreeSet;

struct ResourceCancellationAdmission {
    request_id: ResourceRequestId,
    handle: ResourceRequestHandle,
}

impl ResourceRuntimeState {
    pub fn cancel_resource_request(
        &mut self,
        handle: ResourceRequestHandle,
        reason: ResourceCancellationReason,
        mut telemetry: Option<&mut ResourceTelemetry>,
    ) -> ResourceCancellationReport {
        if let Some(telemetry) = telemetry.as_deref_mut() {
            telemetry.resource_hot_in_flight_lookup_count += 1;
        }
        let admission = match self.classify_resource_cancellation(handle) {
            Ok(admission) => admission,
            Err(class) => {
                return self.deny_cancellation(handle.request_id(), class, telemetry.as_deref_mut())
            }
        };
        let handle = admission.handle;

        let applied = self
            .apply_resource_cancellation(
                admission.request_id,
                reason,
                &mut BTreeSet::new(),
                telemetry.as_deref_mut(),
            )
            .expect("active cancellation should resolve through the runtime");
        let cancelled_width = 1u32.saturating_add(applied.propagated_dependents.len() as u32);
        let dependent_propagation = (!applied.propagated_dependents.is_empty()).then(|| {
            ResourceDependentCancellationPropagation::new(
                handle,
                applied.propagated_dependents.clone(),
            )
        });
        let cancellation_visibility_width = u32::from(
            applied.transition.output_continuity() != ResourceOutputContinuity::NoPriorOutput,
        ) + applied
            .propagated_dependents
            .iter()
            .filter(|cancelled| {
                cancelled.lifecycle_transition().output_continuity()
                    != ResourceOutputContinuity::NoPriorOutput
            })
            .count() as u32;
        if let Some(telemetry) = telemetry.as_deref_mut() {
            telemetry.resource_in_flight_request_count = self.in_flight_by_request.len() as u64;
        }
        let performance = Self::record_boundary_performance_optional(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::cancellation(cancelled_width, 0)
                .with_output_continuity_classification_width(cancellation_visibility_width),
        );

        ResourceCancellationReport::admitted(
            applied.cancelled,
            dependent_propagation,
            applied.lifecycle,
            applied.transition,
            performance,
        )
    }

    fn classify_resource_cancellation(
        &self,
        handle: ResourceRequestHandle,
    ) -> Result<ResourceCancellationAdmission, ResourceCancellationDenialClass> {
        let request_id = handle.request_id();
        let Some(in_flight) = self.in_flight_by_request.get(&request_id) else {
            return Err(ResourceCancellationDenialClass::UnknownOrStaleRequest);
        };
        if in_flight.handle() != handle {
            return Err(ResourceCancellationDenialClass::UnknownOrStaleRequest);
        }
        if in_flight.status() != ResourceInFlightStatus::Active
            || in_flight.lifecycle() != ResourceLifecycleClass::Pending
        {
            return Err(ResourceCancellationDenialClass::NonActiveRequest);
        }
        Ok(ResourceCancellationAdmission { request_id, handle })
    }

    pub(in crate::logic::transaction::runtime::state::resource) fn collect_active_timeout_wakes_for_cancellation_footprint(
        &self,
        request_id: ResourceRequestId,
        expected_handle: ResourceRequestHandle,
        visited_requests: &mut BTreeSet<ResourceRequestId>,
        collected_wakes: &mut BTreeSet<TemporalWakeId>,
    ) {
        if !visited_requests.insert(request_id) {
            return;
        }
        let Some(in_flight) = self.in_flight_by_request.get(&request_id) else {
            return;
        };
        if in_flight.handle() != expected_handle
            || in_flight.status() != ResourceInFlightStatus::Active
            || in_flight.lifecycle() != ResourceLifecycleClass::Pending
        {
            return;
        }
        if let Some(timeout_wake_id) = in_flight.timeout_wake_id() {
            collected_wakes.insert(timeout_wake_id);
        }
        let Some(descriptor) = self.descriptors.get(&in_flight.descriptor_id()) else {
            return;
        };
        for dependent_node in descriptor
            .cancellation_decision_plan()
            .declared_dependent_cancellation_nodes()
        {
            let Some(dependent_request_id) =
                self.active_request_by_node.get(dependent_node).copied()
            else {
                continue;
            };
            let Some(dependent_in_flight) = self.in_flight_by_request.get(&dependent_request_id)
            else {
                continue;
            };
            self.collect_active_timeout_wakes_for_cancellation_footprint(
                dependent_request_id,
                dependent_in_flight.handle(),
                visited_requests,
                collected_wakes,
            );
        }
    }
}
