use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_edge_splitting::event_participation_index::{
    PlanarBooleanSplitEventParticipationCounters, PlanarBooleanSplitEventParticipationIndex,
    PlanarBooleanSplitEventParticipationRow,
};
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanCollinearRelation, PlanarBooleanCollinearRelationKind, PlanarBooleanIntervalEvent,
    PlanarBooleanIntervalEventKind, PlanarBooleanNormalizedInterval, PlanarBooleanSourceInterval,
    PlanarBooleanSourceIntervalSense,
};

use super::PlanarBooleanIntervalSplitCandidateDenialKind;

#[test]
fn interval_split_candidate_extraction_rejects_missing_index_owned_event() {
    let index = participation_index_with_rows_and_events(
        vec![participation_row(
            "carrier",
            "source edge",
            vec!["missing interval event".to_string()],
        )],
        Vec::new(),
    );

    let denial = index
        .extract_interval_split_candidates()
        .expect_err("row-local interval event missing from index ownership must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanIntervalSplitCandidateDenialKind::MissingParticipationRow
    );
    assert_eq!(denial.evidence_identity(), "missing interval event");
    assert_eq!(denial.rejected_missing_participation_rows(), 1);
    assert_eq!(denial.rejected_missing_source_ranges(), 0);
}

#[test]
fn interval_split_candidate_extraction_rejects_missing_source_interval() {
    let event = interval_event(
        PlanarBooleanIntervalEventKind::PartialOverlap,
        [0.25, 0.75],
        "test-left-carrier",
        [0.25, 0.75],
        "test-right-carrier",
        [0.25, 0.75],
    );
    let event_identity = event.event_identity().to_string();
    let index = participation_index_with_rows_and_events(
        vec![participation_row(
            "foreign-carrier",
            "foreign source edge",
            vec![event_identity.clone()],
        )],
        vec![event],
    );

    let denial = index
        .extract_interval_split_candidates()
        .expect_err("row carrier without a matching source interval must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanIntervalSplitCandidateDenialKind::MissingSourceInterval
    );
    assert_eq!(denial.evidence_identity(), event_identity);
    assert_eq!(denial.rejected_missing_source_ranges(), 1);
}

#[test]
fn interval_split_candidate_extraction_rejects_source_interval_carrier_mismatch() {
    let event = interval_event(
        PlanarBooleanIntervalEventKind::PartialOverlap,
        [0.25, 0.75],
        "foreign-left-source-carrier",
        [0.25, 0.75],
        "test-right-carrier",
        [0.25, 0.75],
    );
    let event_identity = event.event_identity().to_string();
    let index = participation_index_with_rows_and_events(
        vec![participation_row(
            "test-left-carrier",
            "left source edge",
            vec![event_identity.clone()],
        )],
        vec![event],
    );

    let denial = index
        .extract_interval_split_candidates()
        .expect_err("event-side carrier/source-interval carrier mismatch must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanIntervalSplitCandidateDenialKind::MissingSourceInterval
    );
    assert_eq!(denial.evidence_identity(), event_identity);
    assert_eq!(denial.rejected_missing_source_ranges(), 1);
}

#[test]
fn interval_split_candidates_are_all_or_nothing_for_mixed_valid_and_invalid_rows() {
    let event = interval_event(
        PlanarBooleanIntervalEventKind::PartialOverlap,
        [0.25, 0.75],
        "test-left-carrier",
        [0.25, 0.75],
        "test-right-carrier",
        [0.25, 0.75],
    );
    let event_identity = event.event_identity().to_string();
    let index = participation_index_with_rows_and_events(
        vec![
            participation_row(
                "test-left-carrier",
                "left source edge",
                vec![event_identity.clone()],
            ),
            participation_row(
                "foreign-carrier",
                "foreign source edge",
                vec![event_identity.clone()],
            ),
        ],
        vec![event],
    );

    let denial = index
        .extract_interval_split_candidates()
        .expect_err("a poisoned interval row must deny the whole candidate set");

    assert_eq!(
        denial.kind(),
        PlanarBooleanIntervalSplitCandidateDenialKind::MissingSourceInterval
    );
    assert_eq!(denial.evidence_identity(), event_identity);
}

