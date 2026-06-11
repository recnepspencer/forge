use super::query_handles::*;
use super::{canonical_retained_replay_error, MOVEMENT, NEIGHBORHOOD, TOPOLOGY};
use crate::workload_platform::retained_replay_workload::UnsupportedReplayWorkload;

use crate::facade::planar_contract_bundle::PlanarContractBundleValidationReceipt;
use crate::facade::planar_local_frame::{
    planar_local_frame_certificate_entry, planar_local_frame_certificate_facts,
    PlanarLocalFrameBasis, PlanarLocalFrameCertificateCase, PlanarLocalFrameCertificateReceipt,
};
use crate::facade::planar_motion_posture::{
    PlanarMotionCancellation, PlanarMotionPosture, PlanarMotionPostureContracts,
    PlanarMotionPostureReceipt, PlanarReorientation,
};
use crate::facade::planar_overlap::{
    CertifiedCoplanarOverlapFace2D, CoplanarOverlapContractContracts,
    CoplanarOverlapContractExtractor, CoplanarOverlapContractReceipt,
};
use crate::facade::planar_precision::{
    planar_precision_certification_entry, planar_precision_certification_facts,
    PlanarPrecisionBasis, PlanarPrecisionCertificateReceipt, PlanarPrecisionCertificationCase,
};
use crate::facade::planar_predicate_consumption::{
    PredicateCertificateConsumption, PredicateCertificateConsumptionContracts,
    PredicateCertificateConsumptionReceipt,
};
use crate::facade::planar_predicates::{
    planar_predicate_authority_entry, planar_predicate_authority_facts,
    PlanarPredicateAuthorityCase, PlanarPredicateFactReceipt, PlanarPredicateInputBasis,
};
use crate::facade::planar_projection::{
    project_point_to_certified_plane_2d_entry, project_point_to_certified_plane_2d_facts,
    ProjectPointToCertifiedPlane2DBasis, ProjectPointToCertifiedPlane2DCase,
    ProjectPointToCertifiedPlane2DReceipt,
};
use crate::facade::planar_segment_segment::{
    CertifiedProjectedSegment2D, CertifiedSegmentSegment2D, CertifiedSegmentSegment2DContracts,
    CertifiedSegmentSegment2DReceipt,
};
use crate::facade::planar_signed_area::{
    AreaDegeneracyPolicy, CertifiedSignedArea2D, CertifiedSignedArea2DContracts,
    CertifiedSignedArea2DReceipt,
};
use crate::facade::planar_structural_identity::{
    PlanarStructuralIdentity, PlanarStructuralIdentityContracts, PlanarStructuralIdentityReceipt,
};
use crate::facade::planar_winding::{
    CertifiedPolygonWinding2D, CertifiedPolygonWinding2DContracts,
    CertifiedPolygonWinding2DReceipt, CertifiedProjectedLoop2D, CertifiedTopologyLoopBasis2D,
};
pub(super) fn precision_receipt(
    world: &'static str,
    predicate: &PlanarPredicateFactReceipt,
) -> Result<PlanarPrecisionCertificateReceipt, UnsupportedReplayWorkload> {
    let basis = PlanarPrecisionBasis::builder()
        .local_frame_identity("frame:canonical-retained")
        .topology_basis_identity(TOPOLOGY)
        .movement_rotation_posture_identity(MOVEMENT)
        .tolerance_policy_identity("tolerance:canonical-retained")
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .predicate_receipt(predicate)
        .build()
        .map_err(|_| canonical_retained_replay_error("Could not build precision basis."))?;
    planar_precision_certification_facts(
        &planar_precision_certification_entry(
            PlanarPrecisionCertificationCase::from_predicate_receipt(predicate.clone(), basis),
        ),
        &precision_handle(world),
    )
    .map_err(|_| canonical_retained_replay_error("Could not certify precision."))
}

pub(super) fn frame_receipt(
    world: &'static str,
    precision: &PlanarPrecisionCertificateReceipt,
) -> Result<PlanarLocalFrameCertificateReceipt, UnsupportedReplayWorkload> {
    let basis = PlanarLocalFrameBasis::builder()
        .frame_identity("frame:canonical-retained")
        .origin([1.0e12, 0.0, 0.0])
        .normal([0.0, 0.0, 1.0])
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .transform_chain_digest("transform:canonical-retained")
        .movement_rotation_posture_identity(MOVEMENT)
        .tolerance_policy_identity("tolerance:canonical-retained")
        .precision_receipt(precision)
        .build()
        .map_err(|_| canonical_retained_replay_error("Could not build local frame basis."))?;
    planar_local_frame_certificate_facts(
        &planar_local_frame_certificate_entry(
            PlanarLocalFrameCertificateCase::from_precision_basis(basis),
        ),
        &frame_handle(world),
    )
    .map_err(|_| canonical_retained_replay_error("Could not certify local frame."))
}

