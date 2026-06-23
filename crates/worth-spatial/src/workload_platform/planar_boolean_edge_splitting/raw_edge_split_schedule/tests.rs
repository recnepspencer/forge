use crate::workload_platform::planar_boolean_edge_splitting::interval_parameter_admission::{
    PlanarBooleanAdmittedIntervalSplitCandidateSet, PlanarBooleanSplitIntervalAdmissionCounters,
};
use crate::workload_platform::planar_boolean_edge_splitting::proof_chain_support::{
    empty_intervals, interval_with_event_identity, interval_with_range, point_candidate,
    point_candidate_set, point_candidate_with_source_edge,
};
use crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::assembly::{
    reject_mixed_source_edges, SourceEdgeScheduleKey,
};
use crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::schedule::{
    PlanarBooleanRawEdgeSplitScheduleEntry, PlanarBooleanRawEdgeSplitScheduleEntryKind,
};
use crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::{
    PlanarBooleanRawEdgeSplitScheduleDenialKind, PlanarBooleanRawEdgeSplitScheduleSet,
    PlanarBooleanRawPointEndpointAuthority,
};
use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanPointSplitPosture;
use crate::workload_platform::planar_boolean_events::PlanarBooleanPointEventKind;

#[test]
fn per_edge_split_schedule_groups_candidates_by_source_edge() {
    let postures = point_candidate_set(vec![
        point_candidate(
            "left",
            "event:left",
            "carrier:a",
            0.25,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        ),
        point_candidate(
            "right",
            "event:right",
            "carrier:b",
            0.75,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        ),
    ])
    .admit_parameter_domain()
    .expect("point candidates should admit")
    .classify_point_split_postures()
    .expect("point postures should classify");

    let raw = PlanarBooleanRawEdgeSplitScheduleSet::assemble_from_admitted_candidates(
        &postures,
        &interval_with_range([0.1, 0.2]),
    )
    .expect("same participation-index candidates should assemble");

    assert_eq!(raw.counters().source_edge_schedules(), 2);
    assert!(raw.schedules().iter().all(|schedule| schedule
        .entries()
        .iter()
        .all(|entry| { entry.source_edge_identity() == schedule.source_edge_identity() })));
}

#[test]
fn per_edge_split_schedule_does_not_merge_distinct_carriers_with_same_source_edge_label() {
    let postures = point_candidate_set(vec![
        point_candidate_with_source_edge("left", "carrier:a", "shared source edge", 0.25),
        point_candidate_with_source_edge("right", "carrier:b", "shared source edge", 0.75),
    ])
    .admit_parameter_domain()
    .expect("point candidates should admit")
    .classify_point_split_postures()
    .expect("point postures should classify");

    let raw = PlanarBooleanRawEdgeSplitScheduleSet::assemble_from_admitted_candidates(
        &postures,
        &empty_intervals(),
    )
    .expect("distinct source-edge carriers should assemble separately");

    assert_eq!(raw.counters().source_edge_schedules(), 2);
    assert!(raw.schedules().iter().all(|schedule| {
        schedule.source_edge_identity() == "shared source edge"
            && schedule
                .entries()
                .iter()
                .all(|entry| entry.carrier_identity() == schedule.carrier_identity())
    }));
}

#[test]
fn per_edge_split_schedule_rejects_foreign_candidate_sets() {
    let postures = point_candidate_set(vec![point_candidate(
        "left",
        "event:left",
        "carrier:a",
        0.25,
        PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
    )])
    .admit_parameter_domain()
    .expect("point candidates should admit")
    .classify_point_split_postures()
    .expect("point postures should classify");
    let foreign_intervals = PlanarBooleanAdmittedIntervalSplitCandidateSet::new(
        "foreign interval set".to_string(),
        "foreign participation index".to_string(),
        Vec::new(),
        PlanarBooleanSplitIntervalAdmissionCounters::default(),
    );

    let denial = PlanarBooleanRawEdgeSplitScheduleSet::assemble_from_admitted_candidates(
        &postures,
        &foreign_intervals,
    )
    .expect_err("foreign point/interval products must be rejected before grouping");

    assert_eq!(
        denial.kind(),
        PlanarBooleanRawEdgeSplitScheduleDenialKind::ForeignCandidateSet
    );
    assert!(denial.evidence_identity().contains("participation index"));
    assert!(denial
        .evidence_identity()
        .contains("foreign participation index"));
}

