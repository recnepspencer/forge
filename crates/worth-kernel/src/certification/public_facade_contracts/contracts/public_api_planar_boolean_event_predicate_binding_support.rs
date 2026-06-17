use std::collections::BTreeMap;

use forge_query::facade::ForgeQueryApplicationFacade;
use worth_kernel::workload_composition::PlanarBooleanCommonPlaneReducedOperandPairRequest;
use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanNormalizedEndpoint, PlanarBooleanSegmentPairEnumerationReceipt,
    PlanarBooleanSegmentPairWorkItem,
};
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
    PredicateCertificateConsumptionReceipt,
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
    CertifiedSegmentSegment2DReceipt, SegmentContactPolicy,
};

use super::reduced_pair_support;

const MOVEMENT: &str = "movement:event-predicate-binding";
const TOPOLOGY: &str = "topology:event-predicate-binding";

#[derive(Clone)]
#[allow(dead_code)]
pub(crate) struct BindingSubject {
    pub(crate) pair: worth_kernel::workload_composition::BuiltBooleanOperandPairRecipe,
    pub(crate) reduced_pair: PlanarBooleanCommonPlaneReducedOperandPairRequest,
    pub(crate) reduced_pair_identity: String,
    pub(crate) pair_worklist: PlanarBooleanSegmentPairEnumerationReceipt,
    pub(crate) segment_receipts: Vec<CertifiedSegmentSegment2DReceipt>,
    pub(crate) predicate_consumption: PredicateCertificateConsumptionReceipt,
}

pub(crate) fn binding_subject(readiness_scope: &'static str) -> BindingSubject {
    let (pair, operand_a, operand_b) =
        reduced_pair_support::event_carrier_projected_operand_requests_from_catalog(
            readiness_scope,
        );
    binding_subject_from_projected_operands(readiness_scope, pair, operand_a, operand_b)
}

#[allow(dead_code)]
pub(crate) fn metaboss_binding_subject(readiness_scope: &'static str) -> BindingSubject {
    let (pair, operand_a, operand_b) =
        reduced_pair_support::metaboss_projected_operand_requests_from_catalog(readiness_scope);
    binding_subject_from_projected_operands(readiness_scope, pair, operand_a, operand_b)
}

fn binding_subject_from_projected_operands(
    readiness_scope: &'static str,
    pair: worth_kernel::workload_composition::BuiltBooleanOperandPairRecipe,
    operand_a: worth_kernel::workload_composition::PlanarBooleanCommonPlaneOperandAProjectedRequest,
    operand_b: worth_kernel::workload_composition::PlanarBooleanCommonPlaneOperandBProjectedRequest,
) -> BindingSubject {
    let reduced_pair =
        PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
            operand_a, operand_b,
        )
        .expect("reduced pair should certify");
    let pair_worklist = reduced_pair
        .segment_carrier_set()
        .expect("carrier set should certify")
        .canonical_segment_set()
        .expect("canonical segments should certify")
        .segment_pair_enumeration_receipt()
        .expect("pair worklist should certify");
    let frame = certified_frame(
        readiness_scope,
        pair_worklist.work_items()[0].left().local_frame_identity(),
        pair_worklist.work_items()[0]
            .left()
            .precision_basis_identity(),
    );
    let segment_receipts = pair_worklist
        .work_items()
        .iter()
        .map(|work_item| segment_receipt(readiness_scope, &frame, work_item))
        .collect::<Vec<_>>();
    let predicates = unique_orientation_predicates(&segment_receipts);
    let predicate_consumption =
        predicate_consumption_receipt(readiness_scope, segment_receipts.clone(), predicates);
    BindingSubject {
        pair,
        reduced_pair: reduced_pair.clone(),
        reduced_pair_identity: reduced_pair.reduced_operand_pair_identity().to_string(),
        pair_worklist,
        segment_receipts,
        predicate_consumption,
    }
}

pub(crate) fn binding_subject_with_segment_contract_frame_mismatch(
    readiness_scope: &'static str,
) -> BindingSubject {
    binding_subject_with_alternate_segment_contract_scope(
        readiness_scope,
        Some("frame:event-predicate-binding:mismatch"),
        None,
    )
}

