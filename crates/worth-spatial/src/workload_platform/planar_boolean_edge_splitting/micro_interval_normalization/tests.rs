use crate::workload_platform::planar_boolean_edge_splitting::duplicate_split_normalization::tests_support::{
    raw_schedule, raw_set_from_schedules,
};
use crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::{
    PlanarBooleanRawEdgeSplitScheduleEntry, PlanarBooleanRawEdgeSplitScheduleEntryKind,
    PlanarBooleanRawIntervalAuthority, PlanarBooleanRawPointEndpointAuthority,
};
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanIntervalEventKind, PlanarBooleanSourceIntervalSense,
};

use super::{
    PlanarBooleanIntervalSubdivisionNormalizationDenialKind, PlanarBooleanMicroIntervalAction,
    PlanarBooleanMicroIntervalPolicy,
};

#[test]
fn overlap_interval_subdivision_normalizes_redundant_collinear_boundaries() {
    let normalized = normalize_entries(vec![
        interval_entry(
            "interval a",
            [0.2, 0.7],
            PlanarBooleanSourceIntervalSense::Forward,
        ),
        interval_entry(
            "interval b",
            [0.2, 0.7],
            PlanarBooleanSourceIntervalSense::Forward,
        ),
    ]);

    let counters = normalized.counters();
    assert_eq!(counters.retained_interval_rows_inspected(), 2);
    assert_eq!(counters.normalized_interval_subdivisions(), 1);
    assert_eq!(counters.redundant_interval_rows_collapsed(), 1);
    let row = &normalized.schedules()[0].interval_subdivisions()[0];
    assert_eq!(row.admitted_parameter_range(), [0.2, 0.7]);
    assert_eq!(
        row.provenance_entry_identities(),
        &["interval a".to_string(), "interval b".to_string()]
    );
}

#[test]
fn redundant_interval_subdivision_identity_is_stable_under_source_row_order() {
    let ordered = normalize_entries(vec![
        interval_entry(
            "interval a",
            [0.2, 0.7],
            PlanarBooleanSourceIntervalSense::Forward,
        ),
        interval_entry(
            "interval b",
            [0.2, 0.7],
            PlanarBooleanSourceIntervalSense::Forward,
        ),
    ]);
    let reversed = normalize_entries(vec![
        interval_entry(
            "interval b",
            [0.2, 0.7],
            PlanarBooleanSourceIntervalSense::Forward,
        ),
        interval_entry(
            "interval a",
            [0.2, 0.7],
            PlanarBooleanSourceIntervalSense::Forward,
        ),
    ]);

    let ordered_row = &ordered.schedules()[0].interval_subdivisions()[0];
    let reversed_row = &reversed.schedules()[0].interval_subdivisions()[0];
    assert_eq!(
        ordered_row.subdivision_identity(),
        reversed_row.subdivision_identity()
    );
    assert_eq!(
        reversed_row.provenance_entry_identities(),
        &["interval a".to_string(), "interval b".to_string()]
    );
}

#[test]
fn micro_interval_policy_denies_or_collapses_with_explicit_decision() {
    let denied = normalize_entries_with_policy(
        vec![interval_entry(
            "micro interval",
            [0.4, 0.4 + 1.0e-13],
            PlanarBooleanSourceIntervalSense::Forward,
        )],
        PlanarBooleanMicroIntervalPolicy::DenyBelowTolerance,
    )
    .expect_err("micro interval must deny without explicit policy");
    assert_eq!(
        denied.kind(),
        PlanarBooleanIntervalSubdivisionNormalizationDenialKind::MicroIntervalBelowAdmittedPolicy
    );

    let admitted = normalize_entries_with_policy(
        vec![interval_entry(
            "micro interval",
            [0.4, 0.4 + 1.0e-13],
            PlanarBooleanSourceIntervalSense::Forward,
        )],
        PlanarBooleanMicroIntervalPolicy::AdmitExplicitCollapse,
    )
    .expect("explicit collapse policy should admit micro interval");
    assert_eq!(admitted.counters().micro_intervals_admitted(), 1);
    assert_eq!(
        admitted.schedules()[0].interval_subdivisions()[0].action(),
        PlanarBooleanMicroIntervalAction::AdmittedCollapse
    );

    let policy_required = normalize_entries_with_policy(
        vec![interval_entry(
            "micro interval",
            [0.4, 0.4 + 1.0e-13],
            PlanarBooleanSourceIntervalSense::Forward,
        )],
        PlanarBooleanMicroIntervalPolicy::RequireExplicitDecision,
    )
    .expect("explicit decision policy should retain a policy-required micro interval");
    assert_eq!(
        policy_required.counters().micro_intervals_policy_required(),
        1
    );
    assert_eq!(
        policy_required.schedules()[0].interval_subdivisions()[0].action(),
        PlanarBooleanMicroIntervalAction::PolicyRequired
    );
}

