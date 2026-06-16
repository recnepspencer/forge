use worth_spatial::facade::planar_boolean_events::PlanarBooleanCanonicalSegment;
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
    PlanarPredicateAuthorityCase, PlanarPredicateFactReceipt, PlanarPredicateInputBasis,
};
use worth_spatial::facade::planar_projection::{
    project_point_to_certified_plane_2d_entry, project_point_to_certified_plane_2d_facts,
    ProjectPointToCertifiedPlane2DBasis, ProjectPointToCertifiedPlane2DCase,
    ProjectPointToCertifiedPlane2DReceipt,
};

use super::point_event_contract_handles::predicate_handle;
use super::point_event_contract_handles::{frame_handle, precision_handle, projection_handle};

pub(crate) const MOVEMENT: &str = "movement:point-event";
pub(crate) const TOPOLOGY: &str = "topology:point-event";

pub(crate) fn certified_point_event_frame(
    world: &'static str,
    frame_identity: &str,
    precision_identity: &str,
) -> PlanarLocalFrameCertificateReceipt {
    let predicate = predicate_receipt(
        frame_identity,
        precision_identity,
        [[0.0, 0.0], [1.0e-9, 0.0], [0.0, 1.0e-9]],
    );
    let precision = precision_receipt(world, frame_identity, precision_identity, &predicate);
    let basis = PlanarLocalFrameBasis::builder()
        .frame_identity(frame_identity)
        .origin([1.0e12, 0.0, 0.0])
        .normal([0.0, 0.0, 1.0])
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .transform_chain_digest("transform:point-event")
        .movement_rotation_posture_identity(MOVEMENT)
        .tolerance_policy_identity(precision_identity)
        .precision_receipt(&precision)
        .build()
        .expect("valid point-event frame basis");
    planar_local_frame_certificate_facts(
        &planar_local_frame_certificate_entry(
            PlanarLocalFrameCertificateCase::from_precision_basis(basis),
        ),
        &frame_handle(world),
    )
    .expect("point-event frame receipt")
}

pub(crate) fn project_synthetic_endpoint(
    world: &'static str,
    frame: &PlanarLocalFrameCertificateReceipt,
    segment: &PlanarBooleanCanonicalSegment,
    is_low_endpoint: bool,
    point: [f64; 2],
) -> ProjectPointToCertifiedPlane2DReceipt {
    let endpoint = if is_low_endpoint {
        segment.normalized_endpoints().low()
    } else {
        segment.normalized_endpoints().high()
    };
    let basis = ProjectPointToCertifiedPlane2DBasis::builder()
        .source_point_identity(endpoint.source_endpoint_identity())
        .source_point([1.0e12, 0.0, 0.0])
        .source_point_basis_digest(endpoint.projected_endpoint_fact_identity())
        .local_delta_from_frame_origin([point[0], point[1], 0.0])
        .local_frame_receipt(frame)
        .build()
        .expect("valid point-event projection basis");
    project_point_to_certified_plane_2d_facts(
        &project_point_to_certified_plane_2d_entry(
            ProjectPointToCertifiedPlane2DCase::from_local_frame(basis),
        ),
        &projection_handle(world),
    )
    .expect("point-event projection receipt")
}

pub(crate) fn predicate_receipt(
    frame_identity: &str,
    precision_identity: &str,
    points: [[f64; 2]; 3],
) -> PlanarPredicateFactReceipt {
    let basis = PlanarPredicateInputBasis::from_projected_orient2d_points(
        frame_identity,
        TOPOLOGY,
        MOVEMENT,
        precision_identity,
        points,
    );
    planar_predicate_authority_facts(
        &planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(basis)),
        &predicate_handle(),
    )
    .expect("point-event predicate receipt")
}

fn precision_receipt(
    world: &'static str,
    frame_identity: &str,
    precision_identity: &str,
    predicate: &PlanarPredicateFactReceipt,
) -> PlanarPrecisionCertificateReceipt {
    let basis = PlanarPrecisionBasis::builder()
        .local_frame_identity(frame_identity)
        .topology_basis_identity(TOPOLOGY)
        .movement_rotation_posture_identity(MOVEMENT)
        .tolerance_policy_identity(precision_identity)
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .predicate_receipt(predicate)
        .build()
        .expect("valid point-event precision basis");
    planar_precision_certification_facts(
        &planar_precision_certification_entry(
            PlanarPrecisionCertificationCase::from_predicate_receipt(predicate.clone(), basis),
        ),
        &precision_handle(world),
    )
    .expect("point-event precision receipt")
}
