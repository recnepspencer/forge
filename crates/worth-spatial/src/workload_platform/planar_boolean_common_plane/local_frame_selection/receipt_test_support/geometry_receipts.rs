use crate::facade::planar_local_frame::{
    planar_local_frame_certificate_entry, planar_local_frame_certificate_facts,
    PlanarLocalFrameBasis, PlanarLocalFrameCertificateCase, PlanarLocalFrameCertificateReceipt,
};
use crate::facade::planar_overlap::{
    CertifiedCoplanarOverlapFace2D, CoplanarOverlapContractContracts,
    CoplanarOverlapContractExtractor,
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
};
use crate::facade::planar_signed_area::{
    AreaDegeneracyPolicy, CertifiedSignedArea2D, CertifiedSignedArea2DContracts,
    CertifiedSignedArea2DReceipt,
};
use crate::facade::planar_topology_contract::{
    PlanarTopologyContractCompleteness, PlanarTopologyContractCompletenessContracts,
    PlanarTopologyContractCompletenessReceipt,
};
use crate::facade::planar_winding::{
    CertifiedPolygonWinding2D, CertifiedPolygonWinding2DContracts, CertifiedProjectedLoop2D,
    CertifiedTopologyLoopBasis2D,
};

use super::handles::{
    frame_handle, overlap_handle, precision_handle, predicate_consumption_handle, predicate_handle,
    projection_handle, segment_handle, signed_area_handle, topology_contract_handle,
    winding_handle,
};
use super::{MOVEMENT, NEIGHBORHOOD, TOPOLOGY};
use topology::facade::{
    prepare_primitive_construction_query_receipt, TopologyPrimitiveConstructionBirthFamily,
    TopologyPrimitiveConstructionQueryBirthSynopsis,
};
use worth_primitives::{PrimitiveConstructionFamilyContractRegistry, PrimitiveWitnessDescriptor};

pub(crate) fn topology_contract_receipt() -> PlanarTopologyContractCompletenessReceipt {
    let synopsis = TopologyPrimitiveConstructionQueryBirthSynopsis::new(
        TopologyPrimitiveConstructionBirthFamily::ShellWithHole,
        PrimitiveConstructionFamilyContractRegistry::contract_for(
            &PrimitiveWitnessDescriptor::ShellWithHole {
                outer_loop_edge_count: 4,
                hole_loop_edge_counts: vec![4],
            },
        ),
        "topology-scaffold:local-frame-test".to_string(),
        TOPOLOGY.to_string(),
        "planar_shell_with_hole_body".to_string(),
        8,
        8,
        2,
        0,
        1,
        1,
        1,
    );
    PlanarTopologyContractCompleteness::from_topology_query_receipt(
        prepare_primitive_construction_query_receipt(&synopsis).expect("topology query receipt"),
    )
    .consume_declared_topology_surfaces("topology.query.declared-surfaces:local-frame-test")
    .within_planar_neighborhood(NEIGHBORHOOD)
    .compile(&PlanarTopologyContractCompletenessContracts::new(
        topology_contract_handle(),
    ))
    .expect("topology completeness plan")
    .certify()
    .expect("topology completeness receipt")
}

pub(crate) fn predicate_receipt() -> PlanarPredicateFactReceipt {
    planar_predicate_authority_facts(
        &planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(
            PlanarPredicateInputBasis::from_projected_orient2d_points(
                "frame:local-frame-test",
                TOPOLOGY,
                MOVEMENT,
                "tolerance:local-frame-test",
                [[0.0, 0.0], [1.0e-9, 0.0], [0.0, 1.0e-9]],
            ),
        )),
        &predicate_handle(),
    )
    .expect("predicate receipt")
}

