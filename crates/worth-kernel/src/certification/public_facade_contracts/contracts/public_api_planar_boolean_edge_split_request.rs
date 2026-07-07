use super::metaboss_support::MetabossEventExtractionSubject;
use super::reduced_pair_support;
use worth_kernel::workload_composition::{WorthWorkload, WorthWorkloadParts};
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanCandidateIndexConsumptionGate, PlanarBooleanCandidateIndexConsumptionInput,
    PlanarBooleanEdgeSplitRequest, PlanarBooleanEdgeSplitRequestInput,
};
use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceLedger, WorkloadEvidenceLedgerError, WorkloadEvidenceRow, WorkloadEvidenceStage,
};

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
        assert!(!request
            .event_ledger_lookup_selected_plan_digest()
            .is_empty());
        assert!(!request
            .event_ledger_lookup_execution_receipt_digest()
            .is_empty());
        assert!(!request
            .event_ledger_lookup_product_output_digest()
            .is_empty());
    });
}

#[test]
fn edge_split_request_requires_boolean_event_ledger_evidence_row() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject =
            MetabossEventExtractionSubject::certify("phase7.3 edge split missing ledger evidence");
        let segment_pairs = &subject.inputs().pair_worklist;
        let ledger = subject.ledger();
        let missing_ledger_evidence = WorkloadEvidenceLedger::from_rows(vec![
            WorkloadEvidenceRow::from_boolean_evidence_receipt(segment_pairs),
        ])
        .expect("hostile evidence should index before request admission");

        let denial = missing_ledger_evidence
            .require_boolean_receipt_lookup(ledger)
            .expect_err("edge split request input must require event-ledger evidence lookup");

        assert_eq!(
            denial,
            WorkloadEvidenceLedgerError::MissingBooleanStage(
                WorkloadEvidenceStage::BooleanEventLedger
            )
        );
    });
}

#[test]
fn edge_split_request_lookup_execution_helper_accepts_workload_with_existing_event_ledger_stage() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject =
            MetabossEventExtractionSubject::certify("phase7.3 existing event-ledger stage");
        let workload = subject.pair().left().workload();
        let evidence_ledger = workload
            .evidence_ledger()
            .with_boolean_evidence_receipt(subject.ledger())
            .expect("event-ledger receipt should append once for completed workload fixture");
        let completed_workload = WorthWorkload::compose(WorthWorkloadParts {
            topology: workload.topology().clone(),
            geometry_binding: workload.geometry_binding().clone(),
            surface_support: workload.surface_support().clone(),
            projection: workload.projection().clone(),
            transform: workload.transform().clone(),
            retained_replay: workload.retained_replay().clone(),
            batch_admission_execution: workload.batch_admission_execution().cloned(),
            diagnostics: workload.diagnostics().clone(),
            response: workload.response().clone(),
            evidence_ledger,
        })
        .expect("completed workload fixture should compose");

        let packet = completed_workload
            .require_boolean_event_ledger_lookup_execution_packet(subject.ledger())
            .expect(
                "packet helper should reuse existing event-ledger stage instead of duplicating it",
            );
        let witness = completed_workload
            .require_boolean_event_ledger_lookup_execution(subject.ledger())
            .expect("helper should reuse existing event-ledger stage instead of duplicating it");

        assert_eq!(
            packet.witness().event_ledger_identity(),
            subject.ledger().event_ledger_identity()
        );
        assert!(!packet.selected_family_identity().is_empty());
        assert!(!packet.selected_plan().selected_plan_digest().is_empty());
        assert!(!packet
            .execution_receipt()
            .execution_receipt_digest()
            .is_empty());
        assert_eq!(
            witness.event_ledger_identity(),
            subject.ledger().event_ledger_identity()
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
    let workload = subject.pair().left().workload();
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
    let event_ledger_lookup = workload
        .require_boolean_event_ledger_lookup_execution_packet(ledger)
        .expect("execution-backed event-ledger lookup packet should admit before split request");
    let request = PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
        ledger,
        &gate,
        event_ledger_lookup.witness(),
        None,
    ))
    .expect("edge split request should admit from event ledger and candidate-index gate");
    (request, gate)
}
