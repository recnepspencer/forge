use std::sync::OnceLock;
use worth_spatial::facade::planar_contract_bundle::{
    PlanarBooleanReadinessBundle, PlanarContractBundleValidationContracts,
    PlanarContractBundleValidationReceipt, PlanarContractBundleValidator,
};
use worth_spatial::facade::planar_contracts::{
    admit_planar_contract_family, PlanarAdmissionFamily, PlanarRuntimeConcern,
};
use worth_spatial::facade::planar_local_frame::{
    planar_local_frame_certificate_entry, planar_local_frame_certificate_facts,
    PlanarLocalFrameBasis, PlanarLocalFrameCertificateCase, PlanarLocalFrameCertificateReceipt,
};
use worth_spatial::facade::planar_overlap::{
    CertifiedCoplanarOverlapFace2D, CoplanarOverlapContractContracts,
    CoplanarOverlapContractExtractor,
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
};
use worth_spatial::facade::planar_signed_area::{
    AreaDegeneracyPolicy, CertifiedSignedArea2D, CertifiedSignedArea2DContracts,
    CertifiedSignedArea2DReceipt,
};
use worth_spatial::facade::planar_winding::{
    CertifiedPolygonWinding2D, CertifiedPolygonWinding2DContracts, CertifiedProjectedLoop2D,
    CertifiedTopologyLoopBasis2D,
};

use super::query_handles::{
    bundle_handle, frame_handle, overlap_handle, precision_handle, predicate_consumption_handle,
    predicate_handle, projection_handle, segment_handle, signed_area_handle, winding_handle,
    MOVEMENT,
};
use super::topology_fixture::kernel_topology_contract_receipt;

const TOPOLOGY: &str = "topology:kernel-bundle";
const NEIGHBORHOOD: &str = "neighborhood:kernel-bundle";

pub(crate) fn readiness_receipt() -> PlanarContractBundleValidationReceipt {
    static READINESS_RECEIPT: OnceLock<PlanarContractBundleValidationReceipt> = OnceLock::new();
    READINESS_RECEIPT
        .get_or_init(build_readiness_receipt)
        .clone()
}

fn build_readiness_receipt() -> PlanarContractBundleValidationReceipt {
    let predicate = predicate_receipt();
    let precision = precision_receipt(&predicate);
    let frame = frame_receipt(&precision);
    let segment_contracts =
        CertifiedSegmentSegment2DContracts::new(segment_handle(), predicate_handle());
    let left = projected_points(
        &frame,
        "left",
        &[[0.0, 0.0], [2.0e-9, 0.0], [2.0e-9, 2.0e-9], [0.0, 2.0e-9]],
    );
    let right = projected_points(
        &frame,
        "right",
        &[
            [2.0e-9, 0.0],
            [4.0e-9, 0.0],
            [4.0e-9, 2.0e-9],
            [2.0e-9, 2.0e-9],
        ],
    );
    let left_winding = winding_receipt("left", left.clone(), segment_contracts.clone());
    let right_winding = winding_receipt("right", right.clone(), segment_contracts.clone());
    let left_area = signed_area_receipt(left_winding.clone(), precision.clone());
    let right_area = signed_area_receipt(right_winding, precision.clone());
    let overlap = overlap_receipt(left_area.clone(), right_area, segment_contracts.clone());
    let segment = segment_receipt(
        left[1].clone(),
        left[2].clone(),
        right[3].clone(),
        right[0].clone(),
        segment_contracts,
    );
    let segment_predicates = segment_orientation_predicates(&segment);
    let predicate_consumption =
        predicate_consumption_receipt(segment.clone(), segment_predicates.clone());
    let mut predicates = vec![predicate];
    predicates.extend(segment_predicates);
    let mut projections = left;
    projections.extend(right);
    let bundle = PlanarBooleanReadinessBundle::builder()
        .admission(
            admit_planar_contract_family(
                PlanarAdmissionFamily::PlanarContractBundle,
                PlanarRuntimeConcern::BooleanReadinessBundle,
            )
            .expect("bundle admission"),
        )
        .topology_contract(kernel_topology_contract_receipt())
        .precision(precision)
        .local_frame(frame)
        .projection_consumption(projections)
        .predicate_authority(predicates)
        .segment_contacts(vec![segment])
        .winding(left_winding)
        .signed_area(left_area)
        .coplanar_overlap(overlap)
        .predicate_consumption(predicate_consumption)
        .topology_basis(TOPOLOGY)
        .movement_rotation_posture(MOVEMENT)
        .diagnostic_scope("diagnostics:kernel-bundle")
        .build()
        .expect("kernel bundle");
    PlanarContractBundleValidator::for_boolean_readiness(bundle)
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&PlanarContractBundleValidationContracts::new(
            bundle_handle(),
        ))
        .expect("bundle plan")
        .certify()
        .expect("readiness receipt")
}

