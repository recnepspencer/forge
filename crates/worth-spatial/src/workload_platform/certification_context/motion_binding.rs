use forge_query::facade::ForgeQueryDomainOperatingContext;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::bindings::query_native_planar_local_frame::PlanarLocalFrameCertificateQueryDomain;
use crate::bindings::query_native_planar_overlap::CoplanarOverlapContractQueryDomain;
use crate::bindings::query_native_planar_precision::PlanarPrecisionCertificationQueryDomain;
use crate::bindings::query_native_planar_predicate::PlanarPredicateAuthorityQueryDomain;
use crate::bindings::query_native_planar_projection::ProjectPointToCertifiedPlane2DQueryDomain;
use crate::bindings::query_native_planar_segment_segment::CertifiedSegmentSegment2DQueryDomain;
use crate::bindings::query_native_planar_signed_area::CertifiedSignedArea2DQueryDomain;
use crate::bindings::query_native_planar_winding::CertifiedPolygonWinding2DQueryDomain;
use crate::workload_platform::transform_workload::TransformReceiptSet;

use super::context::WorkloadCertificationContext;
use super::denial::{WorkloadCertificationContextDenial, WorkloadCertificationContextDenialKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadMotionBinding {
    motion_binding_identity: String,
    transform_stage_identity: String,
    projected_workload_identity: String,
    movement_rotation_posture_identity: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadMotionAdversary {
    TinyRotationExitsCoplanarClass,
}

impl WorkloadMotionAdversary {
    fn movement_rotation_posture_identity(self) -> &'static str {
        match self {
            Self::TinyRotationExitsCoplanarClass => "movement:tiny-rotation-exits-coplanar-class",
        }
    }
}

impl WorkloadMotionBinding {
    pub(crate) fn from_transform_receipts(
        projected_workload_identity: &str,
        transform_receipts: &TransformReceiptSet,
    ) -> Result<Self, WorkloadCertificationContextDenial> {
        if transform_receipts.projected_workload_identity() != projected_workload_identity {
            return Err(WorkloadCertificationContextDenial::new(
                WorkloadCertificationContextDenialKind::MismatchedTransformReceipts,
                "workload certification context requires transform receipts from the same projected workload",
            ));
        }
        Ok(Self::from_parts(
            transform_receipts.stage_identity().receipt_identity(),
            projected_workload_identity,
            transform_receipts
                .transform_posture_receipt()
                .posture_identity(),
        ))
    }

    pub fn adversarial_for_context<OC, SC, PC, PRC, WC, AC, PXC, FC>(
        context: &WorkloadCertificationContext<'_, OC, SC, PC, PRC, WC, AC, PXC, FC>,
        adversary: WorkloadMotionAdversary,
    ) -> Self
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
        Self::from_parts(
            context.motion_binding().transform_stage_identity(),
            context.motion_binding().projected_workload_identity(),
            adversary.movement_rotation_posture_identity(),
        )
    }

    pub(crate) fn from_parts(
        transform_stage_identity: impl Into<String>,
        projected_workload_identity: impl Into<String>,
        posture_identity: impl Into<String>,
    ) -> Self {
        let transform_stage_identity = transform_stage_identity.into();
        let projected_workload_identity = projected_workload_identity.into();
        let movement_rotation_posture_identity = posture_identity.into();
        let motion_binding_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                format!("transform-stage:{transform_stage_identity}"),
                format!("projected-workload:{projected_workload_identity}"),
                format!("movement-rotation:{movement_rotation_posture_identity}"),
            ],
        );
        Self {
            motion_binding_identity,
            transform_stage_identity,
            projected_workload_identity,
            movement_rotation_posture_identity,
        }
    }

    pub fn motion_binding_identity(&self) -> &str {
        &self.motion_binding_identity
    }

    pub fn transform_stage_identity(&self) -> &str {
        &self.transform_stage_identity
    }

    pub fn projected_workload_identity(&self) -> &str {
        &self.projected_workload_identity
    }

    pub fn movement_rotation_posture_identity(&self) -> &str {
        &self.movement_rotation_posture_identity
    }
}
