use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_local_frame::{
    planar_local_frame_certificate_entry, planar_local_frame_certificate_facts,
    PlanarLocalFrameBasis, PlanarLocalFrameCertificateCase, PlanarLocalFrameCertificateQueryDomain,
    PlanarLocalFrameCertificateQueryWorld, PlanarLocalFrameCertificateReceipt,
};
use worth_spatial::facade::planar_overlap::{
    CertifiedCoplanarOverlapFace2D, CoplanarOverlapContractContracts,
    CoplanarOverlapContractExtractor, CoplanarOverlapContractQueryDomain,
    CoplanarOverlapContractQueryWorld,
};
use worth_spatial::facade::planar_precision::{
    planar_precision_certification_entry, planar_precision_certification_facts,
    PlanarPrecisionBasis, PlanarPrecisionCertificateReceipt, PlanarPrecisionCertificationCase,
    PlanarPrecisionCertificationQueryDomain, PlanarPrecisionCertificationQueryWorld,
};
use worth_spatial::facade::planar_predicates::{
    planar_predicate_authority_entry, planar_predicate_authority_facts,
    PlanarPredicateAuthorityCase, PlanarPredicateAuthorityQueryDomain,
    PlanarPredicateAuthorityQueryWorld, PlanarPredicateFactReceipt, PlanarPredicateInputBasis,
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
    AreaDegeneracyPolicy, CertifiedSignedArea2D, CertifiedSignedArea2DContracts,
    CertifiedSignedArea2DQueryDomain, CertifiedSignedArea2DQueryWorld,
};
use worth_spatial::facade::planar_winding::{
    CertifiedPolygonWinding2D, CertifiedPolygonWinding2DContracts,
    CertifiedPolygonWinding2DQueryDomain, CertifiedPolygonWinding2DQueryWorld,
    CertifiedProjectedLoop2D, CertifiedTopologyLoopBasis2D,
};

const NEIGHBORHOOD: &str = "topology:kernel-overlap";

#[test]
fn kernel_consumes_coplanar_overlap_contract_without_boolean_or_imprint_synthesis() {
    let world = "kernel-coplanar-overlap";
    let left = overlap_face(
        world,
        "face:kernel-left",
        &[[0.0, 0.0], [2.0e-9, 0.0], [2.0e-9, 2.0e-9], [0.0, 2.0e-9]],
    );
    let right = overlap_face(
        world,
        "face:kernel-right",
        &[
            [2.0e-9, 0.0],
            [4.0e-9, 0.0],
            [4.0e-9, 2.0e-9],
            [2.0e-9, 2.0e-9],
        ],
    );
    let contracts = overlap_contracts(world);
    let plan = CoplanarOverlapContractExtractor::between(left, right)
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&contracts)
        .expect("overlap plan");
    assert_eq!(plan.candidate_pair_breadth(), 16);
    assert_eq!(plan.topology_mutations(), 0);
    assert_eq!(plan.boolean_classifications(), 0);

    let receipt = plan.extract().expect("overlap receipt");
    assert_eq!(receipt.shared_intervals().len(), 1);
    assert_eq!(receipt.overlap_islands().len(), 1);
    assert_eq!(receipt.boolean_result(), None);
    assert_eq!(receipt.imprint_action(), None);
    assert!(!receipt.fact_digest().is_empty());
}

fn overlap_face(
    world: &'static str,
    face: &'static str,
    points: &[[f64; 2]],
) -> CertifiedCoplanarOverlapFace2D {
    let precision = precision_receipt(world);
    let frame = frame_receipt(world, &precision);
    let loop_identity = format!("loop:{face}");
    let projected_loop = CertifiedProjectedLoop2D::from_projected_vertices(
        loop_identity.clone(),
        topology_basis(&loop_identity),
        points.iter().enumerate().map(|(index, point)| {
            projected_point(world, &frame, format!("{face}:point:{index}"), point)
        }),
    )
    .expect("projected loop");
    let winding = CertifiedPolygonWinding2D::certify(projected_loop)
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&winding_contracts(world))
        .expect("winding plan")
        .certify()
        .expect("winding receipt");
    let signed_area = CertifiedSignedArea2D::measure_face(winding)
        .using_precision_basis(precision)
        .classifying_degeneracy(AreaDegeneracyPolicy::ClassifyWithoutRepair)
        .compile(&CertifiedSignedArea2DContracts::new(signed_area_handle(
            world,
        )))
        .expect("area plan")
        .certify()
        .expect("area receipt");
    CertifiedCoplanarOverlapFace2D::from_certified_area(face, signed_area).expect("overlap face")
}

