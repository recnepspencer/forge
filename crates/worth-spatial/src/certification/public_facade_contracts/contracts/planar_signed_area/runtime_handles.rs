use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_local_frame::{
    PlanarLocalFrameCertificateQueryDomain, PlanarLocalFrameCertificateQueryWorld,
};
use worth_spatial::facade::planar_precision::{
    PlanarPrecisionCertificationQueryDomain, PlanarPrecisionCertificationQueryWorld,
};
use worth_spatial::facade::planar_predicates::{
    PlanarPredicateAuthorityQueryDomain, PlanarPredicateAuthorityQueryWorld,
};
use worth_spatial::facade::planar_projection::{
    ProjectPointToCertifiedPlane2DQueryDomain, ProjectPointToCertifiedPlane2DQueryWorld,
};
use worth_spatial::facade::planar_segment_segment::{
    CertifiedSegmentSegment2DQueryDomain, CertifiedSegmentSegment2DQueryWorld,
};
use worth_spatial::facade::planar_signed_area::{
    CertifiedSignedArea2DQueryDomain, CertifiedSignedArea2DQueryWorld,
};
use worth_spatial::facade::planar_winding::{
    CertifiedPolygonWinding2DQueryDomain, CertifiedPolygonWinding2DQueryWorld,
};

pub(crate) fn signed_area_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    CertifiedSignedArea2DQueryDomain,
    CertifiedSignedArea2DQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(CertifiedSignedArea2DQueryDomain)
        .with_operating_context(CertifiedSignedArea2DQueryWorld::new(world))
        .validate()
        .expect("validated signed area")
        .admit()
        .expect("admitted signed area")
}

pub(crate) fn winding_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    CertifiedPolygonWinding2DQueryDomain,
    CertifiedPolygonWinding2DQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(CertifiedPolygonWinding2DQueryDomain)
        .with_operating_context(CertifiedPolygonWinding2DQueryWorld::new(world))
        .validate()
        .expect("validated winding")
        .admit()
        .expect("admitted winding")
}

pub(crate) fn segment_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    CertifiedSegmentSegment2DQueryDomain,
    CertifiedSegmentSegment2DQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(CertifiedSegmentSegment2DQueryDomain)
        .with_operating_context(CertifiedSegmentSegment2DQueryWorld::new(world))
        .validate()
        .expect("validated segment")
        .admit()
        .expect("admitted segment")
}

pub(crate) fn projection_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    ProjectPointToCertifiedPlane2DQueryDomain,
    ProjectPointToCertifiedPlane2DQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(ProjectPointToCertifiedPlane2DQueryDomain)
        .with_operating_context(ProjectPointToCertifiedPlane2DQueryWorld::new(world))
        .validate()
        .expect("validated projection")
        .admit()
        .expect("admitted projection")
}

pub(crate) fn precision_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarPrecisionCertificationQueryDomain,
    PlanarPrecisionCertificationQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarPrecisionCertificationQueryDomain)
        .with_operating_context(PlanarPrecisionCertificationQueryWorld::new(world))
        .validate()
        .expect("validated precision")
        .admit()
        .expect("admitted precision")
}

pub(crate) fn frame_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarLocalFrameCertificateQueryDomain,
    PlanarLocalFrameCertificateQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarLocalFrameCertificateQueryDomain)
        .with_operating_context(PlanarLocalFrameCertificateQueryWorld::new(world))
        .validate()
        .expect("validated frame")
        .admit()
        .expect("admitted frame")
}

pub(crate) fn predicate_handle() -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarPredicateAuthorityQueryDomain,
    PlanarPredicateAuthorityQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarPredicateAuthorityQueryDomain)
        .with_operating_context(PlanarPredicateAuthorityQueryWorld::new(
            "signed-area-predicate",
        ))
        .validate()
        .expect("validated predicate")
        .admit()
        .expect("admitted predicate")
}