fn precision_receipt(predicate: &PlanarPredicateFactReceipt) -> PlanarPrecisionCertificateReceipt {
    let basis = PlanarPrecisionBasis::builder()
        .local_frame_identity("frame:kernel-bundle")
        .topology_basis_identity(TOPOLOGY)
        .movement_rotation_posture_identity(MOVEMENT)
        .tolerance_policy_identity("tolerance:kernel-bundle")
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
        .frame_identity("frame:kernel-bundle")
        .origin([1.0e12, 0.0, 0.0])
        .normal([0.0, 0.0, 1.0])
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .transform_chain_digest("transform:kernel-bundle")
        .movement_rotation_posture_identity(MOVEMENT)
        .tolerance_policy_identity("tolerance:kernel-bundle")
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

fn projected_points(
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

fn winding_receipt(
    label: &'static str,
    projections: Vec<ProjectPointToCertifiedPlane2DReceipt>,
    segment_contracts: CertifiedSegmentSegment2DContracts<
        worth_spatial::facade::planar_segment_segment::CertifiedSegmentSegment2DQueryWorld,
        worth_spatial::facade::planar_predicates::PlanarPredicateAuthorityQueryWorld,
    >,
) -> worth_spatial::facade::planar_winding::CertifiedPolygonWinding2DReceipt {
    let loop_identity = format!("loop:{label}");
    let projected_loop = CertifiedProjectedLoop2D::from_projected_vertices(
        loop_identity.clone(),
        CertifiedTopologyLoopBasis2D::from_topology_loop_fact(
            &loop_identity,
            format!("membership:{loop_identity}"),
            "topology-spatial-contract:kernel-bundle",
        ),
        projections,
    )
    .expect("loop");
    CertifiedPolygonWinding2D::certify(projected_loop)
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&CertifiedPolygonWinding2DContracts::new(
            winding_handle(),
            segment_contracts,
            predicate_handle(),
        ))
        .expect("winding plan")
        .certify()
        .expect("winding receipt")
}

fn signed_area_receipt(
    winding: worth_spatial::facade::planar_winding::CertifiedPolygonWinding2DReceipt,
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

fn overlap_receipt(
    left: CertifiedSignedArea2DReceipt,
    right: CertifiedSignedArea2DReceipt,
    segment_contracts: CertifiedSegmentSegment2DContracts<
        worth_spatial::facade::planar_segment_segment::CertifiedSegmentSegment2DQueryWorld,
        worth_spatial::facade::planar_predicates::PlanarPredicateAuthorityQueryWorld,
    >,
) -> worth_spatial::facade::planar_overlap::CoplanarOverlapContractReceipt {
    CoplanarOverlapContractExtractor::between(
        CertifiedCoplanarOverlapFace2D::from_certified_area("left", left).expect("left"),
        CertifiedCoplanarOverlapFace2D::from_certified_area("right", right).expect("right"),
    )
    .within_planar_neighborhood(NEIGHBORHOOD)
    .compile(&CoplanarOverlapContractContracts::new(
        overlap_handle(),
        segment_contracts,
    ))
    .expect("overlap plan")
    .extract()
    .expect("overlap receipt")
}

fn segment_receipt(
    left_start: ProjectPointToCertifiedPlane2DReceipt,
    left_end: ProjectPointToCertifiedPlane2DReceipt,
    right_start: ProjectPointToCertifiedPlane2DReceipt,
    right_end: ProjectPointToCertifiedPlane2DReceipt,
    segment_contracts: CertifiedSegmentSegment2DContracts<
        worth_spatial::facade::planar_segment_segment::CertifiedSegmentSegment2DQueryWorld,
        worth_spatial::facade::planar_predicates::PlanarPredicateAuthorityQueryWorld,
    >,
) -> worth_spatial::facade::planar_segment_segment::CertifiedSegmentSegment2DReceipt {
    let left =
        CertifiedProjectedSegment2D::from_projected_endpoints("left-edge", left_start, left_end)
            .expect("left segment");
    let right =
        CertifiedProjectedSegment2D::from_projected_endpoints("right-edge", right_start, right_end)
            .expect("right segment");
    CertifiedSegmentSegment2D::classify(left, right)
        .within_topology_basis(TOPOLOGY)
        .compile(&segment_contracts)
        .expect("segment plan")
        .certify()
        .expect("segment receipt")
}

fn segment_orientation_predicates(
    segment: &worth_spatial::facade::planar_segment_segment::CertifiedSegmentSegment2DReceipt,
) -> Vec<PlanarPredicateFactReceipt> {
    segment
        .orientation_predicate_receipts()
        .iter()
        .cloned()
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

fn predicate_consumption_receipt(
    segment: worth_spatial::facade::planar_segment_segment::CertifiedSegmentSegment2DReceipt,
    predicates: Vec<PlanarPredicateFactReceipt>,
) -> PredicateCertificateConsumptionReceipt {
    PredicateCertificateConsumption::for_planar_workload()
        .expecting_topology_basis(TOPOLOGY)
        .expecting_movement_rotation_posture(MOVEMENT)
        .expecting_local_frame("frame:kernel-bundle")
        .with_predicate_authority(predicates)
        .with_segment_contacts(vec![segment])
        .compile(&PredicateCertificateConsumptionContracts::new(
            predicate_consumption_handle(),
        ))
        .expect("predicate consumption plan")
        .certify()
        .expect("predicate consumption receipt")
}

fn predicate_receipt() -> PlanarPredicateFactReceipt {
    let basis = PlanarPredicateInputBasis::from_projected_orient2d_points(
        "frame:kernel-bundle",
        TOPOLOGY,
        MOVEMENT,
        "tolerance:kernel-bundle",
        [[0.0, 0.0], [1.0e-9, 0.0], [0.0, 1.0e-9]],
    );
    planar_predicate_authority_facts(
        &planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(basis)),
        &predicate_handle(),
    )
    .expect("predicate receipt")
}
