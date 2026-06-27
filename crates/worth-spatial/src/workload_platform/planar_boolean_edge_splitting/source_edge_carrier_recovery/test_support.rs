use crate::workload_platform::evidence_ledger::{WorkloadEvidenceLedger, WorkloadEvidenceRow};
use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_edge_splitting::{
    event_ledger_lookup_execution_subject, PlanarBooleanCandidateIndexConsumptionGate,
    PlanarBooleanCandidateIndexConsumptionInput, PlanarBooleanEdgeSplitRequest,
    PlanarBooleanEdgeSplitRequestInput, PlanarBooleanEdgeSplitScopeAdmission,
    PlanarBooleanEdgeSplitScopeAdmissionInput,
};
use crate::workload_platform::planar_boolean_events::{
    normalize_endpoint_order, validate_segment_endpoint_admissibility,
    PlanarBooleanCanonicalSegment, PlanarBooleanCanonicalSegmentSet,
    PlanarBooleanCollinearRelation, PlanarBooleanCollinearRelationKind, PlanarBooleanEventGroup,
    PlanarBooleanEventGroupInput, PlanarBooleanEventGroupKind, PlanarBooleanEventLedgerCounters,
    PlanarBooleanEventLedgerReceipt, PlanarBooleanEventLedgerReceiptInput,
    PlanarBooleanIntervalEvent, PlanarBooleanIntervalEventKind, PlanarBooleanLoopRole,
    PlanarBooleanNormalizedInterval, PlanarBooleanOrderedEventSet, PlanarBooleanSegmentCarrier,
    PlanarBooleanSegmentCarrierEndpointFacts, PlanarBooleanSegmentCarrierInput,
    PlanarBooleanSegmentPairEnumerationReceipt, PlanarBooleanSourceInterval,
};

use super::{
    PlanarBooleanSplitSourceEdgeCarrierRecoveryInput, PlanarBooleanSplitSourceEdgeCarrierSet,
};

pub(crate) struct SourceEdgeCarrierRecoverySubject {
    pub(crate) segment_pairs: PlanarBooleanSegmentPairEnumerationReceipt,
    pub(crate) ledger: PlanarBooleanEventLedgerReceipt,
    pub(crate) request: PlanarBooleanEdgeSplitRequest,
    pub(crate) scope: PlanarBooleanEdgeSplitScopeAdmission,
}

pub(crate) fn recover(
    subject: &SourceEdgeCarrierRecoverySubject,
) -> PlanarBooleanSplitSourceEdgeCarrierSet {
    PlanarBooleanSplitSourceEdgeCarrierSet::recover(
        PlanarBooleanSplitSourceEdgeCarrierRecoveryInput::from_scope_and_event_ledger(
            &subject.scope,
            &subject.ledger,
        ),
    )
    .expect("source-edge carrier recovery should admit")
}

pub(crate) fn subject_with_carriers(
    carriers: Vec<PlanarBooleanSegmentCarrier>,
) -> SourceEdgeCarrierRecoverySubject {
    let segment_pairs = production_segment_pair_receipt(&carriers);
    let ledger = event_ledger_for(
        segment_pairs.segment_pair_enumeration_identity(),
        carriers,
        Vec::new(),
        "event-ledger",
    );
    subject_with_ledger(ledger)
}

