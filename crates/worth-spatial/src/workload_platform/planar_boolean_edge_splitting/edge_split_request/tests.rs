use crate::workload_platform::evidence_ledger::{
    WorkloadEvidenceLedger, WorkloadEvidenceRow, WorkloadEvidenceStage,
};
use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanCandidateIndexConsumptionGate, PlanarBooleanCandidateIndexConsumptionInput,
};
use crate::workload_platform::planar_boolean_events::{
    normalize_endpoint_order, validate_segment_endpoint_admissibility,
    PlanarBooleanCanonicalSegment, PlanarBooleanCanonicalSegmentSet,
    PlanarBooleanEventLedgerCounters, PlanarBooleanEventLedgerReceipt,
    PlanarBooleanEventLedgerReceiptInput, PlanarBooleanOrderedEventSet,
    PlanarBooleanSegmentCarrier, PlanarBooleanSegmentCarrierEndpointFacts,
    PlanarBooleanSegmentPairEnumerationReceipt,
};

use super::{
    PlanarBooleanEdgeSplitRequest, PlanarBooleanEdgeSplitRequestDenialKind,
    PlanarBooleanEdgeSplitRequestInput,
};

#[test]
fn edge_split_request_binds_event_ledger_and_candidate_index_gate() {
    let subject = request_subject();
    let request = PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
        &subject.event_ledger,
        &subject.candidate_index_gate,
        subject.evidence.stage_index(),
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

    let denial = PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
        &event_ledger,
        &gate,
        evidence.stage_index(),
    ))
    .expect_err("split request must require event-ledger evidence");

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

    let denial = PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
        &event_ledger,
        &gate,
        evidence.stage_index(),
    ))
    .expect_err("split request must reject hand-filled event-ledger evidence");

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

    let denial = PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
        &foreign_event_ledger,
        &subject.candidate_index_gate,
        evidence.stage_index(),
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

    let denial = PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
        &foreign_event_ledger,
        &subject.candidate_index_gate,
        evidence.stage_index(),
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

    let denial = PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
        &foreign_event_ledger,
        &subject.candidate_index_gate,
        evidence.stage_index(),
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

    let denial = PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
        &foreign_event_ledger,
        &subject.candidate_index_gate,
        evidence.stage_index(),
    ))
    .expect_err("split request must reject a foreign segment-pair enumeration lineage");

    assert_eq!(
        denial.kind(),
        PlanarBooleanEdgeSplitRequestDenialKind::CandidateIndexGateSegmentPairMismatch
    );
}

struct EdgeSplitRequestSubject {
    segment_pairs: PlanarBooleanSegmentPairEnumerationReceipt,
    event_ledger: PlanarBooleanEventLedgerReceipt,
    candidate_index_gate: PlanarBooleanCandidateIndexConsumptionGate,
    evidence: WorkloadEvidenceLedger,
}

fn request_subject() -> EdgeSplitRequestSubject {
    let segment_pairs = production_segment_pair_receipt();
    let event_ledger = event_ledger_for(segment_pairs.segment_pair_enumeration_identity());
    let evidence = indexed_evidence(&segment_pairs, &event_ledger);
    let candidate_index_gate = PlanarBooleanCandidateIndexConsumptionGate::admit(
        PlanarBooleanCandidateIndexConsumptionInput::new(
            &event_ledger,
            &segment_pairs,
            evidence.stage_index(),
        ),
    )
    .expect("candidate-index gate should admit for production test subject");
    EdgeSplitRequestSubject {
        segment_pairs,
        event_ledger,
        candidate_index_gate,
        evidence,
    }
}

fn candidate_index_gate_for(
    event_ledger: &PlanarBooleanEventLedgerReceipt,
    segment_pairs: &PlanarBooleanSegmentPairEnumerationReceipt,
) -> PlanarBooleanCandidateIndexConsumptionGate {
    let evidence = indexed_evidence(segment_pairs, event_ledger);
    PlanarBooleanCandidateIndexConsumptionGate::admit(
        PlanarBooleanCandidateIndexConsumptionInput::new(
            event_ledger,
            segment_pairs,
            evidence.stage_index(),
        ),
    )
    .expect("candidate-index gate should admit")
}

