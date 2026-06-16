use crate::workload_platform::planar_boolean_edge_splitting::duplicate_split_normalization::tests_support::{
    raw_interval_entry, raw_point_entry, raw_schedule, raw_set_from_schedules,
};
use crate::workload_platform::planar_boolean_edge_splitting::duplicate_split_normalization::{
    PlanarBooleanNormalizedEndpointAuthority, PlanarBooleanNormalizedSplitCut,
};
use crate::workload_platform::planar_boolean_edge_splitting::micro_interval_normalization::{
    PlanarBooleanIntervalSubdivisionNormalizationCounters,
    PlanarBooleanIntervalSubdivisionNormalizedSchedule,
    PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
};
use crate::workload_platform::planar_boolean_edge_splitting::micro_interval_normalization::PlanarBooleanMicroIntervalPolicy;
use crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::{
    PlanarBooleanRawEdgeSplitScheduleEntry, PlanarBooleanRawEdgeSplitScheduleEntryKind,
    PlanarBooleanRawPointEndpointAuthority,
};
use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanPointSplitPosture;

use super::{
    PlanarBooleanSplitVertexCoalescenceReason, PlanarBooleanSplitVertexIdentityDenialKind,
};

#[test]
fn split_vertex_identity_is_stable_under_replay_and_event_order_variation() {
    let ordered = mint_vertices(vec![
        raw_point_entry("point a", "source edge", "carrier", "event:a", 0.2),
        raw_interval_entry("interval a", "source edge", "carrier", "event:b", 0.2),
        raw_point_entry("point b", "source edge", "carrier", "event:c", 0.7),
    ]);
    let replayed = mint_vertices(vec![
        raw_point_entry("point b", "source edge", "carrier", "event:c", 0.7),
        raw_interval_entry("interval a", "source edge", "carrier", "event:b", 0.2),
        raw_point_entry("point a", "source edge", "carrier", "event:a", 0.2),
    ]);

    assert_eq!(
        ordered.split_vertex_identity_set_identity(),
        replayed.split_vertex_identity_set_identity()
    );
    let ordered_identities = ordered
        .vertices()
        .map(|vertex| vertex.split_vertex_identity().to_string())
        .collect::<Vec<_>>();
    let replayed_identities = replayed
        .vertices()
        .map(|vertex| vertex.split_vertex_identity().to_string())
        .collect::<Vec<_>>();
    assert_eq!(ordered_identities, replayed_identities);
}

#[test]
fn interval_endpoint_and_point_cut_coalesce_by_certified_source_parameter() {
    let vertices = mint_vertices(vec![
        raw_point_entry("point a", "source edge", "carrier", "event:a", 0.2),
        raw_interval_entry("interval a", "source edge", "carrier", "event:b", 0.2),
    ]);

    assert_eq!(vertices.counters().point_cuts_inspected(), 1);
    assert_eq!(
        vertices.counters().interval_endpoint_candidates_inspected(),
        2
    );
    assert_eq!(vertices.counters().split_vertices_minted(), 2);
    assert_eq!(vertices.counters().split_vertices_coalesced(), 1);
    assert_eq!(
        vertices
            .counters()
            .interval_point_endpoint_collisions_resolved(),
        1
    );
    let decision = vertices
        .coalescence_decisions()
        .next()
        .expect("point cut and interval endpoint should produce coalescence decision");
    assert_eq!(
        decision.reason(),
        PlanarBooleanSplitVertexCoalescenceReason::IntervalEndpointAndPointCut
    );
    assert_eq!(decision.point_cut_identities().len(), 1);
    assert_eq!(decision.interval_subdivision_identities().len(), 1);
    let point_vertex = vertices
        .vertices()
        .find(|vertex| !vertex.point_cut_identities().is_empty())
        .expect("point-derived split vertex should be present");
    assert_eq!(
        point_vertex.parameter_fact_identities(),
        &["parameter-fact:point a".to_string()]
    );
}

#[test]
fn same_parameter_on_different_source_edges_does_not_coordinate_coalesce() {
    let normalized = raw_set_from_schedules(vec![
        raw_schedule(
            "raw schedule a",
            "source edge a",
            "carrier a",
            vec![raw_point_entry(
                "point a",
                "source edge a",
                "carrier a",
                "event:a",
                0.5,
            )],
        ),
        raw_schedule(
            "raw schedule b",
            "source edge b",
            "carrier b",
            vec![raw_point_entry(
                "point b",
                "source edge b",
                "carrier b",
                "event:b",
                0.5,
            )],
        ),
    ])
    .canonicalize_split_schedule_order()
    .expect("raw schedules should order")
    .collapse_duplicate_split_points()
    .expect("duplicate normalization should pass")
    .normalize_endpoint_boundary_splits()
    .expect("endpoint normalization should pass")
    .normalize_overlap_interval_subdivisions(PlanarBooleanMicroIntervalPolicy::DenyBelowTolerance)
    .expect("interval normalization should pass")
    .mint_split_vertex_identities()
    .expect("split vertices should mint");

    assert_eq!(normalized.schedules().len(), 2);
    assert_eq!(normalized.counters().split_vertices_minted(), 2);
    assert_eq!(normalized.counters().split_vertices_coalesced(), 0);
    assert_eq!(normalized.coalescence_decisions().count(), 0);
}