pub(crate) fn subject_with_ledger(
    ledger: PlanarBooleanEventLedgerReceipt,
) -> SourceEdgeCarrierRecoverySubject {
    let segment_pairs = production_segment_pair_receipt(ledger.segment_carriers());
    let evidence = WorkloadEvidenceLedger::from_rows(vec![
        WorkloadEvidenceRow::from_boolean_evidence_receipt(&segment_pairs),
        WorkloadEvidenceRow::from_boolean_evidence_receipt(&ledger),
    ])
    .expect("receipt-backed evidence should index");
    let gate = PlanarBooleanCandidateIndexConsumptionGate::admit(
        PlanarBooleanCandidateIndexConsumptionInput::new(
            &ledger,
            &segment_pairs,
            evidence.stage_index(),
        ),
    )
    .expect("candidate-index gate should admit");
    let event_ledger_lookup = event_ledger_lookup_execution_subject(
        "source-edge-recovery",
        &ledger,
        vec![
            WorkloadEvidenceRow::from_boolean_evidence_receipt(&segment_pairs),
            WorkloadEvidenceRow::from_boolean_evidence_receipt(&ledger),
        ],
    );
    let request = PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
        &ledger,
        &gate,
        &event_ledger_lookup.witness,
        None,
    ))
    .expect("split request should admit");
    let scope = PlanarBooleanEdgeSplitScopeAdmission::admit(
        PlanarBooleanEdgeSplitScopeAdmissionInput::from_split_request(&request),
    )
    .expect("scope should admit");
    SourceEdgeCarrierRecoverySubject {
        segment_pairs,
        ledger,
        request,
        scope,
    }
}

pub(crate) fn source_carriers() -> Vec<PlanarBooleanSegmentCarrier> {
    vec![
        carrier(PlanarBooleanCommonPlaneOperandSide::Left, "left-edge-a"),
        carrier(PlanarBooleanCommonPlaneOperandSide::Left, "left-edge-b"),
        carrier(PlanarBooleanCommonPlaneOperandSide::Right, "right-edge-a"),
        carrier(PlanarBooleanCommonPlaneOperandSide::Right, "right-edge-b"),
    ]
}

pub(crate) fn carrier(
    side: PlanarBooleanCommonPlaneOperandSide,
    source_edge_identity: &str,
) -> PlanarBooleanSegmentCarrier {
    let carrier = PlanarBooleanSegmentCarrier::for_canonical_segment_test_on_side_with_source_edge(
        side,
        source_edge_identity,
        endpoint([0.0, 0.0], "start"),
        endpoint([1.0, 0.0], "end"),
    );
    validate_segment_endpoint_admissibility(&carrier).expect("test segment should be admissible");
    carrier
}

pub(crate) fn carrier_with_source_edge(source_edge_identity: &str) -> PlanarBooleanSegmentCarrier {
    carrier_with_provenance(PlanarBooleanSegmentCarrierInput {
        operand_side: PlanarBooleanCommonPlaneOperandSide::Left,
        source_face_identity: "test face".to_string(),
        source_loop_identity: "test loop".to_string(),
        source_edge_identity: source_edge_identity.to_string(),
        loop_role: PlanarBooleanLoopRole::OuterBoundary,
        start: endpoint([0.0, 0.0], "start"),
        end: endpoint([1.0, 0.0], "end"),
        local_frame_identity: "test local frame".to_string(),
        projection_stage_identity: "test projection stage".to_string(),
        precision_basis_identity: "test precision basis".to_string(),
    })
}

pub(crate) fn carrier_with_provenance(
    input: PlanarBooleanSegmentCarrierInput,
) -> PlanarBooleanSegmentCarrier {
    PlanarBooleanSegmentCarrier::new(input)
}

pub(crate) fn carrier_input_with_all_provenance() -> PlanarBooleanSegmentCarrierInput {
    PlanarBooleanSegmentCarrierInput {
        operand_side: PlanarBooleanCommonPlaneOperandSide::Left,
        source_face_identity: "test face".to_string(),
        source_loop_identity: "test loop".to_string(),
        source_edge_identity: "test source edge".to_string(),
        loop_role: PlanarBooleanLoopRole::OuterBoundary,
        start: endpoint([0.0, 0.0], "start"),
        end: endpoint([1.0, 0.0], "end"),
        local_frame_identity: "test local frame".to_string(),
        projection_stage_identity: "test projection stage".to_string(),
        precision_basis_identity: "test precision basis".to_string(),
    }
}

