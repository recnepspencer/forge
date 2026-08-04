use super::super::catalog::{
    ResourceMilestoneCPolicyCertificationFamily,
    REQUIRED_RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_FAMILIES,
};
use super::assembly::resource_milestone_c_policy_certification_bundle;
use super::contract::{
    ResourceMilestoneCPolicyCertificationBundle, ResourceMilestoneCPolicyCertificationRecord,
};
use super::evidence::ResourceMilestoneCPolicyCertificationEvidence;
use crate::data::error::SignalError;
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

#[derive(Debug, Clone, Default)]
pub struct ResourceMilestoneCPolicyCertificationBuilder {
    async_resource_policy_family_certification: Option<ResourceMilestoneCPolicyCertificationRecord>,
    async_retry_budget_and_backoff_certification:
        Option<ResourceMilestoneCPolicyCertificationRecord>,
    async_timeout_deadline_certification: Option<ResourceMilestoneCPolicyCertificationRecord>,
    async_cancellation_supersession_policy_certification:
        Option<ResourceMilestoneCPolicyCertificationRecord>,
    async_revalidation_freshness_certification: Option<ResourceMilestoneCPolicyCertificationRecord>,
    async_observation_output_continuity_certification:
        Option<ResourceMilestoneCPolicyCertificationRecord>,
    async_retention_replay_policy_certification:
        Option<ResourceMilestoneCPolicyCertificationRecord>,
}

impl ResourceMilestoneCPolicyCertificationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_async_resource_policy_family_certification(
        mut self,
        freeze_report: &ResourcePolicyRegistryFreezeReport,
    ) -> Result<Self, SignalError> {
        self.async_resource_policy_family_certification = Some(Self::record(
            self.async_resource_policy_family_certification.take(),
            ResourceMilestoneCPolicyCertificationFamily::AsyncResourcePolicyFamilyCertification,
            ResourceMilestoneCPolicyCertificationEvidence::resource_policy_family(freeze_report),
        )?);
        Ok(self)
    }

    pub fn with_async_retry_budget_and_backoff_certification(
        mut self,
        report: &ResourceRetryScheduleReport,
    ) -> Result<Self, SignalError> {
        self.async_retry_budget_and_backoff_certification = Some(Self::record(
            self.async_retry_budget_and_backoff_certification.take(),
            ResourceMilestoneCPolicyCertificationFamily::AsyncRetryBudgetAndBackoffCertification,
            ResourceMilestoneCPolicyCertificationEvidence::retry_budget_and_backoff(report)?,
        )?);
        Ok(self)
    }

    pub fn with_async_timeout_deadline_certification(
        mut self,
        timeout_report: &ResourceTimeoutReport,
        heartbeat_report: &ResourceTimeoutHeartbeatExtensionReport,
    ) -> Result<Self, SignalError> {
        self.async_timeout_deadline_certification = Some(Self::record(
            self.async_timeout_deadline_certification.take(),
            ResourceMilestoneCPolicyCertificationFamily::AsyncTimeoutDeadlineCertification,
            ResourceMilestoneCPolicyCertificationEvidence::timeout_deadline(
                timeout_report,
                heartbeat_report,
            )?,
        )?);
        Ok(self)
    }

    pub fn with_async_cancellation_supersession_policy_certification(
        mut self,
        cancellation_report: &ResourceCancellationReport,
        overlap_admission: &ResourceOverlappingGenerationAdmission,
        intent_coalescing: &ResourceIntentEquivalenceCoalescing,
    ) -> Result<Self, SignalError> {
        self.async_cancellation_supersession_policy_certification = Some(Self::record(
            self.async_cancellation_supersession_policy_certification.take(),
            ResourceMilestoneCPolicyCertificationFamily::AsyncCancellationSupersessionPolicyCertification,
            ResourceMilestoneCPolicyCertificationEvidence::cancellation_supersession(
                cancellation_report,
                overlap_admission,
                intent_coalescing,
            )?,
        )?);
        Ok(self)
    }

    pub fn with_async_revalidation_freshness_certification(
        mut self,
        report: &ResourceRevalidationReport,
    ) -> Result<Self, SignalError> {
        self.async_revalidation_freshness_certification = Some(Self::record(
            self.async_revalidation_freshness_certification.take(),
            ResourceMilestoneCPolicyCertificationFamily::AsyncRevalidationFreshnessCertification,
            ResourceMilestoneCPolicyCertificationEvidence::revalidation_freshness(report)?,
        )?);
        Ok(self)
    }

    pub fn with_async_observation_output_continuity_certification(
        mut self,
        report: &ResourceObservationBatchReport,
    ) -> Result<Self, SignalError> {
        self.async_observation_output_continuity_certification = Some(Self::record(
            self.async_observation_output_continuity_certification.take(),
            ResourceMilestoneCPolicyCertificationFamily::AsyncObservationOutputContinuityCertification,
            ResourceMilestoneCPolicyCertificationEvidence::observation_output_continuity(report)?,
        )?);
        Ok(self)
    }

    pub fn with_async_retention_replay_policy_certification(
        mut self,
        retention_report: &ResourceLifecycleRetentionCompactionReport,
        replay_availability: &ResourceReplayAvailabilityReport,
    ) -> Result<Self, SignalError> {
        self.async_retention_replay_policy_certification = Some(Self::record(
            self.async_retention_replay_policy_certification.take(),
            ResourceMilestoneCPolicyCertificationFamily::AsyncRetentionReplayPolicyCertification,
            ResourceMilestoneCPolicyCertificationEvidence::retention_replay(
                retention_report,
                replay_availability,
            )?,
        )?);
        Ok(self)
    }

    pub fn build(self) -> Result<ResourceMilestoneCPolicyCertificationBundle, SignalError> {
        let records = [
            self.async_resource_policy_family_certification,
            self.async_retry_budget_and_backoff_certification,
            self.async_timeout_deadline_certification,
            self.async_cancellation_supersession_policy_certification,
            self.async_revalidation_freshness_certification,
            self.async_observation_output_continuity_certification,
            self.async_retention_replay_policy_certification,
        ];
        let mut complete =
            Vec::with_capacity(REQUIRED_RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_FAMILIES.len());
        for (family, record) in REQUIRED_RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_FAMILIES
            .into_iter()
            .zip(records)
        {
            let Some(record) = record else {
                return Err(SignalError::invalid_input(format!(
                    "invalid milestone C policy certification evidence for {}: required certification family was not supplied",
                    family.label()
                )));
            };
            complete.push(record);
        }
        let bundle = resource_milestone_c_policy_certification_bundle(complete);
        bundle.ensure_passed()?;
        Ok(bundle)
    }

    fn record(
        existing: Option<ResourceMilestoneCPolicyCertificationRecord>,
        family: ResourceMilestoneCPolicyCertificationFamily,
        evidence: ResourceMilestoneCPolicyCertificationEvidence,
    ) -> Result<ResourceMilestoneCPolicyCertificationRecord, SignalError> {
        if existing.is_some() {
            return Err(SignalError::invalid_input(format!(
                "invalid milestone C policy certification evidence for {}: duplicate certification family evidence",
                family.label()
            )));
        }
        ResourceMilestoneCPolicyCertificationRecord::passing(
            family,
            evidence.digest,
            evidence.performance,
        )
    }
}

pub fn resource_milestone_c_policy_certification_builder(
) -> ResourceMilestoneCPolicyCertificationBuilder {
    ResourceMilestoneCPolicyCertificationBuilder::new()
}
