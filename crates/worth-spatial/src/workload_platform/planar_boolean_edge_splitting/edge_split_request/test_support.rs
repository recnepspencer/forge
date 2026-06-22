use crate::workload_platform::evidence_ledger::{WorkloadEvidenceLedger, WorkloadEvidenceRow};
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

pub(super) struct EdgeSplitRequestSubject {
    pub(super) segment_pairs: PlanarBooleanSegmentPairEnumerationReceipt,
    pub(super) event_ledger: PlanarBooleanEventLedgerReceipt,
    pub(super) candidate_index_gate: PlanarBooleanCandidateIndexConsumptionGate,
    pub(super) evidence: WorkloadEvidenceLedger,
}

pub(super) fn request_subject() -> EdgeSplitRequestSubject {
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

pub(super) fn candidate_index_gate_for(
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

pub(super) fn production_segment_pair_receipt() -> PlanarBooleanSegmentPairEnumerationReceipt {
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

pub(super) fn event_ledger_for(
    segment_pair_enumeration_identity: &str,
) -> PlanarBooleanEventLedgerReceipt {
    event_ledger_with_identities(
        segment_pair_enumeration_identity,
        &format!("event-ledger:{segment_pair_enumeration_identity}"),
        &format!("downstream-consumption:{segment_pair_enumeration_identity}"),
        "reduced-pair",
    )
}

pub(super) fn event_ledger_with_identities(
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

pub(super) fn indexed_evidence(
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
