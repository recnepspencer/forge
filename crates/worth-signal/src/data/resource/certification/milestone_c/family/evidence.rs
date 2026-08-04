use super::super::super::digest::resource_canonical_digest;
use super::super::digest_basis::{
    ResourceMilestoneCCancellationSupersessionEvidenceBasis,
    ResourceMilestoneCObservationEvidenceBasis, ResourceMilestoneCPolicyFamilyEvidenceBasis,
    ResourceMilestoneCRetentionReplayEvidenceBasis, ResourceMilestoneCRetryPolicyEvidenceBasis,
    ResourceMilestoneCRevalidationEvidenceBasis, ResourceMilestoneCTimeoutPolicyEvidenceBasis,
};
use crate::data::error::SignalError;
use crate::data::resource::ResourceBoundaryPerformanceEnvelope;
use crate::data::resource::ResourceCancellationReport;
use crate::data::resource::ResourceIntentEquivalenceCoalescing;
use crate::data::resource::ResourceLifecycleRetentionCompactionReport;
use crate::data::resource::ResourceObservationBatchReport;
use crate::data::resource::ResourceOverlappingGenerationAdmission;
use crate::data::resource::ResourcePolicyRegistryFreezeReport;
use crate::data::resource::ResourceReplayAvailabilityReport;
use crate::data::resource::ResourceRetryScheduleReport;
use crate::data::resource::ResourceRevalidationReport;
use crate::data::resource::ResourceTimeoutHeartbeatExtensionReport;
use crate::data::resource::ResourceTimeoutReport;

#[derive(Debug)]
pub(super) struct ResourceMilestoneCPolicyCertificationEvidence {
    pub(super) digest: String,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceMilestoneCPolicyCertificationEvidence {
    pub(super) fn resource_policy_family(
        freeze_report: &ResourcePolicyRegistryFreezeReport,
    ) -> Self {
        let performance = ResourceBoundaryPerformanceEnvelope::policy_compatibility(
            freeze_report.descriptor_count() as u32,
            0,
        );
        Self {
            digest: resource_canonical_digest(&ResourceMilestoneCPolicyFamilyEvidenceBasis {
                descriptor_count: freeze_report.descriptor_count(),
                id_index_width: freeze_report.id_index_width(),
                kind_name_index_width: freeze_report.kind_name_index_width(),
                registry_digest: freeze_report.registry_digest().as_str(),
                performance,
            }),
            performance,
        }
    }

    pub(super) fn retry_budget_and_backoff(
        report: &ResourceRetryScheduleReport,
    ) -> Result<Self, SignalError> {
        let performance = report.performance();
        if report.scheduled_retry().is_none() && report.denied_retry().is_none() {
            return Err(SignalError::invalid_input(
                "milestone C retry certification requires scheduled or denied retry evidence",
            ));
        }
        Ok(Self {
            digest: resource_canonical_digest(&ResourceMilestoneCRetryPolicyEvidenceBasis {
                scheduled_retry: report.scheduled_retry(),
                denied_retry: report.denied_retry(),
                performance,
            }),
            performance,
        })
    }

    pub(super) fn timeout_deadline(
        timeout_report: &ResourceTimeoutReport,
        heartbeat_report: &ResourceTimeoutHeartbeatExtensionReport,
    ) -> Result<Self, SignalError> {
        let performance = timeout_report.performance();
        if timeout_report.timed_out_request().is_none() && timeout_report.denied_timeout().is_none()
        {
            return Err(SignalError::invalid_input(
                "milestone C timeout certification requires timeout evidence",
            ));
        }
        Ok(Self {
            digest: resource_canonical_digest(&ResourceMilestoneCTimeoutPolicyEvidenceBasis {
                timed_out_request: timeout_report.timed_out_request(),
                denied_timeout: timeout_report.denied_timeout(),
                heartbeat_extension: heartbeat_report.extended_heartbeat(),
                denied_heartbeat_extension: heartbeat_report.denied_extension(),
                timeout_performance: timeout_report.performance(),
                heartbeat_performance: heartbeat_report.performance(),
            }),
            performance,
        })
    }

    pub(super) fn cancellation_supersession(
        cancellation_report: &ResourceCancellationReport,
        overlap_admission: &ResourceOverlappingGenerationAdmission,
        intent_coalescing: &ResourceIntentEquivalenceCoalescing,
    ) -> Result<Self, SignalError> {
        let performance = cancellation_report.performance();
        if cancellation_report.cancelled_request().is_none()
            && cancellation_report.denied_cancellation().is_none()
        {
            return Err(SignalError::invalid_input(
                "milestone C cancellation certification requires cancellation evidence",
            ));
        }
        Ok(Self {
            digest: resource_canonical_digest(
                &ResourceMilestoneCCancellationSupersessionEvidenceBasis {
                    cancelled_request: cancellation_report.cancelled_request(),
                    denied_cancellation: cancellation_report.denied_cancellation(),
                    dependent_propagation: cancellation_report.dependent_propagation(),
                    overlap_admission,
                    intent_coalescing,
                    performance,
                },
            ),
            performance,
        })
    }

    pub(super) fn revalidation_freshness(
        report: &ResourceRevalidationReport,
    ) -> Result<Self, SignalError> {
        let performance = report.performance();
        if report.admitted_revalidation().is_none() && report.denied_revalidation().is_none() {
            return Err(SignalError::invalid_input(
                "milestone C revalidation certification requires revalidation evidence",
            ));
        }
        Ok(Self {
            digest: resource_canonical_digest(&ResourceMilestoneCRevalidationEvidenceBasis {
                admitted_revalidation: report.admitted_revalidation(),
                denied_revalidation: report.denied_revalidation(),
                lifecycle: report.lifecycle(),
                transition: report.transition(),
                performance,
            }),
            performance,
        })
    }

    pub(super) fn observation_output_continuity(
        report: &ResourceObservationBatchReport,
    ) -> Result<Self, SignalError> {
        let performance = report.performance();
        if report.events().is_empty() {
            return Err(SignalError::invalid_input(
                "milestone C observation certification requires observation event evidence",
            ));
        }
        Ok(Self {
            digest: resource_canonical_digest(&ResourceMilestoneCObservationEvidenceBasis {
                events: report.events(),
                performance,
            }),
            performance,
        })
    }

    pub(super) fn retention_replay(
        retention_report: &ResourceLifecycleRetentionCompactionReport,
        replay_availability: &ResourceReplayAvailabilityReport,
    ) -> Result<Self, SignalError> {
        let performance = replay_availability.performance();
        Ok(Self {
            digest: resource_canonical_digest(&ResourceMilestoneCRetentionReplayEvidenceBasis {
                retention_report,
                replay_class: replay_availability.class(),
                replay_denial_class: replay_availability.denial_class(),
                retained_history_unavailable_count: replay_availability
                    .retained_history_unavailable_count(),
                denied_completion_unavailable_count: replay_availability
                    .denied_completion_unavailable_count(),
                retry_lineage_unavailable_count: replay_availability
                    .retry_lineage_unavailable_count(),
                availability_digest: replay_availability.availability_digest(),
                performance,
            }),
            performance,
        })
    }
}
