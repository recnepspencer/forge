use crate::workload_platform::planar_boolean_edge_splitting::proof_chain_support::{
    empty_intervals, interval_with_range, point_candidate, point_candidate_set, raw_schedule_for,
    shared_endpoint_candidate,
};
use crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::{
    PlanarBooleanRawEdgeSplitSchedule, PlanarBooleanRawEdgeSplitScheduleCounters,
    PlanarBooleanRawEdgeSplitScheduleEntry, PlanarBooleanRawEdgeSplitScheduleEntryKind,
    PlanarBooleanRawEdgeSplitScheduleSet, PlanarBooleanRawPointEndpointAuthority,
};
use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanEndpointBoundaryNormalizationDenialKind, PlanarBooleanPointSplitPosture,
};
use crate::workload_platform::planar_boolean_events::PlanarBooleanPointEventKind;

#[test]
fn endpoint_noop_split_preserves_contact_decision_without_fragment() {
    let endpoint_normalized = raw_schedule_for(vec![point_candidate(
        "endpoint",
        "event:endpoint",
        "carrier:a",
        0.0,
        PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
    )])
    .canonicalize_split_schedule_order()
    .expect("raw endpoint schedule should order")
    .collapse_duplicate_split_points()
    .expect("endpoint noop should duplicate-normalize")
    .normalize_endpoint_boundary_splits()
    .expect("endpoint noop should become a decision");

    assert_eq!(endpoint_normalized.counters().endpoint_noop_decisions(), 1);
    assert_eq!(endpoint_normalized.counters().fragment_point_cuts(), 0);
    let decision = endpoint_normalized
        .endpoint_contact_decisions()
        .next()
        .expect("endpoint no-op should emit one contact decision");
    assert_eq!(decision.source_endpoint_identity(), "endpoint:start");
    assert_eq!(
        decision.projected_endpoint_fact_identity(),
        "projection:start"
    );
    assert_eq!(decision.boundary_position_name(), "start");
    assert_eq!(decision.provenance_entry_identities().len(), 1);
}

#[test]
fn endpoint_boundary_split_rejects_zero_length_fragment_creation() {
    let raw = raw_set_with_entries(vec![raw_point_entry(
        "entry:bad-interior",
        0.0,
        PlanarBooleanPointSplitPosture::InteriorSplit,
        endpoint_authority("endpoint:start", "projection:start"),
    )]);

    let denial = raw
        .canonicalize_split_schedule_order()
        .expect("raw boundary interior split should order")
        .collapse_duplicate_split_points()
        .expect("raw boundary interior split should normalize")
        .normalize_endpoint_boundary_splits()
        .expect_err("interior split at an endpoint would create zero-length fragment");

    assert_eq!(
        denial.kind(),
        PlanarBooleanEndpointBoundaryNormalizationDenialKind::EndpointSplitWouldCreateZeroLengthFragment
    );
}

#[test]
fn endpoint_normalization_distinguishes_noop_from_t_junction_promotion() {
    let postures = point_candidate_set(vec![
        point_candidate(
            "endpoint-side",
            "event:t-junction",
            "carrier:a",
            0.0,
            PlanarBooleanPointEventKind::OperandAEndpointOnOperandBInterior,
        ),
        point_candidate(
            "interior-side",
            "event:t-junction",
            "carrier:b",
            0.5,
            PlanarBooleanPointEventKind::OperandAEndpointOnOperandBInterior,
        ),
    ])
    .admit_parameter_domain()
    .expect("t-junction parameters should admit")
    .classify_point_split_postures()
    .expect("t-junction should classify");
    let raw = PlanarBooleanRawEdgeSplitScheduleSet::assemble_from_admitted_candidates(
        &postures,
        &empty_intervals(),
    )
    .expect("t-junction raw schedules should assemble");

    let endpoint_normalized = raw
        .canonicalize_split_schedule_order()
        .expect("t-junction schedule should order")
        .collapse_duplicate_split_points()
        .expect("t-junction schedule should normalize")
        .normalize_endpoint_boundary_splits()
        .expect("boundary t-junction participant should become a decision");

    assert_eq!(
        endpoint_normalized
            .counters()
            .t_junction_boundary_decisions(),
        1
    );
    assert_eq!(endpoint_normalized.counters().fragment_point_cuts(), 1);
    assert!(endpoint_normalized
        .endpoint_contact_decisions()
        .all(|decision| decision.posture() == PlanarBooleanPointSplitPosture::TJunctionPromotion));
}

