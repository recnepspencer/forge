use crate::workload_platform::planar_boolean_edge_splitting::canonical_parameter::canonical_parameter_bits;
use crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::{
    PlanarBooleanRawEdgeSplitScheduleEntry, PlanarBooleanRawEdgeSplitScheduleEntryKind,
};
use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanDuplicateSplitNormalizationDenialKind, PlanarBooleanPointSplitPosture,
};

use super::tests_support::*;

#[test]
fn duplicate_split_points_collapse_to_one_cut_with_all_event_provenance() {
    let normalized = raw_set_from_schedules(vec![raw_schedule(
        "schedule:duplicates",
        "source:a",
        "carrier:a",
        vec![
            raw_point_entry("entry:a", "source:a", "carrier:a", "event:a", 0.5),
            raw_point_entry("entry:b", "source:a", "carrier:a", "event:b", 0.5),
        ],
    )])
    .canonicalize_split_schedule_order()
    .expect("raw duplicate schedule should order")
    .collapse_duplicate_split_points()
    .expect("compatible duplicate point cuts should collapse");

    let cut = &normalized.schedules()[0].cuts()[0];
    assert_eq!(normalized.counters().raw_point_cuts(), 2);
    assert_eq!(normalized.counters().normalized_point_cuts(), 1);
    assert_eq!(normalized.counters().duplicate_reports_collapsed(), 1);
    assert_eq!(normalized.counters().provenance_rows_retained(), 2);
    assert_eq!(
        cut.provenance_entry_identities(),
        &["entry:a".to_string(), "entry:b".to_string()]
    );
    assert_eq!(
        cut.event_identities(),
        &["event:a".to_string(), "event:b".to_string()]
    );
    assert_eq!(
        cut.event_group_identities(),
        &[
            "event-group:event:a".to_string(),
            "event-group:event:b".to_string()
        ]
    );
    assert_eq!(
        cut.segment_pair_identities(),
        &[
            "segment-pair:event:a".to_string(),
            "segment-pair:event:b".to_string()
        ]
    );
    assert_eq!(
        cut.predicate_receipt_identities(),
        &[
            "predicate:event:a".to_string(),
            "predicate:event:b".to_string()
        ]
    );
}

#[test]
fn duplicate_split_point_collapse_counters_are_exact() {
    let normalized = raw_set_from_schedules(vec![raw_schedule(
        "schedule:counters",
        "source:a",
        "carrier:a",
        vec![
            raw_point_entry("entry:a", "source:a", "carrier:a", "event:a", 0.25),
            raw_point_entry("entry:b", "source:a", "carrier:a", "event:b", 0.25),
            raw_point_entry("entry:c", "source:a", "carrier:a", "event:c", 0.25),
            raw_point_entry("entry:d", "source:a", "carrier:a", "event:d", 0.75),
            raw_interval_entry("interval:a", "source:a", "carrier:a", "event:i-a", 0.2),
            raw_interval_entry("interval:b", "source:a", "carrier:a", "event:i-b", 0.8),
        ],
    )])
    .canonicalize_split_schedule_order()
    .expect("counter schedule should order")
    .collapse_duplicate_split_points()
    .expect("counter schedule should normalize");

    assert_eq!(normalized.counters().normalized_schedules(), 1);
    assert_eq!(normalized.counters().raw_point_cuts(), 4);
    assert_eq!(normalized.counters().normalized_point_cuts(), 2);
    assert_eq!(normalized.counters().duplicate_reports_collapsed(), 2);
    assert_eq!(normalized.counters().provenance_rows_retained(), 4);
    assert_eq!(normalized.counters().retained_interval_entries(), 2);
}