#[test]
fn opposite_sense_interval_normalization_preserves_source_sense() {
    let normalized = normalize_entries(vec![interval_entry(
        "reversed interval",
        [0.9, 0.1],
        PlanarBooleanSourceIntervalSense::Reversed,
    )]);

    assert_eq!(normalized.counters().opposite_sense_rows_preserved(), 1);
    let row = &normalized.schedules()[0].interval_subdivisions()[0];
    assert_eq!(
        row.source_sense(),
        PlanarBooleanSourceIntervalSense::Reversed
    );
    assert_eq!(row.admitted_parameter_range(), [0.9, 0.1]);
}

#[test]
fn collapsed_interval_subdivision_denies_before_fragment_minting() {
    let denied = normalize_entries_with_policy(
        vec![interval_entry(
            "collapsed interval",
            [-0.0, 0.0],
            PlanarBooleanSourceIntervalSense::Forward,
        )],
        PlanarBooleanMicroIntervalPolicy::AdmitExplicitCollapse,
    )
    .expect_err("signed zero collapse must deny as zero-length interval");

    assert_eq!(
        denied.kind(),
        PlanarBooleanIntervalSubdivisionNormalizationDenialKind::CollapsedIntervalSubdivision
    );
}

#[test]
fn interval_subdivision_denies_mixed_frame_precision_for_same_span_basis() {
    let denied = normalize_entries_with_policy(
        vec![
            interval_entry_with_basis(
                "interval a",
                [0.2, 0.7],
                PlanarBooleanSourceIntervalSense::Forward,
                "local frame",
                "precision basis",
            ),
            interval_entry_with_basis(
                "interval b",
                [0.2, 0.7],
                PlanarBooleanSourceIntervalSense::Forward,
                "foreign frame",
                "precision basis",
            ),
        ],
        PlanarBooleanMicroIntervalPolicy::DenyBelowTolerance,
    )
    .expect_err("same interval subdivision must not admit mixed frame basis");

    assert_eq!(
        denied.kind(),
        PlanarBooleanIntervalSubdivisionNormalizationDenialKind::ContradictoryIntervalSubdivisionBasis
    );
}

#[test]
fn interval_subdivision_preserves_same_span_opposite_sense_as_distinct_rows() {
    let normalized = normalize_entries_with_policy(
        vec![
            interval_entry(
                "interval a",
                [0.2, 0.7],
                PlanarBooleanSourceIntervalSense::Forward,
            ),
            interval_entry(
                "interval b",
                [0.2, 0.7],
                PlanarBooleanSourceIntervalSense::Reversed,
            ),
        ],
        PlanarBooleanMicroIntervalPolicy::DenyBelowTolerance,
    )
    .expect("same-span opposite-sense intervals should preserve both facts");

    assert_eq!(normalized.counters().normalized_interval_subdivisions(), 2);
    assert_eq!(normalized.counters().opposite_sense_rows_preserved(), 1);
    let source_senses = normalized.schedules()[0]
        .interval_subdivisions()
        .iter()
        .map(|row| row.source_sense())
        .collect::<Vec<_>>();
    assert!(source_senses.contains(&PlanarBooleanSourceIntervalSense::Forward));
    assert!(source_senses.contains(&PlanarBooleanSourceIntervalSense::Reversed));
}

