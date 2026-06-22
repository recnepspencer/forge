use crate::workload_platform::evidence_ledger::{
    WorkloadEvidenceLedger, WorkloadEvidenceRow, WorkloadEvidenceStage,
};

use super::test_support::{
    candidate_index_gate_for, event_ledger_for, event_ledger_with_identities, indexed_evidence,
    production_segment_pair_receipt, request_subject,
};
use super::{
    PlanarBooleanEdgeSplitRequest, PlanarBooleanEdgeSplitRequestDenial,
    PlanarBooleanEdgeSplitRequestDenialKind, PlanarBooleanEdgeSplitRequestInput,
};

#[test]
fn edge_split_request_binds_event_ledger_and_candidate_index_gate() {
    let subject = request_subject();
    let event_ledger_lookup = subject
        .evidence
        .require_boolean_receipt_lookup(&subject.event_ledger)
        .expect("typed event-ledger lookup should admit");
    let request = PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
        &subject.event_ledger,
        &subject.candidate_index_gate,
        &event_ledger_lookup,
        None,
    ))
    .expect("split request should admit from receipt-backed event ledger and candidate-index gate");

    assert_eq!(
        request.event_ledger_identity(),
        subject.event_ledger.event_ledger_identity()
    );
    assert_eq!(
        request.downstream_consumption_identity(),
        subject.event_ledger.downstream_consumption_identity()
    );
    assert_eq!(
        request.reduced_pair_identity(),
        subject.event_ledger.reduced_pair_identity()
    );
    assert_eq!(
        request.segment_pair_enumeration_identity(),
        subject.event_ledger.segment_pair_enumeration_identity()
    );
    assert_eq!(
        request.candidate_index_consumption_gate_identity(),
        subject.candidate_index_gate.gate_identity()
    );
    assert_eq!(
        request.candidate_index_product_identity(),
        subject
            .candidate_index_gate
            .candidate_index_product_identity()
    );
    assert_eq!(
        request.query_index_plan_digest(),
        subject.candidate_index_gate.query_index_plan_digest()
    );
    assert_eq!(request.counters().split_request_count(), 1);
}

#[test]
fn edge_split_request_rejects_missing_event_ledger_evidence() {
    let segment_pairs = production_segment_pair_receipt();
    let event_ledger = event_ledger_for(segment_pairs.segment_pair_enumeration_identity());
    let evidence = WorkloadEvidenceLedger::from_rows(vec![
        WorkloadEvidenceRow::from_boolean_evidence_receipt(&segment_pairs),
    ])
    .expect("segment-pair row should index without event ledger row");
    let gate = candidate_index_gate_for(&event_ledger, &segment_pairs);

    let lookup_error = evidence
        .require_boolean_receipt_lookup(&event_ledger)
        .map(|lookup| {
            PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
                &event_ledger,
                &gate,
                &lookup,
                None,
            ))
        })
        .expect_err("split request input must require event-ledger evidence");
    let denial = PlanarBooleanEdgeSplitRequestDenial::from_event_ledger_evidence_error(
        lookup_error,
        event_ledger.event_ledger_identity(),
    );

    assert_eq!(
        denial.kind(),
        PlanarBooleanEdgeSplitRequestDenialKind::MissingEventLedgerEvidence
    );
}

#[test]
fn edge_split_request_rejects_manual_event_ledger_evidence() {
    let segment_pairs = production_segment_pair_receipt();
    let event_ledger = event_ledger_for(segment_pairs.segment_pair_enumeration_identity());
    let evidence = WorkloadEvidenceLedger::from_rows(vec![
        WorkloadEvidenceRow::from_boolean_evidence_receipt(&segment_pairs),
        WorkloadEvidenceRow::new(
            WorkloadEvidenceStage::BooleanEventLedger,
            event_ledger.event_ledger_identity(),
        ),
    ])
    .expect("manual event-ledger row should index before split request admission");
    let gate = candidate_index_gate_for(&event_ledger, &segment_pairs);

    let lookup_error = evidence
        .require_boolean_receipt_lookup(&event_ledger)
        .map(|lookup| {
            PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
                &event_ledger,
                &gate,
                &lookup,
                None,
            ))
        })
        .expect_err("split request input must reject hand-filled event-ledger evidence");
    let denial = PlanarBooleanEdgeSplitRequestDenial::from_event_ledger_evidence_error(
        lookup_error,
        event_ledger.event_ledger_identity(),
    );

    assert_eq!(
        denial.kind(),
        PlanarBooleanEdgeSplitRequestDenialKind::ManualEventLedgerEvidence
    );
}