#[test]
fn duplicate_split_reports_retain_duplicate_provenance_row_multiplicity() {
    let normalized = raw_set_from_schedules(vec![raw_schedule(
        "schedule:duplicate-entry-identity",
        "source:a",
        "carrier:a",
        vec![
            raw_point_entry("entry:repeated", "source:a", "carrier:a", "event:a", 0.25),
            raw_point_entry("entry:repeated", "source:a", "carrier:a", "event:b", 0.25),
        ],
    )])
    .canonicalize_split_schedule_order()
    .expect("duplicate raw entry identity schedule should still order")
    .collapse_duplicate_split_points()
    .expect("compatible duplicate reports should normalize");

    let cut = &normalized.schedules()[0].cuts()[0];
    assert_eq!(cut.provenance_entry_identities().len(), 2);
    assert_eq!(
        cut.provenance_entry_identities(),
        &["entry:repeated".to_string(), "entry:repeated".to_string()]
    );
    assert_eq!(normalized.counters().provenance_rows_retained(), 2);
}

#[test]
fn contradictory_duplicate_split_points_deny_instead_of_merging() {
    let denial = raw_set_from_schedules(vec![raw_schedule(
        "schedule:contradiction",
        "source:a",
        "carrier:a",
        vec![
            raw_point_entry("entry:interior", "source:a", "carrier:a", "event:a", 0.5),
            raw_point_entry_with_frame_precision(
                "entry:foreign-frame",
                "source:a",
                "carrier:a",
                "event:b",
                0.5,
                "foreign frame",
                "precision basis",
            ),
        ],
    )])
    .canonicalize_split_schedule_order()
    .expect("contradictory raw schedule should still order")
    .collapse_duplicate_split_points()
    .expect_err("contradictory duplicate point cuts must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanDuplicateSplitNormalizationDenialKind::ContradictoryDuplicateSplitPoint
    );
}

#[test]
fn duplicate_cut_identity_distinguishes_equal_parameter_postures() {
    let normalized = raw_set_from_schedules(vec![raw_schedule(
        "schedule:postures",
        "source:a",
        "carrier:a",
        vec![
            raw_entry(
                "entry:same",
                "source:a",
                "carrier:a",
                "event:same",
                0.5,
                PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(
                    PlanarBooleanPointSplitPosture::InteriorSplit,
                ),
                "local frame",
                "precision basis",
            ),
            raw_entry(
                "entry:same",
                "source:a",
                "carrier:a",
                "event:same",
                0.5,
                PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(
                    PlanarBooleanPointSplitPosture::TJunctionPromotion,
                ),
                "local frame",
                "precision basis",
            ),
        ],
    )])
    .canonicalize_split_schedule_order()
    .expect("equal-parameter posture schedule should order")
    .collapse_duplicate_split_points()
    .expect("distinct posture cuts should normalize separately");

    let cuts = normalized.schedules()[0].cuts();
    assert_eq!(cuts.len(), 2);
    assert_ne!(cuts[0].cut_identity(), cuts[1].cut_identity());
}

#[test]
fn duplicate_split_points_deny_precision_or_frame_conflict() {
    let denial = raw_set_from_schedules(vec![raw_schedule(
        "schedule:precision-conflict",
        "source:a",
        "carrier:a",
        vec![
            raw_point_entry("entry:precision-a", "source:a", "carrier:a", "event:a", 0.5),
            raw_point_entry_with_frame_precision(
                "entry:precision-b",
                "source:a",
                "carrier:a",
                "event:b",
                0.5,
                "local frame",
                "different precision",
            ),
        ],
    )])
    .canonicalize_split_schedule_order()
    .expect("precision conflict should still order")
    .collapse_duplicate_split_points()
    .expect_err("same parameter with different precision basis must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanDuplicateSplitNormalizationDenialKind::ContradictoryDuplicateSplitPoint
    );
}