pub(crate) fn production_segment_pair_receipt(
    carriers: &[PlanarBooleanSegmentCarrier],
) -> PlanarBooleanSegmentPairEnumerationReceipt {
    let left = carriers
        .iter()
        .filter(|carrier| carrier.operand_side() == PlanarBooleanCommonPlaneOperandSide::Left)
        .map(canonical_segment)
        .collect();
    let right = carriers
        .iter()
        .filter(|carrier| carrier.operand_side() == PlanarBooleanCommonPlaneOperandSide::Right)
        .map(canonical_segment)
        .collect();
    PlanarBooleanCanonicalSegmentSet::for_pair_enumeration_test(left, right)
        .segment_pair_enumeration_receipt()
        .expect("indexed pair enumeration should certify")
}

pub(crate) fn event_ledger_for(
    segment_pair_enumeration_identity: &str,
    segment_carriers: Vec<PlanarBooleanSegmentCarrier>,
    event_groups: Vec<PlanarBooleanEventGroup>,
    event_ledger_identity: &str,
) -> PlanarBooleanEventLedgerReceipt {
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
        ordered_events: PlanarBooleanOrderedEventSet::from_events_and_groups(
            &[],
            &[],
            &event_groups,
            Vec::new(),
        ),
        event_groups,
        counters: PlanarBooleanEventLedgerCounters::default(),
        event_ledger_identity: event_ledger_identity.to_string(),
        downstream_consumption_identity: "downstream-consumption".to_string(),
    })
}

pub(crate) fn event_ledger_with_interval_event(
    segment_pair_enumeration_identity: &str,
    segment_carriers: Vec<PlanarBooleanSegmentCarrier>,
    interval_event: PlanarBooleanIntervalEvent,
    event_ledger_identity: &str,
) -> PlanarBooleanEventLedgerReceipt {
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
        interval_events: vec![interval_event],
        relation_diagnostics: Vec::new(),
        ordered_events: PlanarBooleanOrderedEventSet::from_events_and_groups(
            &[],
            &[],
            &[],
            Vec::new(),
        ),
        event_groups: Vec::new(),
        counters: PlanarBooleanEventLedgerCounters::default(),
        event_ledger_identity: event_ledger_identity.to_string(),
        downstream_consumption_identity: "downstream-consumption".to_string(),
    })
}

pub(crate) fn interval_event_with_unknown_relation_carriers() -> PlanarBooleanIntervalEvent {
    let relation = PlanarBooleanCollinearRelation::from_interval_event_test_parts(
        PlanarBooleanCollinearRelationKind::PartialOverlap,
        None,
    );
    PlanarBooleanIntervalEvent::new(
        PlanarBooleanIntervalEventKind::PartialOverlap,
        &relation,
        PlanarBooleanNormalizedInterval::new([0.2, 0.8], "test-local-frame", "test-precision"),
        PlanarBooleanSourceInterval::new("test-left-segment", "test-left-carrier", [0.2, 0.8]),
        PlanarBooleanSourceInterval::new("test-right-segment", "test-right-carrier", [0.2, 0.8]),
    )
}

pub(crate) fn group_with_carrier(
    group_identity: &str,
    carrier_identity: &str,
) -> PlanarBooleanEventGroup {
    PlanarBooleanEventGroup::new(PlanarBooleanEventGroupInput {
        group_identity: group_identity.to_string(),
        kind: PlanarBooleanEventGroupKind::CoincidentPoint,
        canonical_group_key: format!("{group_identity}:key"),
        point_event_identities: Vec::new(),
        interval_event_identities: Vec::new(),
        segment_pair_identities: Vec::new(),
        participating_carrier_identities: vec![carrier_identity.to_string()],
        source_endpoint_identities: Vec::new(),
        source_interval_identities: Vec::new(),
    })
}

fn canonical_segment(carrier: &PlanarBooleanSegmentCarrier) -> PlanarBooleanCanonicalSegment {
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