pub(crate) fn binding_subject_with_segment_contract_precision_mismatch(
    readiness_scope: &'static str,
) -> BindingSubject {
    binding_subject_with_alternate_segment_contract_scope(
        readiness_scope,
        None,
        Some("precision:event-predicate-binding:mismatch"),
    )
}

fn binding_subject_with_alternate_segment_contract_scope(
    readiness_scope: &'static str,
    alternate_frame_identity: Option<&'static str>,
    alternate_precision_identity: Option<&'static str>,
) -> BindingSubject {
    let mut subject = binding_subject(readiness_scope);
    let first_work_item = &subject.pair_worklist.work_items()[0];
    let frame_identity =
        alternate_frame_identity.unwrap_or_else(|| first_work_item.left().local_frame_identity());
    let precision_identity = alternate_precision_identity
        .unwrap_or_else(|| first_work_item.left().precision_basis_identity());
    let frame = certified_frame(readiness_scope, frame_identity, precision_identity);
    subject.segment_receipts = subject
        .pair_worklist
        .work_items()
        .iter()
        .map(|work_item| segment_receipt(readiness_scope, &frame, work_item))
        .collect();
    let predicates = unique_orientation_predicates(&subject.segment_receipts);
    subject.predicate_consumption = predicate_consumption_receipt(
        readiness_scope,
        subject.segment_receipts.clone(),
        predicates,
    );
    subject
}

fn certified_frame(
    world: &'static str,
    frame_identity: &str,
    precision_identity: &str,
) -> PlanarLocalFrameCertificateReceipt {
    let predicate = seed_predicate(frame_identity, precision_identity);
    let precision = precision_receipt(world, frame_identity, precision_identity, &predicate);
    let basis = PlanarLocalFrameBasis::builder()
        .frame_identity(frame_identity)
        .origin([1.0e12, 0.0, 0.0])
        .normal([0.0, 0.0, 1.0])
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .transform_chain_digest("transform:event-predicate-binding")
        .movement_rotation_posture_identity(MOVEMENT)
        .tolerance_policy_identity(precision_identity)
        .precision_receipt(&precision)
        .build()
        .expect("valid frame basis");
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
        .expect("valid precision basis");
    planar_precision_certification_facts(
        &planar_precision_certification_entry(
            PlanarPrecisionCertificationCase::from_predicate_receipt(predicate.clone(), basis),
        ),
        &precision_handle(world),
    )
    .expect("precision receipt")
}

fn segment_receipt(
    world: &'static str,
    frame: &PlanarLocalFrameCertificateReceipt,
    work_item: &PlanarBooleanSegmentPairWorkItem,
) -> CertifiedSegmentSegment2DReceipt {
    let left = CertifiedProjectedSegment2D::from_projected_endpoints(
        work_item.left().canonical_segment_identity(),
        projected_endpoint(
            world,
            frame,
            work_item.left().source_ordered_start_endpoint(),
        ),
        projected_endpoint(world, frame, work_item.left().source_ordered_end_endpoint()),
    )
    .expect("left projected segment");
    let right = CertifiedProjectedSegment2D::from_projected_endpoints(
        work_item.right().canonical_segment_identity(),
        projected_endpoint(
            world,
            frame,
            work_item.right().source_ordered_start_endpoint(),
        ),
        projected_endpoint(
            world,
            frame,
            work_item.right().source_ordered_end_endpoint(),
        ),
    )
    .expect("right projected segment");

    CertifiedSegmentSegment2D::classify(left, right)
        .within_topology_basis(TOPOLOGY)
        .with_policy(SegmentContactPolicy::CertifyContactsDenyImprintRequired)
        .compile(&CertifiedSegmentSegment2DContracts::new(
            segment_handle(world),
            predicate_handle(),
        ))
        .expect("segment plan")
        .certify()
        .expect("segment receipt")
}

