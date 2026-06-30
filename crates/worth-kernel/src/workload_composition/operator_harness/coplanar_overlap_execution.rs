use forge_query::facade::ForgeQueryDomainOperatingContext;
use worth_spatial::facade::planar_local_frame::PlanarLocalFrameCertificateQueryDomain;
use worth_spatial::facade::planar_overlap::CoplanarOverlapContractQueryDomain;
use worth_spatial::facade::planar_precision::PlanarPrecisionCertificationQueryDomain;
use worth_spatial::facade::planar_predicates::PlanarPredicateAuthorityQueryDomain;
use worth_spatial::facade::planar_projection::ProjectPointToCertifiedPlane2DQueryDomain;
use worth_spatial::facade::planar_segment_segment::CertifiedSegmentSegment2DQueryDomain;
use worth_spatial::facade::planar_signed_area::CertifiedSignedArea2DQueryDomain;
use worth_spatial::facade::planar_winding::CertifiedPolygonWinding2DQueryDomain;
use worth_spatial::facade::projected_overlap_faces::CoplanarOverlapExtractionBundle;
use worth_spatial::facade::workload_operators::CoplanarOverlapWorkloadOperator;

use super::{
    BatchAdmissionExecutionOperatorRun, OperatorOutcome, OperatorRun, OperatorWorkloadError,
    WorkloadOperatorFamily,
};
use crate::workload_composition::{
    BatchAdmissionExecutionReceipt, BatchAdmissionFamilyPosture, WorkloadStageRequirement,
    WorthWorkload,
};
use worth_spatial::facade::workload_certification_context::WorkloadCertificationContext;

impl OperatorRun {
    pub fn execute_coplanar_overlap_with_batch_execution<OC, SC, PC, PRC, WC, AC, PXC, FC>(
        &self,
        workload: &WorthWorkload,
        batch_execution: &BatchAdmissionExecutionReceipt,
        context: &WorkloadCertificationContext<'_, OC, SC, PC, PRC, WC, AC, PXC, FC>,
        extraction_bundle: &CoplanarOverlapExtractionBundle,
    ) -> Result<OperatorOutcome, OperatorWorkloadError>
    where
        OC: ForgeQueryDomainOperatingContext<CoplanarOverlapContractQueryDomain>,
        SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
        PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
        PRC: ForgeQueryDomainOperatingContext<ProjectPointToCertifiedPlane2DQueryDomain>,
        WC: ForgeQueryDomainOperatingContext<CertifiedPolygonWinding2DQueryDomain>,
        AC: ForgeQueryDomainOperatingContext<CertifiedSignedArea2DQueryDomain>,
        PXC: ForgeQueryDomainOperatingContext<PlanarPrecisionCertificationQueryDomain>,
        FC: ForgeQueryDomainOperatingContext<PlanarLocalFrameCertificateQueryDomain>,
    {
        if self.family() != WorkloadOperatorFamily::CoplanarOverlap {
            return Err(OperatorWorkloadError::WrongOperatorFamily {
                expected: WorkloadOperatorFamily::CoplanarOverlap,
                actual: self.family(),
            });
        }
        if self.requirement() != WorkloadStageRequirement::BatchAdmissionExecution {
            return Err(OperatorWorkloadError::MissingBatchAdmissionExecution);
        }
        require_matching_batch_execution(workload, batch_execution)?;
        require_parallel_posture(batch_execution)?;

        let receipt = CoplanarOverlapWorkloadOperator::from_stage_links(
            self.evidence_binding().required_stage_links(),
        )
        .with_certification_context(context)
        .with_extraction_bundle(extraction_bundle)
        .execute()?;

        OperatorOutcome::from_coplanar_overlap_receipt(self.clone(), receipt)
    }
}

impl BatchAdmissionExecutionOperatorRun {
    pub fn execute_coplanar_overlap<OC, SC, PC, PRC, WC, AC, PXC, FC>(
        &self,
        context: &WorkloadCertificationContext<'_, OC, SC, PC, PRC, WC, AC, PXC, FC>,
        extraction_bundle: &CoplanarOverlapExtractionBundle,
    ) -> Result<OperatorOutcome, OperatorWorkloadError>
    where
        OC: ForgeQueryDomainOperatingContext<CoplanarOverlapContractQueryDomain>,
        SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
        PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
        PRC: ForgeQueryDomainOperatingContext<ProjectPointToCertifiedPlane2DQueryDomain>,
        WC: ForgeQueryDomainOperatingContext<CertifiedPolygonWinding2DQueryDomain>,
        AC: ForgeQueryDomainOperatingContext<CertifiedSignedArea2DQueryDomain>,
        PXC: ForgeQueryDomainOperatingContext<PlanarPrecisionCertificationQueryDomain>,
        FC: ForgeQueryDomainOperatingContext<PlanarLocalFrameCertificateQueryDomain>,
    {
        self.run().execute_coplanar_overlap_with_batch_execution(
            self.workload(),
            self.batch_execution(),
            context,
            extraction_bundle,
        )
    }
}

fn require_matching_batch_execution(
    workload: &WorthWorkload,
    batch_execution: &BatchAdmissionExecutionReceipt,
) -> Result<(), OperatorWorkloadError> {
    let Some(bound_execution) = workload.batch_admission_execution() else {
        return Err(OperatorWorkloadError::MissingBatchAdmissionExecution);
    };
    if bound_execution.execution_receipt_digest() == batch_execution.execution_receipt_digest() {
        Ok(())
    } else {
        Err(OperatorWorkloadError::MismatchedBatchAdmissionExecution)
    }
}

fn require_parallel_posture(
    batch_execution: &BatchAdmissionExecutionReceipt,
) -> Result<(), OperatorWorkloadError> {
    if batch_execution.posture() == BatchAdmissionFamilyPosture::ParallelAdmit {
        Ok(())
    } else {
        Err(
            OperatorWorkloadError::GroupedExecutionRequiresParallelBatchPosture(
                batch_execution.posture(),
            ),
        )
    }
}
