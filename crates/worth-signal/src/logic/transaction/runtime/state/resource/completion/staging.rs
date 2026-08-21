use super::super::ResourceRuntimeState;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;

impl ResourceRuntimeState {
    pub fn stage_admitted_resource_completion(
        &mut self,
        admitted: AdmittedResourceCompletion,
        mut telemetry: Option<&mut ResourceTelemetry>,
    ) -> Result<ResourceCompletionStagingReport, crate::data::error::SignalError> {
        if let Some(telemetry) = telemetry.as_deref_mut() {
            telemetry.resource_hot_in_flight_lookup_count += 1;
        }
        let handle = admitted.handle();
        let Some(in_flight) = self.in_flight_by_request.get(&handle.request_id()) else {
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot stage resource completion for unknown request {}",
                handle.request_id().get()
            )));
        };
        if in_flight.handle() != handle
            || in_flight.status() != ResourceInFlightStatus::Active
            || in_flight.lifecycle() != ResourceLifecycleClass::Pending
        {
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot stage resource completion for non-active request {}",
                handle.request_id().get()
            )));
        }

        if let Some(telemetry) = telemetry.as_deref_mut() {
            telemetry.resource_completion_staging_count += 1;
        }
        let performance = telemetry
            .as_deref_mut()
            .map(|telemetry| {
                Self::record_boundary_performance(
                    telemetry,
                    ResourceBoundaryPerformanceEnvelope::completion_staging(),
                )
            })
            .unwrap_or_else(ResourceBoundaryPerformanceEnvelope::completion_staging);
        Ok(ResourceCompletionStagingReport::new(
            StagedResourceCompletionEffect::new(admitted),
            performance,
        ))
    }
    pub fn stage_denied_resource_completion(
        &mut self,
        denied: DeniedResourceCompletion,
        mut telemetry: Option<&mut ResourceTelemetry>,
    ) -> Result<ResourceCompletionDenialStagingReport, crate::data::error::SignalError> {
        if let Some(telemetry) = telemetry.as_deref_mut() {
            telemetry.resource_hot_in_flight_lookup_count += 1;
        }
        let Some(retained) = self.denied_completions.get(&denied.denial_id()) else {
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot stage unretained denied resource completion {}",
                denied.denial_id().get()
            )));
        };
        if *retained != denied {
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot stage mismatched denied resource completion {}",
                denied.denial_id().get()
            )));
        }

        if let Some(telemetry) = telemetry.as_deref_mut() {
            telemetry.resource_completion_denial_staging_count += 1;
        }
        let performance = telemetry
            .as_deref_mut()
            .map(|telemetry| {
                Self::record_boundary_performance(
                    telemetry,
                    ResourceBoundaryPerformanceEnvelope::completion_denial_staging(),
                )
            })
            .unwrap_or_else(ResourceBoundaryPerformanceEnvelope::completion_denial_staging);
        Ok(ResourceCompletionDenialStagingReport::new(
            StagedDeniedResourceCompletionEffect::new(denied),
            performance,
        ))
    }
    pub fn rollback_staged_resource_completion(
        &mut self,
        staged: StagedResourceCompletionEffect,
        mut telemetry: Option<&mut ResourceTelemetry>,
    ) -> ResourceCompletionRollbackReport {
        if let Some(telemetry) = telemetry.as_deref_mut() {
            telemetry.resource_completion_rollback_count += 1;
        }
        let performance = telemetry
            .as_deref_mut()
            .map(|telemetry| {
                Self::record_boundary_performance(
                    telemetry,
                    ResourceBoundaryPerformanceEnvelope::completion_rollback(1, 0),
                )
            })
            .unwrap_or_else(|| ResourceBoundaryPerformanceEnvelope::completion_rollback(1, 0));
        ResourceCompletionRollbackReport::new(
            RolledBackResourceCompletionArtifact::admitted(staged),
            performance,
        )
    }
    pub fn rollback_staged_denied_resource_completion(
        &mut self,
        staged: StagedDeniedResourceCompletionEffect,
        mut telemetry: Option<&mut ResourceTelemetry>,
    ) -> ResourceCompletionRollbackReport {
        if let Some(telemetry) = telemetry.as_deref_mut() {
            telemetry.resource_completion_rollback_count += 1;
        }
        let performance = telemetry
            .as_deref_mut()
            .map(|telemetry| {
                Self::record_boundary_performance(
                    telemetry,
                    ResourceBoundaryPerformanceEnvelope::completion_rollback(0, 1),
                )
            })
            .unwrap_or_else(|| ResourceBoundaryPerformanceEnvelope::completion_rollback(0, 1));
        ResourceCompletionRollbackReport::new(
            RolledBackResourceCompletionArtifact::denied(staged),
            performance,
        )
    }
}
