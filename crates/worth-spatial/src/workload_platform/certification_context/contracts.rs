use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
};

use crate::bindings::query_native_planar_local_frame::PlanarLocalFrameCertificateQueryDomain;
use crate::bindings::query_native_planar_overlap::{
    CoplanarOverlapContractContracts, CoplanarOverlapContractQueryDomain,
};
use crate::bindings::query_native_planar_precision::PlanarPrecisionCertificationQueryDomain;
use crate::bindings::query_native_planar_predicate::PlanarPredicateAuthorityQueryDomain;
use crate::bindings::query_native_planar_projection::ProjectPointToCertifiedPlane2DQueryDomain;
use crate::bindings::query_native_planar_segment_segment::CertifiedSegmentSegment2DQueryDomain;
use crate::bindings::query_native_planar_signed_area::{
    CertifiedSignedArea2DContracts, CertifiedSignedArea2DQueryDomain,
};
use crate::bindings::query_native_planar_winding::{
    CertifiedPolygonWinding2DContracts, CertifiedPolygonWinding2DQueryDomain,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorkloadCertificationContextContracts<OC, SC, PC, PRC, WC, AC, PXC, FC>
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
    pub projection_handle:
        ForgeQueryAdmittedConfiguredDomainHandle<ProjectPointToCertifiedPlane2DQueryDomain, PRC>,
    pub winding_contracts: CertifiedPolygonWinding2DContracts<WC, SC, PC>,
    pub signed_area_contracts: CertifiedSignedArea2DContracts<AC>,
    pub overlap_contracts: CoplanarOverlapContractContracts<OC, SC, PC>,
    pub predicate_handle:
        ForgeQueryAdmittedConfiguredDomainHandle<PlanarPredicateAuthorityQueryDomain, PC>,
    pub precision_handle:
        ForgeQueryAdmittedConfiguredDomainHandle<PlanarPrecisionCertificationQueryDomain, PXC>,
    pub local_frame_handle:
        ForgeQueryAdmittedConfiguredDomainHandle<PlanarLocalFrameCertificateQueryDomain, FC>,
}

impl<OC, SC, PC, PRC, WC, AC, PXC, FC>
    WorkloadCertificationContextContracts<OC, SC, PC, PRC, WC, AC, PXC, FC>
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
    pub fn new(
        projection_handle: ForgeQueryAdmittedConfiguredDomainHandle<
            ProjectPointToCertifiedPlane2DQueryDomain,
            PRC,
        >,
        winding_contracts: CertifiedPolygonWinding2DContracts<WC, SC, PC>,
        signed_area_contracts: CertifiedSignedArea2DContracts<AC>,
        overlap_contracts: CoplanarOverlapContractContracts<OC, SC, PC>,
        predicate_handle: ForgeQueryAdmittedConfiguredDomainHandle<
            PlanarPredicateAuthorityQueryDomain,
            PC,
        >,
        precision_handle: ForgeQueryAdmittedConfiguredDomainHandle<
            PlanarPrecisionCertificationQueryDomain,
            PXC,
        >,
        local_frame_handle: ForgeQueryAdmittedConfiguredDomainHandle<
            PlanarLocalFrameCertificateQueryDomain,
            FC,
        >,
    ) -> Self {
        Self {
            projection_handle,
            winding_contracts,
            signed_area_contracts,
            overlap_contracts,
            predicate_handle,
            precision_handle,
            local_frame_handle,
        }
    }
}
