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
use worth_spatial::facade::planar_predicates::{
    planar_predicate_authority_entry, planar_predicate_authority_facts,
    PlanarPredicateAuthorityCase, PlanarPredicateAuthorityQueryWorld, PlanarPredicateFactReceipt,
    PlanarPredicateInputBasis,
};
use worth_spatial::facade::planar_projection::{
    project_point_to_certified_plane_2d_entry, project_point_to_certified_plane_2d_facts,
    ProjectPointToCertifiedPlane2DBasis, ProjectPointToCertifiedPlane2DCase,
};
use worth_spatial::facade::planar_segment_segment::{
    CertifiedSegmentSegment2DContracts, CertifiedSegmentSegment2DQueryWorld,
};
use worth_spatial::facade::planar_signed_area::{
    AreaDegeneracyPolicy, CertifiedSignedArea2D, CertifiedSignedArea2DContracts,
};
use worth_spatial::facade::planar_winding::{
    CertifiedPolygonWinding2D, CertifiedPolygonWinding2DContracts, CertifiedProjectedLoop2D,
    CertifiedTopologyLoopBasis2D,
};

use super::scenario::{storm_regions, StormRegion};
use crate::public_api_planar_overlap::runtime_handles::{
    frame_handle, overlap_handle, precision_handle, predicate_handle, projection_handle,
    segment_handle, signed_area_handle, winding_handle,
};

const STORM_NEIGHBORHOOD: &str = "topology:mb1-storm-overlap-neighborhood";
const STORM_MOVEMENT: &str = "movement:mb1-storm-canonical";

pub(crate) fn certify_storm_overlap_extractions(
    world: &'static str,
) -> Vec<CoplanarOverlapContractReceipt> {
    storm_regions()
        .into_iter()
        .map(|region| certify_storm_region_overlap(world, &region))
        .collect()
}

pub(crate) fn certify_storm_region_overlap(
    world: &'static str,
    region: &StormRegion,
) -> CoplanarOverlapContractReceipt {
    CoplanarOverlapContractExtractor::between(
        storm_face(world, region.first_face_identity(), &region.first_face),
        storm_face(world, region.second_face_identity(), &region.second_face),
    )
    .within_planar_neighborhood(STORM_NEIGHBORHOOD)
    .compile(&overlap_contracts(world))
    .expect("MB1 storm overlap extraction plan")
    .extract()
    .expect("MB1 storm overlap extraction receipt")
}

pub(crate) fn deny_storm_tiny_rotation(
    world: &'static str,
    region: &StormRegion,
) -> worth_spatial::facade::planar_overlap::CoplanarOverlapDenial {
    let mut moved = region.clone();
    moved.second_face = moved
        .second_face
        .iter()
        .map(|point| [point[0], point[1] + 1.0e-15])
        .collect();
    match CoplanarOverlapContractExtractor::between(
        storm_face(world, region.first_face_identity(), &region.first_face),
        storm_face_with_movement(
            world,
            moved.second_face_identity(),
            "movement:tiny-rotation-exits-coplanar-class",
            &moved.second_face,
        ),
    )
    .within_planar_neighborhood(STORM_NEIGHBORHOOD)
    .compile(&overlap_contracts(world))
    {
        Ok(_) => panic!("tiny rotation must deny before overlap extraction"),
        Err(denial) => denial,
    }
}

pub(crate) fn certify_storm_policy_required_overlap(
    world: &'static str,
) -> CoplanarOverlapContractReceipt {
    let region = super::scenario::policy_required_region();
    CoplanarOverlapContractExtractor::between(
        storm_face_with_candidate(
            world,
            region.first_face_identity(),
            &region.first_face,
            region
                .containment_candidate
                .as_ref()
                .expect("policy region candidate"),
        ),
        storm_face(world, region.second_face_identity(), &region.second_face),
    )
    .within_planar_neighborhood(STORM_NEIGHBORHOOD)
    .compile(&overlap_contracts(world))
    .expect("MB1 storm policy overlap plan")
    .extract()
    .expect("MB1 storm policy overlap receipt")
}

