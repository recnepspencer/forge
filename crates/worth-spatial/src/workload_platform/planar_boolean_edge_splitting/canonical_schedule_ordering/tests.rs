use crate::workload_platform::planar_boolean_edge_splitting::canonical_parameter::canonical_parameter_bits;
use crate::workload_platform::planar_boolean_edge_splitting::proof_chain_support::{
    interval_with_range, point_candidate, point_candidate_set, point_candidate_with_source_edge,
    raw_schedule_for,
};
use crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::{
    PlanarBooleanRawEdgeSplitSchedule, PlanarBooleanRawEdgeSplitScheduleCounters,
    PlanarBooleanRawEdgeSplitScheduleEntry, PlanarBooleanRawEdgeSplitScheduleEntryKind,
    PlanarBooleanRawEdgeSplitScheduleSet, PlanarBooleanRawPointEndpointAuthority,
};
use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanOrderedEdgeSplitScheduleDenialKind, PlanarBooleanPointSplitPosture,
};
use crate::workload_platform::planar_boolean_events::PlanarBooleanPointEventKind;

#[test]
fn split_schedule_order_preserves_same_source_edge_distinct_carriers() {
    let raw = raw_schedule_for(vec![
        point_candidate_with_source_edge("left", "carrier:a", "shared source edge", 0.25),
        point_candidate_with_source_edge("right", "carrier:b", "shared source edge", 0.75),
    ]);

    let ordered = raw
        .canonicalize_split_schedule_order()
        .expect("distinct carrier schedules should order without merging");

    assert_eq!(ordered.counters().ordered_schedules(), 2);
    let carrier_identities: Vec<_> = ordered
        .schedules()
        .iter()
        .map(|schedule| schedule.carrier_identity())
        .collect();
    assert_eq!(carrier_identities, vec!["carrier:a", "carrier:b"]);
    assert!(ordered.schedules().iter().all(|schedule| {
        schedule.source_edge_identity() == "shared source edge"
            && schedule
                .ordered_entries()
                .iter()
                .all(|entry| entry.raw_entry().carrier_identity() == schedule.carrier_identity())
    }));
}

#[test]
fn split_schedule_order_preserves_raw_entry_multiplicity() {
    let raw = raw_schedule_for(vec![
        point_candidate(
            "dup:a",
            "event:shared",
            "carrier:a",
            0.5,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        ),
        point_candidate(
            "dup:b",
            "event:shared",
            "carrier:a",
            0.5,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        ),
    ]);

    let ordered = raw
        .canonicalize_split_schedule_order()
        .expect("duplicate reports should order without collapsing");

    assert_eq!(ordered.counters().ordered_entries(), 2);
    assert_eq!(ordered.schedules()[0].ordered_entries().len(), 2);
    let event_identities: Vec<_> = ordered.schedules()[0]
        .ordered_entries()
        .iter()
        .map(|entry| entry.raw_entry().event_identity())
        .collect();
    assert_eq!(event_identities, vec!["event:shared", "event:shared"]);
}

#[test]
fn split_schedule_order_key_collapses_negative_zero_and_positive_zero() {
    let ordered = raw_schedule_for(vec![
        point_candidate(
            "negative-zero",
            "event:negative-zero",
            "carrier:a",
            -0.0,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        ),
        point_candidate(
            "positive-zero",
            "event:positive-zero",
            "carrier:a",
            0.0,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        ),
    ])
    .canonicalize_split_schedule_order()
    .expect("signed-zero parameters should order");

    let entries = ordered.schedules()[0].ordered_entries();
    assert_eq!(
        entries[0].order_key().parameter_bits(),
        entries[1].order_key().parameter_bits()
    );
    assert_eq!(
        entries[0].order_key().parameter_bits(),
        canonical_parameter_bits(0.0)
    );
}

#[test]
fn split_schedule_set_identity_is_stable_under_raw_schedule_order_variation() {
    let first = raw_set_from_schedules(vec![
        raw_schedule("schedule:a", "source:a", "carrier:a", "entry:a", 0.25),
        raw_schedule("schedule:b", "source:b", "carrier:b", "entry:b", 0.75),
    ]);
    let second = raw_set_from_schedules(vec![
        raw_schedule("schedule:b", "source:b", "carrier:b", "entry:b", 0.75),
        raw_schedule("schedule:a", "source:a", "carrier:a", "entry:a", 0.25),
    ]);

    let first_ordered = first
        .canonicalize_split_schedule_order()
        .expect("first schedule order should canonicalize");
    let second_ordered = second
        .canonicalize_split_schedule_order()
        .expect("second schedule order should canonicalize");

    assert_eq!(
        first_ordered.schedule_set_identity(),
        second_ordered.schedule_set_identity()
    );
}