pub(crate) fn precision_receipt(
    predicate: &PlanarPredicateFactReceipt,
) -> PlanarPrecisionCertificateReceipt {
    let basis = PlanarPrecisionBasis::builder()
        .local_frame_identity("frame:local-frame-test")
        .topology_basis_identity(TOPOLOGY)
        .movement_rotation_posture_identity(MOVEMENT)
        .tolerance_policy_identity("tolerance:local-frame-test")
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

pub(crate) fn frame_receipt(
    precision: &PlanarPrecisionCertificateReceipt,
) -> PlanarLocalFrameCertificateReceipt {
    let basis = PlanarLocalFrameBasis::builder()
        .frame_identity("frame:local-frame-test")
        .origin([1.0e12, 0.0, 0.0])
        .normal([0.0, 0.0, 1.0])
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .transform_chain_digest("transform:local-frame-test")
        .movement_rotation_posture_identity(MOVEMENT)
        .tolerance_policy_identity("tolerance:local-frame-test")
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

pub(crate) fn projected_points(
    frame: &PlanarLocalFrameCertificateReceipt,
    label: &'static str,
    points: &[[f64; 2]; 4],
) -> Vec<ProjectPointToCertifiedPlane2DReceipt> {
    points
        .iter()
        .enumerate()
        .map(|(index, point)| {
            let basis = ProjectPointToCertifiedPlane2DBasis::builder()
                .source_point_identity(format!("{label}:point:{index}"))
                .source_point([1.0e12, 0.0, 0.0])
                .source_point_basis_digest(format!("basis:{label}:{index}"))
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
        })
        .collect()
}

pub(crate) fn winding_receipt(
    label: &'static str,
    projections: Vec<ProjectPointToCertifiedPlane2DReceipt>,
) -> crate::facade::planar_winding::CertifiedPolygonWinding2DReceipt {
    let loop_identity = format!("loop:{label}");
    let projected_loop = CertifiedProjectedLoop2D::from_projected_vertices(
        loop_identity.clone(),
        CertifiedTopologyLoopBasis2D::from_topology_loop_fact(
            &loop_identity,
            format!("membership:{loop_identity}"),
            "topology-spatial-contract:local-frame-test",
        ),
        projections,
    )
    .expect("loop");
    CertifiedPolygonWinding2D::certify(projected_loop)
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&CertifiedPolygonWinding2DContracts::new(
            winding_handle(),
            CertifiedSegmentSegment2DContracts::new(segment_handle(), predicate_handle()),
            predicate_handle(),
        ))
        .expect("winding plan")
        .certify()
        .expect("winding receipt")
}

pub(crate) fn signed_area_receipt(
    winding: crate::facade::planar_winding::CertifiedPolygonWinding2DReceipt,
    precision: PlanarPrecisionCertificateReceipt,
) -> CertifiedSignedArea2DReceipt {
    CertifiedSignedArea2D::measure_face(winding)
        .using_precision_basis(precision)
        .classifying_degeneracy(AreaDegeneracyPolicy::ClassifyWithoutRepair)
        .compile(&CertifiedSignedArea2DContracts::new(signed_area_handle()))
        .expect("area plan")
        .certify()
        .expect("area receipt")
}

pub(crate) fn overlap_receipt(
    left: CertifiedSignedArea2DReceipt,
    right: CertifiedSignedArea2DReceipt,
) -> crate::facade::planar_overlap::CoplanarOverlapContractReceipt {
    CoplanarOverlapContractExtractor::between(
        CertifiedCoplanarOverlapFace2D::from_certified_area("left", left).expect("left"),
        CertifiedCoplanarOverlapFace2D::from_certified_area("right", right).expect("right"),
    )
    .within_planar_neighborhood(NEIGHBORHOOD)
    .compile(&CoplanarOverlapContractContracts::new(
        overlap_handle(),
        CertifiedSegmentSegment2DContracts::new(segment_handle(), predicate_handle()),
    ))
    .expect("overlap plan")
    .extract()
    .expect("overlap receipt")
}

pub(crate) fn segment_receipt(
    left_start: ProjectPointToCertifiedPlane2DReceipt,
    left_end: ProjectPointToCertifiedPlane2DReceipt,
    right_start: ProjectPointToCertifiedPlane2DReceipt,
    right_end: ProjectPointToCertifiedPlane2DReceipt,
) -> crate::facade::planar_segment_segment::CertifiedSegmentSegment2DReceipt {
    let left =
        CertifiedProjectedSegment2D::from_projected_endpoints("left-edge", left_start, left_end)
            .expect("left segment");
    let right =
        CertifiedProjectedSegment2D::from_projected_endpoints("right-edge", right_start, right_end)
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

pub(crate) fn segment_orientation_predicates(
    segment: &crate::facade::planar_segment_segment::CertifiedSegmentSegment2DReceipt,
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
    .map(|points| {
        planar_predicate_authority_facts(
            &planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(
                PlanarPredicateInputBasis::from_projected_orient2d_points(
                    basis.frame_identity(),
                    basis.topology_basis_identity(),
                    basis.movement_rotation_posture_identity(),
                    basis.tolerance_policy_identity(),
                    points,
                ),
            )),
            &predicate_handle(),
        )
        .expect("segment predicate receipt")
    })
    .fold(
        Vec::<PlanarPredicateFactReceipt>::new(),
        |mut unique, receipt| {
            if unique
                .iter()
                .all(|existing| existing.fact_digest() != receipt.fact_digest())
            {
                unique.push(receipt);
            }
            unique
        },
    )
}

pub(crate) fn predicate_consumption_receipt(
    segment: crate::facade::planar_segment_segment::CertifiedSegmentSegment2DReceipt,
    predicates: Vec<PlanarPredicateFactReceipt>,
) -> PredicateCertificateConsumptionReceipt {
    PredicateCertificateConsumption::for_planar_workload()
        .expecting_topology_basis(TOPOLOGY)
        .expecting_movement_rotation_posture(MOVEMENT)
        .expecting_local_frame("frame:local-frame-test")
        .with_predicate_authority(predicates)
        .with_segment_contacts(vec![segment])
        .compile(&PredicateCertificateConsumptionContracts::new(
            predicate_consumption_handle(),
        ))
        .expect("predicate consumption plan")
        .certify()
        .expect("predicate consumption receipt")
}
