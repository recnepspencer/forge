use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_local_frame::{
    planar_local_frame_certificate_entry, planar_local_frame_certificate_facts,
    PlanarLocalFrameBasis, PlanarLocalFrameCertificateCase, PlanarLocalFrameCertificateQueryDomain,
    PlanarLocalFrameCertificateQueryWorld, PlanarLocalFrameCertificateReceipt,
};
use worth_spatial::facade::planar_precision::{
    planar_precision_certification_entry, planar_precision_certification_facts,
    PlanarPrecisionBasis, PlanarPrecisionCertificateReceipt, PlanarPrecisionCertificationCase,
    PlanarPrecisionCertificationQueryDomain, PlanarPrecisionCertificationQueryWorld,
};
use worth_spatial::facade::planar_predicates::{
    planar_predicate_authority_entry, planar_predicate_authority_facts,
    PlanarPredicateAuthorityCase, PlanarPredicateAuthorityQueryDomain,
    PlanarPredicateAuthorityQueryWorld, PlanarPredicateInputBasis,
};
use worth_spatial::facade::planar_projection::{
    project_point_to_certified_plane_2d_entry, project_point_to_certified_plane_2d_facts,
    ProjectPointToCertifiedPlane2DBasis, ProjectPointToCertifiedPlane2DCase,
    ProjectPointToCertifiedPlane2DQueryDomain, ProjectPointToCertifiedPlane2DQueryWorld,
    ProjectPointToCertifiedPlane2DReceipt,
};
use worth_spatial::facade::planar_segment_segment::{
    CertifiedSegmentSegment2DContracts, CertifiedSegmentSegment2DQueryDomain,
    CertifiedSegmentSegment2DQueryWorld,
};
use worth_spatial::facade::planar_signed_area::{
    AreaDegeneracyClass, CertifiedSignedArea2D, CertifiedSignedArea2DContracts,
    CertifiedSignedArea2DQueryDomain, CertifiedSignedArea2DQueryWorld, SignedAreaOrientation,
};
use worth_spatial::facade::planar_winding::{
    CertifiedPolygonWinding2D, CertifiedPolygonWinding2DContracts,
    CertifiedPolygonWinding2DQueryDomain, CertifiedPolygonWinding2DQueryWorld,
    CertifiedProjectedLoop2D, CertifiedTopologyLoopBasis2D,
};

#[test]
fn kernel_consumes_certified_signed_area_without_synthesizing_area_truth() {
    let world = "kernel-signed-area";
    let (precision, frame) = precision_and_frame(world);
    let loop_ = CertifiedProjectedLoop2D::from_projected_vertices(
        "loop:kernel-area",
        topology_basis("loop:kernel-area"),
        loop_points(
            world,
            &frame,
            "area",
            &[[0.0, 0.0], [5.0e-9, 0.0], [5.0e-9, 5.0e-9], [0.0, 5.0e-9]],
        ),
    )
    .expect("projected loop");
    let winding = CertifiedPolygonWinding2D::certify(loop_)
        .within_planar_neighborhood("topology:kernel-area-face")
        .compile(&winding_contracts(world))
        .expect("winding plan")
        .certify()
        .expect("winding receipt");

    let area_contracts = signed_area_contracts(world);
    let plan = CertifiedSignedArea2D::measure_face(winding)
        .using_precision_basis(precision)
        .compile(&area_contracts)
        .expect("signed area plan");
    assert_eq!(plan.loop_edges_walked(), 4);

    let receipt = plan.certify().expect("signed area receipt");
    assert_eq!(
        receipt.orientation(),
        SignedAreaOrientation::CounterClockwise
    );
    assert_eq!(receipt.degeneracy(), AreaDegeneracyClass::WellFormed);
    assert!(receipt.used_local_frame_scale());
    assert_eq!(receipt.counters().area_terms_evaluated(), 4);
    assert!(!receipt.fact_digest().is_empty());
}

