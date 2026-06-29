use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_local_frame::{
    PlanarLocalFrameCertificateQueryDomain, PlanarLocalFrameCertificateQueryWorld,
};
use worth_spatial::facade::planar_overlap::{
    CoplanarOverlapContractContracts, CoplanarOverlapContractQueryDomain,
    CoplanarOverlapContractQueryWorld,
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
    CertifiedSegmentSegment2DContracts, CertifiedSegmentSegment2DQueryDomain,
    CertifiedSegmentSegment2DQueryWorld,
};
use worth_spatial::facade::planar_signed_area::{
    CertifiedSignedArea2DContracts, CertifiedSignedArea2DQueryDomain,
    CertifiedSignedArea2DQueryWorld,
};
use worth_spatial::facade::planar_winding::{
    CertifiedPolygonWinding2DContracts, CertifiedPolygonWinding2DQueryDomain,
    CertifiedPolygonWinding2DQueryWorld,
};
use worth_spatial::facade::projected_overlap_faces::{
    CertifiedProjectedOverlapBridgeAuthority, CoplanarOverlapExtractionBundle,
};
use worth_spatial::facade::projection_workload::ProjectedPlanarWorkload;
use worth_spatial::facade::transform_workload::TransformReceiptSet;
use worth_spatial::facade::workload_certification_context::{
    WorkloadCertificationContext, WorkloadCertificationContextContracts, WorkloadPrecisionPolicy,
};

type StormContext<'a> = WorkloadCertificationContext<
    'a,
    CoplanarOverlapContractQueryWorld,
    CertifiedSegmentSegment2DQueryWorld,
    PlanarPredicateAuthorityQueryWorld,
    ProjectPointToCertifiedPlane2DQueryWorld,
    CertifiedPolygonWinding2DQueryWorld,
    CertifiedSignedArea2DQueryWorld,
    PlanarPrecisionCertificationQueryWorld,
    PlanarLocalFrameCertificateQueryWorld,
>;

pub(crate) fn operator_context_and_bundle<'a>(
    world: &'static str,
    projected: &'a ProjectedPlanarWorkload,
    transform_receipts: &'a TransformReceiptSet,
) -> (StormContext<'a>, CoplanarOverlapExtractionBundle) {
    let context = WorkloadCertificationContext::from_projected_workload(projected)
        .with_transform_receipts(transform_receipts)
        .with_precision_policy(WorkloadPrecisionPolicy::LocalFeatureScale)
        .compile(context_contracts(world))
        .expect("vertical slice context should certify from real workload receipts");
    let bundle = CertifiedProjectedOverlapBridgeAuthority::from_context(&context)
        .expect("vertical slice bridge authority should certify")
        .extraction_bundle()
        .clone();
    (context, bundle)
}

fn context_contracts(
    world: &'static str,
) -> WorkloadCertificationContextContracts<
    CoplanarOverlapContractQueryWorld,
    CertifiedSegmentSegment2DQueryWorld,
    PlanarPredicateAuthorityQueryWorld,
    ProjectPointToCertifiedPlane2DQueryWorld,
    CertifiedPolygonWinding2DQueryWorld,
    CertifiedSignedArea2DQueryWorld,
    PlanarPrecisionCertificationQueryWorld,
    PlanarLocalFrameCertificateQueryWorld,
> {
    let segment_contracts =
        CertifiedSegmentSegment2DContracts::new(segment_handle(world), predicate_handle());
    WorkloadCertificationContextContracts::new(
        projection_handle(world),
        CertifiedPolygonWinding2DContracts::new(
            winding_handle(world),
            segment_contracts.clone(),
            predicate_handle(),
        ),
        CertifiedSignedArea2DContracts::new(signed_area_handle(world)),
        CoplanarOverlapContractContracts::new(overlap_handle(world), segment_contracts),
        predicate_handle(),
        precision_handle(world),
        frame_handle(world),
    )
}

macro_rules! handle {
    ($fn_name:ident, $domain:expr, $world:expr, $domain_ty:ty, $world_ty:ty) => {
        fn $fn_name(
            world: &'static str,
        ) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<$domain_ty, $world_ty> {
            ForgeQueryApplicationFacade::runtime_backed_default()
                .domain($domain)
                .with_operating_context($world(world))
                .validate()
                .expect("validated phase10 vertical migration domain")
                .admit()
                .expect("admitted phase10 vertical migration domain")
        }
    };
}

handle!(
    overlap_handle,
    CoplanarOverlapContractQueryDomain,
    CoplanarOverlapContractQueryWorld::new,
    CoplanarOverlapContractQueryDomain,
    CoplanarOverlapContractQueryWorld
);
handle!(
    winding_handle,
    CertifiedPolygonWinding2DQueryDomain,
    CertifiedPolygonWinding2DQueryWorld::new,
    CertifiedPolygonWinding2DQueryDomain,
    CertifiedPolygonWinding2DQueryWorld
);
handle!(
    signed_area_handle,
    CertifiedSignedArea2DQueryDomain,
    CertifiedSignedArea2DQueryWorld::new,
    CertifiedSignedArea2DQueryDomain,
    CertifiedSignedArea2DQueryWorld
);
handle!(
    segment_handle,
    CertifiedSegmentSegment2DQueryDomain,
    CertifiedSegmentSegment2DQueryWorld::new,
    CertifiedSegmentSegment2DQueryDomain,
    CertifiedSegmentSegment2DQueryWorld
);
handle!(
    projection_handle,
    ProjectPointToCertifiedPlane2DQueryDomain,
    ProjectPointToCertifiedPlane2DQueryWorld::new,
    ProjectPointToCertifiedPlane2DQueryDomain,
    ProjectPointToCertifiedPlane2DQueryWorld
);
handle!(
    precision_handle,
    PlanarPrecisionCertificationQueryDomain,
    PlanarPrecisionCertificationQueryWorld::new,
    PlanarPrecisionCertificationQueryDomain,
    PlanarPrecisionCertificationQueryWorld
);
handle!(
    frame_handle,
    PlanarLocalFrameCertificateQueryDomain,
    PlanarLocalFrameCertificateQueryWorld::new,
    PlanarLocalFrameCertificateQueryDomain,
    PlanarLocalFrameCertificateQueryWorld
);

fn predicate_handle() -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarPredicateAuthorityQueryDomain,
    PlanarPredicateAuthorityQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarPredicateAuthorityQueryDomain)
        .with_operating_context(PlanarPredicateAuthorityQueryWorld::new(
            "phase10-overlap-predicate",
        ))
        .validate()
        .expect("validated phase10 predicate")
        .admit()
        .expect("admitted phase10 predicate")
}