#[test]
fn anti_parallel_interval_candidate_preserves_opposite_source_sense() {
    let event = interval_event(
        PlanarBooleanIntervalEventKind::IdenticalAntiParallel,
        [0.0, 1.0],
        "test-left-carrier",
        [0.0, 1.0],
        "test-right-carrier",
        [1.0, 0.0],
    );
    let event_identity = event.event_identity().to_string();
    let index = participation_index_with_rows_and_events(
        vec![
            participation_row(
                "test-left-carrier",
                "left source edge",
                vec![event_identity.clone()],
            ),
            participation_row(
                "test-right-carrier",
                "right source edge",
                vec![event_identity],
            ),
        ],
        vec![event],
    );

    let candidates = index
        .extract_interval_split_candidates()
        .expect("anti-parallel interval event should lower to source-edge candidates");

    assert_eq!(candidates.candidates().len(), 2);
    assert!(candidates.candidates().iter().any(|candidate| {
        candidate.carrier_identity() == "test-right-carrier"
            && candidate.source_parameter_range() == [1.0, 0.0]
            && candidate.source_sense() == PlanarBooleanSourceIntervalSense::Reversed
            && candidate.normalized_parameter_range() == [0.0, 1.0]
            && candidate.event_group_identities() == &["event-group:row".to_string()]
    }));
}

#[test]
fn interval_split_candidates_preserve_distinct_interval_kinds() {
    for kind in [
        PlanarBooleanIntervalEventKind::PartialOverlap,
        PlanarBooleanIntervalEventKind::ContainmentOverlap,
        PlanarBooleanIntervalEventKind::IdenticalSameDirection,
        PlanarBooleanIntervalEventKind::IdenticalAntiParallel,
    ] {
        let event = interval_event(
            kind,
            [0.25, 0.75],
            "test-left-carrier",
            [0.25, 0.75],
            "test-right-carrier",
            [0.25, 0.75],
        );
        let event_identity = event.event_identity().to_string();
        let index = participation_index_with_rows_and_events(
            vec![participation_row(
                "test-left-carrier",
                "left source edge",
                vec![event_identity],
            )],
            vec![event],
        );

        let candidates = index
            .extract_interval_split_candidates()
            .expect("interval kind should lower without flattening");
        assert_eq!(candidates.candidates()[0].interval_event_kind(), kind);
    }
}

#[test]
fn interval_split_candidate_identity_is_stable_under_row_order_variation() {
    let event = interval_event(
        PlanarBooleanIntervalEventKind::PartialOverlap,
        [0.25, 0.75],
        "test-left-carrier",
        [0.25, 0.75],
        "test-right-carrier",
        [0.75, 0.25],
    );
    let event_identity = event.event_identity().to_string();
    let left_row = participation_row(
        "test-left-carrier",
        "left source edge",
        vec![event_identity.clone()],
    );
    let right_row = participation_row(
        "test-right-carrier",
        "right source edge",
        vec![event_identity],
    );
    let ordinary = participation_index_with_rows_and_events(
        vec![left_row.clone(), right_row.clone()],
        vec![event.clone()],
    );
    let reordered =
        participation_index_with_rows_and_events(vec![right_row, left_row], vec![event]);

    let ordinary_candidates = ordinary
        .extract_interval_split_candidates()
        .expect("ordinary row order should lower");
    let reordered_candidates = reordered
        .extract_interval_split_candidates()
        .expect("reordered row order should lower");

    assert_eq!(
        ordinary_candidates.candidate_set_identity(),
        reordered_candidates.candidate_set_identity()
    );
    assert_eq!(
        candidate_identities(&ordinary_candidates),
        candidate_identities(&reordered_candidates)
    );
}

