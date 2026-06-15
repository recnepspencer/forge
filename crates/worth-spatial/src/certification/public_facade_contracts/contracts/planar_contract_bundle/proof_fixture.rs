use worth_spatial::facade::planar_contracts::{
    admit_planar_contract_family, PlanarAdmissionFamily, PlanarAdmissionReceipt,
    PlanarRuntimeConcern,
};
use worth_spatial::facade::planar_local_frame::{
    planar_local_frame_certificate_entry, planar_local_frame_certificate_facts,
    PlanarLocalFrameBasis, PlanarLocalFrameCertificateCase, PlanarLocalFrameCertificateReceipt,
};
use worth_spatial::facade::planar_overlap::{
    CertifiedCoplanarOverlapFace2D, CoplanarOverlapContractContracts,
    CoplanarOverlapContractExtractor, CoplanarOverlapContractReceipt,
};
use worth_spatial::facade::planar_precision::{
    planar_precision_certification_entry, planar_precision_certification_facts,
    PlanarPrecisionBasis, PlanarPrecisionCertificateReceipt, PlanarPrecisionCertificationCase,
};
use worth_spatial::facade::planar_predicate_consumption::{
    PredicateCertificateConsumption, PredicateCertificateConsumptionContracts,
    PredicateCertificateConsumptionReceipt,
};
use worth_spatial::facade::planar_predicates::{
    planar_predicate_authority_entry, planar_predicate_authority_facts,
    PlanarPredicateAuthorityCase, PlanarPredicateFactReceipt, PlanarPredicateInputBasis,
};
use worth_spatial::facade::planar_projection::{
    project_point_to_certified_plane_2d_entry, project_point_to_certified_plane_2d_facts,
    ProjectPointToCertifiedPlane2DBasis, ProjectPointToCertifiedPlane2DCase,
    ProjectPointToCertifiedPlane2DReceipt,
};
use worth_spatial::facade::planar_segment_segment::{
    CertifiedProjectedSegment2D, CertifiedSegmentSegment2D, CertifiedSegmentSegment2DContracts,
    CertifiedSegmentSegment2DReceipt,
};
use worth_spatial::facade::planar_signed_area::{
    AreaDegeneracyPolicy, CertifiedSignedArea2D, CertifiedSignedArea2DContracts,
    CertifiedSignedArea2DReceipt,
};
use worth_spatial::facade::planar_topology_contract::PlanarTopologyContractCompletenessReceipt;
use worth_spatial::facade::planar_winding::{
    CertifiedPolygonWinding2D, CertifiedPolygonWinding2DContracts,
    CertifiedPolygonWinding2DReceipt, CertifiedProjectedLoop2D, CertifiedTopologyLoopBasis2D,
};

use super::runtime_handles::{
    frame_handle, overlap_handle, precision_handle, predicate_consumption_handle, predicate_handle,
    projection_handle, segment_handle, signed_area_handle, winding_handle,
};
use super::topology_fixture::topology_contract_receipt;

pub(crate) const TOPOLOGY: &str = "topology:bundle";
pub(crate) const MOVEMENT: &str = "movement:bundle-stable";
pub(crate) const NEIGHBORHOOD: &str = "neighborhood:bundle";

#[derive(Clone)]
pub(crate) struct CompleteBundleParts {
    pub(crate) admission: PlanarAdmissionReceipt,
    pub(crate) topology_contract: PlanarTopologyContractCompletenessReceipt,
    pub(crate) precision: PlanarPrecisionCertificateReceipt,
    pub(crate) frame: PlanarLocalFrameCertificateReceipt,
    pub(crate) projections: Vec<ProjectPointToCertifiedPlane2DReceipt>,
    pub(crate) predicates: Vec<PlanarPredicateFactReceipt>,
    pub(crate) segments: Vec<CertifiedSegmentSegment2DReceipt>,
    pub(crate) winding: CertifiedPolygonWinding2DReceipt,
    pub(crate) signed_area: CertifiedSignedArea2DReceipt,
    pub(crate) overlap: CoplanarOverlapContractReceipt,
    pub(crate) predicate_consumption: PredicateCertificateConsumptionReceipt,
}

