use std::collections::{BTreeMap, BTreeSet};

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
use worth_spatial::facade::planar_predicate_consumption::{
    PredicateCertificateConsumption, PredicateCertificateConsumptionContracts,
    PredicateCertificateConsumptionQueryDomain, PredicateCertificateConsumptionQueryWorld,
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
    CertifiedProjectedSegment2D, CertifiedSegmentSegment2D, CertifiedSegmentSegment2DContracts,
    CertifiedSegmentSegment2DQueryDomain, CertifiedSegmentSegment2DQueryWorld,
    CertifiedSegmentSegment2DReceipt,
};

const WORLD: &str = "kernel-predicate-consumption";
const TOPOLOGY: &str = "topology:kernel-predicate-consumption";
const MOVEMENT: &str = "movement:kernel-predicate-consumption-stable";
const FRAME: &str = "frame:kernel-predicate-consumption";
const TOLERANCE: &str = "tolerance:kernel-predicate-consumption";

#[test]
fn kernel_consumes_predicate_certificate_validation_without_local_predicate_summary() {
    let segment = segment_receipt();
    let predicates = segment_orientation_predicates(&segment);
    let predicate_digests = predicates
        .iter()
        .map(|receipt| receipt.fact_digest().to_string())
        .collect::<BTreeSet<_>>();

    let receipt = PredicateCertificateConsumption::for_planar_workload()
        .expecting_topology_basis(TOPOLOGY)
        .expecting_movement_rotation_posture(MOVEMENT)
        .expecting_local_frame(FRAME)
        .with_predicate_authority(predicates)
        .with_segment_contacts(vec![segment])
        .compile(&PredicateCertificateConsumptionContracts::new(
            predicate_consumption_handle(),
        ))
        .expect("kernel predicate consumption plan")
        .certify()
        .expect("kernel predicate consumption receipt");

    let consumed_digests = receipt
        .basis()
        .consumption_rows()
        .iter()
        .map(|row| {
            assert!(!row.certified_sign_identity().is_empty());
            assert!(!row.precision_escalation_identity().is_empty());
            assert_eq!(row.local_frame_identity(), FRAME);
            assert_eq!(row.topology_basis_identity(), TOPOLOGY);
            assert_eq!(row.movement_rotation_posture_identity(), MOVEMENT);
            row.predicate_fact_digest().to_string()
        })
        .collect::<BTreeSet<_>>();

    assert_eq!(consumed_digests, predicate_digests);
    assert!(receipt.proves_no_second_predicate_engine());
    assert_eq!(receipt.certified_predicate_rows(), 4);
    assert_eq!(receipt.counters().precision_metadata_rows(), 4);
    assert_eq!(receipt.counters().rejected_substitute_rows(), 0);
}

fn segment_receipt() -> CertifiedSegmentSegment2DReceipt {
    let predicate = seed_predicate_receipt();
    let precision = precision_receipt(&predicate);
    let frame = frame_receipt(&precision);
    let left_start = projection_receipt(&frame, "left:start", [0.0, 0.0]);
    let left_end = projection_receipt(&frame, "left:end", [0.0, 2.0e-9]);
    let right_start = projection_receipt(&frame, "right:start", [0.0, 2.0e-9]);
    let right_end = projection_receipt(&frame, "right:end", [0.0, 0.0]);
    let left = CertifiedProjectedSegment2D::from_projected_endpoints(
        "kernel-left-shared-edge",
        left_start,
        left_end,
    )
    .expect("left segment");
    let right = CertifiedProjectedSegment2D::from_projected_endpoints(
        "kernel-right-shared-edge",
        right_start,
        right_end,
    )
    .expect("right segment");

    CertifiedSegmentSegment2D::classify(left, right)
        .within_topology_basis(TOPOLOGY)
        .compile(&CertifiedSegmentSegment2DContracts::new(
            segment_handle(),
            predicate_handle(),
        ))
        .expect("segment plan")
        .certify()
        .expect("segment receipt")
}

fn segment_orientation_predicates(
    segment: &CertifiedSegmentSegment2DReceipt,
) -> Vec<PlanarPredicateFactReceipt> {
    let basis = segment.basis();
    let orientation_points = [
        [
            basis.first_start_point_2d(),
            basis.first_end_point_2d(),
            basis.second_start_point_2d(),
        ],
        [
            basis.first_start_point_2d(),
            basis.first_end_point_2d(),
            basis.second_end_point_2d(),
        ],
        [
            basis.second_start_point_2d(),
            basis.second_end_point_2d(),
            basis.first_start_point_2d(),
        ],
        [
            basis.second_start_point_2d(),
            basis.second_end_point_2d(),
            basis.first_end_point_2d(),
        ],
    ];
    let mut receipts = BTreeMap::new();
    for points in orientation_points {
        let predicate_basis = PlanarPredicateInputBasis::from_projected_orient2d_points(
            basis.frame_identity(),
            basis.topology_basis_identity(),
            basis.movement_rotation_posture_identity(),
            basis.tolerance_policy_identity(),
            points,
        );
        let receipt = planar_predicate_authority_facts(
            &planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(
                predicate_basis,
            )),
            &predicate_handle(),
        )
        .expect("segment predicate receipt");
        receipts.insert(receipt.fact_digest().to_string(), receipt);
    }
    receipts.into_values().collect()
}