#[test]
fn per_edge_split_schedule_rejects_mixed_source_edge_candidates() {
    let entries = vec![
        raw_point_entry_for_source_edge("entry:a", "source edge a", "carrier:a"),
        raw_point_entry_for_source_edge("entry:b", "source edge b", "carrier:a"),
    ];

    let denial =
        reject_mixed_source_edges(&SourceEdgeScheduleKey::from_entry(&entries[0]), &entries)
            .expect_err("mixed source edges must deny before schedule construction");

    assert_eq!(
        denial.kind(),
        PlanarBooleanRawEdgeSplitScheduleDenialKind::MixedSourceEdgeSchedule
    );
}

#[test]
fn per_edge_split_schedule_preserves_raw_candidate_participation_counts() {
    let postures = point_candidate_set(vec![
        point_candidate(
            "first",
            "event:shared-source",
            "carrier:a",
            0.5,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        ),
        point_candidate(
            "second",
            "event:shared-source",
            "carrier:a",
            0.5,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        ),
    ])
    .admit_parameter_domain()
    .expect("point candidates should admit")
    .classify_point_split_postures()
    .expect("point postures should classify");

    let raw = PlanarBooleanRawEdgeSplitScheduleSet::assemble_from_admitted_candidates(
        &postures,
        &empty_intervals(),
    )
    .expect("raw schedule should preserve duplicate raw participation");

    assert_eq!(raw.counters().source_edge_schedules(), 1);
    assert_eq!(raw.counters().point_entries(), 2);
    assert_eq!(raw.counters().interval_entries(), 0);
    assert_eq!(raw.counters().source_event_groups(), 1);
    assert_eq!(raw.schedules()[0].entries().len(), 2);
}

#[test]
fn per_edge_split_schedule_counts_point_and_interval_event_namespaces_separately() {
    let postures = point_candidate_set(vec![point_candidate(
        "point",
        "event:shared-text",
        "carrier:a",
        0.5,
        PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
    )])
    .admit_parameter_domain()
    .expect("point candidate should admit")
    .classify_point_split_postures()
    .expect("point posture should classify");

    let raw = PlanarBooleanRawEdgeSplitScheduleSet::assemble_from_admitted_candidates(
        &postures,
        &interval_with_event_identity([0.25, 0.75], "event:shared-text"),
    )
    .expect("raw schedule should preserve point/interval event namespaces");

    assert_eq!(raw.counters().point_entries(), 1);
    assert_eq!(raw.counters().interval_entries(), 1);
    assert_eq!(raw.counters().source_event_groups(), 2);
}

fn raw_point_entry_for_source_edge(
    entry_identity: &str,
    source_edge_identity: &str,
    carrier_identity: &str,
) -> PlanarBooleanRawEdgeSplitScheduleEntry {
    PlanarBooleanRawEdgeSplitScheduleEntry::new(
        entry_identity.to_string(),
        source_edge_identity.to_string(),
        carrier_identity.to_string(),
        format!("candidate:{entry_identity}"),
        format!("event:{entry_identity}"),
        Some(format!("parameter-fact:{entry_identity}")),
        0.5,
        None,
        "local frame".to_string(),
        "precision basis".to_string(),
        PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(
            PlanarBooleanPointSplitPosture::InteriorSplit,
        ),
        vec![format!("segment-pair:{entry_identity}")],
        vec![format!("predicate:{entry_identity}")],
        vec![format!("event-group:{entry_identity}")],
        PlanarBooleanRawPointEndpointAuthority::default(),
        None,
    )
}