#[test]
fn shared_crossing_vertices_coalesce_by_event_provenance_not_coordinate_string() {
    let vertices = mint_vertices(vec![
        raw_point_entry_with_projected_fact(
            "point a",
            "source edge",
            "carrier",
            "event:a",
            0.5,
            "coordinate fact from left",
        ),
        raw_point_entry_with_projected_fact(
            "point b",
            "source edge",
            "carrier",
            "event:b",
            0.5,
            "coordinate fact from right",
        ),
    ]);

    assert_eq!(vertices.counters().point_cuts_inspected(), 1);
    assert_eq!(vertices.counters().split_vertices_minted(), 1);
    let vertex = vertices
        .vertices()
        .next()
        .expect("duplicate crossing reports should mint one split vertex");
    assert!(vertex.coordinate_fact_identities().is_empty());
    assert_eq!(
        vertex.parameter_fact_identities(),
        &[
            "parameter-fact:point a".to_string(),
            "parameter-fact:point b".to_string()
        ]
    );
    assert_eq!(
        vertex.coalescence_provenance(),
        &[
            "event:a".to_string(),
            "event:b".to_string(),
            "point a".to_string(),
            "point b".to_string()
        ]
    );
}

#[test]
fn coordinate_only_split_vertex_identity_is_rejected_before_minting() {
    let denial = schedule_set_with_coordinate_only_split_cut()
        .mint_split_vertex_identities()
        .expect_err("coordinate-only split vertex input must deny");
    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitVertexIdentityDenialKind::CoordinateOnlySplitVertexIdentity
    );
}

fn mint_vertices(
    entries: Vec<PlanarBooleanRawEdgeSplitScheduleEntry>,
) -> super::PlanarBooleanSplitVertexIdentitySet {
    raw_set_from_schedules(vec![raw_schedule(
        "raw schedule",
        "source edge",
        "carrier",
        entries,
    )])
    .canonicalize_split_schedule_order()
    .expect("raw schedule should order")
    .collapse_duplicate_split_points()
    .expect("duplicate normalization should pass")
    .normalize_endpoint_boundary_splits()
    .expect("endpoint normalization should pass")
    .normalize_overlap_interval_subdivisions(PlanarBooleanMicroIntervalPolicy::DenyBelowTolerance)
    .expect("interval normalization should pass")
    .mint_split_vertex_identities()
    .expect("split vertices should mint")
}

fn raw_point_entry_with_projected_fact(
    entry_identity: &str,
    source_edge_identity: &str,
    carrier_identity: &str,
    event_identity: &str,
    parameter: f64,
    projected_fact_identity: &str,
) -> PlanarBooleanRawEdgeSplitScheduleEntry {
    PlanarBooleanRawEdgeSplitScheduleEntry::new(
        entry_identity.to_string(),
        source_edge_identity.to_string(),
        carrier_identity.to_string(),
        format!("candidate:{entry_identity}"),
        event_identity.to_string(),
        Some(format!("parameter-fact:{entry_identity}")),
        parameter,
        None,
        "local frame".to_string(),
        "precision basis".to_string(),
        PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(
            PlanarBooleanPointSplitPosture::InteriorSplit,
        ),
        vec![format!("segment-pair:{event_identity}")],
        vec![format!("predicate:{event_identity}")],
        vec![format!("event-group:{event_identity}")],
        PlanarBooleanRawPointEndpointAuthority {
            exact_endpoint_source_identity: None,
            exact_projected_endpoint_fact_identity: Some(projected_fact_identity.to_string()),
            shared_endpoint_source_identities: Vec::new(),
            shared_endpoint_projection_fact_digests: Vec::new(),
        },
        None,
    )
}

fn schedule_set_with_coordinate_only_split_cut(
) -> PlanarBooleanIntervalSubdivisionNormalizedScheduleSet {
    let schedule = PlanarBooleanIntervalSubdivisionNormalizedSchedule::new(
        "schedule".to_string(),
        "endpoint schedule".to_string(),
        "source edge".to_string(),
        "carrier".to_string(),
        vec![coordinate_only_normalized_split_cut()],
        Vec::new(),
        Vec::new(),
    );
    PlanarBooleanIntervalSubdivisionNormalizedScheduleSet::new(
        "schedule set".to_string(),
        "endpoint schedule set".to_string(),
        vec![schedule],
        PlanarBooleanIntervalSubdivisionNormalizationCounters::new(1, 0, 0, 0, 0, 0, 0, 1, 0),
    )
}

fn coordinate_only_normalized_split_cut() -> PlanarBooleanNormalizedSplitCut {
    PlanarBooleanNormalizedSplitCut::new(
        "coordinate-only-cut".to_string(),
        "duplicate report".to_string(),
        "source edge".to_string(),
        "carrier".to_string(),
        0.5,
        0.5f64.to_bits(),
        PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(
            PlanarBooleanPointSplitPosture::InteriorSplit,
        ),
        "local frame".to_string(),
        "precision basis".to_string(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        PlanarBooleanNormalizedEndpointAuthority {
            exact_endpoint_source_identity: None,
            exact_projected_endpoint_fact_identity: Some("coordinate fact".to_string()),
            shared_endpoint_source_identities: Vec::new(),
            shared_endpoint_projection_fact_digests: Vec::new(),
        },
    )
}
