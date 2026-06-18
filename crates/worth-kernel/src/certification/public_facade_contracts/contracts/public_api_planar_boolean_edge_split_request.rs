#[path = "public_api_planar_boolean_collinear_relations_support/mod.rs"]
#[allow(dead_code)]
mod collinear_relation_support;
#[path = "public_api_planar_boolean_event_ledger_support.rs"]
#[allow(dead_code)]
mod event_ledger_support;
#[path = "public_api_planar_boolean_event_extraction_metaboss_support/mod.rs"]
#[allow(dead_code, unused_imports)]
mod metaboss_support;
#[path = "public_api_planar_boolean_point_events_support/mod.rs"]
#[allow(dead_code)]
mod point_event_support;
#[path = "public_api_planar_boolean_event_predicate_binding_support.rs"]
#[allow(dead_code)]
mod predicate_binding_support;
#[path = "public_api_planar_boolean_common_plane_reduced_operand_pair_support.rs"]
mod reduced_pair_support;

use metaboss_support::MetabossEventExtractionSubject;
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanCandidateIndexConsumptionGate, PlanarBooleanCandidateIndexConsumptionInput,
    PlanarBooleanEdgeSplitRequest, PlanarBooleanEdgeSplitRequestDenialKind,
    PlanarBooleanEdgeSplitRequestInput,
};
use worth_spatial::facade::workload_vocabulary::{WorkloadEvidenceLedger, WorkloadEvidenceRow};

#[test]
fn edge_split_request_preserves_event_ledger_and_reduced_pair_identities() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject = MetabossEventExtractionSubject::certify("phase7.3 edge split request");
        let (request, gate) = metaboss_edge_split_request(&subject);

        assert_eq!(
            request.event_ledger_identity(),
            subject.ledger().event_ledger_identity()
        );
        assert_eq!(
            request.downstream_consumption_identity(),
            subject.ledger().downstream_consumption_identity()
        );
        assert_eq!(
            request.reduced_pair_identity(),
            subject.ledger().reduced_pair_identity()
        );
        assert_eq!(
            request.segment_pair_enumeration_identity(),
            subject.ledger().segment_pair_enumeration_identity()
        );
        assert_eq!(
            request.candidate_index_consumption_gate_identity(),
            gate.gate_identity()
        );
        assert_eq!(request.counters().split_request_count(), 1);
        assert_eq!(
            request.counters().segment_carrier_count(),
            subject.ledger().segment_carriers().len()
        );
    });
}

#[test]
fn edge_split_request_preserves_candidate_index_product_identity() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject =
            MetabossEventExtractionSubject::certify("phase7.3 edge split candidate index product");
        let (request, gate) = metaboss_edge_split_request(&subject);

        assert_eq!(
            request.candidate_index_product_identity(),
            gate.candidate_index_product_identity()
        );
        assert_eq!(
            request.query_index_plan_digest(),
            gate.query_index_plan_digest()
        );
    });
}

#[test]
fn edge_split_request_requires_boolean_event_ledger_evidence_row() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject =
            MetabossEventExtractionSubject::certify("phase7.3 edge split missing ledger evidence");
        let segment_pairs = &subject.inputs().pair_worklist;
        let ledger = subject.ledger();
        let gate_evidence = WorkloadEvidenceLedger::from_rows(vec![
            WorkloadEvidenceRow::from_boolean_evidence_receipt(segment_pairs),
            WorkloadEvidenceRow::from_boolean_evidence_receipt(ledger),
        ])
        .expect("candidate-index gate evidence should index");
        let gate = PlanarBooleanCandidateIndexConsumptionGate::admit(
            PlanarBooleanCandidateIndexConsumptionInput::new(
                ledger,
                segment_pairs,
                gate_evidence.stage_index(),
            ),
        )
        .expect("candidate-index gate should admit");
        let missing_ledger_evidence = WorkloadEvidenceLedger::from_rows(vec![
            WorkloadEvidenceRow::from_boolean_evidence_receipt(segment_pairs),
        ])
        .expect("hostile evidence should index before request admission");

        let denial = PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
            ledger,
            &gate,
            missing_ledger_evidence.stage_index(),
        ))
        .expect_err("edge split request must require event-ledger evidence");

        assert_eq!(
            denial.kind(),
            PlanarBooleanEdgeSplitRequestDenialKind::MissingEventLedgerEvidence
        );
    });
}

fn metaboss_edge_split_request(
    subject: &MetabossEventExtractionSubject,
) -> (
    PlanarBooleanEdgeSplitRequest,
    PlanarBooleanCandidateIndexConsumptionGate,
) {
    let segment_pairs = &subject.inputs().pair_worklist;
    let ledger = subject.ledger();
    let evidence = WorkloadEvidenceLedger::from_rows(vec![
        WorkloadEvidenceRow::from_boolean_evidence_receipt(segment_pairs),
        WorkloadEvidenceRow::from_boolean_evidence_receipt(ledger),
    ])
    .expect("metaboss boolean evidence should index");
    let gate = PlanarBooleanCandidateIndexConsumptionGate::admit(
        PlanarBooleanCandidateIndexConsumptionInput::new(
            ledger,
            segment_pairs,
            evidence.stage_index(),
        ),
    )
    .expect("candidate-index gate should admit");
    let request = PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
        ledger,
        &gate,
        evidence.stage_index(),
    ))
    .expect("edge split request should admit from event ledger and candidate-index gate");
    (request, gate)
}