fn overlap_contracts(
    world: &'static str,
) -> CoplanarOverlapContractContracts<
    worth_spatial::facade::planar_overlap::CoplanarOverlapContractQueryWorld,
    CertifiedSegmentSegment2DQueryWorld,
    PlanarPredicateAuthorityQueryWorld,
> {
    CoplanarOverlapContractContracts::new(
        overlap_handle(world),
        CertifiedSegmentSegment2DContracts::new(segment_handle(world), predicate_handle()),
    )
}

fn storm_face(
    world: &'static str,
    face: String,
    points: &[[f64; 2]],
) -> CertifiedCoplanarOverlapFace2D {
    storm_face_with_movement(world, face, STORM_MOVEMENT, points)
}

fn storm_face_with_candidate(
    world: &'static str,
    face: String,
    outer_points: &[[f64; 2]],
    candidate_points: &[[f64; 2]],
) -> CertifiedCoplanarOverlapFace2D {
    let precision = precision_receipt(world, STORM_MOVEMENT);
    let frame = frame_receipt(world, STORM_MOVEMENT, &precision);
    let outer_identity = format!("loop:{face}:outer");
    let candidate_identity = format!("loop:{face}:candidate");
    let outer_loop = projected_loop(world, &frame, &face, &outer_identity, outer_points);
    let candidate_loop =
        projected_loop(world, &frame, &face, &candidate_identity, candidate_points);
    let winding_contracts = CertifiedPolygonWinding2DContracts::new(
        winding_handle(world),
        CertifiedSegmentSegment2DContracts::new(segment_handle(world), predicate_handle()),
        predicate_handle(),
    );
    let winding = CertifiedPolygonWinding2D::certify(outer_loop)
        .with_containment_candidate(candidate_loop)
        .within_planar_neighborhood(STORM_NEIGHBORHOOD)
        .compile(&winding_contracts)
        .expect("MB1 candidate winding plan")
        .certify()
        .expect("MB1 candidate winding receipt");
    certified_face_from_winding(world, face, precision, winding)
}

fn storm_face_with_movement(
    world: &'static str,
    face: String,
    movement: &'static str,
    points: &[[f64; 2]],
) -> CertifiedCoplanarOverlapFace2D {
    let precision = precision_receipt(world, movement);
    let frame = frame_receipt(world, movement, &precision);
    let loop_identity = format!("loop:{face}");
    let projected_loop = projected_loop(world, &frame, &face, &loop_identity, points);
    let winding_contracts = CertifiedPolygonWinding2DContracts::new(
        winding_handle(world),
        CertifiedSegmentSegment2DContracts::new(segment_handle(world), predicate_handle()),
        predicate_handle(),
    );
    let winding = CertifiedPolygonWinding2D::certify(projected_loop)
        .within_planar_neighborhood(STORM_NEIGHBORHOOD)
        .compile(&winding_contracts)
        .expect("MB1 storm winding plan")
        .certify()
        .expect("MB1 storm winding receipt");
    certified_face_from_winding(world, face, precision, winding)
}

fn projected_loop(
    world: &'static str,
    frame: &PlanarLocalFrameCertificateReceipt,
    face: &str,
    loop_identity: &str,
    points: &[[f64; 2]],
) -> CertifiedProjectedLoop2D {
    CertifiedProjectedLoop2D::from_projected_vertices(
        loop_identity.to_string(),
        topology_basis(loop_identity),
        points.iter().enumerate().map(|(index, point)| {
            projected_point(world, frame, format!("{face}:point:{index}"), point)
        }),
    )
    .expect("MB1 projected storm loop")
}