fn projected_point(
    world: &'static str,
    frame: &PlanarLocalFrameCertificateReceipt,
    identity: String,
    point: &[f64; 2],
) -> ProjectPointToCertifiedPlane2DReceipt {
    let basis = ProjectPointToCertifiedPlane2DBasis::builder()
        .source_point_identity(identity)
        .source_point([1.0e12, 0.0, 0.0])
        .source_point_basis_digest("point-basis:kernel-overlap")
        .local_delta_from_frame_origin([point[0], point[1], 0.0])
        .local_frame_receipt(frame)
        .build()
        .expect("projection basis");
    project_point_to_certified_plane_2d_facts(
        &project_point_to_certified_plane_2d_entry(
            ProjectPointToCertifiedPlane2DCase::from_local_frame(basis),
        ),
        &projection_handle(world),
    )
    .expect("projection receipt")
}

fn frame_receipt(
    world: &'static str,
    precision: &PlanarPrecisionCertificateReceipt,
) -> PlanarLocalFrameCertificateReceipt {
    let basis = PlanarLocalFrameBasis::builder()
        .frame_identity("frame:kernel-overlap")
        .origin([1.0e12, 0.0, 0.0])
        .normal([0.0, 0.0, 1.0])
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .transform_chain_digest("transform:kernel-overlap")
        .movement_rotation_posture_identity("movement:kernel-overlap-stable")
        .tolerance_policy_identity("tolerance:kernel-overlap-exact")
        .precision_receipt(precision)
        .build()
        .expect("frame basis");
    planar_local_frame_certificate_facts(
        &planar_local_frame_certificate_entry(
            PlanarLocalFrameCertificateCase::from_precision_basis(basis),
        ),
        &frame_handle(world),
    )
    .expect("frame receipt")
}

fn precision_receipt(world: &'static str) -> PlanarPrecisionCertificateReceipt {
    let predicate = predicate_receipt();
    let basis = PlanarPrecisionBasis::builder()
        .local_frame_identity("frame:kernel-overlap")
        .topology_basis_identity("topology:kernel-overlap")
        .movement_rotation_posture_identity("movement:kernel-overlap-stable")
        .tolerance_policy_identity("tolerance:kernel-overlap-exact")
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .predicate_receipt(&predicate)
        .build()
        .expect("precision basis");
    planar_precision_certification_facts(
        &planar_precision_certification_entry(
            PlanarPrecisionCertificationCase::from_predicate_receipt(predicate, basis),
        ),
        &precision_handle(world),
    )
    .expect("precision receipt")
}

fn predicate_receipt() -> PlanarPredicateFactReceipt {
    let basis = PlanarPredicateInputBasis::from_projected_orient2d_points(
        "frame:kernel-overlap",
        "topology:kernel-overlap",
        "movement:kernel-overlap-stable",
        "tolerance:kernel-overlap-exact",
        [[0.0, 0.0], [1.0e-9, 0.0], [0.0, 1.0e-9]],
    );
    planar_predicate_authority_facts(
        &planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(basis)),
        &predicate_handle(),
    )
    .expect("predicate receipt")
}

fn topology_basis(identity: &str) -> CertifiedTopologyLoopBasis2D {
    CertifiedTopologyLoopBasis2D::from_topology_loop_fact(
        identity,
        format!("membership:{identity}"),
        "topology-spatial-contract:kernel-overlap",
    )
}

fn overlap_contracts(
    world: &'static str,
) -> CoplanarOverlapContractContracts<
    CoplanarOverlapContractQueryWorld,
    CertifiedSegmentSegment2DQueryWorld,
    PlanarPredicateAuthorityQueryWorld,
> {
    CoplanarOverlapContractContracts::new(
        overlap_handle(world),
        CertifiedSegmentSegment2DContracts::new(segment_handle(world), predicate_handle()),
    )
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

macro_rules! handle {
    ($fn_name:ident, $domain:expr, $world:expr, $domain_ty:ty, $world_ty:ty) => {
        fn $fn_name(
            world: &'static str,
        ) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<$domain_ty, $world_ty> {
            ForgeQueryApplicationFacade::runtime_backed_default()
                .domain($domain)
                .with_operating_context($world(world))
                .validate()
                .expect("validated kernel overlap domain")
                .admit()
                .expect("admitted kernel overlap domain")
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
            "kernel-overlap-predicate",
        ))
        .validate()
        .expect("validated predicate")
        .admit()
        .expect("admitted predicate")
}
