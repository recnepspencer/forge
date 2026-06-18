use worth_kernel::workload_composition::{
    BuiltBooleanOperandPairRecipe, PlanarBooleanCommonPlaneReducedOperandPairRequest,
    PlanarBooleanEventExtractionRequest,
};
use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanCollinearRelationExtraction, PlanarBooleanEventLedger,
    PlanarBooleanEventLedgerReceipt, PlanarBooleanEventPredicateBinding,
    PlanarBooleanIntervalEventExtraction, PlanarBooleanPointEventExtraction,
};
use worth_spatial::facade::workload_vocabulary::{
    BooleanEvidenceReceipt, BooleanEvidenceRowAuthority, BooleanEvidenceStageKind,
    WorkloadEvidenceStageCounters, WorkloadEvidenceSupport,
};

use super::collinear_relation_support::{self, SyntheticCollinearRelation};
use super::point_event_support::{self, SyntheticPointRelation};
use super::predicate_binding_support;

pub(crate) struct CertifiedEventLedgerInputs {
    pub(crate) event_request: PlanarBooleanEventExtractionRequest,
    pub(crate) carriers:
        worth_spatial::facade::planar_boolean_events::PlanarBooleanSegmentCarrierSet,
    pub(crate) pair_worklist:
        worth_spatial::facade::planar_boolean_events::PlanarBooleanSegmentPairEnumerationReceipt,
    pub(crate) binding:
        worth_spatial::facade::planar_boolean_events::PlanarBooleanEventPredicateBinding,
    pub(crate) point_events:
        worth_spatial::facade::planar_boolean_events::PlanarBooleanPointEventExtractionReceipt,
    pub(crate) collinear_relations:
        worth_spatial::facade::planar_boolean_events::PlanarBooleanCollinearRelationReceipt,
    pub(crate) interval_events:
        worth_spatial::facade::planar_boolean_events::PlanarBooleanIntervalEventExtractionReceipt,
}

pub(crate) fn ledger_for_collinear_relation(
    readiness_scope: &'static str,
    relation: SyntheticCollinearRelation,
) -> PlanarBooleanEventLedgerReceipt {
    pair_and_ledger_for_collinear_relation(readiness_scope, relation).1
}

pub(crate) fn pair_and_ledger_for_collinear_relation(
    readiness_scope: &'static str,
    relation: SyntheticCollinearRelation,
) -> (
    BuiltBooleanOperandPairRecipe,
    PlanarBooleanEventLedgerReceipt,
) {
    let subject =
        collinear_relation_support::binding_subject_with_relation(readiness_scope, relation);
    let ledger = ledger_for_binding_subject(subject.clone());
    (subject.pair, ledger)
}

pub(crate) fn ledger_for_point_relation(
    readiness_scope: &'static str,
    relation: SyntheticPointRelation,
) -> PlanarBooleanEventLedgerReceipt {
    let subject = point_event_support::binding_subject_with_relation(readiness_scope, relation);
    ledger_for_binding_subject(subject)
}

pub(crate) fn certified_inputs_for_collinear_relation(
    readiness_scope: &'static str,
    relation: SyntheticCollinearRelation,
) -> CertifiedEventLedgerInputs {
    let subject =
        collinear_relation_support::binding_subject_with_relation(readiness_scope, relation);
    certified_inputs_for_binding_subject(subject)
}

pub(crate) fn ledger_for_binding_subject(
    subject: predicate_binding_support::BindingSubject,
) -> PlanarBooleanEventLedgerReceipt {
    let inputs = certified_inputs_for_binding_subject(subject);
    ledger_from_certified_inputs(&inputs)
}

pub(crate) fn ledger_from_certified_inputs(
    inputs: &CertifiedEventLedgerInputs,
) -> PlanarBooleanEventLedgerReceipt {
    PlanarBooleanEventLedger::assemble()
        .for_reduced_pair_identity(inputs.event_request.reduced_operand_pair_identity())
        .for_event_extraction_request_identity(
            inputs.event_request.event_extraction_request_identity(),
        )
        .with_segment_carriers(&inputs.carriers)
        .with_segment_pair_enumeration(&inputs.pair_worklist)
        .with_predicate_binding(&inputs.binding)
        .with_point_events(&inputs.point_events)
        .with_collinear_relations(&inputs.collinear_relations)
        .with_interval_events(&inputs.interval_events)
        .compile()
        .expect("event ledger assembly should compile")
        .certify()
        .expect("event ledger should certify")
}

fn certified_inputs_for_binding_subject(
    subject: predicate_binding_support::BindingSubject,
) -> CertifiedEventLedgerInputs {
    let event_request = PlanarBooleanEventExtractionRequest::from_reduced_operand_pair(
        reduced_pair_from_subject(&subject),
    );
    let carriers = subject
        .reduced_pair
        .segment_carrier_set()
        .expect("segment carriers should certify");
    let binding = PlanarBooleanEventPredicateBinding::plan(&subject.pair_worklist)
        .for_reduced_pair(subject.reduced_pair_identity.clone())
        .with_segment_segment_receipts(subject.segment_receipts)
        .with_predicate_consumption_receipt(subject.predicate_consumption)
        .compile()
        .expect("predicate binding plan should compile")
        .certify()
        .expect("predicate binding should certify");
    let point_events = PlanarBooleanPointEventExtraction::from_predicate_binding(&binding)
        .compile()
        .expect("point extraction should compile")
        .certify()
        .expect("point extraction should certify");
    let collinear_relations =
        PlanarBooleanCollinearRelationExtraction::from_predicate_binding(&binding)
            .compile()
            .expect("collinear extraction should compile")
            .certify()
            .expect("collinear extraction should certify");
    let interval_events =
        PlanarBooleanIntervalEventExtraction::from_collinear_relations(&collinear_relations)
            .compile()
            .expect("interval extraction should compile")
            .certify()
            .expect("interval extraction should certify");

    CertifiedEventLedgerInputs {
        event_request,
        carriers,
        pair_worklist: subject.pair_worklist,
        binding,
        point_events,
        collinear_relations,
        interval_events,
    }
}

fn reduced_pair_from_subject(
    subject: &predicate_binding_support::BindingSubject,
) -> PlanarBooleanCommonPlaneReducedOperandPairRequest {
    subject.reduced_pair.clone()
}

pub(crate) struct CounterlessEventLedgerEvidence {
    identity: String,
}

impl CounterlessEventLedgerEvidence {
    pub(crate) fn new(receipt: &PlanarBooleanEventLedgerReceipt) -> Self {
        Self {
            identity: receipt.event_ledger_identity().to_string(),
        }
    }
}

impl BooleanEvidenceReceipt for CounterlessEventLedgerEvidence {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::EventLedger
    }

    fn evidence_identity(&self) -> &str {
        &self.identity
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::default()
    }
}

impl BooleanEvidenceRowAuthority for CounterlessEventLedgerEvidence {}