#[test]
fn shared_endpoint_contact_records_shared_endpoint_decision() {
    let raw = raw_schedule_for(vec![
        shared_endpoint_candidate("left", "carrier:a", 0.0),
        shared_endpoint_candidate("right", "carrier:b", 1.0),
    ]);

    let endpoint_normalized = raw
        .canonicalize_split_schedule_order()
        .expect("shared endpoint schedule should order")
        .collapse_duplicate_split_points()
        .expect("shared endpoint schedule should normalize")
        .normalize_endpoint_boundary_splits()
        .expect("shared endpoint should normalize to decisions");

    assert_eq!(
        endpoint_normalized.counters().shared_endpoint_decisions(),
        2
    );
    assert_eq!(endpoint_normalized.counters().fragment_point_cuts(), 0);
    assert!(endpoint_normalized
        .endpoint_contact_decisions()
        .all(|decision| !decision.shared_endpoint_source_identities().is_empty()));
}

#[test]
fn signed_zero_endpoint_noop_uses_start_endpoint_identity() {
    let endpoint_normalized = raw_schedule_for(vec![point_candidate(
        "negative-zero",
        "event:negative-zero",
        "carrier:a",
        -0.0,
        PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
    )])
    .canonicalize_split_schedule_order()
    .expect("signed zero schedule should order")
    .collapse_duplicate_split_points()
    .expect("signed zero should normalize")
    .normalize_endpoint_boundary_splits()
    .expect("signed zero endpoint should become a start decision");

    let decision = endpoint_normalized
        .endpoint_contact_decisions()
        .next()
        .expect("signed-zero endpoint should emit one contact decision");
    assert_eq!(decision.boundary_position_name(), "start");
    assert_eq!(decision.source_endpoint_identity(), "endpoint:start");
}

#[test]
fn end_endpoint_noop_preserves_end_endpoint_authority() {
    let endpoint_normalized = raw_schedule_for(vec![point_candidate(
        "end-endpoint",
        "event:end-endpoint",
        "carrier:a",
        1.0,
        PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
    )])
    .canonicalize_split_schedule_order()
    .expect("end endpoint schedule should order")
    .collapse_duplicate_split_points()
    .expect("end endpoint should normalize")
    .normalize_endpoint_boundary_splits()
    .expect("end endpoint should become an endpoint decision");

    let decision = endpoint_normalized
        .endpoint_contact_decisions()
        .next()
        .expect("end endpoint should emit one contact decision");
    assert_eq!(decision.boundary_position_name(), "end");
    assert_eq!(decision.source_endpoint_identity(), "endpoint:end");
    assert_eq!(
        decision.projected_endpoint_fact_identity(),
        "projection:end"
    );
}

#[test]
fn interior_endpoint_noop_posture_denies_as_contradictory_boundary_action() {
    let raw = raw_set_with_entries(vec![raw_point_entry(
        "entry:interior-noop",
        0.5,
        PlanarBooleanPointSplitPosture::EndpointNoOp,
        endpoint_authority("endpoint:start", "projection:start"),
    )]);

    let denial = raw
        .canonicalize_split_schedule_order()
        .expect("interior endpoint-noop schedule should order")
        .collapse_duplicate_split_points()
        .expect("interior endpoint-noop schedule should duplicate-normalize")
        .normalize_endpoint_boundary_splits()
        .expect_err("endpoint no-op posture away from a boundary is contradictory");

    assert_eq!(
        denial.kind(),
        PlanarBooleanEndpointBoundaryNormalizationDenialKind::ContradictoryBoundaryAction
    );
}

#[test]
fn shared_endpoint_decision_denies_mismatched_shared_endpoint_authority() {
    let raw = raw_set_with_entries(vec![raw_point_entry(
        "entry:bad-shared-authority",
        0.0,
        PlanarBooleanPointSplitPosture::SharedEndpoint,
        shared_endpoint_authority_with_projection_mismatch(),
    )]);

    let denial = raw
        .canonicalize_split_schedule_order()
        .expect("bad shared-endpoint schedule should order")
        .collapse_duplicate_split_points()
        .expect("bad shared-endpoint schedule should duplicate-normalize")
        .normalize_endpoint_boundary_splits()
        .expect_err("shared endpoint decisions require matched shared authority");

    assert_eq!(
        denial.kind(),
        PlanarBooleanEndpointBoundaryNormalizationDenialKind::MissingEndpointBoundaryAuthority
    );
}