pub(crate) fn complete_bundle_parts(world: &'static str) -> CompleteBundleParts {
    let admission = admit_planar_contract_family(
        PlanarAdmissionFamily::PlanarContractBundle,
        PlanarRuntimeConcern::BooleanReadinessBundle,
    )
    .expect("bundle admission");
    let topology_contract = topology_contract_receipt(world);
    let predicate = predicate_receipt(MOVEMENT);
    let precision = precision_receipt(world, &predicate);
    let frame = frame_receipt(world, &precision);
    let left_points = [[0.0, 0.0], [2.0e-9, 0.0], [2.0e-9, 2.0e-9], [0.0, 2.0e-9]];
    let right_points = [
        [2.0e-9, 0.0],
        [4.0e-9, 0.0],
        [4.0e-9, 2.0e-9],
        [2.0e-9, 2.0e-9],
    ];
    let left_projection = projected_points(world, &frame, "face:left", &left_points);
    let right_projection = projected_points(world, &frame, "face:right", &right_points);
    let left_winding = winding_receipt(world, "face:left", left_projection.clone());
    let right_winding = winding_receipt(world, "face:right", right_projection.clone());
    let signed_area = signed_area_receipt(world, left_winding.clone(), precision.clone());
    let right_area = signed_area_receipt(world, right_winding, precision.clone());
    let overlap = overlap_receipt(world, signed_area.clone(), right_area);
    let segment = segment_receipt(
        world,
        left_projection[1].clone(),
        left_projection[2].clone(),
        right_projection[3].clone(),
        right_projection[0].clone(),
    );
    let segment_predicates = segment_orientation_predicates(&segment);
    let predicate_consumption =
        predicate_consumption_receipt(world, segment.clone(), segment_predicates.clone());
    let mut predicates = vec![predicate];
    predicates.extend(segment_predicates);
    let mut projections = left_projection;
    projections.extend(right_projection);
    CompleteBundleParts {
        admission,
        topology_contract,
        precision,
        frame,
        projections,
        predicates,
        segments: vec![segment],
        winding: left_winding,
        signed_area,
        overlap,
        predicate_consumption,
    }
}

pub(crate) fn stray_projection_receipt(
    world: &'static str,
    frame: &PlanarLocalFrameCertificateReceipt,
) -> ProjectPointToCertifiedPlane2DReceipt {
    let basis = ProjectPointToCertifiedPlane2DBasis::builder()
        .source_point_identity("stray:projection:not-consumed")
        .source_point([1.0e12, 0.0, 0.0])
        .source_point_basis_digest("point-basis:stray:not-consumed")
        .local_delta_from_frame_origin([9.0e-9, 9.0e-9, 0.0])
        .local_frame_receipt(frame)
        .build()
        .expect("stray projection basis");
    project_point_to_certified_plane_2d_facts(
        &project_point_to_certified_plane_2d_entry(
            ProjectPointToCertifiedPlane2DCase::from_local_frame(basis),
        ),
        &projection_handle(world),
    )
    .expect("stray projection receipt")
}

fn precision_receipt(
    world: &'static str,
    predicate: &PlanarPredicateFactReceipt,
) -> PlanarPrecisionCertificateReceipt {
    let basis = PlanarPrecisionBasis::builder()
        .local_frame_identity("frame:bundle")
        .topology_basis_identity(TOPOLOGY)
        .movement_rotation_posture_identity(MOVEMENT)
        .tolerance_policy_identity("tolerance:bundle")
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
        &precision_handle(world),
    )
    .expect("precision receipt")
}

fn frame_receipt(
    world: &'static str,
    precision: &PlanarPrecisionCertificateReceipt,
) -> PlanarLocalFrameCertificateReceipt {
    let basis = PlanarLocalFrameBasis::builder()
        .frame_identity("frame:bundle")
        .origin([1.0e12, 0.0, 0.0])
        .normal([0.0, 0.0, 1.0])
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .transform_chain_digest("transform:bundle")
        .movement_rotation_posture_identity(MOVEMENT)
        .tolerance_policy_identity("tolerance:bundle")
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

fn projected_points(
    world: &'static str,
    frame: &PlanarLocalFrameCertificateReceipt,
    face: &'static str,
    points: &[[f64; 2]; 4],
) -> Vec<ProjectPointToCertifiedPlane2DReceipt> {
    points
        .iter()
        .enumerate()
        .map(|(index, point)| {
            let basis = ProjectPointToCertifiedPlane2DBasis::builder()
                .source_point_identity(format!("{face}:point:{index}"))
                .source_point([1.0e12, 0.0, 0.0])
                .source_point_basis_digest(format!("point-basis:{face}:{index}"))
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
        })
        .collect()
}

fn winding_receipt(
    world: &'static str,
    face: &'static str,
    projections: Vec<ProjectPointToCertifiedPlane2DReceipt>,
) -> CertifiedPolygonWinding2DReceipt {
    let loop_identity = format!("loop:{face}");
    let projected_loop = CertifiedProjectedLoop2D::from_projected_vertices(
        loop_identity.clone(),
        topology_basis(&loop_identity),
        projections,
    )
    .expect("projected loop");
    CertifiedPolygonWinding2D::certify(projected_loop)
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&CertifiedPolygonWinding2DContracts::new(
            winding_handle(world),
            CertifiedSegmentSegment2DContracts::new(segment_handle(world), predicate_handle()),
            predicate_handle(),
        ))
        .expect("winding plan")
        .certify()
        .expect("winding receipt")
}