#[test]
fn duplicate_split_points_do_not_merge_distinct_carriers() {
    let normalized = raw_set_from_schedules(vec![
        raw_schedule(
            "schedule:a",
            "shared-source",
            "carrier:a",
            vec![raw_point_entry(
                "entry:a",
                "shared-source",
                "carrier:a",
                "event:a",
                0.5,
            )],
        ),
        raw_schedule(
            "schedule:b",
            "shared-source",
            "carrier:b",
            vec![raw_point_entry(
                "entry:b",
                "shared-source",
                "carrier:b",
                "event:b",
                0.5,
            )],
        ),
    ])
    .canonicalize_split_schedule_order()
    .expect("distinct carrier schedules should order")
    .collapse_duplicate_split_points()
    .expect("distinct carriers should normalize separately");

    assert_eq!(normalized.schedules().len(), 2);
    assert_eq!(normalized.counters().raw_point_cuts(), 2);
    assert_eq!(normalized.counters().normalized_point_cuts(), 2);
    assert_eq!(normalized.counters().duplicate_reports_collapsed(), 0);
}

#[test]
fn signed_zero_duplicate_split_points_collapse_to_one_cut() {
    let normalized = raw_set_from_schedules(vec![raw_schedule(
        "schedule:signed-zero",
        "source:a",
        "carrier:a",
        vec![
            raw_point_entry(
                "entry:negative-zero",
                "source:a",
                "carrier:a",
                "event:a",
                -0.0,
            ),
            raw_point_entry(
                "entry:positive-zero",
                "source:a",
                "carrier:a",
                "event:b",
                0.0,
            ),
        ],
    )])
    .canonicalize_split_schedule_order()
    .expect("signed-zero duplicate schedule should order")
    .collapse_duplicate_split_points()
    .expect("signed-zero duplicate cuts should collapse");

    let cut = &normalized.schedules()[0].cuts()[0];
    assert_eq!(normalized.counters().normalized_point_cuts(), 1);
    assert_eq!(cut.parameter_bits(), canonical_parameter_bits(0.0));
}

#[test]
fn interval_entries_survive_point_duplicate_normalization() {
    let normalized = raw_set_from_schedules(vec![raw_schedule(
        "schedule:interval-retention",
        "source:a",
        "carrier:a",
        vec![
            raw_point_entry("entry:point-a", "source:a", "carrier:a", "event:a", 0.5),
            raw_point_entry("entry:point-b", "source:a", "carrier:a", "event:b", 0.5),
            raw_interval_entry(
                "entry:interval",
                "source:a",
                "carrier:a",
                "event:interval",
                0.25,
            ),
        ],
    )])
    .canonicalize_split_schedule_order()
    .expect("schedule with interval should order")
    .collapse_duplicate_split_points()
    .expect("point duplicate normalization should retain intervals");

    assert_eq!(normalized.counters().retained_interval_entries(), 1);
    assert_eq!(
        normalized.schedules()[0].retained_interval_entry_identities(),
        &["entry:interval".to_string()]
    );
}

#[test]
fn duplicate_split_identity_stable_under_duplicate_report_order_variation() {
    let first = normalized_identity_for_entries(vec![
        raw_point_entry("entry:a", "source:a", "carrier:a", "event:a", 0.5),
        raw_point_entry("entry:b", "source:a", "carrier:a", "event:b", 0.5),
    ]);
    let second = normalized_identity_for_entries(vec![
        raw_point_entry("entry:b", "source:a", "carrier:a", "event:b", 0.5),
        raw_point_entry("entry:a", "source:a", "carrier:a", "event:a", 0.5),
    ]);

    assert_eq!(first, second);
}

fn normalized_identity_for_entries(entries: Vec<PlanarBooleanRawEdgeSplitScheduleEntry>) -> String {
    raw_set_from_schedules(vec![raw_schedule(
        "schedule:stable-duplicates",
        "source:a",
        "carrier:a",
        entries,
    )])
    .canonicalize_split_schedule_order()
    .expect("duplicate schedule should order")
    .collapse_duplicate_split_points()
    .expect("duplicate schedule should normalize")
    .schedule_set_identity()
    .to_string()
}
