use crate::workload_platform::evidence_ledger::{WorkloadEvidenceLedger, WorkloadEvidenceRow};
use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanCandidateIndexConsumptionGate, PlanarBooleanCandidateIndexConsumptionInput,
    PlanarBooleanEdgeSplitRequest, PlanarBooleanEdgeSplitRequestInput,
};
use crate::workload_platform::planar_boolean_events::{
    normalize_endpoint_order, validate_segment_endpoint_admissibility,
    PlanarBooleanCanonicalSegment, PlanarBooleanCanonicalSegmentSet,
    PlanarBooleanEventLedgerCounters, PlanarBooleanEventLedgerReceipt,
    PlanarBooleanEventLedgerReceiptInput, PlanarBooleanOrderedEventSet,
    PlanarBooleanSegmentCarrier, PlanarBooleanSegmentCarrierEndpointFacts,
    PlanarBooleanSegmentPairEnumerationReceipt,
};
use crate::workload_platform::vocabulary::WorkloadStageSupport;

use super::{
    PlanarBooleanEdgeSplitDegeneracyPolicy, PlanarBooleanEdgeSplitDeterminismPolicy,
    PlanarBooleanEdgeSplitOverlapPolicy, PlanarBooleanEdgeSplitPolicyOutcomeKind,
    PlanarBooleanEdgeSplitScopeAdmission, PlanarBooleanEdgeSplitScopeAdmissionDenialKind,
    PlanarBooleanEdgeSplitScopeAdmissionInput, PlanarBooleanEdgeSplitScopeClass,
};

#[test]
fn edge_split_scope_admits_only_event_families_closed_by_7_2() {
    let subject = request_subject_with_source_carriers();
    let admission = PlanarBooleanEdgeSplitScopeAdmission::admit(
        PlanarBooleanEdgeSplitScopeAdmissionInput::from_split_request(&subject.request)
            .with_degeneracy_policy(PlanarBooleanEdgeSplitDegeneracyPolicy::fail_closed())
            .with_determinism_policy(PlanarBooleanEdgeSplitDeterminismPolicy::canonical_replay())
            .with_overlap_policy(PlanarBooleanEdgeSplitOverlapPolicy::preserve_interval_posture()),
    )
    .expect("source-carrier-backed edge split request should admit into 7.3 scope");

    assert_eq!(
        admission.scope_class(),
        PlanarBooleanEdgeSplitScopeClass::PlanarBRepLineSegmentEdgeSurgery
    );
    assert_eq!(
        admission.split_request_identity(),
        subject.request.split_request_identity()
    );
    assert_eq!(
        admission.event_ledger_identity(),
        subject.request.event_ledger_identity()
    );
    assert_eq!(
        admission.candidate_index_product_identity(),
        subject.request.candidate_index_product_identity()
    );
    assert_eq!(
        admission.query_index_plan_digest(),
        subject.request.query_index_plan_digest()
    );
    assert_eq!(admission.counters().scope_admission_count(), 1);
    assert_eq!(
        admission.counters().source_carrier_count(),
        subject.request.counters().segment_carrier_count()
    );
    assert!(admission
        .policy_outcome()
        .is_admitted_for_source_edge_recovery());
}

#[test]
fn edge_split_scope_denies_unsupported_event_family_before_schedule_building() {
    let subject = request_subject_without_source_carriers();
    let denial = PlanarBooleanEdgeSplitScopeAdmission::admit(
        PlanarBooleanEdgeSplitScopeAdmissionInput::from_split_request(&subject.request),
    )
    .expect_err("empty source-carrier scope must deny before recovery/schedule construction");

    assert_eq!(
        denial.kind(),
        PlanarBooleanEdgeSplitScopeAdmissionDenialKind::UnsupportedEmptySourceCarrierScope
    );
    assert_eq!(
        denial.policy_outcome().kind(),
        PlanarBooleanEdgeSplitPolicyOutcomeKind::Unsupported
    );
    assert_eq!(
        denial.policy_outcome().support(),
        WorkloadStageSupport::Unsupported
    );
    assert_eq!(
        denial.policy_outcome().event_ledger_identity(),
        subject.request.event_ledger_identity()
    );
    assert_eq!(
        denial.policy_outcome().split_request_identity(),
        subject.request.split_request_identity()
    );
}

#[test]
fn edge_split_policy_outcomes_preserve_machine_kind_and_event_ledger_identity() {
    let admitted_subject = request_subject_with_source_carriers();
    let admission = PlanarBooleanEdgeSplitScopeAdmission::admit(
        PlanarBooleanEdgeSplitScopeAdmissionInput::from_split_request(&admitted_subject.request),
    )
    .expect("carrier-backed request should admit");
    assert_eq!(
        admission.policy_outcome().kind(),
        PlanarBooleanEdgeSplitPolicyOutcomeKind::Admitted
    );
    assert_eq!(
        admission.policy_outcome().support(),
        WorkloadStageSupport::Admitted
    );
    assert_eq!(
        admission.policy_outcome().event_ledger_identity(),
        admitted_subject.request.event_ledger_identity()
    );

    let denied_subject = request_subject_without_source_carriers();
    let denial = PlanarBooleanEdgeSplitScopeAdmission::admit(
        PlanarBooleanEdgeSplitScopeAdmissionInput::from_split_request(&denied_subject.request),
    )
    .expect_err("carrierless request should deny");
    assert_eq!(
        denial.policy_outcome().kind(),
        PlanarBooleanEdgeSplitPolicyOutcomeKind::Unsupported
    );
    assert_eq!(
        denial.policy_outcome().event_ledger_identity(),
        denied_subject.request.event_ledger_identity()
    );
}

