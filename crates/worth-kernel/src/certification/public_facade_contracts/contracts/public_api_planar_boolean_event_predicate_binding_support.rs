use std::collections::BTreeMap;

#[path = "public_api_planar_boolean_event_predicate_binding_handles.rs"]
mod handles;
#[path = "public_api_planar_boolean_event_predicate_binding_predicate_cache.rs"]
mod predicate_cache;
#[path = "public_api_planar_boolean_event_predicate_binding_projection_cache.rs"]
mod projection_cache;
use handles::{
    frame_handle, precision_handle, predicate_consumption_handle, predicate_handle,
    projection_handle, segment_handle,
};
use worth_kernel::workload_composition::PlanarBooleanCommonPlaneReducedOperandPairRequest;
use worth_spatial::facade::planar_boolean_events::PlanarBooleanSegmentPairEnumerationReceipt;
use worth_spatial::facade::planar_local_frame::{
    planar_local_frame_certificate_entry, planar_local_frame_certificate_facts,
    PlanarLocalFrameBasis, PlanarLocalFrameCertificateCase, PlanarLocalFrameCertificateReceipt,
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
use worth_spatial::facade::planar_segment_segment::{
    CertifiedSegmentSegment2DContracts, CertifiedSegmentSegment2DReceipt,
};

use super::reduced_pair_support;
use worth_kernel::workload_composition::trace_scope;

const MOVEMENT: &str = "movement:event-predicate-binding";
const TOPOLOGY: &str = "topology:event-predicate-binding";

#[derive(Clone)]
pub(crate) struct BindingSubject {
    pub(crate) pair: worth_kernel::workload_composition::BuiltBooleanOperandPairRecipe,
    pub(crate) reduced_pair: PlanarBooleanCommonPlaneReducedOperandPairRequest,
    pub(crate) reduced_pair_identity: String,
    pub(crate) pair_worklist: PlanarBooleanSegmentPairEnumerationReceipt,
    pub(crate) segment_receipts: Vec<CertifiedSegmentSegment2DReceipt>,
    pub(crate) predicate_consumption: PredicateCertificateConsumptionReceipt,
}

impl BindingSubject {
    fn provenance_tuple(&self) -> (&str, &str) {
        (
            self.pair.operand_pair_identity(),
            self.reduced_pair.reduced_operand_pair_identity(),
        )
    }
}

pub(crate) fn binding_subject(readiness_scope: &'static str) -> BindingSubject {
    let (pair, operand_a, operand_b) =
        trace_scope("binding_subject_event_carrier_operands", || {
            reduced_pair_support::event_carrier_projected_operand_requests_from_catalog(
                readiness_scope,
            )
        });
    binding_subject_from_projected_operands(readiness_scope, pair, operand_a, operand_b)
}

pub(crate) fn binding_subject_from_projected_operands(
    readiness_scope: &'static str,
    pair: worth_kernel::workload_composition::BuiltBooleanOperandPairRecipe,
    operand_a: worth_kernel::workload_composition::PlanarBooleanCommonPlaneOperandAProjectedRequest,
    operand_b: worth_kernel::workload_composition::PlanarBooleanCommonPlaneOperandBProjectedRequest,
) -> BindingSubject {
    trace_scope("binding_subject_from_projected_operands", || {
        let reduced_pair = trace_scope("binding_subject_reduced_pair", || {
            PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
                operand_a, operand_b,
            )
            .expect("reduced pair should certify")
        });
        let pair_worklist = trace_scope("binding_subject_pair_worklist", || {
            reduced_pair
                .segment_carrier_set()
                .expect("carrier set should certify")
                .canonical_segment_set()
                .expect("canonical segments should certify")
                .segment_pair_enumeration_receipt()
                .expect("pair worklist should certify")
        });
        let frame = trace_scope("binding_subject_certified_frame", || {
            certified_frame(
                readiness_scope,
                pair_worklist.work_items()[0].left().local_frame_identity(),
                pair_worklist.work_items()[0]
                    .left()
                    .precision_basis_identity(),
            )
        });
        let segment_contracts = trace_scope("binding_subject_segment_contract_handles", || {
            CertifiedSegmentSegment2DContracts::new(
                segment_handle(readiness_scope),
                predicate_handle(),
            )
        });
        let projection_handle = trace_scope("binding_subject_projection_handle", || {
            projection_handle(readiness_scope)
        });
        let mut projection_cache = BTreeMap::new();
        let segment_receipts = trace_scope("binding_subject_segment_receipts", || {
            let mut receipts = Vec::with_capacity(pair_worklist.work_items().len());
            for work_item in pair_worklist.work_items() {
                receipts.push(projection_cache::segment_receipt_from_cached_projection(
                    &frame,
                    work_item,
                    TOPOLOGY,
                    &segment_contracts,
                    &projection_handle,
                    &mut projection_cache,
                ));
            }
            receipts
        });
        let predicates = trace_scope("binding_subject_unique_predicates", || {
            predicate_cache::unique_orientation_predicates_from_segment_receipts(&segment_receipts)
        });
        let predicate_consumption = trace_scope("binding_subject_predicate_consumption", || {
            predicate_consumption_receipt(readiness_scope, segment_receipts.clone(), predicates)
        });
        let subject = BindingSubject {
            pair,
            reduced_pair: reduced_pair.clone(),
            reduced_pair_identity: reduced_pair.reduced_operand_pair_identity().to_string(),
            pair_worklist,
            segment_receipts,
            predicate_consumption,
        };
        let _ = subject.provenance_tuple();
        subject
    })
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
    let segment_contracts = CertifiedSegmentSegment2DContracts::new(
        segment_handle(readiness_scope),
        predicate_handle(),
    );
    let projection_handle = projection_handle(readiness_scope);
    let mut projection_cache = BTreeMap::new();
    subject.segment_receipts = subject
        .pair_worklist
        .work_items()
        .iter()
        .map(|work_item| {
            projection_cache::segment_receipt_from_cached_projection(
                &frame,
                work_item,
                TOPOLOGY,
                &segment_contracts,
                &projection_handle,
                &mut projection_cache,
            )
        })
        .collect();
    let predicates = predicate_cache::unique_orientation_predicates_from_segment_receipts(
        &subject.segment_receipts,
    );
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

const _: () = {
    let _ = binding_subject_with_segment_contract_frame_mismatch;
    let _ = binding_subject_with_segment_contract_precision_mismatch;
};
