use worth_spatial::facade::planar_local_frame::{
    planar_local_frame_certificate_entry, planar_local_frame_certificate_facts,
    PlanarLocalFrameBasis, PlanarLocalFrameCertificateCase, PlanarLocalFrameCertificateReceipt,
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
    ProjectPointToCertifiedPlane2DReceipt,
};
use worth_spatial::facade::planar_segment_segment::{
    CertifiedSegmentSegment2DContracts, CertifiedSegmentSegment2DQueryWorld,
};
use worth_spatial::facade::planar_signed_area::{
    CertifiedSignedArea2DContracts, CertifiedSignedArea2DQueryWorld,
};
use worth_spatial::facade::planar_winding::{
    CertifiedPolygonWinding2DContracts, CertifiedPolygonWinding2DQueryWorld,
    CertifiedTopologyLoopBasis2D,
};

use super::runtime_handles::{
    frame_handle, precision_handle, predicate_handle, projection_handle, segment_handle,
    signed_area_handle, winding_handle,
};

pub(crate) fn signed_area_contracts(
    world: &'static str,
) -> CertifiedSignedArea2DContracts<CertifiedSignedArea2DQueryWorld> {
    CertifiedSignedArea2DContracts::new(signed_area_handle(world))
}

pub(crate) fn winding_contracts(
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

pub(crate) fn precision_and_frame(
    world: &'static str,
    movement_rotation: &'static str,
) -> (
    PlanarPrecisionCertificateReceipt,
    PlanarLocalFrameCertificateReceipt,
) {
    let predicate = predicate_receipt(movement_rotation);
    let precision = precision_receipt(world, &predicate);
    let frame = frame_receipt(world, movement_rotation, &precision);
    (precision, frame)
}

pub(crate) fn loop_points(
    world: &'static str,
    frame: &PlanarLocalFrameCertificateReceipt,
    prefix: &'static str,
    points: &[[f64; 2]],
) -> Vec<ProjectPointToCertifiedPlane2DReceipt> {
    points
        .iter()
        .enumerate()
        .map(|(index, point)| {
            let identity = format!("{prefix}:point:{index}");
            projected_point(world, frame, identity, [point[0], point[1], 0.0])
        })
        .collect()
}

pub(crate) fn topology_basis(identity: &'static str) -> CertifiedTopologyLoopBasis2D {
    CertifiedTopologyLoopBasis2D::from_topology_loop_fact(
        identity,
        format!("membership:{identity}"),
        "topology-spatial-contract:signed-area",
    )
}

fn projected_point(
    world: &'static str,
    frame: &PlanarLocalFrameCertificateReceipt,
    identity: String,
    local_delta: [f64; 3],
) -> ProjectPointToCertifiedPlane2DReceipt {
    let basis = ProjectPointToCertifiedPlane2DBasis::builder()
        .source_point_identity(identity)
        .source_point([1.0e12, 0.0, 0.0])
        .source_point_basis_digest("point-basis:signed-area-local")
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

fn frame_receipt(
    world: &'static str,
    movement_rotation: &'static str,
    precision: &PlanarPrecisionCertificateReceipt,
) -> PlanarLocalFrameCertificateReceipt {
    let basis = PlanarLocalFrameBasis::builder()
        .frame_identity("frame:signed-area-local-xy")
        .origin([1.0e12, 0.0, 0.0])
        .normal([0.0, 0.0, 1.0])
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .transform_chain_digest("transform:signed-area")
        .movement_rotation_posture_identity(movement_rotation)
        .tolerance_policy_identity("tolerance:signed-area-exact")
        .precision_receipt(precision)
        .build()
        .expect("frame basis");
    let entry = planar_local_frame_certificate_entry(
        PlanarLocalFrameCertificateCase::from_precision_basis(basis),
    );
    planar_local_frame_certificate_facts(&entry, &frame_handle(world)).expect("frame receipt")
}

fn precision_receipt(
    world: &'static str,
    predicate: &PlanarPredicateFactReceipt,
) -> PlanarPrecisionCertificateReceipt {
    let basis = PlanarPrecisionBasis::builder()
        .local_frame_identity("frame:signed-area-local-xy")
        .topology_basis_identity("topology:signed-area")
        .movement_rotation_posture_identity(
            predicate.input_basis().movement_rotation_posture_identity(),
        )
        .tolerance_policy_identity("tolerance:signed-area-exact")
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .predicate_receipt(predicate)
        .build()
        .expect("precision basis");
    let entry = planar_precision_certification_entry(
        PlanarPrecisionCertificationCase::from_predicate_receipt(predicate.clone(), basis),
    );
    planar_precision_certification_facts(&entry, &precision_handle(world))
        .expect("precision receipt")
}

fn predicate_receipt(movement_rotation: &'static str) -> PlanarPredicateFactReceipt {
    let basis = PlanarPredicateInputBasis::from_projected_orient2d_points(
        "frame:signed-area-local-xy",
        "topology:signed-area",
        movement_rotation,
        "tolerance:signed-area-exact",
        [[0.0, 0.0], [1.0e-9, 0.0], [0.0, 1.0e-9]],
    );
    let entry = planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(basis));
    planar_predicate_authority_facts(&entry, &predicate_handle()).expect("predicate receipt")
}