#[test]
fn interval_subdivision_denies_mixed_precision_for_same_span_basis() {
    let denied = normalize_entries_with_policy(
        vec![
            interval_entry_with_basis(
                "interval a",
                [0.2, 0.7],
                PlanarBooleanSourceIntervalSense::Forward,
                "local frame",
                "precision basis",
            ),
            interval_entry_with_basis(
                "interval b",
                [0.2, 0.7],
                PlanarBooleanSourceIntervalSense::Reversed,
                "local frame",
                "foreign precision basis",
            ),
        ],
        PlanarBooleanMicroIntervalPolicy::DenyBelowTolerance,
    )
    .expect_err("same interval subdivision must not admit mixed precision basis");

    assert_eq!(
        denied.kind(),
        PlanarBooleanIntervalSubdivisionNormalizationDenialKind::ContradictoryIntervalSubdivisionBasis
    );
}

#[test]
fn non_finite_interval_subdivision_denies_before_policy_handling() {
    let denied = normalize_entries_with_policy(
        vec![interval_entry(
            "non finite interval",
            [0.2, f64::INFINITY],
            PlanarBooleanSourceIntervalSense::Forward,
        )],
        PlanarBooleanMicroIntervalPolicy::AdmitExplicitCollapse,
    )
    .expect_err("non-finite interval boundary must deny before row minting");

    assert_eq!(
        denied.kind(),
        PlanarBooleanIntervalSubdivisionNormalizationDenialKind::NonFiniteIntervalBoundary
    );
}

fn normalize_entries(
    entries: Vec<PlanarBooleanRawEdgeSplitScheduleEntry>,
) -> super::PlanarBooleanIntervalSubdivisionNormalizedScheduleSet {
    normalize_entries_with_policy(
        entries,
        PlanarBooleanMicroIntervalPolicy::DenyBelowTolerance,
    )
    .expect("interval subdivision normalization should succeed")
}

fn normalize_entries_with_policy(
    entries: Vec<PlanarBooleanRawEdgeSplitScheduleEntry>,
    policy: PlanarBooleanMicroIntervalPolicy,
) -> Result<
    super::PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    super::PlanarBooleanIntervalSubdivisionNormalizationDenial,
> {
    raw_set_from_schedules(vec![raw_schedule(
        "raw schedule",
        "source edge",
        "carrier",
        entries,
    )])
    .canonicalize_split_schedule_order()
    .expect("raw schedule should order")
    .collapse_duplicate_split_points()
    .expect("duplicate normalization should retain interval rows")
    .normalize_endpoint_boundary_splits()
    .expect("endpoint boundary normalization should pass through interval rows")
    .normalize_overlap_interval_subdivisions(policy)
}

fn interval_entry(
    entry_identity: &str,
    range: [f64; 2],
    source_sense: PlanarBooleanSourceIntervalSense,
) -> PlanarBooleanRawEdgeSplitScheduleEntry {
    interval_entry_with_basis(
        entry_identity,
        range,
        source_sense,
        "local frame",
        "precision basis",
    )
}

fn interval_entry_with_basis(
    entry_identity: &str,
    range: [f64; 2],
    source_sense: PlanarBooleanSourceIntervalSense,
    local_frame_identity: &str,
    precision_basis_identity: &str,
) -> PlanarBooleanRawEdgeSplitScheduleEntry {
    PlanarBooleanRawEdgeSplitScheduleEntry::new(
        entry_identity.to_string(),
        "source edge".to_string(),
        "carrier".to_string(),
        format!("candidate:{entry_identity}"),
        format!("event:{entry_identity}"),
        None,
        range[0],
        Some(range),
        local_frame_identity.to_string(),
        precision_basis_identity.to_string(),
        PlanarBooleanRawEdgeSplitScheduleEntryKind::Interval,
        Vec::new(),
        Vec::new(),
        vec![format!("event-group:{entry_identity}")],
        PlanarBooleanRawPointEndpointAuthority::default(),
        Some(PlanarBooleanRawIntervalAuthority::new(
            PlanarBooleanIntervalEventKind::PartialOverlap,
            format!("source-interval:{entry_identity}"),
            range,
            source_sense,
            "normalized interval".to_string(),
            range,
            format!("participation-row:{entry_identity}"),
        )),
    )
}
