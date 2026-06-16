use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_edge_splitting::event_participation_index::{
    PlanarBooleanSplitEventParticipationCounters, PlanarBooleanSplitEventParticipationIndex,
    PlanarBooleanSplitEventParticipationRow,
};
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanPointEvent, PlanarBooleanPointEventCoordinateFact, PlanarBooleanPointEventKind,
    PlanarBooleanPointEventSegmentParameterFact,
};

use super::{PlanarBooleanPointSplitCandidateDenialKind, PlanarBooleanPointSplitCandidateSet};

#[test]
fn point_split_candidate_extraction_rejects_missing_index_owned_event() {
    let index = PlanarBooleanSplitEventParticipationIndex::new(
        "test participation index".to_string(),
        "test event ledger".to_string(),
        "test recovered carrier set".to_string(),
        vec![PlanarBooleanSplitEventParticipationRow::new(
            "test event ledger",
            "carrier",
            "source edge",
            "start source endpoint",
            "start projected endpoint",
            "end source endpoint",
            "end projected endpoint",
            vec!["missing point event".to_string()],
            Vec::new(),
            Vec::new(),
        )],
        BTreeMap::new(),
        BTreeMap::new(),
        PlanarBooleanSplitEventParticipationCounters::default(),
    );

    let denial = index
        .extract_point_split_candidates()
        .expect_err("row-local point event missing from index ownership must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanPointSplitCandidateDenialKind::MissingParticipationRow
    );
    assert_eq!(denial.evidence_identity(), "missing point event");
}

#[test]
fn point_split_candidate_extraction_skips_retained_participant_without_parameter() {
    let event = point_event_with_participating_carriers(
        "event with retained third carrier",
        [
            "parameter-carrier-a",
            "parameter-carrier-b",
            "participating-carrier-without-parameter",
        ],
    );
    let index =
        index_for_row_and_point_events("participating-carrier-without-parameter", vec![event]);

    let candidates = index
        .extract_point_split_candidates()
        .expect("retained participants without parameter facts are not split candidates");

    assert!(candidates.candidates().is_empty());
    assert_eq!(candidates.counters().inspected_point_events(), 1);
    assert_eq!(candidates.counters().emitted_point_candidates(), 0);
}

#[test]
fn point_split_candidates_preserve_event_kind_coordinate_and_parameter_facts() {
    let event = point_event_with_participating_carriers(
        "event with bound carrier",
        ["carrier-a", "carrier-b"],
    );
    let expected_event_identity = event.event_identity().to_string();
    let expected_kind = event.kind();
    let expected_coordinate_fact = event.coordinate_fact().clone();
    let expected_parameter_fact_identity = event
        .operand_a_parameter()
        .parameter_fact_identity()
        .to_string();
    let expected_segment_identity = event.operand_a_parameter().segment_identity().to_string();
    let expected_parameter = event.operand_a_parameter().parameter();
    let index = index_for_row_and_point_events("carrier-a", vec![event]);

    let candidates = index
        .extract_point_split_candidates()
        .expect("bound carrier parameter should lower into one point split candidate");

    assert_eq!(candidates.counters().inspected_point_events(), 1);
    assert_eq!(candidates.counters().emitted_point_candidates(), 1);
    let candidate = candidates
        .candidates()
        .first()
        .expect("one candidate should be emitted for the bound row carrier");
    assert_eq!(candidate.point_event_identity(), expected_event_identity);
    assert_eq!(candidate.point_event_kind(), expected_kind);
    assert_eq!(candidate.coordinate_fact(), &expected_coordinate_fact);
    assert_eq!(
        candidate.parameter_fact_identity(),
        expected_parameter_fact_identity
    );
    assert_eq!(candidate.segment_identity(), expected_segment_identity);
    assert_eq!(candidate.parameter(), expected_parameter);
    assert_eq!(candidate.carrier_identity(), "carrier-a");
    assert_eq!(candidate.source_edge_identity(), "source edge");
    assert_eq!(
        candidate.event_group_identities(),
        &["event-group:row".to_string()]
    );
}

#[test]
fn point_split_candidate_extraction_rejects_missing_carrier_parameter() {
    let event = point_event(
        "event with unauthorized carrier parameter",
        "parameter-carrier-not-retained",
        "retained-carrier",
        ["retained-carrier"],
    );
    let index = index_for_row_and_point_events("parameter-carrier-not-retained", vec![event]);

    let denial = index
        .extract_point_split_candidates()
        .expect_err("carrier parameter without participation authority must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanPointSplitCandidateDenialKind::MissingCarrierParameter
    );
    assert_eq!(denial.rejected_missing_parameter_facts(), 1);
}

