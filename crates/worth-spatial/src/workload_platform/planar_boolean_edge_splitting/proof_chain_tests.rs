use super::proof_chain_support::*;
use crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::PlanarBooleanRawEdgeSplitScheduleEntryKind;
use crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::PlanarBooleanRawEdgeSplitScheduleSet;
use crate::workload_platform::planar_boolean_events::PlanarBooleanPointEventKind;

#[test]
fn t_junction_endpoint_on_interior_promotes_to_vertex_split() {
    let postures = point_candidate_set(vec![
        point_candidate(
            "endpoint",
            "event:t",
            "carrier:a",
            0.0,
            PlanarBooleanPointEventKind::OperandAEndpointOnOperandBInterior,
        ),
        point_candidate(
            "interior",
            "event:t",
            "carrier:b",
            0.5,
            PlanarBooleanPointEventKind::OperandAEndpointOnOperandBInterior,
        ),
    ])
    .admit_parameter_domain()
    .expect("parameters should admit")
    .classify_point_split_postures()
    .expect("t-junction should classify");

    assert_eq!(postures.counters().t_junction_promotions(), 2);
    assert!(postures
        .postured_candidates()
        .iter()
        .all(|candidate| candidate.posture().produces_split_vertex()));
}

#[test]
fn shared_endpoint_contact_preserves_endpoint_identities_without_extra_fragment() {
    let postures = point_candidate_set(vec![
        shared_endpoint_candidate("a", "carrier:a", 0.0),
        shared_endpoint_candidate("b", "carrier:b", 1.0),
    ])
    .admit_parameter_domain()
    .expect("shared endpoint parameters should admit")
    .classify_point_split_postures()
    .expect("shared endpoint provenance should classify");

    assert_eq!(postures.counters().shared_endpoint_noops(), 2);
    assert!(postures
        .postured_candidates()
        .iter()
        .all(|candidate| !candidate.posture().produces_split_vertex()));
    assert!(postures.postured_candidates().iter().all(|candidate| {
        candidate
            .admitted_candidate()
            .candidate()
            .shared_endpoint_source_identities()
            .len()
            == 2
    }));
    let raw = PlanarBooleanRawEdgeSplitScheduleSet::assemble_from_admitted_candidates(
        &postures,
        &empty_intervals(),
    )
    .expect("shared endpoint schedule should assemble");
    assert_eq!(raw.counters().shared_endpoint_noop_entries(), 2);
    assert_eq!(raw.counters().endpoint_noop_entries(), 0);
}

#[test]
fn shared_endpoint_without_provenance_denies_before_schedule_assembly() {
    let denial = point_candidate_set(vec![point_candidate(
        "missing-shared",
        "event:shared",
        "carrier:a",
        0.0,
        PlanarBooleanPointEventKind::SharedEndpoint,
    )])
    .admit_parameter_domain()
    .expect("endpoint parameter should admit")
    .classify_point_split_postures()
    .expect_err("shared endpoint without shared provenance must deny");

    assert_eq!(
        denial.kind(),
        crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanPointSplitPostureDenialKind::SharedEndpointMissingProvenance
    );
}

#[test]
fn endpoint_only_noop_split_is_counted_and_does_not_create_zero_length_edge() {
    let postures = point_candidate_set(vec![
        point_candidate(
            "start",
            "event:endpoint-only",
            "carrier:a",
            0.0,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        ),
        point_candidate(
            "end",
            "event:endpoint-only",
            "carrier:a",
            1.0,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        ),
    ])
    .admit_parameter_domain()
    .expect("endpoint parameters should admit")
    .classify_point_split_postures()
    .expect("endpoint-only group should classify");

    assert_eq!(postures.counters().endpoint_noops(), 2);
    assert!(postures
        .postured_candidates()
        .iter()
        .all(|candidate| !candidate.posture().produces_split_vertex()));
    let raw = PlanarBooleanRawEdgeSplitScheduleSet::assemble_from_admitted_candidates(
        &postures,
        &empty_intervals(),
    )
    .expect("endpoint no-op schedule should assemble");
    assert_eq!(raw.counters().endpoint_noop_entries(), 2);
}

#[test]
fn per_edge_schedule_order_and_duplicate_collapse_preserve_provenance() {
    let normalized = run_point_pipeline(vec![
        point_candidate(
            "dup:a",
            "event:a",
            "carrier:a",
            0.5,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        ),
        point_candidate(
            "dup:b",
            "event:b",
            "carrier:a",
            0.5,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        ),
        point_candidate(
            "other-edge",
            "event:c",
            "carrier:b",
            0.25,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        ),
    ]);

    assert_eq!(normalized.counters().raw_point_cuts(), 3);
    assert_eq!(normalized.counters().normalized_point_cuts(), 2);
    assert_eq!(normalized.counters().duplicate_reports_collapsed(), 1);
    let duplicate_cut = normalized
        .schedules()
        .iter()
        .flat_map(|schedule| schedule.cuts())
        .find(|cut| cut.parameter() == 0.5)
        .expect("duplicate normalized cut should exist");
    assert_eq!(duplicate_cut.provenance_entry_identities().len(), 2);
    assert_eq!(duplicate_cut.event_identities().len(), 2);
    assert!(!duplicate_cut.duplicate_report_identity().is_empty());
}

#[test]
fn per_edge_split_schedule_groups_candidates_by_source_edge() {
    let raw = raw_schedule_for(vec![
        point_candidate(
            "edge-a",
            "event:a",
            "carrier:a",
            0.5,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        ),
        point_candidate(
            "edge-b",
            "event:b",
            "carrier:b",
            0.5,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        ),
    ]);

    assert_eq!(raw.counters().source_edge_schedules(), 2);
    let schedule_source_edges: Vec<_> = raw
        .schedules()
        .iter()
        .map(|schedule| schedule.source_edge_identity())
        .collect();
    assert_eq!(
        schedule_source_edges,
        vec!["source edge a", "source edge b"]
    );
    assert!(raw.schedules().iter().all(|schedule| schedule
        .entries()
        .iter()
        .all(|entry| entry.source_edge_identity() == schedule.source_edge_identity())));
}