#[test]
fn interval_split_candidate_identity_distinguishes_left_and_right_source_ranges() {
    let event = interval_event(
        PlanarBooleanIntervalEventKind::PartialOverlap,
        [0.25, 0.75],
        "test-left-carrier",
        [0.25, 0.75],
        "test-right-carrier",
        [0.75, 0.25],
    );
    let event_identity = event.event_identity().to_string();
    let index = participation_index_with_rows_and_events(
        vec![
            participation_row(
                "test-left-carrier",
                "left source edge",
                vec![event_identity.clone()],
            ),
            participation_row(
                "test-right-carrier",
                "right source edge",
                vec![event_identity],
            ),
        ],
        vec![event],
    );

    let candidates = index
        .extract_interval_split_candidates()
        .expect("left and right source ranges should lower");
    let identities = candidate_identities(&candidates);

    assert_eq!(identities.len(), 2);
    assert_ne!(identities[0], identities[1]);
}

fn candidate_identities(candidates: &super::PlanarBooleanIntervalSplitCandidateSet) -> Vec<String> {
    candidates
        .candidates()
        .iter()
        .map(|candidate| candidate.candidate_identity().to_string())
        .collect()
}

fn participation_index_with_rows_and_events(
    rows: Vec<PlanarBooleanSplitEventParticipationRow>,
    events: Vec<PlanarBooleanIntervalEvent>,
) -> PlanarBooleanSplitEventParticipationIndex {
    PlanarBooleanSplitEventParticipationIndex::new(
        "test participation index".to_string(),
        "test event ledger".to_string(),
        "test recovered carrier set".to_string(),
        rows,
        BTreeMap::new(),
        events
            .into_iter()
            .map(|event| (event.event_identity().to_string(), event))
            .collect(),
        PlanarBooleanSplitEventParticipationCounters::default(),
    )
}

fn participation_row(
    carrier_identity: &str,
    source_edge_identity: &str,
    interval_event_identities: Vec<String>,
) -> PlanarBooleanSplitEventParticipationRow {
    PlanarBooleanSplitEventParticipationRow::new(
        "test event ledger",
        carrier_identity,
        source_edge_identity,
        "start source endpoint",
        "start projected endpoint",
        "end source endpoint",
        "end projected endpoint",
        Vec::new(),
        interval_event_identities,
        vec!["event-group:row".to_string()],
    )
}

fn interval_event(
    kind: PlanarBooleanIntervalEventKind,
    normalized_range: [f64; 2],
    left_source_carrier_identity: &str,
    left_source_range: [f64; 2],
    right_source_carrier_identity: &str,
    right_source_range: [f64; 2],
) -> PlanarBooleanIntervalEvent {
    let relation =
        PlanarBooleanCollinearRelation::from_interval_event_test_parts(relation_kind(kind), None);
    PlanarBooleanIntervalEvent::new(
        kind,
        &relation,
        PlanarBooleanNormalizedInterval::new(
            normalized_range,
            relation.local_frame_identity(),
            relation.precision_basis_identity(),
        ),
        PlanarBooleanSourceInterval::new(
            relation.left_segment_identity(),
            left_source_carrier_identity,
            left_source_range,
        ),
        PlanarBooleanSourceInterval::new(
            relation.right_segment_identity(),
            right_source_carrier_identity,
            right_source_range,
        ),
    )
}

fn relation_kind(kind: PlanarBooleanIntervalEventKind) -> PlanarBooleanCollinearRelationKind {
    match kind {
        PlanarBooleanIntervalEventKind::PartialOverlap => {
            PlanarBooleanCollinearRelationKind::PartialOverlap
        }
        PlanarBooleanIntervalEventKind::ContainmentOverlap => {
            PlanarBooleanCollinearRelationKind::ContainmentOverlap
        }
        PlanarBooleanIntervalEventKind::IdenticalSameDirection => {
            PlanarBooleanCollinearRelationKind::IdenticalSameDirection
        }
        PlanarBooleanIntervalEventKind::IdenticalAntiParallel => {
            PlanarBooleanCollinearRelationKind::IdenticalAntiParallel
        }
    }
}