pub(super) fn projected_face_pair(
    world: &'static str,
    frame: &PlanarLocalFrameCertificateReceipt,
) -> Result<Vec<ProjectPointToCertifiedPlane2DReceipt>, UnsupportedReplayWorkload> {
    let left = [[0.0, 0.0], [2.0e-9, 0.0], [2.0e-9, 2.0e-9], [0.0, 2.0e-9]];
    let right = [
        [2.0e-9, 0.0],
        [4.0e-9, 0.0],
        [4.0e-9, 2.0e-9],
        [2.0e-9, 2.0e-9],
    ];
    let mut projections = projected_points(world, frame, "face:left", &left)?;
    projections.extend(projected_points(world, frame, "face:right", &right)?);
    Ok(projections)
}

pub(super) fn projected_points(
    world: &'static str,
    frame: &PlanarLocalFrameCertificateReceipt,
    face: &'static str,
    points: &[[f64; 2]; 4],
) -> Result<Vec<ProjectPointToCertifiedPlane2DReceipt>, UnsupportedReplayWorkload> {
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
                .map_err(|_| {
                    canonical_retained_replay_error("Could not build projection basis.")
                })?;
            project_point_to_certified_plane_2d_facts(
                &project_point_to_certified_plane_2d_entry(
                    ProjectPointToCertifiedPlane2DCase::from_local_frame(basis),
                ),
                &projection_handle(world),
            )
            .map_err(|_| canonical_retained_replay_error("Could not project canonical point."))
        })
        .collect()
}

pub(super) fn winding_receipt(
    world: &'static str,
    face: &'static str,
    projections: Vec<ProjectPointToCertifiedPlane2DReceipt>,
) -> Result<CertifiedPolygonWinding2DReceipt, UnsupportedReplayWorkload> {
    let loop_identity = format!("loop:{face}");
    let projected_loop = CertifiedProjectedLoop2D::from_projected_vertices(
        loop_identity.clone(),
        topology_basis(&loop_identity),
        projections,
    )
    .map_err(|_| canonical_retained_replay_error("Could not build projected loop."))?;
    CertifiedPolygonWinding2D::certify(projected_loop)
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&CertifiedPolygonWinding2DContracts::new(
            winding_handle(world),
            CertifiedSegmentSegment2DContracts::new(segment_handle(world), predicate_handle()),
            predicate_handle(),
        ))
        .map_err(|_| canonical_retained_replay_error("Could not compile winding receipt."))?
        .certify()
        .map_err(|_| canonical_retained_replay_error("Could not certify winding."))
}

pub(super) fn signed_area_receipt(
    world: &'static str,
    winding: CertifiedPolygonWinding2DReceipt,
    precision: PlanarPrecisionCertificateReceipt,
) -> Result<CertifiedSignedArea2DReceipt, UnsupportedReplayWorkload> {
    CertifiedSignedArea2D::measure_face(winding)
        .using_precision_basis(precision)
        .classifying_degeneracy(AreaDegeneracyPolicy::ClassifyWithoutRepair)
        .compile(&CertifiedSignedArea2DContracts::new(signed_area_handle(
            world,
        )))
        .map_err(|_| canonical_retained_replay_error("Could not compile signed area."))?
        .certify()
        .map_err(|_| canonical_retained_replay_error("Could not certify signed area."))
}

pub(super) fn overlap_receipt(
    world: &'static str,
    left: CertifiedSignedArea2DReceipt,
    right: CertifiedSignedArea2DReceipt,
) -> Result<CoplanarOverlapContractReceipt, UnsupportedReplayWorkload> {
    let left = CertifiedCoplanarOverlapFace2D::from_certified_area("face:left", left)
        .map_err(|_| canonical_retained_replay_error("Could not build left overlap face."))?;
    let right = CertifiedCoplanarOverlapFace2D::from_certified_area("face:right", right)
        .map_err(|_| canonical_retained_replay_error("Could not build right overlap face."))?;
    CoplanarOverlapContractExtractor::between(left, right)
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&CoplanarOverlapContractContracts::new(
            overlap_handle(world),
            CertifiedSegmentSegment2DContracts::new(segment_handle(world), predicate_handle()),
        ))
        .map_err(|_| canonical_retained_replay_error("Could not compile overlap contract."))?
        .extract()
        .map_err(|_| canonical_retained_replay_error("Could not extract overlap contract."))
}

