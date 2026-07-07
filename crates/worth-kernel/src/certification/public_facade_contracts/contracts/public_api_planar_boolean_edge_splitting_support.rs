use super::metaboss_support::MetabossEventExtractionSubject;
use std::collections::BTreeMap;
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanAdmittedIntervalSplitCandidateSet, PlanarBooleanAdmittedPointSplitCandidateSet,
    PlanarBooleanCandidateIndexConsumptionGate, PlanarBooleanCandidateIndexConsumptionInput,
    PlanarBooleanEdgeSplitRequest, PlanarBooleanEdgeSplitRequestInput,
    PlanarBooleanEdgeSplitScopeAdmission, PlanarBooleanEdgeSplitScopeAdmissionInput,
    PlanarBooleanIntervalSplitCandidateSet, PlanarBooleanPointSplitCandidateSet,
    PlanarBooleanSplitPointEndpointPosture, PlanarBooleanSplitSourceEdgeCarrierRecoveryInput,
    PlanarBooleanSplitSourceEdgeCarrierSet,
};
use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanEventLedgerReceipt, PlanarBooleanIntervalEventKind, PlanarBooleanPointEventKind,
    PlanarBooleanSourceIntervalSense,
};
use worth_spatial::facade::workload_vocabulary::{WorkloadEvidenceLedger, WorkloadEvidenceRow};

pub(crate) fn recovered_carriers_for(
    subject: &MetabossEventExtractionSubject,
) -> PlanarBooleanSplitSourceEdgeCarrierSet {
    let segment_pairs = &subject.inputs().pair_worklist;
    let ledger = subject.ledger();
    let workload = subject.pair().left().workload();
    let evidence = WorkloadEvidenceLedger::from_rows(vec![
        WorkloadEvidenceRow::from_boolean_evidence_receipt(segment_pairs),
        WorkloadEvidenceRow::from_boolean_evidence_receipt(ledger),
    ])
    .expect("metaboss boolean receipts should build an indexed evidence product");
    let gate = PlanarBooleanCandidateIndexConsumptionGate::admit(
        PlanarBooleanCandidateIndexConsumptionInput::new(
            ledger,
            segment_pairs,
            evidence.stage_index(),
        ),
    )
    .expect("candidate-index gate should admit before split request");
    let event_ledger_lookup = workload
        .require_boolean_event_ledger_lookup_execution_packet(ledger)
        .expect("execution-backed event-ledger lookup packet should admit before split request");
    let request = PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
        ledger,
        &gate,
        event_ledger_lookup.witness(),
        None,
    ))
    .expect("split request should admit from event ledger and candidate-index gate");
    let scope = PlanarBooleanEdgeSplitScopeAdmission::admit(
        PlanarBooleanEdgeSplitScopeAdmissionInput::from_split_request(&request),
    )
    .expect("split scope should admit before source-edge carrier recovery");
    PlanarBooleanSplitSourceEdgeCarrierSet::recover(
        PlanarBooleanSplitSourceEdgeCarrierRecoveryInput::from_scope_and_event_ledger(
            &scope, ledger,
        ),
    )
    .expect("source-edge carriers should recover from scoped metaboss ledger")
}

pub(crate) fn expected_point_candidate_facts(
    ledger: &PlanarBooleanEventLedgerReceipt,
) -> BTreeMap<(String, String), ExpectedPointCandidateFact> {
    let mut expected = BTreeMap::new();
    for event in ledger.point_events() {
        for parameter in [event.operand_a_parameter(), event.operand_b_parameter()] {
            expected.insert(
                (
                    event.event_identity().to_string(),
                    parameter.carrier_identity().to_string(),
                ),
                ExpectedPointCandidateFact {
                    point_event_kind: event.kind(),
                    coordinate_fact_identity: event
                        .coordinate_fact()
                        .coordinate_fact_identity()
                        .to_string(),
                    parameter_fact_identity: parameter.parameter_fact_identity().to_string(),
                    segment_identity: parameter.segment_identity().to_string(),
                    parameter: parameter.parameter(),
                },
            );
        }
    }
    expected
}

pub(crate) struct ExpectedPointCandidateFact {
    pub(crate) point_event_kind: PlanarBooleanPointEventKind,
    pub(crate) coordinate_fact_identity: String,
    pub(crate) parameter_fact_identity: String,
    pub(crate) segment_identity: String,
    pub(crate) parameter: f64,
}