#[test]
fn endpoint_noop_without_endpoint_authority_denies() {
    let raw = raw_set_with_entries(vec![raw_point_entry(
        "entry:missing-authority",
        0.0,
        PlanarBooleanPointSplitPosture::EndpointNoOp,
        PlanarBooleanRawPointEndpointAuthority::default(),
    )]);

    let denial = raw
        .canonicalize_split_schedule_order()
        .expect("missing authority schedule should order")
        .collapse_duplicate_split_points()
        .expect("missing authority schedule should normalize")
        .normalize_endpoint_boundary_splits()
        .expect_err("endpoint decision requires endpoint authority");

    assert_eq!(
        denial.kind(),
        PlanarBooleanEndpointBoundaryNormalizationDenialKind::MissingEndpointBoundaryAuthority
    );
}

#[test]
fn interval_entries_pass_through_endpoint_boundary_normalization() {
    let postures = point_candidate_set(vec![point_candidate(
        "interior",
        "event:interior",
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
        &interval_with_range([0.25, 0.75]),
    )
    .expect("raw schedule should assemble");

    let endpoint_normalized = raw
        .canonicalize_split_schedule_order()
        .expect("schedule should order")
        .collapse_duplicate_split_points()
        .expect("schedule should normalize")
        .normalize_endpoint_boundary_splits()
        .expect("endpoint normalization should retain intervals");

    assert_eq!(
        endpoint_normalized.counters().retained_interval_entries(),
        1
    );
    assert_eq!(
        endpoint_normalized.schedules()[0]
            .retained_interval_entry_identities()
            .len(),
        1
    );
}

fn raw_set_with_entries(
    entries: Vec<PlanarBooleanRawEdgeSplitScheduleEntry>,
) -> PlanarBooleanRawEdgeSplitScheduleSet {
    PlanarBooleanRawEdgeSplitScheduleSet::new(
        "raw schedule set".to_string(),
        "point posture set".to_string(),
        "interval set".to_string(),
        vec![PlanarBooleanRawEdgeSplitSchedule::new(
            "raw schedule".to_string(),
            "source edge a".to_string(),
            "carrier:a".to_string(),
            entries,
        )],
        PlanarBooleanRawEdgeSplitScheduleCounters::new(1, 1, 0, 0, 0, 1, 1),
    )
}

fn raw_point_entry(
    entry_identity: &str,
    parameter: f64,
    posture: PlanarBooleanPointSplitPosture,
    endpoint_authority: PlanarBooleanRawPointEndpointAuthority,
) -> PlanarBooleanRawEdgeSplitScheduleEntry {
    PlanarBooleanRawEdgeSplitScheduleEntry::new(
        entry_identity.to_string(),
        "source edge a".to_string(),
        "carrier:a".to_string(),
        format!("candidate:{entry_identity}"),
        format!("event:{entry_identity}"),
        Some(format!("parameter-fact:{entry_identity}")),
        parameter,
        None,
        "local frame".to_string(),
        "precision basis".to_string(),
        PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(posture),
        vec![format!("segment-pair:{entry_identity}")],
        vec![format!("predicate:{entry_identity}")],
        vec![format!("event-group:{entry_identity}")],
        endpoint_authority,
        None,
    )
}

fn endpoint_authority(
    source_endpoint_identity: &str,
    projected_endpoint_fact_identity: &str,
) -> PlanarBooleanRawPointEndpointAuthority {
    PlanarBooleanRawPointEndpointAuthority {
        exact_endpoint_source_identity: Some(source_endpoint_identity.to_string()),
        exact_projected_endpoint_fact_identity: Some(projected_endpoint_fact_identity.to_string()),
        shared_endpoint_source_identities: Vec::new(),
        shared_endpoint_projection_fact_digests: Vec::new(),
    }
}

fn shared_endpoint_authority_with_projection_mismatch() -> PlanarBooleanRawPointEndpointAuthority {
    PlanarBooleanRawPointEndpointAuthority {
        exact_endpoint_source_identity: Some("endpoint:start".to_string()),
        exact_projected_endpoint_fact_identity: Some("projection:start".to_string()),
        shared_endpoint_source_identities: vec![
            "endpoint:start".to_string(),
            "endpoint:peer".to_string(),
        ],
        shared_endpoint_projection_fact_digests: vec!["projection:start".to_string()],
    }
}