fn seed_predicate_receipt() -> PlanarPredicateFactReceipt {
    let basis = PlanarPredicateInputBasis::from_projected_orient2d_points(
        FRAME,
        TOPOLOGY,
        MOVEMENT,
        TOLERANCE,
        [[0.0, 0.0], [1.0e-9, 0.0], [0.0, 1.0e-9]],
    );
    planar_predicate_authority_facts(
        &planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(basis)),
        &predicate_handle(),
    )
    .expect("seed predicate")
}

fn precision_receipt(predicate: &PlanarPredicateFactReceipt) -> PlanarPrecisionCertificateReceipt {
    let basis = PlanarPrecisionBasis::builder()
        .local_frame_identity(FRAME)
        .topology_basis_identity(TOPOLOGY)
        .movement_rotation_posture_identity(MOVEMENT)
        .tolerance_policy_identity(TOLERANCE)
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .predicate_receipt(predicate)
        .build()
        .expect("precision basis");
    planar_precision_certification_facts(
        &planar_precision_certification_entry(
            PlanarPrecisionCertificationCase::from_predicate_receipt(predicate.clone(), basis),
        ),
        &precision_handle(),
    )
    .expect("precision receipt")
}

fn frame_receipt(
    precision: &PlanarPrecisionCertificateReceipt,
) -> PlanarLocalFrameCertificateReceipt {
    let basis = PlanarLocalFrameBasis::builder()
        .frame_identity(FRAME)
        .origin([1.0e12, 0.0, 0.0])
        .normal([0.0, 0.0, 1.0])
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .transform_chain_digest("transform:kernel-predicate-consumption")
        .movement_rotation_posture_identity(MOVEMENT)
        .tolerance_policy_identity(TOLERANCE)
        .precision_receipt(precision)
        .build()
        .expect("frame basis");
    planar_local_frame_certificate_facts(
        &planar_local_frame_certificate_entry(
            PlanarLocalFrameCertificateCase::from_precision_basis(basis),
        ),
        &frame_handle(),
    )
    .expect("frame receipt")
}

fn projection_receipt(
    frame: &PlanarLocalFrameCertificateReceipt,
    identity: &'static str,
    point: [f64; 2],
) -> ProjectPointToCertifiedPlane2DReceipt {
    let basis = ProjectPointToCertifiedPlane2DBasis::builder()
        .source_point_identity(identity)
        .source_point([1.0e12, 0.0, 0.0])
        .source_point_basis_digest(format!("basis:{identity}"))
        .local_delta_from_frame_origin([point[0], point[1], 0.0])
        .local_frame_receipt(frame)
        .build()
        .expect("projection basis");
    project_point_to_certified_plane_2d_facts(
        &project_point_to_certified_plane_2d_entry(
            ProjectPointToCertifiedPlane2DCase::from_local_frame(basis),
        ),
        &projection_handle(),
    )
    .expect("projection receipt")
}

macro_rules! handle {
    ($fn_name:ident, $domain:expr, $world:expr, $domain_ty:ty, $world_ty:ty) => {
        fn $fn_name(
        ) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<$domain_ty, $world_ty> {
            ForgeQueryApplicationFacade::runtime_backed_default()
                .domain($domain)
                .with_operating_context($world(WORLD))
                .validate()
                .expect("validated kernel predicate consumption domain")
                .admit()
                .expect("admitted kernel predicate consumption domain")
        }
    };
}

handle!(
    predicate_consumption_handle,
    PredicateCertificateConsumptionQueryDomain,
    PredicateCertificateConsumptionQueryWorld::new,
    PredicateCertificateConsumptionQueryDomain,
    PredicateCertificateConsumptionQueryWorld
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
handle!(
    projection_handle,
    ProjectPointToCertifiedPlane2DQueryDomain,
    ProjectPointToCertifiedPlane2DQueryWorld::new,
    ProjectPointToCertifiedPlane2DQueryDomain,
    ProjectPointToCertifiedPlane2DQueryWorld
);
handle!(
    segment_handle,
    CertifiedSegmentSegment2DQueryDomain,
    CertifiedSegmentSegment2DQueryWorld::new,
    CertifiedSegmentSegment2DQueryDomain,
    CertifiedSegmentSegment2DQueryWorld
);

fn predicate_handle() -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarPredicateAuthorityQueryDomain,
    PlanarPredicateAuthorityQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarPredicateAuthorityQueryDomain)
        .with_operating_context(PlanarPredicateAuthorityQueryWorld::new(
            "kernel-predicate-consumption-authority",
        ))
        .validate()
        .expect("validated predicate")
        .admit()
        .expect("admitted predicate")
}
