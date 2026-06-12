use worth_spatial::facade::planar_local_frame::PlanarLocalFrameCertificateQueryWorld;
use worth_spatial::facade::planar_overlap::{
    CoplanarOverlapContractContracts, CoplanarOverlapContractQueryWorld, CoplanarOverlapDenial,
};
use worth_spatial::facade::planar_precision::PlanarPrecisionCertificationQueryWorld;
use worth_spatial::facade::planar_predicates::PlanarPredicateAuthorityQueryWorld;
use worth_spatial::facade::planar_projection::ProjectPointToCertifiedPlane2DQueryWorld;
use worth_spatial::facade::planar_segment_segment::{
    CertifiedSegmentSegment2DContracts, CertifiedSegmentSegment2DQueryWorld,
};
use worth_spatial::facade::planar_signed_area::{
    CertifiedSignedArea2DContracts, CertifiedSignedArea2DQueryWorld,
};
use worth_spatial::facade::planar_winding::{
    CertifiedPolygonWinding2DContracts, CertifiedPolygonWinding2DQueryWorld,
};
use worth_spatial::facade::projected_overlap_faces::{
    CertifiedProjectedOverlapBridgeAuthority, CoplanarOverlapExtractionBundle,
};
use worth_spatial::facade::projection_workload::ProjectedPlanarWorkload;
use worth_spatial::facade::transform_workload::TransformReceiptSet;
use worth_spatial::facade::workload_certification_context::{
    WorkloadCertificationContext, WorkloadCertificationContextContracts, WorkloadMotionAdversary,
    WorkloadMotionBinding, WorkloadPrecisionPolicy,
};

use crate::public_api_planar_overlap::runtime_handles::{
    frame_handle, overlap_handle, precision_handle, predicate_handle, projection_handle,
    segment_handle, signed_area_handle, winding_handle,
};

pub(crate) type StormCertificationContext = WorkloadCertificationContext<
    'static,
    CoplanarOverlapContractQueryWorld,
    CertifiedSegmentSegment2DQueryWorld,
    PlanarPredicateAuthorityQueryWorld,
    ProjectPointToCertifiedPlane2DQueryWorld,
    CertifiedPolygonWinding2DQueryWorld,
    CertifiedSignedArea2DQueryWorld,
    PlanarPrecisionCertificationQueryWorld,
    PlanarLocalFrameCertificateQueryWorld,
>;

pub(crate) fn certify_projected_storm_context(
    world: &'static str,
    projected: &ProjectedPlanarWorkload,
    transform_receipts: &TransformReceiptSet,
) -> StormCertificationContext {
    WorkloadCertificationContext::from_projected_workload(projected)
        .with_transform_receipts(transform_receipts)
        .with_precision_policy(WorkloadPrecisionPolicy::LocalFeatureScale)
        .compile(context_contracts(world))
        .expect("projected storm context should certify from workload receipts")
}

pub(crate) fn certify_projected_storm_extraction_bundle(
    world: &'static str,
    projected: &ProjectedPlanarWorkload,
    transform_receipts: &TransformReceiptSet,
) -> CoplanarOverlapExtractionBundle {
    let context = certify_projected_storm_context(world, projected, transform_receipts);
    certify_projected_storm_bridge_authority_from_context(&context)
        .extraction_bundle()
        .clone()
}

pub(crate) fn certify_projected_storm_bridge_authority(
    world: &'static str,
    projected: &ProjectedPlanarWorkload,
    transform_receipts: &TransformReceiptSet,
) -> CertifiedProjectedOverlapBridgeAuthority {
    let context = certify_projected_storm_context(world, projected, transform_receipts);
    certify_projected_storm_bridge_authority_from_context(&context)
}

pub(crate) fn certify_projected_storm_bridge_authority_from_context(
    context: &StormCertificationContext,
) -> CertifiedProjectedOverlapBridgeAuthority {
    CertifiedProjectedOverlapBridgeAuthority::from_context(context)
        .expect("projected storm bridge authority should certify")
}

pub(crate) fn deny_storm_tiny_rotation(
    world: &'static str,
    projected: &ProjectedPlanarWorkload,
    transform_receipts: &TransformReceiptSet,
) -> CoplanarOverlapDenial {
    let context = certify_projected_storm_context(world, projected, transform_receipts);
    let authority = certify_projected_storm_bridge_authority_from_context(&context);
    let pair = authority
        .candidate_pairs()
        .first_pair()
        .expect("real storm bridge authority should expose at least one certified pair");
    let tiny_rotation_context = context
        .with_motion_binding(WorkloadMotionBinding::adversarial_for_context(
            &context,
            WorkloadMotionAdversary::TinyRotationExitsCoplanarClass,
        ))
        .expect("tiny rotation adversary should stay bound to the storm context");
    let mismatched_second = pair
        .second_face()
        .recertify_with_context(&tiny_rotation_context)
        .expect("real projected face should recertify under tiny rotation posture");
    match pair.compile_overlap_with_second_face(
        &mismatched_second,
        tiny_rotation_context.overlap_contracts(),
        tiny_rotation_context.topology_neighborhood_identity(),
    ) {
        Ok(_) => panic!("movement and rotation mismatch must deny before overlap extraction"),
        Err(denial) => denial,
    }
}

pub(crate) fn context_contracts(
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