struct EdgeSplitScopeSubject {
    request: PlanarBooleanEdgeSplitRequest,
}

fn request_subject_with_source_carriers() -> EdgeSplitScopeSubject {
    request_subject(true)
}

fn request_subject_without_source_carriers() -> EdgeSplitScopeSubject {
    request_subject(false)
}

fn request_subject(include_source_carriers: bool) -> EdgeSplitScopeSubject {
    let carriers = source_carriers();
    let segment_pairs = segment_pair_receipt_from(&carriers);
    let ledger_carriers = include_source_carriers
        .then_some(carriers)
        .unwrap_or_default();
    let event_ledger = event_ledger_for(
        segment_pairs.segment_pair_enumeration_identity(),
        ledger_carriers,
    );
    let evidence = WorkloadEvidenceLedger::from_rows(vec![
        WorkloadEvidenceRow::from_boolean_evidence_receipt(&segment_pairs),
        WorkloadEvidenceRow::from_boolean_evidence_receipt(&event_ledger),
    ])
    .expect("receipt-backed evidence should index");
    let gate = PlanarBooleanCandidateIndexConsumptionGate::admit(
        PlanarBooleanCandidateIndexConsumptionInput::new(
            &event_ledger,
            &segment_pairs,
            evidence.stage_index(),
        ),
    )
    .expect("candidate-index consumption gate should admit");
    let event_ledger_lookup = evidence
        .require_boolean_receipt_lookup(&event_ledger)
        .expect("typed event-ledger lookup should admit");
    let request = PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
        &event_ledger,
        &gate,
        &event_ledger_lookup,
        None,
    ))
    .expect("split request should admit before scope classification");
    EdgeSplitScopeSubject { request }
}

fn segment_pair_receipt_from(
    carriers: &[PlanarBooleanSegmentCarrier],
) -> PlanarBooleanSegmentPairEnumerationReceipt {
    let left = carriers
        .iter()
        .filter(|carrier| carrier.operand_side() == PlanarBooleanCommonPlaneOperandSide::Left)
        .map(canonical_segment_from_carrier)
        .collect();
    let right = carriers
        .iter()
        .filter(|carrier| carrier.operand_side() == PlanarBooleanCommonPlaneOperandSide::Right)
        .map(canonical_segment_from_carrier)
        .collect();
    PlanarBooleanCanonicalSegmentSet::for_pair_enumeration_test(left, right)
        .segment_pair_enumeration_receipt()
        .expect("test segment pair enumeration should certify from Query candidate index product")
}

fn source_carriers() -> Vec<PlanarBooleanSegmentCarrier> {
    vec![
        segment_carrier(
            PlanarBooleanCommonPlaneOperandSide::Left,
            "left-edge-a",
            [0.0, 0.0],
            [0.0, 1.0],
        ),
        segment_carrier(
            PlanarBooleanCommonPlaneOperandSide::Left,
            "left-edge-b",
            [10.0, 0.0],
            [10.0, 1.0],
        ),
        segment_carrier(
            PlanarBooleanCommonPlaneOperandSide::Right,
            "right-edge-a",
            [0.0, 0.5],
            [0.0, 1.5],
        ),
        segment_carrier(
            PlanarBooleanCommonPlaneOperandSide::Right,
            "right-edge-b",
            [10.0, 0.5],
            [10.0, 1.5],
        ),
    ]
}

fn segment_carrier(
    side: PlanarBooleanCommonPlaneOperandSide,
    source_edge_identity: &str,
    start: [f64; 2],
    end: [f64; 2],
) -> PlanarBooleanSegmentCarrier {
    let carrier = PlanarBooleanSegmentCarrier::for_canonical_segment_test_on_side_with_source_edge(
        side,
        source_edge_identity,
        endpoint(start, "start"),
        endpoint(end, "end"),
    );
    validate_segment_endpoint_admissibility(&carrier).expect("test segment should be admissible");
    carrier
}

fn canonical_segment_from_carrier(
    carrier: &PlanarBooleanSegmentCarrier,
) -> PlanarBooleanCanonicalSegment {
    PlanarBooleanCanonicalSegment::from_carrier(carrier, normalize_endpoint_order(carrier))
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

fn event_ledger_for(
    segment_pair_enumeration_identity: &str,
    segment_carriers: Vec<PlanarBooleanSegmentCarrier>,
) -> PlanarBooleanEventLedgerReceipt {
    let ordered_events =
        PlanarBooleanOrderedEventSet::from_events_and_groups(&[], &[], &[], Vec::new());
    PlanarBooleanEventLedgerReceipt::new(PlanarBooleanEventLedgerReceiptInput {
        reduced_pair_identity: "reduced-pair".to_string(),
        event_extraction_request_identity: "event-extraction-request".to_string(),
        segment_carrier_set_identity: "segment-carrier-set".to_string(),
        segment_carriers,
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
        event_ledger_identity: format!("event-ledger:{segment_pair_enumeration_identity}"),
        downstream_consumption_identity: format!(
            "downstream-consumption:{segment_pair_enumeration_identity}"
        ),
    })
}