#[test]
fn split_schedule_order_denies_non_finite_raw_parameter() {
    let raw = raw_set_from_schedules(vec![raw_schedule(
        "schedule:nan",
        "source:a",
        "carrier:a",
        "entry:nan",
        f64::NAN,
    )]);

    let denial = raw
        .canonicalize_split_schedule_order()
        .expect_err("non-finite raw schedule parameter must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanOrderedEdgeSplitScheduleDenialKind::NonFiniteScheduleParameter
    );
}

#[test]
fn split_schedule_order_denies_missing_tie_breaker_identity() {
    let raw = raw_set_from_schedules(vec![PlanarBooleanRawEdgeSplitSchedule::new(
        "schedule:missing".to_string(),
        "source:a".to_string(),
        "carrier:a".to_string(),
        vec![raw_entry(
            "entry:missing",
            "source:a",
            "carrier:a",
            "",
            "event:missing",
            0.5,
        )],
    )]);

    let denial = raw
        .canonicalize_split_schedule_order()
        .expect_err("missing candidate identity must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanOrderedEdgeSplitScheduleDenialKind::MissingTieBreakerIdentity
    );
}

#[test]
fn split_schedule_tie_breaker_key_exposes_named_basis() {
    let postures = point_candidate_set(vec![point_candidate(
        "point",
        "event:point",
        "carrier:a",
        0.25,
        PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
    )])
    .admit_parameter_domain()
    .expect("point parameter should admit")
    .classify_point_split_postures()
    .expect("point posture should classify");
    let raw = PlanarBooleanRawEdgeSplitScheduleSet::assemble_from_admitted_candidates(
        &postures,
        &interval_with_range([0.25, 0.75]),
    )
    .expect("point and interval schedule should assemble");

    let ordered = raw
        .canonicalize_split_schedule_order()
        .expect("equal parameter entries should order");
    let entries = ordered.schedules()[0].ordered_entries();

    assert_eq!(ordered.counters().equal_parameter_ties(), 1);
    assert_eq!(
        entries[0].order_key().source_edge_identity(),
        "source edge a"
    );
    assert_eq!(entries[0].order_key().carrier_identity(), "carrier:a");
    assert_eq!(entries[0].order_key().event_identity(), "event:point");
    assert_eq!(
        entries[0].order_key().event_group_identities(),
        &["event-group:event:point".to_string()]
    );
    assert_eq!(entries[0].order_key().entry_kind_rank(), 0);
    assert_eq!(entries[1].order_key().entry_kind_rank(), 2);
    assert!(!entries[0].order_key().candidate_identity().is_empty());
}

#[test]
fn split_schedule_order_uses_event_group_identity_before_candidate_identity() {
    let raw = raw_set_from_schedules(vec![PlanarBooleanRawEdgeSplitSchedule::new(
        "schedule:groups".to_string(),
        "source:a".to_string(),
        "carrier:a".to_string(),
        vec![
            raw_entry_with_event_groups(
                "entry:group-z",
                "source:a",
                "carrier:a",
                "candidate:a",
                "event:shared",
                0.5,
                vec!["event-group:z".to_string()],
            ),
            raw_entry_with_event_groups(
                "entry:group-a",
                "source:a",
                "carrier:a",
                "candidate:z",
                "event:shared",
                0.5,
                vec!["event-group:a".to_string()],
            ),
        ],
    )]);

    let ordered = raw
        .canonicalize_split_schedule_order()
        .expect("event groups should be part of canonical order");

    let entries = ordered.schedules()[0].ordered_entries();
    assert_eq!(entries[0].raw_entry().entry_identity(), "entry:group-a");
    assert_eq!(
        entries[0].order_key().event_group_identities(),
        &["event-group:a".to_string()]
    );
    assert_eq!(entries[0].order_key().candidate_identity(), "candidate:z");
}

#[test]
fn split_schedule_order_key_canonicalizes_event_group_identity_basis() {
    let raw = raw_set_from_schedules(vec![PlanarBooleanRawEdgeSplitSchedule::new(
        "schedule:canonical-groups".to_string(),
        "source:a".to_string(),
        "carrier:a".to_string(),
        vec![raw_entry_with_event_groups(
            "entry:canonical-groups",
            "source:a",
            "carrier:a",
            "candidate:a",
            "event:shared",
            0.5,
            vec![
                "event-group:z".to_string(),
                "event-group:a".to_string(),
                "event-group:a".to_string(),
            ],
        )],
    )]);

    let ordered = raw
        .canonicalize_split_schedule_order()
        .expect("event group basis should canonicalize");

    assert_eq!(
        ordered.schedules()[0].ordered_entries()[0]
            .order_key()
            .event_group_identities(),
        &["event-group:a".to_string(), "event-group:z".to_string()]
    );
}

