use worth_spatial::facade::planar_local_frame::{
    planar_local_frame_certificate_entry, planar_local_frame_certificate_facts,
    PlanarLocalFrameBasis, PlanarLocalFrameCertificateCase, PlanarLocalFrameCertificateReceipt,
};
use worth_spatial::facade::planar_overlap::{
    CertifiedCoplanarOverlapFace2D, CoplanarOverlapContractContracts,
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

use super::runtime_handles::{
    frame_handle, overlap_handle, precision_handle, predicate_handle, projection_handle,
    segment_handle, signed_area_handle, winding_handle,
};

pub(crate) const NEIGHBORHOOD: &str = "topology:overlap-neighborhood";

pub(crate) fn overlap_contracts(
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

pub(crate) fn overlap_face(
    world: &'static str,
    face: impl Into<String>,
    movement: &'static str,
    points: &[[f64; 2]],
) -> CertifiedCoplanarOverlapFace2D {
    let face = face.into();
    let precision = precision_receipt(world, movement);
    let frame = frame_receipt(world, movement, &precision);
    let loop_identity = format!("loop:{face}");
    let projected_loop = CertifiedProjectedLoop2D::from_projected_vertices(
        loop_identity.clone(),
        topology_basis(&loop_identity),
        points.iter().enumerate().map(|(index, point)| {
            projected_point(world, &frame, format!("{face}:point:{index}"), point)
        }),
    )
    .expect("projected overlap loop");
    let winding_contracts = CertifiedPolygonWinding2DContracts::new(
        winding_handle(world),
        CertifiedSegmentSegment2DContracts::new(segment_handle(world), predicate_handle()),
        predicate_handle(),
    );
    let winding = CertifiedPolygonWinding2D::certify(projected_loop)
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&winding_contracts)
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
    CertifiedCoplanarOverlapFace2D::from_certified_area(face, signed_area)
        .expect("certified overlap face")
}

pub(crate) fn overlap_face_with_containment_candidate(
    world: &'static str,
    face: impl Into<String>,
    movement: &'static str,
    outer_points: &[[f64; 2]],
    candidate_points: &[[f64; 2]],
) -> CertifiedCoplanarOverlapFace2D {
    let face = face.into();
    let precision = precision_receipt(world, movement);
    let frame = frame_receipt(world, movement, &precision);
    let outer_identity = format!("loop:{face}:outer");
    let candidate_identity = format!("loop:{face}:candidate");
    let outer_loop = CertifiedProjectedLoop2D::from_projected_vertices(
        outer_identity.clone(),
        topology_basis(&outer_identity),
        outer_points.iter().enumerate().map(|(index, point)| {
            projected_point(world, &frame, format!("{face}:outer:point:{index}"), point)
        }),
    )
    .expect("outer overlap loop");
    let candidate_loop = CertifiedProjectedLoop2D::from_projected_vertices(
        candidate_identity.clone(),
        topology_basis(&candidate_identity),
        candidate_points.iter().enumerate().map(|(index, point)| {
            projected_point(
                world,
                &frame,
                format!("{face}:candidate:point:{index}"),
                point,
            )
        }),
    )
    .expect("candidate overlap loop");
    let winding_contracts = CertifiedPolygonWinding2DContracts::new(
        winding_handle(world),
        CertifiedSegmentSegment2DContracts::new(segment_handle(world), predicate_handle()),
        predicate_handle(),
    );
    let winding = CertifiedPolygonWinding2D::certify(outer_loop)
        .with_containment_candidate(candidate_loop)
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&winding_contracts)
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
    CertifiedCoplanarOverlapFace2D::from_certified_area(face, signed_area)
        .expect("certified overlap face")
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
        .source_point_basis_digest("point-basis:overlap-local")
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
    movement: &'static str,
    precision: &PlanarPrecisionCertificateReceipt,
) -> PlanarLocalFrameCertificateReceipt {
    let basis = PlanarLocalFrameBasis::builder()
        .frame_identity("frame:overlap-local-xy")
        .origin([1.0e12, 0.0, 0.0])
        .normal([0.0, 0.0, 1.0])
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .transform_chain_digest("transform:overlap")
        .movement_rotation_posture_identity(movement)
        .tolerance_policy_identity("tolerance:overlap-exact")
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

fn precision_receipt(
    world: &'static str,
    movement: &'static str,
) -> PlanarPrecisionCertificateReceipt {
    let predicate = predicate_receipt(movement);
    let basis = PlanarPrecisionBasis::builder()
        .local_frame_identity("frame:overlap-local-xy")
        .topology_basis_identity("topology:overlap")
        .movement_rotation_posture_identity(movement)
        .tolerance_policy_identity("tolerance:overlap-exact")
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

fn predicate_receipt(movement: &'static str) -> PlanarPredicateFactReceipt {
    let basis = PlanarPredicateInputBasis::from_projected_orient2d_points(
        "frame:overlap-local-xy",
        "topology:overlap",
        movement,
        "tolerance:overlap-exact",
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
        "topology-spatial-contract:overlap",
    )
}