#[test]
fn point_split_candidate_extraction_rejects_conflicting_same_carrier_parameters() {
    let event = point_event(
        "event with conflicting same-carrier parameters",
        "shared-carrier",
        "shared-carrier",
        ["shared-carrier"],
    );
    let index = index_for_row_and_point_events("shared-carrier", vec![event]);

    let denial = index
        .extract_point_split_candidates()
        .expect_err("two different parameter facts for one carrier must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanPointSplitCandidateDenialKind::ConflictingCarrierParameterFacts
    );
    assert_eq!(denial.rejected_missing_parameter_facts(), 0);
    assert_eq!(denial.rejected_conflicting_parameter_facts(), 1);
}

#[test]
fn point_split_candidates_are_stable_under_point_event_order_variation() {
    let first =
        point_event_with_participating_carriers("first point event", ["carrier-a", "carrier-b"]);
    let second =
        point_event_with_participating_carriers("second point event", ["carrier-a", "carrier-c"]);
    let ordinary = index_for_row_and_point_events("carrier-a", vec![first.clone(), second.clone()]);
    let reversed = index_for_row_and_point_events("carrier-a", vec![second, first]);

    let ordinary_candidates = ordinary
        .extract_point_split_candidates()
        .expect("ordinary point candidates should extract");
    let reversed_candidates = reversed
        .extract_point_split_candidates()
        .expect("reordered point candidates should extract");

    assert_eq!(
        candidate_identities(&ordinary_candidates),
        candidate_identities(&reversed_candidates)
    );
    assert_eq!(
        ordinary_candidates.candidate_set_identity(),
        reversed_candidates.candidate_set_identity()
    );
}

fn candidate_identities(candidates: &PlanarBooleanPointSplitCandidateSet) -> Vec<String> {
    candidates
        .candidates()
        .iter()
        .map(|candidate| candidate.candidate_identity().to_string())
        .collect()
}

fn index_for_row_and_point_events(
    row_carrier_identity: &str,
    point_events: Vec<PlanarBooleanPointEvent>,
) -> PlanarBooleanSplitEventParticipationIndex {
    let point_event_identities = point_events
        .iter()
        .map(|event| event.event_identity().to_string())
        .collect::<Vec<_>>();
    let point_events_by_identity = point_events
        .into_iter()
        .map(|event| (event.event_identity().to_string(), event))
        .collect::<BTreeMap<_, _>>();
    PlanarBooleanSplitEventParticipationIndex::new(
        "test participation index".to_string(),
        "test event ledger".to_string(),
        "test recovered carrier set".to_string(),
        vec![PlanarBooleanSplitEventParticipationRow::new(
            "test event ledger",
            row_carrier_identity,
            "source edge",
            "start source endpoint",
            "start projected endpoint",
            "end source endpoint",
            "end projected endpoint",
            point_event_identities,
            Vec::new(),
            vec!["event-group:row".to_string()],
        )],
        point_events_by_identity,
        BTreeMap::new(),
        PlanarBooleanSplitEventParticipationCounters::default(),
    )
}

fn point_event_with_participating_carriers<const N: usize>(
    label: &str,
    participating_carriers: [&str; N],
) -> PlanarBooleanPointEvent {
    point_event(
        label,
        participating_carriers[0],
        participating_carriers[1],
        participating_carriers,
    )
}

fn point_event<const N: usize>(
    label: &str,
    operand_a_carrier: &str,
    operand_b_carrier: &str,
    participating_carriers: [&str; N],
) -> PlanarBooleanPointEvent {
    PlanarBooleanPointEvent::for_split_candidate_test(
        PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        PlanarBooleanPointEventCoordinateFact::new(
            coordinate_for(label),
            "test local frame",
            "test precision",
        ),
        PlanarBooleanPointEventSegmentParameterFact::new(
            &format!("{label}:segment-a"),
            operand_a_carrier,
            0.25,
        ),
        PlanarBooleanPointEventSegmentParameterFact::new(
            &format!("{label}:segment-b"),
            operand_b_carrier,
            0.75,
        ),
        participating_carriers
            .iter()
            .map(|carrier| carrier.to_string())
            .collect(),
        Vec::new(),
        Vec::new(),
    )
}

fn coordinate_for(label: &str) -> [f64; 2] {
    let offset = label.bytes().fold(0_u8, |acc, byte| acc.wrapping_add(byte)) as f64;
    [offset / 255.0, offset / 510.0]
}