fn precision_and_frame(
    world: &'static str,
) -> (
    PlanarPrecisionCertificateReceipt,
    PlanarLocalFrameCertificateReceipt,
) {
    let predicate_basis = PlanarPredicateInputBasis::from_projected_orient2d_points(
        "frame:kernel-area",
        "topology:kernel-area",
        "movement:kernel-area-stable",
        "tolerance:kernel-area-exact",
        [[0.0, 0.0], [1.0e-9, 0.0], [0.0, 1.0e-9]],
    );
    let predicate_entry =
        planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(predicate_basis));
    let predicate =
        planar_predicate_authority_facts(&predicate_entry, &predicate_handle()).expect("predicate");
    let precision_basis = PlanarPrecisionBasis::builder()
        .local_frame_identity("frame:kernel-area")
        .topology_basis_identity("topology:kernel-area")
        .movement_rotation_posture_identity("movement:kernel-area-stable")
        .tolerance_policy_identity("tolerance:kernel-area-exact")
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .predicate_receipt(&predicate)
        .build()
        .expect("precision basis");
    let precision_entry = planar_precision_certification_entry(
        PlanarPrecisionCertificationCase::from_predicate_receipt(predicate, precision_basis),
    );
    let precision =
        planar_precision_certification_facts(&precision_entry, &precision_handle(world))
            .expect("precision receipt");
    let frame_basis = PlanarLocalFrameBasis::builder()
        .frame_identity("frame:kernel-area")
        .origin([1.0e12, 0.0, 0.0])
        .normal([0.0, 0.0, 1.0])
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .transform_chain_digest("transform:kernel-area")
        .movement_rotation_posture_identity("movement:kernel-area-stable")
        .tolerance_policy_identity("tolerance:kernel-area-exact")
        .precision_receipt(&precision)
        .build()
        .expect("frame basis");
    let frame_entry = planar_local_frame_certificate_entry(
        PlanarLocalFrameCertificateCase::from_precision_basis(frame_basis),
    );
    let frame = planar_local_frame_certificate_facts(&frame_entry, &frame_handle(world))
        .expect("frame receipt");
    (precision, frame)
}

fn loop_points(
    world: &'static str,
    frame: &PlanarLocalFrameCertificateReceipt,
    prefix: &'static str,
    points: &[[f64; 2]],
) -> Vec<ProjectPointToCertifiedPlane2DReceipt> {
    points
        .iter()
        .enumerate()
        .map(|(index, point)| {
            let basis = ProjectPointToCertifiedPlane2DBasis::builder()
                .source_point_identity(format!("{prefix}:point:{index}"))
                .source_point([1.0e12, 0.0, 0.0])
                .source_point_basis_digest("point-basis:kernel-area")
                .local_delta_from_frame_origin([point[0], point[1], 0.0])
                .local_frame_receipt(frame)
                .build()
                .expect("projection basis");
            let entry = project_point_to_certified_plane_2d_entry(
                ProjectPointToCertifiedPlane2DCase::from_local_frame(basis),
            );
            project_point_to_certified_plane_2d_facts(&entry, &projection_handle(world))
                .expect("projection")
        })
        .collect()
}

fn topology_basis(identity: &'static str) -> CertifiedTopologyLoopBasis2D {
    CertifiedTopologyLoopBasis2D::from_topology_loop_fact(
        identity,
        format!("membership:{identity}"),
        "topology-spatial-contract:kernel-area",
    )
}

fn signed_area_contracts(
    world: &'static str,
) -> CertifiedSignedArea2DContracts<CertifiedSignedArea2DQueryWorld> {
    CertifiedSignedArea2DContracts::new(signed_area_handle(world))
}

fn winding_contracts(
    world: &'static str,
) -> CertifiedPolygonWinding2DContracts<
    CertifiedPolygonWinding2DQueryWorld,
    CertifiedSegmentSegment2DQueryWorld,
    PlanarPredicateAuthorityQueryWorld,
> {
    CertifiedPolygonWinding2DContracts::new(
        winding_handle(world),
        CertifiedSegmentSegment2DContracts::new(segment_handle(world), predicate_handle()),
        predicate_handle(),
    )
}

fn signed_area_handle(
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

fn winding_handle(
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

fn segment_handle(
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

fn projection_handle(
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

fn precision_handle(
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

fn frame_handle(
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

fn predicate_handle() -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarPredicateAuthorityQueryDomain,
    PlanarPredicateAuthorityQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarPredicateAuthorityQueryDomain)
        .with_operating_context(PlanarPredicateAuthorityQueryWorld::new(
            "kernel-area-predicate",
        ))
        .validate()
        .expect("validated predicate")
        .admit()
        .expect("admitted predicate")
}
