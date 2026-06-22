use crate::workload_platform::evidence_ledger::{WorkloadEvidenceLedger, WorkloadEvidenceRow};
use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_events::{
    normalize_endpoint_order, validate_segment_endpoint_admissibility,
    PlanarBooleanCandidateIndexFallbackPosture, PlanarBooleanCandidateIndexLifecycleOutcome,
    PlanarBooleanCandidateIndexStrategy, PlanarBooleanCanonicalSegment,
    PlanarBooleanCanonicalSegmentSet, PlanarBooleanEventLedgerCounters,
    PlanarBooleanEventLedgerReceipt, PlanarBooleanEventLedgerReceiptInput,
    PlanarBooleanOrderedEventSet, PlanarBooleanSegmentCandidateIndexProduct,
    PlanarBooleanSegmentCandidateIndexProductInput, PlanarBooleanSegmentCarrier,
    PlanarBooleanSegmentCarrierEndpointFacts, PlanarBooleanSegmentPairEnumerationCounters,
    PlanarBooleanSegmentPairEnumerationReceipt,
};

use super::{
    PlanarBooleanCandidateIndexConsumptionDenialKind, PlanarBooleanCandidateIndexConsumptionGate,
    PlanarBooleanCandidateIndexConsumptionInput,
};

#[test]
fn candidate_index_consumption_gate_binds_event_ledger_to_query_product() {
    let segment_pairs = production_segment_pair_receipt();
    let event_ledger = event_ledger_for(segment_pairs.segment_pair_enumeration_identity());
    let evidence = indexed_evidence(&segment_pairs, &event_ledger);

    let gate = PlanarBooleanCandidateIndexConsumptionGate::admit(
        PlanarBooleanCandidateIndexConsumptionInput::new(
            &event_ledger,
            &segment_pairs,
            evidence.stage_index(),
        ),
    )
    .expect("production query-owned candidate-index product should gate split consumption");

    assert_eq!(
        gate.event_ledger_identity(),
        event_ledger.event_ledger_identity()
    );
    assert_eq!(
        gate.segment_pair_enumeration_identity(),
        event_ledger.segment_pair_enumeration_identity()
    );
    assert_eq!(
        gate.candidate_index_product_identity(),
        segment_pairs.candidate_index_product_identity()
    );
    assert_eq!(
        gate.query_index_declaration_digest(),
        segment_pairs.query_index_declaration_digest()
    );
    assert_eq!(
        gate.query_index_plan_digest(),
        segment_pairs.query_index_plan_digest()
    );
    assert_eq!(
        gate.query_index_envelope_digest(),
        segment_pairs.query_index_envelope_digest()
    );
    assert_eq!(
        gate.candidate_index_strategy(),
        PlanarBooleanCandidateIndexStrategy::AabbSweep
    );
    assert_eq!(
        gate.fallback_posture(),
        PlanarBooleanCandidateIndexFallbackPosture::NotUsed
    );
    assert_eq!(
        gate.lifecycle_outcome(),
        PlanarBooleanCandidateIndexLifecycleOutcome::Bound
    );
    assert_eq!(gate.counters().expected_pair_breadth(), 4);
    assert_eq!(gate.counters().indexed_candidate_pair_count(), 2);
    assert_eq!(gate.counters().culled_pair_count(), 2);
    assert!(gate.certifies_production_candidate_discovery());
}

#[test]
fn candidate_index_consumption_gate_rejects_missing_segment_pair_evidence() {
    let segment_pairs = production_segment_pair_receipt();
    let event_ledger = event_ledger_for(segment_pairs.segment_pair_enumeration_identity());
    let evidence = WorkloadEvidenceLedger::from_rows(vec![
        WorkloadEvidenceRow::from_boolean_evidence_receipt(&event_ledger),
    ])
    .expect("single event-ledger evidence row should index");

    let denial = PlanarBooleanCandidateIndexConsumptionGate::admit(
        PlanarBooleanCandidateIndexConsumptionInput::new(
            &event_ledger,
            &segment_pairs,
            evidence.stage_index(),
        ),
    )
    .expect_err("split consumption must require indexed segment-pair evidence");

    assert_eq!(
        denial.kind(),
        PlanarBooleanCandidateIndexConsumptionDenialKind::MissingSegmentPairEnumerationEvidence
    );
}