#[test]
fn edge_split_request_rejects_candidate_index_gate_from_foreign_event_ledger() {
    let subject = request_subject();
    let foreign_event_ledger = event_ledger_for("foreign-segment-pair-enumeration");
    let evidence = WorkloadEvidenceLedger::from_rows(vec![
        WorkloadEvidenceRow::from_boolean_evidence_receipt(&subject.segment_pairs),
        WorkloadEvidenceRow::from_boolean_evidence_receipt(&foreign_event_ledger),
    ])
    .expect("foreign event-ledger row should index before split request binding denial");
    let event_ledger_lookup = evidence
        .require_boolean_receipt_lookup(&foreign_event_ledger)
        .expect("foreign event-ledger lookup should admit before gate binding denial");

    let denial = PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
        &foreign_event_ledger,
        &subject.candidate_index_gate,
        &event_ledger_lookup,
        None,
    ))
    .expect_err("split request must reject a candidate-index gate from another event ledger");

    assert_eq!(
        denial.kind(),
        PlanarBooleanEdgeSplitRequestDenialKind::CandidateIndexGateEventLedgerMismatch
    );
}

#[test]
fn edge_split_request_rejects_candidate_index_gate_from_foreign_downstream_consumption() {
    let subject = request_subject();
    let foreign_event_ledger = event_ledger_with_identities(
        subject.event_ledger.segment_pair_enumeration_identity(),
        subject.event_ledger.event_ledger_identity(),
        "foreign-downstream-consumption",
        subject.event_ledger.reduced_pair_identity(),
    );
    let evidence = indexed_evidence(&subject.segment_pairs, &foreign_event_ledger);
    let event_ledger_lookup = evidence
        .require_boolean_receipt_lookup(&foreign_event_ledger)
        .expect("foreign event-ledger lookup should admit before downstream binding denial");

    let denial = PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
        &foreign_event_ledger,
        &subject.candidate_index_gate,
        &event_ledger_lookup,
        None,
    ))
    .expect_err("split request must reject a foreign downstream-consumption lineage");

    assert_eq!(
        denial.kind(),
        PlanarBooleanEdgeSplitRequestDenialKind::CandidateIndexGateDownstreamMismatch
    );
}

#[test]
fn edge_split_request_rejects_candidate_index_gate_from_foreign_reduced_pair() {
    let subject = request_subject();
    let foreign_event_ledger = event_ledger_with_identities(
        subject.event_ledger.segment_pair_enumeration_identity(),
        subject.event_ledger.event_ledger_identity(),
        subject.event_ledger.downstream_consumption_identity(),
        "foreign-reduced-pair",
    );
    let evidence = indexed_evidence(&subject.segment_pairs, &foreign_event_ledger);
    let event_ledger_lookup = evidence
        .require_boolean_receipt_lookup(&foreign_event_ledger)
        .expect("foreign event-ledger lookup should admit before reduced-pair binding denial");

    let denial = PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
        &foreign_event_ledger,
        &subject.candidate_index_gate,
        &event_ledger_lookup,
        None,
    ))
    .expect_err("split request must reject a foreign reduced-pair lineage");

    assert_eq!(
        denial.kind(),
        PlanarBooleanEdgeSplitRequestDenialKind::CandidateIndexGateReducedPairMismatch
    );
}

#[test]
fn edge_split_request_rejects_candidate_index_gate_from_foreign_segment_pair_enumeration() {
    let subject = request_subject();
    let foreign_event_ledger = event_ledger_with_identities(
        "foreign-segment-pair-enumeration",
        subject.event_ledger.event_ledger_identity(),
        subject.event_ledger.downstream_consumption_identity(),
        subject.event_ledger.reduced_pair_identity(),
    );
    let evidence = WorkloadEvidenceLedger::from_rows(vec![
        WorkloadEvidenceRow::from_boolean_evidence_receipt(&foreign_event_ledger),
    ])
    .expect("foreign event-ledger row should index before split request binding denial");
    let event_ledger_lookup = evidence
        .require_boolean_receipt_lookup(&foreign_event_ledger)
        .expect("foreign event-ledger lookup should admit before segment-pair binding denial");

    let denial = PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
        &foreign_event_ledger,
        &subject.candidate_index_gate,
        &event_ledger_lookup,
        None,
    ))
    .expect_err("split request must reject a foreign segment-pair enumeration lineage");

    assert_eq!(
        denial.kind(),
        PlanarBooleanEdgeSplitRequestDenialKind::CandidateIndexGateSegmentPairMismatch
    );
}
