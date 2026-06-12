use forge_query::facade::ForgeQueryDomainOperatingContext;

use crate::bindings::query_native_planar_local_frame::{
    planar_local_frame_certificate_entry, planar_local_frame_certificate_facts,
    PlanarLocalFrameCertificateCase, PlanarLocalFrameCertificateQueryDomain,
};
use crate::bindings::query_native_planar_overlap::CoplanarOverlapContractQueryDomain;
use crate::bindings::query_native_planar_precision::{
    planar_precision_certification_entry, planar_precision_certification_facts,
    PlanarPrecisionCertificationCase, PlanarPrecisionCertificationQueryDomain,
};
use crate::bindings::query_native_planar_predicate::{
    planar_predicate_authority_entry, planar_predicate_authority_facts,
    PlanarPredicateAuthorityCase, PlanarPredicateAuthorityQueryDomain,
};
use crate::bindings::query_native_planar_projection::ProjectPointToCertifiedPlane2DQueryDomain;
use crate::bindings::query_native_planar_segment_segment::CertifiedSegmentSegment2DQueryDomain;
use crate::bindings::query_native_planar_signed_area::CertifiedSignedArea2DQueryDomain;
use crate::bindings::query_native_planar_winding::CertifiedPolygonWinding2DQueryDomain;
use crate::planar_contracts::local_frame::{
    PlanarLocalFrameBasis, PlanarLocalFrameCertificateReceipt,
};
use crate::planar_contracts::precision_basis::{
    PlanarPrecisionBasis, PlanarPrecisionCertificateReceipt,
};
use crate::planar_contracts::predicate_authority::PlanarPredicateInputBasis;

use super::contracts::WorkloadCertificationContextContracts;
use super::denial::{WorkloadCertificationContextDenial, WorkloadCertificationContextDenialKind};
use super::motion_binding::WorkloadMotionBinding;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadPrecisionPolicy {
    LocalFeatureScale,
}

impl WorkloadPrecisionPolicy {
    pub(crate) fn tolerance_policy_identity(self) -> &'static str {
        match self {
            Self::LocalFeatureScale => "tolerance:workload-local-feature-scale",
        }
    }

    pub(crate) fn local_feature_scale_order(self) -> i32 {
        match self {
            Self::LocalFeatureScale => -9,
        }
    }

    pub(crate) fn world_magnitude_order(self) -> i32 {
        match self {
            Self::LocalFeatureScale => 12,
        }
    }

    pub(crate) fn normalization_scale(self) -> f64 {
        match self {
            Self::LocalFeatureScale => 1.0e-9,
        }
    }

    pub(crate) fn predicate_probe(self) -> [[f64; 2]; 3] {
        let scale = self.normalization_scale();
        [[0.0, 0.0], [scale, 0.0], [0.0, scale]]
    }

    pub(crate) fn frame_origin(self) -> [f64; 3] {
        let magnitude = 10_f64.powi(self.world_magnitude_order());
        [magnitude, 0.0, 0.0]
    }

    pub(crate) fn certify_precision<OC, SC, PC, PRC, WC, AC, PXC, FC>(
        self,
        frame_identity: &str,
        topology_neighborhood_identity: &str,
        motion_binding: &WorkloadMotionBinding,
        contracts: &WorkloadCertificationContextContracts<OC, SC, PC, PRC, WC, AC, PXC, FC>,
    ) -> Result<PlanarPrecisionCertificateReceipt, WorkloadCertificationContextDenial>
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
        let predicate_basis = PlanarPredicateInputBasis::from_projected_orient2d_points(
            frame_identity,
            topology_neighborhood_identity,
            motion_binding.movement_rotation_posture_identity(),
            self.tolerance_policy_identity(),
            self.predicate_probe(),
        );
        let predicate = planar_predicate_authority_facts(
            &planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(
                predicate_basis,
            )),
            &contracts.predicate_handle,
        )
        .map_err(|error| {
            WorkloadCertificationContextDenial::new(
                WorkloadCertificationContextDenialKind::PredicateCertificationFailed,
                format!("{error:?}"),
            )
        })?;
        let precision_basis = PlanarPrecisionBasis::builder()
            .local_frame_identity(frame_identity)
            .topology_basis_identity(topology_neighborhood_identity)
            .movement_rotation_posture_identity(motion_binding.movement_rotation_posture_identity())
            .tolerance_policy_identity(self.tolerance_policy_identity())
            .local_feature_scale_order(self.local_feature_scale_order())
            .world_magnitude_order(self.world_magnitude_order())
            .normalization_scale(self.normalization_scale())
            .predicate_receipt(&predicate)
            .build()
            .map_err(|error| {
                WorkloadCertificationContextDenial::new(
                    WorkloadCertificationContextDenialKind::PrecisionBasisDenied,
                    error.reason().to_string(),
                )
            })?;
        planar_precision_certification_facts(
            &planar_precision_certification_entry(
                PlanarPrecisionCertificationCase::from_predicate_receipt(
                    predicate,
                    precision_basis,
                ),
            ),
            &contracts.precision_handle,
        )
        .map_err(|error| {
            WorkloadCertificationContextDenial::new(
                WorkloadCertificationContextDenialKind::PrecisionCertificationFailed,
                format!("{error:?}"),
            )
        })
    }

    pub(crate) fn certify_local_frame<OC, SC, PC, PRC, WC, AC, PXC, FC>(
        self,
        frame_identity: &str,
        motion_binding: &WorkloadMotionBinding,
        precision: &PlanarPrecisionCertificateReceipt,
        contracts: &WorkloadCertificationContextContracts<OC, SC, PC, PRC, WC, AC, PXC, FC>,
    ) -> Result<PlanarLocalFrameCertificateReceipt, WorkloadCertificationContextDenial>
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
        let basis = PlanarLocalFrameBasis::builder()
            .frame_identity(frame_identity)
            .origin(self.frame_origin())
            .normal([0.0, 0.0, 1.0])
            .local_feature_scale_order(self.local_feature_scale_order())
            .world_magnitude_order(self.world_magnitude_order())
            .normalization_scale(self.normalization_scale())
            .transform_chain_digest(motion_binding.transform_stage_identity())
            .movement_rotation_posture_identity(motion_binding.movement_rotation_posture_identity())
            .tolerance_policy_identity(self.tolerance_policy_identity())
            .precision_receipt(precision)
            .build()
            .map_err(|error| {
                WorkloadCertificationContextDenial::new(
                    WorkloadCertificationContextDenialKind::LocalFrameBasisDenied,
                    error.reason().to_string(),
                )
            })?;
        planar_local_frame_certificate_facts(
            &planar_local_frame_certificate_entry(
                PlanarLocalFrameCertificateCase::from_precision_basis(basis),
            ),
            &contracts.local_frame_handle,
        )
        .map_err(|error| {
            WorkloadCertificationContextDenial::new(
                WorkloadCertificationContextDenialKind::LocalFrameCertificationFailed,
                format!("{error:?}"),
            )
        })
    }
}