fn certified_face_from_winding(
    world: &'static str,
    face: String,
    precision: PlanarPrecisionCertificateReceipt,
    winding: worth_spatial::facade::planar_winding::CertifiedPolygonWinding2DReceipt,
) -> CertifiedCoplanarOverlapFace2D {
    let signed_area = CertifiedSignedArea2D::measure_face(winding)
        .using_precision_basis(precision)
        .classifying_degeneracy(AreaDegeneracyPolicy::ClassifyWithoutRepair)
        .compile(&CertifiedSignedArea2DContracts::new(signed_area_handle(
            world,
        )))
        .expect("MB1 signed area plan")
        .certify()
        .expect("MB1 signed area receipt");
    CertifiedCoplanarOverlapFace2D::from_certified_area(face, signed_area)
        .expect("MB1 certified overlap face")
}

fn projected_point(
    world: &'static str,
    frame: &PlanarLocalFrameCertificateReceipt,
    identity: String,
    point: &[f64; 2],
) -> worth_spatial::facade::planar_projection::ProjectPointToCertifiedPlane2DReceipt {
    let basis = ProjectPointToCertifiedPlane2DBasis::builder()
        .source_point_identity(identity)
        .source_point([1.0e12, 0.0, 0.0])
        .source_point_basis_digest("point-basis:mb1-storm-local")
        .local_delta_from_frame_origin([point[0], point[1], 0.0])
        .local_frame_receipt(frame)
        .build()
        .expect("MB1 storm projection basis");
    project_point_to_certified_plane_2d_facts(
        &project_point_to_certified_plane_2d_entry(
            ProjectPointToCertifiedPlane2DCase::from_local_frame(basis),
        ),
        &projection_handle(world),
    )
    .expect("MB1 storm projection receipt")
}

fn frame_receipt(
    world: &'static str,
    movement: &'static str,
    precision: &PlanarPrecisionCertificateReceipt,
) -> PlanarLocalFrameCertificateReceipt {
    let basis = PlanarLocalFrameBasis::builder()
        .frame_identity("frame:mb1-storm-local-xy")
        .origin([1.0e12, 0.0, 0.0])
        .normal([0.0, 0.0, 1.0])
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .transform_chain_digest("transform:mb1-storm")
        .movement_rotation_posture_identity(movement)
        .tolerance_policy_identity("tolerance:mb1-storm-exact")
        .precision_receipt(precision)
        .build()
        .expect("MB1 storm frame basis");
    planar_local_frame_certificate_facts(
        &planar_local_frame_certificate_entry(
            PlanarLocalFrameCertificateCase::from_precision_basis(basis),
        ),
        &frame_handle(world),
    )
    .expect("MB1 storm frame receipt")
}

fn precision_receipt(
    world: &'static str,
    movement: &'static str,
) -> PlanarPrecisionCertificateReceipt {
    let predicate = predicate_receipt(movement);
    let basis = PlanarPrecisionBasis::builder()
        .local_frame_identity("frame:mb1-storm-local-xy")
        .topology_basis_identity("topology:mb1-storm")
        .movement_rotation_posture_identity(movement)
        .tolerance_policy_identity("tolerance:mb1-storm-exact")
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .predicate_receipt(&predicate)
        .build()
        .expect("MB1 storm precision basis");
    planar_precision_certification_facts(
        &planar_precision_certification_entry(
            PlanarPrecisionCertificationCase::from_predicate_receipt(predicate, basis),
        ),
        &precision_handle(world),
    )
    .expect("MB1 storm precision receipt")
}

fn predicate_receipt(movement: &'static str) -> PlanarPredicateFactReceipt {
    let basis = PlanarPredicateInputBasis::from_projected_orient2d_points(
        "frame:mb1-storm-local-xy",
        "topology:mb1-storm",
        movement,
        "tolerance:mb1-storm-exact",
        [[0.0, 0.0], [1.0e-9, 0.0], [0.0, 1.0e-9]],
    );
    planar_predicate_authority_facts(
        &planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(basis)),
        &predicate_handle(),
    )
    .expect("MB1 storm predicate receipt")
}

fn topology_basis(identity: &str) -> CertifiedTopologyLoopBasis2D {
    CertifiedTopologyLoopBasis2D::from_topology_loop_fact(
        identity,
        format!("membership:{identity}"),
        "topology-spatial-contract:mb1-storm",
    )
}