#[test]
fn per_edge_split_schedule_preserves_interval_entries_and_source_event_group_counts() {
    let postures = point_candidate_set(vec![point_candidate(
        "point",
        "event:point",
        "carrier:a",
        0.5,
        PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
    )])
    .admit_parameter_domain()
    .expect("point parameter should admit")
    .classify_point_split_postures()
    .expect("point posture should classify");
    let raw = PlanarBooleanRawEdgeSplitScheduleSet::assemble_from_admitted_candidates(
        &postures,
        &single_interval(),
    )
    .expect("point and interval schedule should assemble");

    assert_eq!(raw.counters().source_edge_schedules(), 1);
    assert_eq!(raw.counters().source_event_groups(), 2);
    assert_eq!(raw.counters().point_entries(), 1);
    assert_eq!(raw.counters().interval_entries(), 1);
    let normalized = raw
        .canonicalize_split_schedule_order()
        .expect("raw schedule should order")
        .collapse_duplicate_split_points()
        .expect("point normalization should retain interval entries");
    assert_eq!(
        normalized.schedules()[0]
            .retained_interval_entry_identities()
            .len(),
        1
    );
}

#[test]
fn split_schedule_order_digest_is_stable_under_candidate_order_variation() {
    let first = ordered_digest_for(vec![
        point_candidate(
            "a",
            "event:a",
            "carrier:a",
            0.5,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        ),
        point_candidate(
            "b",
            "event:b",
            "carrier:a",
            0.5,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        ),
    ]);
    let second = ordered_digest_for(vec![
        point_candidate(
            "b",
            "event:b",
            "carrier:a",
            0.5,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        ),
        point_candidate(
            "a",
            "event:a",
            "carrier:a",
            0.5,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        ),
    ]);

    assert_eq!(first, second);
}

#[test]
fn split_schedule_tie_breakers_cover_equal_parameter_point_and_interval_edges() {
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
    .expect("equal-parameter point and interval schedule should assemble");

    let ordered = raw
        .canonicalize_split_schedule_order()
        .expect("equal-parameter point and interval should order");

    assert_eq!(ordered.counters().equal_parameter_ties(), 1);
    let ordered_entries = ordered.schedules()[0].ordered_entries();
    assert!(matches!(
        ordered_entries[0].raw_entry().kind(),
        PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(_)
    ));
    assert!(matches!(
        ordered_entries[1].raw_entry().kind(),
        PlanarBooleanRawEdgeSplitScheduleEntryKind::Interval
    ));
}

#[test]
fn split_schedule_order_does_not_depend_on_debug_or_display_strings() {
    let ordered = raw_schedule_for(vec![
        point_candidate(
            "display-z",
            "event:z",
            "carrier:a",
            0.5,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        ),
        point_candidate(
            "display-a",
            "event:a",
            "carrier:a",
            0.5,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        ),
    ])
    .canonicalize_split_schedule_order()
    .expect("equal-parameter points should order by explicit identities");

    let event_identities: Vec<_> = ordered.schedules()[0]
        .ordered_entries()
        .iter()
        .map(|entry| entry.raw_entry().event_identity())
        .collect();
    assert_eq!(event_identities, vec!["event:a", "event:z"]);
}

#[test]
fn canonical_parameter_identity_collapses_negative_zero_and_positive_zero() {
    let normalized = run_point_pipeline(vec![
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
    ]);

    assert_eq!(normalized.counters().raw_point_cuts(), 2);
    assert_eq!(normalized.counters().normalized_point_cuts(), 1);
    assert_eq!(normalized.counters().duplicate_reports_collapsed(), 1);
}

#[test]
fn contradictory_duplicate_split_points_deny_instead_of_merging() {
    let normalized = raw_schedule_for(vec![
        point_candidate(
            "interior",
            "event:interior",
            "carrier:a",
            0.5,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        ),
        point_candidate(
            "t-interior",
            "event:t",
            "carrier:a",
            0.5,
            PlanarBooleanPointEventKind::OperandAEndpointOnOperandBInterior,
        ),
        point_candidate(
            "t-endpoint",
            "event:t",
            "carrier:b",
            0.0,
            PlanarBooleanPointEventKind::OperandAEndpointOnOperandBInterior,
        ),
    ])
    .canonicalize_split_schedule_order()
    .expect("same-parameter posture variation should still order")
    .collapse_duplicate_split_points()
    .expect("same-parameter posture variation is distinct cut authority");

    assert_eq!(normalized.counters().raw_point_cuts(), 3);
    assert_eq!(normalized.counters().normalized_point_cuts(), 3);
    assert_eq!(normalized.counters().duplicate_reports_collapsed(), 0);

    let denial = raw_schedule_for(vec![
        point_candidate(
            "frame-a",
            "event:frame-a",
            "carrier:a",
            0.5,
            PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        ),
        point_candidate_with_frame_precision(
            "frame-b",
            "event:frame-b",
            "carrier:a",
            0.5,
            "foreign frame",
            "precision basis",
        ),
    ])
    .canonicalize_split_schedule_order()
    .expect("frame contradiction should still order before normalization")
    .collapse_duplicate_split_points()
    .expect_err("same source parameter with contradictory frame basis must deny");

    assert_eq!(
        denial.kind(),
        crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanDuplicateSplitNormalizationDenialKind::ContradictoryDuplicateSplitPoint
    );
}
