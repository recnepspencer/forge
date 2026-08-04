use super::assembly::resource_certification_bundle;
use super::catalog::{ResourceCertificationFamily, REQUIRED_RESOURCE_CERTIFICATION_FAMILIES};
use super::contract::{ResourceCertificationBundle, ResourceCertificationRecord};
use super::evidence::{invalid_resource_certification_evidence, ResourceCertificationEvidence};
use crate::data::error::SignalError;
use crate::data::resource::ResourceBoundaryPerformanceEnvelope;
use crate::data::resource::ResourceBranchRestoreReport;
use crate::data::resource::ResourceCompletionRollbackReport;
use crate::data::resource::ResourceDiagnosticsSummary;
use crate::data::resource::ResourceObservationBatchReport;
use crate::data::resource::ResourceReplayReconstructionReport;
use crate::data::resource::ResourceRequestAdmissionReport;
use crate::data::resource::ResourceRuntimeSummary;
use crate::data::telemetry::ResourceTelemetry;

#[derive(Debug, Clone, Default)]
pub struct ResourceCertificationBuilder {
    async_resource_lifecycle_parity: Option<ResourceCertificationRecord>,
    out_of_order_completion_supersession: Option<ResourceCertificationRecord>,
    async_rollback_observation_equivalence: Option<ResourceCertificationRecord>,
    async_branch_restore_replay_equivalence: Option<ResourceCertificationRecord>,
    async_inflight_boundedness: Option<ResourceCertificationRecord>,
}

impl ResourceCertificationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_async_resource_lifecycle_parity(
        mut self,
        baseline: &ResourceReplayReconstructionReport,
        equivalent: &ResourceReplayReconstructionReport,
        baseline_diagnostics: &ResourceDiagnosticsSummary,
        equivalent_diagnostics: &ResourceDiagnosticsSummary,
    ) -> Result<Self, SignalError> {
        self.async_resource_lifecycle_parity = Some(Self::record(
            self.async_resource_lifecycle_parity.take(),
            ResourceCertificationFamily::AsyncResourceLifecycleParity,
            ResourceCertificationEvidence::lifecycle_parity(
                baseline,
                equivalent,
                baseline_diagnostics,
                equivalent_diagnostics,
            )?,
        )?);
        Ok(self)
    }

    pub fn with_out_of_order_completion_supersession(
        mut self,
        admission: ResourceRequestAdmissionReport,
    ) -> Result<Self, SignalError> {
        self.out_of_order_completion_supersession = Some(Self::record(
            self.out_of_order_completion_supersession.take(),
            ResourceCertificationFamily::OutOfOrderCompletionSupersession,
            ResourceCertificationEvidence::out_of_order_supersession(admission)?,
        )?);
        Ok(self)
    }

    pub fn with_async_rollback_observation_equivalence(
        mut self,
        rollback: ResourceCompletionRollbackReport,
        observation: ResourceObservationBatchReport,
        control_observation: ResourceObservationBatchReport,
        pre_rollback: &ResourceReplayReconstructionReport,
        post_rollback: &ResourceReplayReconstructionReport,
        diagnostics: &ResourceDiagnosticsSummary,
    ) -> Result<Self, SignalError> {
        self.async_rollback_observation_equivalence = Some(Self::record(
            self.async_rollback_observation_equivalence.take(),
            ResourceCertificationFamily::AsyncRollbackObservationEquivalence,
            ResourceCertificationEvidence::rollback_observation(
                rollback,
                observation,
                control_observation,
                pre_rollback,
                post_rollback,
                diagnostics,
            )?,
        )?);
        Ok(self)
    }

    pub fn with_async_branch_restore_replay_equivalence(
        mut self,
        restore: ResourceBranchRestoreReport,
        replay: &ResourceReplayReconstructionReport,
    ) -> Result<Self, SignalError> {
        self.async_branch_restore_replay_equivalence = Some(Self::record(
            self.async_branch_restore_replay_equivalence.take(),
            ResourceCertificationFamily::AsyncBranchRestoreReplayEquivalence,
            ResourceCertificationEvidence::branch_restore_replay(restore, replay),
        )?);
        Ok(self)
    }

    pub fn with_async_inflight_boundedness(
        mut self,
        summary: ResourceRuntimeSummary,
        replay: &ResourceReplayReconstructionReport,
        telemetry: ResourceTelemetry,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Result<Self, SignalError> {
        self.async_inflight_boundedness = Some(Self::record(
            self.async_inflight_boundedness.take(),
            ResourceCertificationFamily::AsyncInflightBoundedness,
            ResourceCertificationEvidence::inflight_boundedness(
                summary,
                replay,
                telemetry,
                performance,
            )?,
        )?);
        Ok(self)
    }

    pub fn build(self) -> Result<ResourceCertificationBundle, SignalError> {
        let records = [
            self.async_resource_lifecycle_parity,
            self.out_of_order_completion_supersession,
            self.async_rollback_observation_equivalence,
            self.async_branch_restore_replay_equivalence,
            self.async_inflight_boundedness,
        ];
        let mut complete = Vec::with_capacity(REQUIRED_RESOURCE_CERTIFICATION_FAMILIES.len());
        for (family, record) in REQUIRED_RESOURCE_CERTIFICATION_FAMILIES
            .into_iter()
            .zip(records)
        {
            let Some(record) = record else {
                return Err(invalid_resource_certification_evidence(
                    family,
                    "required certification family was not supplied",
                ));
            };
            complete.push(record);
        }

        let bundle = resource_certification_bundle(complete);
        bundle.ensure_passed()?;
        Ok(bundle)
    }

    fn record(
        existing: Option<ResourceCertificationRecord>,
        family: ResourceCertificationFamily,
        evidence: ResourceCertificationEvidence,
    ) -> Result<ResourceCertificationRecord, SignalError> {
        if existing.is_some() {
            return Err(invalid_resource_certification_evidence(
                family,
                "duplicate certification family evidence",
            ));
        }
        ResourceCertificationRecord::passing(family, evidence.digest, evidence.performance)
    }
}

pub fn resource_certification_builder() -> ResourceCertificationBuilder {
    ResourceCertificationBuilder::new()
}