fn production_segment_pair_receipt() -> PlanarBooleanSegmentPairEnumerationReceipt {
    let left = vec![
        canonical_segment(
            PlanarBooleanCommonPlaneOperandSide::Left,
            "left-a",
            [0.0, 0.0],
            [0.0, 1.0],
        ),
        canonical_segment(
            PlanarBooleanCommonPlaneOperandSide::Left,
            "left-b",
            [10.0, 0.0],
            [10.0, 1.0],
        ),
    ];
    let right = vec![
        canonical_segment(
            PlanarBooleanCommonPlaneOperandSide::Right,
            "right-a",
            [0.0, 0.5],
            [0.0, 1.5],
        ),
        canonical_segment(
            PlanarBooleanCommonPlaneOperandSide::Right,
            "right-b",
            [10.0, 0.5],
            [10.0, 1.5],
        ),
    ];
    PlanarBooleanCanonicalSegmentSet::for_pair_enumeration_test(left, right)
        .segment_pair_enumeration_receipt()
        .expect("indexed pair enumeration should certify")
}

fn event_ledger_for(segment_pair_enumeration_identity: &str) -> PlanarBooleanEventLedgerReceipt {
    event_ledger_with_identities(
        segment_pair_enumeration_identity,
        &format!("event-ledger:{segment_pair_enumeration_identity}"),
        &format!("downstream-consumption:{segment_pair_enumeration_identity}"),
        "reduced-pair",
    )
}

fn event_ledger_with_identities(
    segment_pair_enumeration_identity: &str,
    event_ledger_identity: &str,
    downstream_consumption_identity: &str,
    reduced_pair_identity: &str,
) -> PlanarBooleanEventLedgerReceipt {
    let ordered_events =
        PlanarBooleanOrderedEventSet::from_events_and_groups(&[], &[], &[], Vec::new());
    PlanarBooleanEventLedgerReceipt::new(PlanarBooleanEventLedgerReceiptInput {
        reduced_pair_identity: reduced_pair_identity.to_string(),
        event_extraction_request_identity: "event-extraction-request".to_string(),
        segment_carrier_set_identity: "segment-carrier-set".to_string(),
        segment_carriers: Vec::new(),
        segment_pair_enumeration_identity: segment_pair_enumeration_identity.to_string(),
        predicate_binding_identity: "predicate-binding".to_string(),
        point_event_extraction_identity: "point-event-extraction".to_string(),
        collinear_relation_receipt_identity: "collinear-relation".to_string(),
        interval_event_extraction_identity: "interval-event-extraction".to_string(),
        point_events: Vec::new(),
        interval_events: Vec::new(),
        relation_diagnostics: Vec::new(),
        event_groups: Vec::new(),
        ordered_events,
        counters: PlanarBooleanEventLedgerCounters::default(),
        event_ledger_identity: event_ledger_identity.to_string(),
        downstream_consumption_identity: downstream_consumption_identity.to_string(),
    })
}

fn indexed_evidence(
    segment_pairs: &PlanarBooleanSegmentPairEnumerationReceipt,
    event_ledger: &PlanarBooleanEventLedgerReceipt,
) -> WorkloadEvidenceLedger {
    WorkloadEvidenceLedger::from_rows(vec![
        WorkloadEvidenceRow::from_boolean_evidence_receipt(segment_pairs),
        WorkloadEvidenceRow::from_boolean_evidence_receipt(event_ledger),
    ])
    .expect("receipt-backed boolean stage rows should index")
}

fn canonical_segment(
    side: PlanarBooleanCommonPlaneOperandSide,
    source_edge_identity: &str,
    start: [f64; 2],
    end: [f64; 2],
) -> PlanarBooleanCanonicalSegment {
    let carrier = PlanarBooleanSegmentCarrier::for_canonical_segment_test_on_side_with_source_edge(
        side,
        source_edge_identity,
        endpoint(start, "start"),
        endpoint(end, "end"),
    );
    validate_segment_endpoint_admissibility(&carrier).expect("test segment should be admissible");
    PlanarBooleanCanonicalSegment::from_carrier(&carrier, normalize_endpoint_order(&carrier))
}

fn endpoint(point: [f64; 2], label: &str) -> PlanarBooleanSegmentCarrierEndpointFacts {
    PlanarBooleanSegmentCarrierEndpointFacts::from_projected_loop_boundary(
        point,
        format!("{label}-source-endpoint-{point:?}"),
        "test projected loop",
        "test projection stage",
        "test projection local basis",
    )
}