fn signed_area_receipt(
    world: &'static str,
    winding: CertifiedPolygonWinding2DReceipt,
    precision: PlanarPrecisionCertificateReceipt,
) -> CertifiedSignedArea2DReceipt {
    CertifiedSignedArea2D::measure_face(winding)
        .using_precision_basis(precision)
        .classifying_degeneracy(AreaDegeneracyPolicy::ClassifyWithoutRepair)
        .compile(&CertifiedSignedArea2DContracts::new(signed_area_handle(
            world,
        )))
        .expect("area plan")
        .certify()
        .expect("area receipt")
}

fn overlap_receipt(
    world: &'static str,
    left: CertifiedSignedArea2DReceipt,
    right: CertifiedSignedArea2DReceipt,
) -> CoplanarOverlapContractReceipt {
    let left = CertifiedCoplanarOverlapFace2D::from_certified_area("face:left", left)
        .expect("left overlap face");
    let right = CertifiedCoplanarOverlapFace2D::from_certified_area("face:right", right)
        .expect("right overlap face");
    CoplanarOverlapContractExtractor::between(left, right)
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&CoplanarOverlapContractContracts::new(
            overlap_handle(world),
            CertifiedSegmentSegment2DContracts::new(segment_handle(world), predicate_handle()),
        ))
        .expect("overlap plan")
        .extract()
        .expect("overlap receipt")
}

fn segment_receipt(
    world: &'static str,
    left_start: ProjectPointToCertifiedPlane2DReceipt,
    left_end: ProjectPointToCertifiedPlane2DReceipt,
    right_start: ProjectPointToCertifiedPlane2DReceipt,
    right_end: ProjectPointToCertifiedPlane2DReceipt,
) -> CertifiedSegmentSegment2DReceipt {
    let left = CertifiedProjectedSegment2D::from_projected_endpoints(
        "segment:left-shared",
        left_start,
        left_end,
    )
    .expect("left segment");
    let right = CertifiedProjectedSegment2D::from_projected_endpoints(
        "segment:right-shared",
        right_start,
        right_end,
    )
    .expect("right segment");
    CertifiedSegmentSegment2D::classify(left, right)
        .within_topology_basis(TOPOLOGY)
        .compile(&CertifiedSegmentSegment2DContracts::new(
            segment_handle(world),
            predicate_handle(),
        ))
        .expect("segment plan")
        .certify()
        .expect("segment receipt")
}

fn predicate_receipt(movement: &'static str) -> PlanarPredicateFactReceipt {
    let basis = PlanarPredicateInputBasis::from_projected_orient2d_points(
        "frame:bundle",
        TOPOLOGY,
        movement,
        "tolerance:bundle",
        [[0.0, 0.0], [1.0e-9, 0.0], [0.0, 1.0e-9]],
    );
    planar_predicate_authority_facts(
        &planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(basis)),
        &predicate_handle(),
    )
    .expect("predicate receipt")
}

fn segment_orientation_predicates(
    segment: &CertifiedSegmentSegment2DReceipt,
) -> Vec<PlanarPredicateFactReceipt> {
    let basis = segment.basis();
    [
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
    ]
    .into_iter()
    .fold(Vec::new(), |mut unique, points| {
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
        .expect("segment orientation predicate");
        if unique.iter().all(|existing: &PlanarPredicateFactReceipt| {
            existing.fact_digest() != receipt.fact_digest()
        }) {
            unique.push(receipt);
        }
        unique
    })
}

fn predicate_consumption_receipt(
    world: &'static str,
    segment: CertifiedSegmentSegment2DReceipt,
    predicates: Vec<PlanarPredicateFactReceipt>,
) -> PredicateCertificateConsumptionReceipt {
    PredicateCertificateConsumption::for_planar_workload()
        .expecting_topology_basis(TOPOLOGY)
        .expecting_movement_rotation_posture(MOVEMENT)
        .expecting_local_frame("frame:bundle")
        .with_predicate_authority(predicates)
        .with_segment_contacts(vec![segment])
        .compile(&PredicateCertificateConsumptionContracts::new(
            predicate_consumption_handle(world),
        ))
        .expect("predicate consumption plan")
        .certify()
        .expect("predicate consumption receipt")
}

fn topology_basis(identity: &str) -> CertifiedTopologyLoopBasis2D {
    CertifiedTopologyLoopBasis2D::from_topology_loop_fact(
        identity,
        format!("membership:{identity}"),
        "topology-spatial-contract:bundle",
    )
}