pub(crate) fn expected_interval_candidate_facts(
    ledger: &PlanarBooleanEventLedgerReceipt,
    recovered_carriers: &PlanarBooleanSplitSourceEdgeCarrierSet,
) -> BTreeMap<(String, String), ExpectedIntervalCandidateFact> {
    let mut expected = BTreeMap::new();
    let source_edges_by_carrier = recovered_source_edges_by_carrier(recovered_carriers);
    for event in ledger.interval_events() {
        for source_interval in [event.left_source_interval(), event.right_source_interval()] {
            let source_edge_identity = source_edges_by_carrier
                .get(source_interval.carrier_identity())
                .expect("ledger interval source carrier must be recovered before candidate proof")
                .clone();
            expected.insert(
                (
                    event.event_identity().to_string(),
                    source_interval.carrier_identity().to_string(),
                ),
                ExpectedIntervalCandidateFact {
                    interval_event_kind: event.kind(),
                    source_edge_identity,
                    source_interval_identity: source_interval
                        .source_interval_identity()
                        .to_string(),
                    source_parameter_range: source_interval.source_parameter_range(),
                    source_sense: source_interval.sense(),
                    segment_identity: source_interval.segment_identity().to_string(),
                    normalized_interval_identity: event
                        .normalized_interval()
                        .normalized_interval_identity()
                        .to_string(),
                    normalized_parameter_range: event.normalized_interval().parameter_range(),
                    local_frame_identity: event.local_frame_identity().to_string(),
                    precision_basis_identity: event.precision_basis_identity().to_string(),
                },
            );
        }
    }
    expected
}

pub(crate) struct ExpectedIntervalCandidateFact {
    pub(crate) interval_event_kind: PlanarBooleanIntervalEventKind,
    pub(crate) source_edge_identity: String,
    pub(crate) source_interval_identity: String,
    pub(crate) source_parameter_range: [f64; 2],
    pub(crate) source_sense: PlanarBooleanSourceIntervalSense,
    pub(crate) segment_identity: String,
    pub(crate) normalized_interval_identity: String,
    pub(crate) normalized_parameter_range: [f64; 2],
    pub(crate) local_frame_identity: String,
    pub(crate) precision_basis_identity: String,
}

fn read_expected_point_candidate_fact_fields(fact: &ExpectedPointCandidateFact) {
    let _ = (
        fact.point_event_kind,
        &fact.coordinate_fact_identity,
        &fact.parameter_fact_identity,
        &fact.segment_identity,
        fact.parameter,
    );
}

pub(crate) fn assert_interval_candidates_match_ledger_facts(
    candidates: &PlanarBooleanIntervalSplitCandidateSet,
    expected_candidate_facts: &BTreeMap<(String, String), ExpectedIntervalCandidateFact>,
) {
    assert_eq!(
        candidates.candidates().len(),
        expected_candidate_facts.len()
    );
    for candidate in candidates.candidates() {
        let expected = expected_candidate_facts
            .get(&(
                candidate.interval_event_identity().to_string(),
                candidate.carrier_identity().to_string(),
            ))
            .expect("every interval split candidate must bind a ledger source interval");
        assert_eq!(
            candidate.interval_event_kind(),
            expected.interval_event_kind
        );
        assert_eq!(
            candidate.source_edge_identity(),
            expected.source_edge_identity
        );
        assert_eq!(
            candidate.source_interval_identity(),
            expected.source_interval_identity
        );
        assert_eq!(
            candidate.source_parameter_range(),
            expected.source_parameter_range
        );
        assert_eq!(candidate.source_sense(), expected.source_sense);
        assert_eq!(candidate.segment_identity(), expected.segment_identity);
        assert_eq!(
            candidate.normalized_interval_identity(),
            expected.normalized_interval_identity
        );
        assert_eq!(
            candidate.normalized_parameter_range(),
            expected.normalized_parameter_range
        );
        assert_eq!(
            candidate.local_frame_identity(),
            expected.local_frame_identity
        );
        assert_eq!(
            candidate.precision_basis_identity(),
            expected.precision_basis_identity
        );
    }
}

fn recovered_source_edges_by_carrier(
    recovered_carriers: &PlanarBooleanSplitSourceEdgeCarrierSet,
) -> BTreeMap<String, String> {
    recovered_carriers
        .carriers()
        .iter()
        .map(|carrier| {
            (
                carrier.carrier_identity().to_string(),
                carrier.source_edge_identity().to_string(),
            )
        })
        .collect()
}