#[test]
fn split_schedule_order_digest_commits_to_event_group_order_basis() {
    let first = raw_set_from_schedules(vec![PlanarBooleanRawEdgeSplitSchedule::new(
        "schedule:group-digest".to_string(),
        "source:a".to_string(),
        "carrier:a".to_string(),
        vec![raw_entry_with_event_groups(
            "entry:shared",
            "source:a",
            "carrier:a",
            "candidate:shared",
            "event:shared",
            0.5,
            vec!["event-group:a".to_string()],
        )],
    )]);
    let second = raw_set_from_schedules(vec![PlanarBooleanRawEdgeSplitSchedule::new(
        "schedule:group-digest".to_string(),
        "source:a".to_string(),
        "carrier:a".to_string(),
        vec![raw_entry_with_event_groups(
            "entry:shared",
            "source:a",
            "carrier:a",
            "candidate:shared",
            "event:shared",
            0.5,
            vec!["event-group:b".to_string()],
        )],
    )]);

    let first_ordered = first
        .canonicalize_split_schedule_order()
        .expect("first event-group schedule should order");
    let second_ordered = second
        .canonicalize_split_schedule_order()
        .expect("second event-group schedule should order");

    assert_ne!(
        first_ordered.schedules()[0].order_digest(),
        second_ordered.schedules()[0].order_digest()
    );
    assert_ne!(
        first_ordered.schedule_set_identity(),
        second_ordered.schedule_set_identity()
    );
}

fn raw_set_from_schedules(
    schedules: Vec<PlanarBooleanRawEdgeSplitSchedule>,
) -> PlanarBooleanRawEdgeSplitScheduleSet {
    let entry_count = schedules
        .iter()
        .map(|schedule| schedule.entries().len())
        .sum();
    let schedule_count = schedules.len();
    PlanarBooleanRawEdgeSplitScheduleSet::new(
        "raw schedule set".to_string(),
        "point posture set".to_string(),
        "interval set".to_string(),
        schedules,
        PlanarBooleanRawEdgeSplitScheduleCounters::new(
            schedule_count,
            entry_count,
            0,
            0,
            0,
            0,
            entry_count,
        ),
    )
}

fn raw_schedule(
    schedule_identity: &str,
    source_edge_identity: &str,
    carrier_identity: &str,
    entry_identity: &str,
    parameter: f64,
) -> PlanarBooleanRawEdgeSplitSchedule {
    PlanarBooleanRawEdgeSplitSchedule::new(
        schedule_identity.to_string(),
        source_edge_identity.to_string(),
        carrier_identity.to_string(),
        vec![raw_entry(
            entry_identity,
            source_edge_identity,
            carrier_identity,
            &format!("candidate:{entry_identity}"),
            &format!("event:{entry_identity}"),
            parameter,
        )],
    )
}

fn raw_entry(
    entry_identity: &str,
    source_edge_identity: &str,
    carrier_identity: &str,
    candidate_identity: &str,
    event_identity: &str,
    parameter: f64,
) -> PlanarBooleanRawEdgeSplitScheduleEntry {
    PlanarBooleanRawEdgeSplitScheduleEntry::new(
        entry_identity.to_string(),
        source_edge_identity.to_string(),
        carrier_identity.to_string(),
        candidate_identity.to_string(),
        event_identity.to_string(),
        Some(format!("parameter-fact:{entry_identity}")),
        parameter,
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

fn raw_entry_with_event_groups(
    entry_identity: &str,
    source_edge_identity: &str,
    carrier_identity: &str,
    candidate_identity: &str,
    event_identity: &str,
    parameter: f64,
    event_group_identities: Vec<String>,
) -> PlanarBooleanRawEdgeSplitScheduleEntry {
    PlanarBooleanRawEdgeSplitScheduleEntry::new(
        entry_identity.to_string(),
        source_edge_identity.to_string(),
        carrier_identity.to_string(),
        candidate_identity.to_string(),
        event_identity.to_string(),
        Some(format!("parameter-fact:{entry_identity}")),
        parameter,
        None,
        "local frame".to_string(),
        "precision basis".to_string(),
        PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(
            PlanarBooleanPointSplitPosture::InteriorSplit,
        ),
        vec![format!("segment-pair:{entry_identity}")],
        vec![format!("predicate:{entry_identity}")],
        event_group_identities,
        PlanarBooleanRawPointEndpointAuthority::default(),
        None,
    )
}