fn projected_endpoint(
    world: &'static str,
    frame: &PlanarLocalFrameCertificateReceipt,
    endpoint: &PlanarBooleanNormalizedEndpoint,
) -> ProjectPointToCertifiedPlane2DReceipt {
    let point = endpoint.point();
    let basis = ProjectPointToCertifiedPlane2DBasis::builder()
        .source_point_identity(endpoint.source_endpoint_identity())
        .source_point([1.0e12, 0.0, 0.0])
        .source_point_basis_digest(endpoint.projected_endpoint_fact_identity())
        .local_delta_from_frame_origin([point[0], point[1], 0.0])
        .local_frame_receipt(frame)
        .build()
        .expect("valid projection basis");
    project_point_to_certified_plane_2d_facts(
        &project_point_to_certified_plane_2d_entry(
            ProjectPointToCertifiedPlane2DCase::from_local_frame(basis),
        ),
        &projection_handle(world),
    )
    .expect("projection receipt")
}

fn unique_orientation_predicates(
    segments: &[CertifiedSegmentSegment2DReceipt],
) -> Vec<PlanarPredicateFactReceipt> {
    let mut receipts = BTreeMap::new();
    for segment in segments {
        for receipt in segment_orientation_predicates(segment) {
            receipts.insert(receipt.fact_digest().to_string(), receipt);
        }
    }
    receipts.into_values().collect()
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
    .map(|points| {
        predicate_receipt(
            basis.frame_identity(),
            basis.tolerance_policy_identity(),
            points,
        )
    })
    .collect()
}

fn predicate_consumption_receipt(
    world: &'static str,
    segments: Vec<CertifiedSegmentSegment2DReceipt>,
    predicates: Vec<PlanarPredicateFactReceipt>,
) -> PredicateCertificateConsumptionReceipt {
    PredicateCertificateConsumption::for_planar_workload()
        .expecting_topology_basis(TOPOLOGY)
        .expecting_movement_rotation_posture(MOVEMENT)
        .expecting_local_frame(segments[0].basis().frame_identity())
        .with_predicate_authority(predicates)
        .with_segment_contacts(segments)
        .compile(&PredicateCertificateConsumptionContracts::new(
            predicate_consumption_handle(world),
        ))
        .expect("predicate consumption plan")
        .certify()
        .expect("predicate consumption receipt")
}

fn seed_predicate(frame_identity: &str, precision_identity: &str) -> PlanarPredicateFactReceipt {
    predicate_receipt(
        frame_identity,
        precision_identity,
        [[0.0, 0.0], [1.0e-9, 0.0], [0.0, 1.0e-9]],
    )
}

fn predicate_receipt(
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
    .expect("predicate receipt")
}

macro_rules! handle {
    ($name:ident, $domain:expr, $world:expr, $domain_ty:ty, $world_ty:ty) => {
        fn $name(
            world: &'static str,
        ) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<$domain_ty, $world_ty> {
            ForgeQueryApplicationFacade::runtime_backed_default()
                .domain($domain)
                .with_operating_context($world(world))
                .validate()
                .expect("validated predicate-binding contract domain")
                .admit()
                .expect("admitted predicate-binding contract domain")
        }
    };
}

handle!(
    frame_handle,
    PlanarLocalFrameCertificateQueryDomain,
    PlanarLocalFrameCertificateQueryWorld::new,
    PlanarLocalFrameCertificateQueryDomain,
    PlanarLocalFrameCertificateQueryWorld
);
handle!(
    precision_handle,
    PlanarPrecisionCertificationQueryDomain,
    PlanarPrecisionCertificationQueryWorld::new,
    PlanarPrecisionCertificationQueryDomain,
    PlanarPrecisionCertificationQueryWorld
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
handle!(
    predicate_consumption_handle,
    PredicateCertificateConsumptionQueryDomain,
    PredicateCertificateConsumptionQueryWorld::new,
    PredicateCertificateConsumptionQueryDomain,
    PredicateCertificateConsumptionQueryWorld
);

fn predicate_handle() -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarPredicateAuthorityQueryDomain,
    PlanarPredicateAuthorityQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarPredicateAuthorityQueryDomain)
        .with_operating_context(PlanarPredicateAuthorityQueryWorld::new(
            "event-predicate-binding",
        ))
        .validate()
        .expect("validated predicate domain")
        .admit()
        .expect("admitted predicate domain")
}
