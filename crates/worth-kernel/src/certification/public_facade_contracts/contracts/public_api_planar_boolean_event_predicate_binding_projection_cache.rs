use std::collections::BTreeMap;

use forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle;
use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanNormalizedEndpoint, PlanarBooleanSegmentPairWorkItem,
};
use worth_spatial::facade::planar_local_frame::PlanarLocalFrameCertificateReceipt;
use worth_spatial::facade::planar_predicates::PlanarPredicateAuthorityQueryWorld;
use worth_spatial::facade::planar_projection::{
    project_point_to_certified_plane_2d_entry, project_point_to_certified_plane_2d_facts,
    ProjectPointToCertifiedPlane2DBasis, ProjectPointToCertifiedPlane2DCase,
    ProjectPointToCertifiedPlane2DQueryDomain, ProjectPointToCertifiedPlane2DQueryWorld,
    ProjectPointToCertifiedPlane2DReceipt,
};
use worth_spatial::facade::planar_segment_segment::{
    CertifiedProjectedSegment2D, CertifiedSegmentSegment2D, CertifiedSegmentSegment2DContracts,
    CertifiedSegmentSegment2DQueryWorld, CertifiedSegmentSegment2DReceipt, SegmentContactPolicy,
};

pub(crate) fn segment_receipt_from_cached_projection(
    frame: &PlanarLocalFrameCertificateReceipt,
    work_item: &PlanarBooleanSegmentPairWorkItem,
    topology_basis_identity: &str,
    contracts: &CertifiedSegmentSegment2DContracts<
        CertifiedSegmentSegment2DQueryWorld,
        PlanarPredicateAuthorityQueryWorld,
    >,
    projection_handle: &ForgeQueryAdmittedConfiguredDomainHandle<
        ProjectPointToCertifiedPlane2DQueryDomain,
        ProjectPointToCertifiedPlane2DQueryWorld,
    >,
    projection_cache: &mut BTreeMap<String, ProjectPointToCertifiedPlane2DReceipt>,
) -> CertifiedSegmentSegment2DReceipt {
    let left = CertifiedProjectedSegment2D::from_projected_endpoints(
        work_item.left().canonical_segment_identity(),
        projected_endpoint_cached(
            frame,
            work_item.left().source_ordered_start_endpoint(),
            projection_handle,
            projection_cache,
        ),
        projected_endpoint_cached(
            frame,
            work_item.left().source_ordered_end_endpoint(),
            projection_handle,
            projection_cache,
        ),
    )
    .expect("left projected segment");
    let right = CertifiedProjectedSegment2D::from_projected_endpoints(
        work_item.right().canonical_segment_identity(),
        projected_endpoint_cached(
            frame,
            work_item.right().source_ordered_start_endpoint(),
            projection_handle,
            projection_cache,
        ),
        projected_endpoint_cached(
            frame,
            work_item.right().source_ordered_end_endpoint(),
            projection_handle,
            projection_cache,
        ),
    )
    .expect("right projected segment");

    CertifiedSegmentSegment2D::classify(left, right)
        .within_topology_basis(topology_basis_identity)
        .with_policy(SegmentContactPolicy::CertifyContactsDenyImprintRequired)
        .compile(contracts)
        .expect("segment plan")
        .certify()
        .expect("segment receipt")
}

fn projected_endpoint_cached(
    frame: &PlanarLocalFrameCertificateReceipt,
    endpoint: &PlanarBooleanNormalizedEndpoint,
    projection_handle: &ForgeQueryAdmittedConfiguredDomainHandle<
        ProjectPointToCertifiedPlane2DQueryDomain,
        ProjectPointToCertifiedPlane2DQueryWorld,
    >,
    projection_cache: &mut BTreeMap<String, ProjectPointToCertifiedPlane2DReceipt>,
) -> ProjectPointToCertifiedPlane2DReceipt {
    let cache_key = format!(
        "{}:{}:{}",
        frame.fact_digest(),
        endpoint.source_endpoint_identity(),
        endpoint.projected_endpoint_fact_identity()
    );
    if let Some(receipt) = projection_cache.get(&cache_key) {
        return receipt.clone();
    }
    let point = endpoint.point();
    let basis = ProjectPointToCertifiedPlane2DBasis::builder()
        .source_point_identity(endpoint.source_endpoint_identity())
        .source_point([1.0e12, 0.0, 0.0])
        .source_point_basis_digest(endpoint.projected_endpoint_fact_identity())
        .local_delta_from_frame_origin([point[0], point[1], 0.0])
        .local_frame_receipt(frame)
        .build()
        .expect("valid projection basis");
    project_point_to_certified_plane_2d_facts(
        &project_point_to_certified_plane_2d_entry(
            ProjectPointToCertifiedPlane2DCase::from_local_frame(basis),
        ),
        projection_handle,
    )
    .map(|receipt| {
        projection_cache.insert(cache_key, receipt.clone());
        receipt
    })
    .expect("projection receipt")
}