pub(crate) fn assert_point_domain_admission_matches_candidates(
    candidates: &PlanarBooleanPointSplitCandidateSet,
    admitted: &PlanarBooleanAdmittedPointSplitCandidateSet,
) {
    assert_eq!(
        admitted.point_candidate_set_identity(),
        candidates.candidate_set_identity()
    );
    assert_eq!(
        admitted.counters().admitted_point_candidates(),
        candidates.candidates().len()
    );
    assert_eq!(
        admitted.counters().inspected_point_candidates(),
        candidates.candidates().len()
    );
    assert_eq!(admitted.counters().rejected_out_of_domain_points(), 0);
    assert_eq!(
        admitted.counters().endpoint_candidates() + admitted.counters().interior_candidates(),
        admitted.counters().admitted_point_candidates()
    );
    assert!(admitted.counters().endpoint_candidates() > 0);
    assert!(admitted.counters().interior_candidates() > 0);
    for admitted_candidate in admitted.admitted_candidates() {
        let candidate = admitted_candidate.candidate();
        match admitted_candidate.endpoint_posture() {
            PlanarBooleanSplitPointEndpointPosture::StartEndpoint => {
                assert_eq!(
                    admitted_candidate.exact_endpoint_source_identity(),
                    Some(candidate.start_source_endpoint_identity())
                );
                assert_eq!(
                    admitted_candidate.exact_projected_endpoint_fact_identity(),
                    Some(candidate.start_projected_endpoint_fact_identity())
                );
            }
            PlanarBooleanSplitPointEndpointPosture::EndEndpoint => {
                assert_eq!(
                    admitted_candidate.exact_endpoint_source_identity(),
                    Some(candidate.end_source_endpoint_identity())
                );
                assert_eq!(
                    admitted_candidate.exact_projected_endpoint_fact_identity(),
                    Some(candidate.end_projected_endpoint_fact_identity())
                );
            }
            PlanarBooleanSplitPointEndpointPosture::Interior => {
                assert_eq!(admitted_candidate.exact_endpoint_source_identity(), None);
                assert_eq!(
                    admitted_candidate.exact_projected_endpoint_fact_identity(),
                    None
                );
            }
        }
    }
}

pub(crate) fn assert_interval_domain_admission_matches_candidates(
    candidates: &PlanarBooleanIntervalSplitCandidateSet,
    admitted: &PlanarBooleanAdmittedIntervalSplitCandidateSet,
) {
    assert_eq!(
        admitted.interval_candidate_set_identity(),
        candidates.candidate_set_identity()
    );
    assert_eq!(
        admitted.counters().inspected_interval_candidates(),
        candidates.candidates().len()
    );
    assert_eq!(
        admitted.counters().admitted_interval_candidates(),
        candidates.candidates().len()
    );
    assert_eq!(admitted.counters().collapsed_interval_denials(), 0);
    assert_eq!(admitted.counters().rejected_non_finite_intervals(), 0);
    assert_eq!(admitted.counters().rejected_out_of_domain_intervals(), 0);
    assert_eq!(
        admitted.counters().rejected_contradictory_sense_intervals(),
        0
    );
    assert_eq!(
        admitted.admitted_candidates().len(),
        candidates.candidates().len()
    );
    let mut observed_reversed_interval = false;
    for (candidate, admitted_candidate) in candidates
        .candidates()
        .iter()
        .zip(admitted.admitted_candidates())
    {
        assert_eq!(
            admitted_candidate.candidate().candidate_identity(),
            candidate.candidate_identity()
        );
        assert_eq!(
            admitted_candidate.admitted_parameter_range(),
            ordered_interval_parameter_range(candidate.source_parameter_range())
        );
        if candidate.source_sense() == PlanarBooleanSourceIntervalSense::Reversed {
            assert!(
                candidate.source_parameter_range()[0] > candidate.source_parameter_range()[1],
                "reversed interval source range should remain descending"
            );
            assert!(
                admitted_candidate.admitted_parameter_range()[0]
                    < admitted_candidate.admitted_parameter_range()[1],
                "admitted interval range should be ordered for execution"
            );
            observed_reversed_interval = true;
        }
    }
    assert!(
        observed_reversed_interval,
        "metaboss interval admission should prove a reversed source interval"
    );
}

fn ordered_interval_parameter_range(source_parameter_range: [f64; 2]) -> [f64; 2] {
    if source_parameter_range[0] < source_parameter_range[1] {
        source_parameter_range
    } else {
        [source_parameter_range[1], source_parameter_range[0]]
    }
}

const _: () = {
    let _ = recovered_carriers_for;
    let _ = expected_point_candidate_facts;
    let _ = expected_interval_candidate_facts;
    let _ = assert_interval_candidates_match_ledger_facts;
    let _ = assert_point_domain_admission_matches_candidates;
    let _ = assert_interval_domain_admission_matches_candidates;
    let _: Option<ExpectedPointCandidateFact> = None;
    let _: Option<ExpectedIntervalCandidateFact> = None;
    let _ = read_expected_point_candidate_fact_fields;
};