pub(super) fn segment_receipt(
    world: &'static str,
    left_start: ProjectPointToCertifiedPlane2DReceipt,
    left_end: ProjectPointToCertifiedPlane2DReceipt,
    right_start: ProjectPointToCertifiedPlane2DReceipt,
    right_end: ProjectPointToCertifiedPlane2DReceipt,
) -> Result<CertifiedSegmentSegment2DReceipt, UnsupportedReplayWorkload> {
    let left = CertifiedProjectedSegment2D::from_projected_endpoints(
        "segment:left-shared",
        left_start,
        left_end,
    )
    .map_err(|_| canonical_retained_replay_error("Could not build left segment."))?;
    let right = CertifiedProjectedSegment2D::from_projected_endpoints(
        "segment:right-shared",
        right_start,
        right_end,
    )
    .map_err(|_| canonical_retained_replay_error("Could not build right segment."))?;
    CertifiedSegmentSegment2D::classify(left, right)
        .within_topology_basis(TOPOLOGY)
        .compile(&CertifiedSegmentSegment2DContracts::new(
            segment_handle(world),
            predicate_handle(),
        ))
        .map_err(|_| canonical_retained_replay_error("Could not compile segment contact."))?
        .certify()
        .map_err(|_| canonical_retained_replay_error("Could not certify segment contact."))
}

pub(super) fn predicate_receipt() -> Result<PlanarPredicateFactReceipt, UnsupportedReplayWorkload> {
    let basis = PlanarPredicateInputBasis::from_projected_orient2d_points(
        "frame:canonical-retained",
        TOPOLOGY,
        MOVEMENT,
        "tolerance:canonical-retained",
        [[0.0, 0.0], [1.0e-9, 0.0], [0.0, 1.0e-9]],
    );
    planar_predicate_authority_facts(
        &planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(basis)),
        &predicate_handle(),
    )
    .map_err(|_| canonical_retained_replay_error("Could not certify predicate authority."))
}

pub(super) fn segment_orientation_predicates(
    segment: &CertifiedSegmentSegment2DReceipt,
) -> Result<Vec<PlanarPredicateFactReceipt>, UnsupportedReplayWorkload> {
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
    .try_fold(Vec::new(), |mut unique, points| {
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
        .map_err(|_| {
            canonical_retained_replay_error("Could not certify segment predicate authority.")
        })?;
        if unique.iter().all(|existing: &PlanarPredicateFactReceipt| {
            existing.fact_digest() != receipt.fact_digest()
        }) {
            unique.push(receipt);
        }
        Ok(unique)
    })
}

pub(super) fn predicate_consumption_receipt(
    world: &'static str,
    segment: CertifiedSegmentSegment2DReceipt,
    predicates: Vec<PlanarPredicateFactReceipt>,
) -> Result<PredicateCertificateConsumptionReceipt, UnsupportedReplayWorkload> {
    PredicateCertificateConsumption::for_planar_workload()
        .expecting_topology_basis(TOPOLOGY)
        .expecting_movement_rotation_posture(MOVEMENT)
        .expecting_local_frame("frame:canonical-retained")
        .with_predicate_authority(predicates)
        .with_segment_contacts(vec![segment])
        .compile(&PredicateCertificateConsumptionContracts::new(
            predicate_consumption_handle(world),
        ))
        .map_err(|_| canonical_retained_replay_error("Could not compile predicate consumption."))?
        .certify()
        .map_err(|_| canonical_retained_replay_error("Could not certify predicate consumption."))
}

pub(super) fn motion_receipt(
    world: &'static str,
    readiness: PlanarContractBundleValidationReceipt,
) -> Result<PlanarMotionPostureReceipt, UnsupportedReplayWorkload> {
    PlanarMotionPosture::from_boolean_readiness(readiness)
        .after_exact_translation("motion:canonical-retained-translate")
        .after_exact_rotation("motion:canonical-retained-quarter-turn")
        .after_exact_rotation("motion:canonical-retained-quarter-turn-inverse")
        .after_reorientation(PlanarReorientation::PreservesHandedness)
        .with_cancellation_policy(PlanarMotionCancellation::ExactBasisReplay)
        .compile(&PlanarMotionPostureContracts::new(motion_posture_handle(
            world,
        )))
        .map_err(|_| canonical_retained_replay_error("Could not compile motion posture."))?
        .certify()
        .map_err(|_| canonical_retained_replay_error("Could not certify motion posture."))
}

pub(super) fn structural_receipt(
    world: &'static str,
    readiness: PlanarContractBundleValidationReceipt,
    motion: PlanarMotionPostureReceipt,
) -> Result<PlanarStructuralIdentityReceipt, UnsupportedReplayWorkload> {
    PlanarStructuralIdentity::from_boolean_readiness(readiness)
        .with_motion_posture(motion)
        .with_topology_identity(TOPOLOGY)
        .with_persistent_name("name:canonical-retained")
        .with_binding_identity("binding:canonical-retained")
        .with_lineage_identity("lineage:canonical-retained")
        .compile(&PlanarStructuralIdentityContracts::new(
            structural_identity_handle(world),
        ))
        .map_err(|_| canonical_retained_replay_error("Could not compile structural identity."))?
        .certify()
        .map_err(|_| canonical_retained_replay_error("Could not certify structural identity."))
}

pub(super) fn topology_basis(identity: &str) -> CertifiedTopologyLoopBasis2D {
    CertifiedTopologyLoopBasis2D::from_topology_loop_fact(
        identity,
        format!("membership:{identity}"),
        "topology-spatial-contract:canonical-retained",
    )
}