#[test]
fn candidate_index_consumption_gate_rejects_foreign_event_ledger_binding() {
    let segment_pairs = production_segment_pair_receipt();
    let foreign_ledger = event_ledger_for("foreign-segment-pair-enumeration");
    let evidence = indexed_evidence(&segment_pairs, &foreign_ledger);

    let denial = PlanarBooleanCandidateIndexConsumptionGate::admit(
        PlanarBooleanCandidateIndexConsumptionInput::new(
            &foreign_ledger,
            &segment_pairs,
            evidence.stage_index(),
        ),
    )
    .expect_err("event ledger must bind the same segment-pair enumeration product");

    assert_eq!(
        denial.kind(),
        PlanarBooleanCandidateIndexConsumptionDenialKind::EventLedgerSegmentPairEnumerationMismatch
    );
}

#[test]
fn candidate_index_consumption_gate_rejects_full_breadth_fallback_products() {
    let segment_pairs = fallback_segment_pair_receipt();
    let event_ledger = event_ledger_for(segment_pairs.segment_pair_enumeration_identity());
    let evidence = indexed_evidence(&segment_pairs, &event_ledger);

    let denial = PlanarBooleanCandidateIndexConsumptionGate::admit(
        PlanarBooleanCandidateIndexConsumptionInput::new(
            &event_ledger,
            &segment_pairs,
            evidence.stage_index(),
        ),
    )
    .expect_err("full-breadth fallback products must not certify production split consumption");

    assert_eq!(
        denial.kind(),
        PlanarBooleanCandidateIndexConsumptionDenialKind::NonProductionCandidateIndexFallback
    );
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

fn fallback_segment_pair_receipt() -> PlanarBooleanSegmentPairEnumerationReceipt {
    let counters = PlanarBooleanSegmentPairEnumerationCounters::new(2, 2, 0, 4)
        .with_strategy_counts(0, 4, 0, true);
    let product = PlanarBooleanSegmentCandidateIndexProduct::new(
        PlanarBooleanSegmentCandidateIndexProductInput {
            canonical_segment_set_identity: "fallback-canonical-segment-set".to_string(),
            declaration_digest: "fallback-declaration".to_string(),
            plan_digest: "fallback-plan".to_string(),
            envelope_digest: "fallback-envelope".to_string(),
            strategy: PlanarBooleanCandidateIndexStrategy::AabbSweep,
            fallback_posture: PlanarBooleanCandidateIndexFallbackPosture::FullBreadthNonProduction,
            lifecycle_outcome: PlanarBooleanCandidateIndexLifecycleOutcome::Bound,
            counters,
            rows: Vec::new(),
        },
    )
    .expect("fallback product is coherent but cannot certify production");
    PlanarBooleanSegmentPairEnumerationReceipt::new("fallback-segment-pair-enumeration", product)
}

fn event_ledger_for(segment_pair_enumeration_identity: &str) -> PlanarBooleanEventLedgerReceipt {
    let ordered_events =
        PlanarBooleanOrderedEventSet::from_events_and_groups(&[], &[], &[], Vec::new());
    let ledger_identity = format!("event-ledger:{segment_pair_enumeration_identity}");
    let downstream_consumption_identity =
        format!("downstream-consumption:{segment_pair_enumeration_identity}");
    PlanarBooleanEventLedgerReceipt::new(PlanarBooleanEventLedgerReceiptInput {
        reduced_pair_identity: "reduced-pair".to_string(),
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
        event_ledger_identity: ledger_identity,
        downstream_consumption_identity,
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
