use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_local_frame::{
    planar_local_frame_certificate_entry, planar_local_frame_certificate_facts,
    PlanarLocalFrameBasis, PlanarLocalFrameCertificateCase, PlanarLocalFrameCertificateQueryDomain,
    PlanarLocalFrameCertificateQueryWorld,
};
use worth_spatial::facade::planar_precision::{
    planar_precision_certification_entry, planar_precision_certification_facts,
    PlanarPrecisionBasis, PlanarPrecisionCertificationCase,
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
use worth_spatial::facade::planar_winding::{
    CertifiedLoopContainment, CertifiedLoopWinding, CertifiedPolygonWinding2D,
    CertifiedPolygonWinding2DContracts, CertifiedPolygonWinding2DQueryDomain,
    CertifiedPolygonWinding2DQueryWorld, CertifiedProjectedLoop2D, CertifiedTopologyLoopBasis2D,
};

#[test]
fn kernel_consumes_certified_polygon_winding_without_synthesizing_containment() {
    let world = "kernel-polygon-winding";
    let frame = certified_frame(world);
    let outer = CertifiedProjectedLoop2D::from_projected_vertices(
        "loop:kernel-outer",
        topology_basis("loop:kernel-outer"),
        loop_points(
            world,
            &frame,
            "outer",
            &[[0.0, 0.0], [4.0e-9, 0.0], [4.0e-9, 4.0e-9], [0.0, 4.0e-9]],
        ),
    )
    .expect("outer loop");
    let hole = CertifiedProjectedLoop2D::from_projected_vertices(
        "loop:kernel-hole",
        topology_basis("loop:kernel-hole"),
        loop_points(
            world,
            &frame,
            "hole",
            &[
                [1.0e-9, 1.0e-9],
                [2.0e-9, 1.0e-9],
                [2.0e-9, 2.0e-9],
                [1.0e-9, 2.0e-9],
            ],
        ),
    )
    .expect("hole loop");
    let contracts = CertifiedPolygonWinding2DContracts::new(
        winding_handle(world),
        CertifiedSegmentSegment2DContracts::new(segment_handle(world), predicate_handle()),
        predicate_handle(),
    );

    let plan = CertifiedPolygonWinding2D::certify(outer)
        .with_containment_candidate(hole)
        .within_planar_neighborhood("topology:kernel-face")
        .compile(&contracts)
        .expect("compiled winding plan");
    assert_eq!(plan.loop_count(), 2);
    assert_eq!(plan.segment_contact_pairs_required(), 16);

    let receipt = plan.certify().expect("winding receipt");
    assert_eq!(
        receipt.primary_winding(),
        CertifiedLoopWinding::CounterClockwise
    );
    assert_eq!(
        receipt.containment_for("loop:kernel-hole"),
        Some(CertifiedLoopContainment::ContainedHole)
    );
    assert!(receipt.counters().winding_predicates_consumed() >= 4);
    assert!(receipt.counters().segment_contacts_classified() >= 16);
    assert!(!receipt.fact_digest().is_empty());
}

fn loop_points(
    world: &'static str,
    frame: &worth_spatial::facade::planar_local_frame::PlanarLocalFrameCertificateReceipt,
    prefix: &'static str,
    points: &[[f64; 2]],
) -> Vec<ProjectPointToCertifiedPlane2DReceipt> {
    points
        .iter()
        .enumerate()
        .map(|(index, point)| {
            projected_point(
                world,
                frame,
                format!("point:{prefix}:{index}"),
                [point[0], point[1], 0.0],
            )
        })
        .collect()
}

fn projected_point(
    world: &'static str,
    frame: &worth_spatial::facade::planar_local_frame::PlanarLocalFrameCertificateReceipt,
    identity: String,
    local_delta: [f64; 3],
) -> ProjectPointToCertifiedPlane2DReceipt {
    let basis = ProjectPointToCertifiedPlane2DBasis::builder()
        .source_point_identity(identity)
        .source_point([1.0e12, 0.0, 0.0])
        .source_point_basis_digest("point-basis:kernel-winding")
        .local_delta_from_frame_origin(local_delta)
        .local_frame_receipt(frame)
        .build()
        .expect("projection basis");
    let entry = project_point_to_certified_plane_2d_entry(
        ProjectPointToCertifiedPlane2DCase::from_local_frame(basis),
    );
    project_point_to_certified_plane_2d_facts(&entry, &projection_handle(world))
        .expect("projection receipt")
}

fn certified_frame(
    world: &'static str,
) -> worth_spatial::facade::planar_local_frame::PlanarLocalFrameCertificateReceipt {
    let predicate = predicate_receipt();
    let precision_basis = PlanarPrecisionBasis::builder()
        .local_frame_identity("frame:kernel-winding")
        .topology_basis_identity("topology:kernel-winding")
        .movement_rotation_posture_identity("movement:kernel-stable")
        .tolerance_policy_identity("tolerance:kernel-exact")
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
        .frame_identity("frame:kernel-winding")
        .origin([1.0e12, 0.0, 0.0])
        .normal([0.0, 0.0, 1.0])
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .transform_chain_digest("transform:kernel-winding")
        .movement_rotation_posture_identity("movement:kernel-stable")
        .tolerance_policy_identity("tolerance:kernel-exact")
        .precision_receipt(&precision)
        .build()
        .expect("frame basis");
    let frame_entry = planar_local_frame_certificate_entry(
        PlanarLocalFrameCertificateCase::from_precision_basis(frame_basis),
    );
    planar_local_frame_certificate_facts(&frame_entry, &frame_handle(world)).expect("frame receipt")
}

fn predicate_receipt() -> worth_spatial::facade::planar_predicates::PlanarPredicateFactReceipt {
    let basis = PlanarPredicateInputBasis::from_projected_orient2d_points(
        "frame:kernel-winding",
        "topology:kernel-winding",
        "movement:kernel-stable",
        "tolerance:kernel-exact",
        [[0.0, 0.0], [1.0e-9, 0.0], [0.0, 1.0e-9]],
    );
    let entry = planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(basis));
    planar_predicate_authority_facts(&entry, &predicate_handle()).expect("predicate receipt")
}

fn topology_basis(identity: &'static str) -> CertifiedTopologyLoopBasis2D {
    CertifiedTopologyLoopBasis2D::from_topology_loop_fact(
        identity,
        format!("membership:{identity}"),
        "topology-spatial-contract:kernel-face",
    )
}

fn predicate_handle() -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarPredicateAuthorityQueryDomain,
    PlanarPredicateAuthorityQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarPredicateAuthorityQueryDomain)
        .with_operating_context(PlanarPredicateAuthorityQueryWorld::new("kernel-winding"))
        .validate()
        .expect("validated predicate")
        .admit()
        .expect("admitted predicate")
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
