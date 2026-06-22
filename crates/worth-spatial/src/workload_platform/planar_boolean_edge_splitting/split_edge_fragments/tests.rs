use crate::workload_platform::planar_boolean_edge_splitting::duplicate_split_normalization::tests_support::{
    raw_interval_entry, raw_point_entry, raw_point_entry_with_frame_precision, raw_schedule,
    raw_set_from_schedules,
};
use crate::workload_platform::planar_boolean_edge_splitting::micro_interval_normalization::PlanarBooleanMicroIntervalPolicy;
use crate::workload_platform::planar_boolean_edge_splitting::split_vertex_identity::{
    PlanarBooleanSplitVertexIdentityCounters, PlanarBooleanSplitVertexIdentitySchedule,
    PlanarBooleanSplitVertexIdentitySet,
};
use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanPointSplitPosture, PlanarBooleanSplitEdgeFragmentDenialKind,
    PlanarBooleanSplitEdgeFragmentEndpointKind, PlanarBooleanSplitEdgeFragmentSet,
};
use crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::{
    PlanarBooleanRawEdgeSplitScheduleEntry, PlanarBooleanRawEdgeSplitScheduleEntryKind,
    PlanarBooleanRawIntervalAuthority, PlanarBooleanRawPointEndpointAuthority,
};
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanIntervalEventKind, PlanarBooleanSourceIntervalSense,
};

#[test]
fn split_edge_fragments_cover_source_edge_parameter_domain_without_gaps() {
    let fragments = build_fragments(vec![
        raw_point_entry("point a", "source edge", "carrier", "event:a", 0.2),
        raw_point_entry("point b", "source edge", "carrier", "event:b", 0.7),
    ]);

    assert!(fragments.certifies_domain_coverage());
    assert_eq!(fragments.counters().source_edges_covered(), 1);
    assert_eq!(fragments.counters().fragments_emitted(), 3);
    let ranges = fragment_ranges(&fragments);
    assert_eq!(ranges, vec![[0.0, 0.2], [0.2, 0.7], [0.7, 1.0]]);
}

#[test]
fn split_edge_fragment_construction_rejects_zero_length_fragments() {
    let normalized = normalized_schedule_set(vec![
        raw_point_entry("point a", "source edge", "carrier", "event:a", 0.5),
        raw_point_entry("point b", "source edge", "carrier", "event:b", 0.5),
    ]);
    let mut vertex_schedules = normalized
        .mint_split_vertex_identities()
        .expect("split vertices should mint")
        .schedules()
        .to_vec();
    let duplicate_vertex = vertex_schedules[0].vertices()[0].clone();
    let mut vertices = vertex_schedules[0].vertices().to_vec();
    vertices.push(duplicate_vertex);
    vertex_schedules[0] = PlanarBooleanSplitVertexIdentitySchedule::new(
        "duplicate vertex schedule".to_string(),
        normalized.schedules()[0].schedule_identity().to_string(),
        "source edge".to_string(),
        "carrier".to_string(),
        vertices,
        Vec::new(),
    );
    let forged_same_set = PlanarBooleanSplitVertexIdentitySet::new(
        "forged duplicate vertex set".to_string(),
        normalized.schedule_set_identity().to_string(),
        vertex_schedules,
        PlanarBooleanSplitVertexIdentityCounters::new(1, 2, 0, 0, 2, 0, 0, 0),
    );

    let denial = normalized
        .build_split_edge_fragments(&forged_same_set)
        .expect_err("duplicate boundaries must not emit zero-length fragments");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitEdgeFragmentDenialKind::CollapsedSplitFragment
    );
}

#[test]
fn split_edge_fragments_preserve_source_edge_carrier_and_sense() {
    let fragments = build_fragments(vec![raw_interval_entry(
        "interval",
        "source edge",
        "carrier",
        "event:interval",
        0.2,
    )]);
    let interval_fragment = fragments
        .fragments()
        .find(|fragment| !fragment.interval_subdivision_identities().is_empty())
        .expect("interval subdivision should attribute at least one fragment");

    assert_eq!(interval_fragment.source_edge_identity(), "source edge");
    assert_eq!(interval_fragment.carrier_identity(), "carrier");
    assert_eq!(
        interval_fragment.source_senses(),
        &[PlanarBooleanSourceIntervalSense::Forward]
    );
    assert!(interval_fragment
        .normalized_interval_identities()
        .iter()
        .any(|identity| identity == "normalized-interval:interval"));
}

#[test]
fn reversed_interval_fragments_preserve_opposite_source_sense() {
    let fragments = build_fragments(vec![reversed_interval_entry()]);
    let interval_fragments = fragments
        .fragments()
        .filter(|fragment| !fragment.interval_subdivision_identities().is_empty())
        .collect::<Vec<_>>();

    assert!(!interval_fragments.is_empty());
    for fragment in interval_fragments {
        assert_eq!(
            fragment.source_senses(),
            &[PlanarBooleanSourceIntervalSense::Reversed]
        );
        assert!(fragment
            .normalized_interval_identities()
            .iter()
            .any(|identity| identity == "normalized-interval:reversed interval"));
    }
}

#[test]
fn ordinary_split_fragments_preserve_default_source_edge_sense() {
    let fragments = build_fragments(vec![raw_point_entry(
        "point",
        "source edge",
        "carrier",
        "event:point",
        0.5,
    )]);

    for fragment in fragments.fragments() {
        assert_eq!(
            fragment.source_senses(),
            &[PlanarBooleanSourceIntervalSense::Forward]
        );
    }
}

#[test]
fn endpoint_noop_does_not_emit_zero_length_boundary_fragment() {
    let fragments = build_fragments(vec![
        endpoint_noop_entry(),
        raw_point_entry("interior", "source edge", "carrier", "event:interior", 0.5),
    ]);

    assert_eq!(fragments.counters().endpoint_noop_boundaries_skipped(), 1);
    assert_eq!(fragment_ranges(&fragments), vec![[0.0, 0.5], [0.5, 1.0]]);
    assert!(fragments
        .fragments()
        .all(|fragment| fragment.parameter_range()[0] < fragment.parameter_range()[1]));
}

#[test]
fn fragment_identity_is_stable_under_vertex_order_variation() {
    let ordered = build_fragments(vec![
        raw_point_entry("point a", "source edge", "carrier", "event:a", 0.2),
        raw_point_entry("point b", "source edge", "carrier", "event:b", 0.7),
    ]);
    let replayed = build_fragments(vec![
        raw_point_entry("point b", "source edge", "carrier", "event:b", 0.7),
        raw_point_entry("point a", "source edge", "carrier", "event:a", 0.2),
    ]);

    let ordered_identities = fragment_identities(&ordered);
    let replayed_identities = fragment_identities(&replayed);
    assert_eq!(
        ordered.fragment_set_identity(),
        replayed.fragment_set_identity()
    );
    assert_eq!(ordered_identities, replayed_identities);
}

#[test]
fn fragment_endpoints_distinguish_original_boundaries_from_split_vertices() {
    let fragments = build_fragments(vec![raw_point_entry(
        "point",
        "source edge",
        "carrier",
        "event:point",
        0.5,
    )]);
    let rows = fragments.fragments().collect::<Vec<_>>();

    assert_eq!(
        rows[0].start_endpoint().endpoint_kind(),
        PlanarBooleanSplitEdgeFragmentEndpointKind::OriginalSourceStart
    );
    assert_eq!(
        rows[0].end_endpoint().endpoint_kind(),
        PlanarBooleanSplitEdgeFragmentEndpointKind::SplitVertex
    );
    assert_eq!(
        rows[1].end_endpoint().endpoint_kind(),
        PlanarBooleanSplitEdgeFragmentEndpointKind::OriginalSourceEnd
    );
}

#[test]
fn mismatched_split_vertex_schedule_set_is_rejected() {
    let normalized = normalized_schedule_set(vec![raw_point_entry(
        "point",
        "source edge",
        "carrier",
        "event:point",
        0.5,
    )]);
    let foreign_normalized = normalized_schedule_set(vec![raw_point_entry(
        "foreign point",
        "source edge",
        "carrier",
        "event:foreign",
        0.25,
    )]);
    let foreign_vertices = foreign_normalized
        .mint_split_vertex_identities()
        .expect("foreign split vertices should mint");

    let denial = normalized
        .build_split_edge_fragments(&foreign_vertices)
        .expect_err("foreign split vertex set must not certify this schedule set");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitEdgeFragmentDenialKind::MismatchedSplitVertexScheduleSet
    );
}

#[test]
fn mixed_frame_precision_basis_is_rejected_before_fragment_construction() {
    let normalized = raw_set_from_schedules(vec![
        raw_schedule(
            "raw schedule a",
            "source edge a",
            "carrier a",
            vec![raw_point_entry_with_frame_precision(
                "point a",
                "source edge a",
                "carrier a",
                "event:a",
                0.5,
                "local frame",
                "precision basis",
            )],
        ),
        raw_schedule(
            "raw schedule b",
            "source edge b",
            "carrier b",
            vec![raw_point_entry_with_frame_precision(
                "point b",
                "source edge b",
                "carrier b",
                "event:b",
                0.5,
                "foreign frame",
                "precision basis",
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
    .expect("interval normalization should pass");
    let vertices = normalized
        .mint_split_vertex_identities()
        .expect("split vertices should mint before fragment basis validation");

    let denial = normalized
        .build_split_edge_fragments(&vertices)
        .expect_err("mixed basis schedule set must not construct fragments");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitEdgeFragmentDenialKind::AmbiguousFragmentBasis
    );
}

fn build_fragments(
    entries: Vec<PlanarBooleanRawEdgeSplitScheduleEntry>,
) -> PlanarBooleanSplitEdgeFragmentSet {
    let normalized = normalized_schedule_set(entries);
    let vertices = normalized
        .mint_split_vertex_identities()
        .expect("split vertices should mint");
    normalized
        .build_split_edge_fragments(&vertices)
        .expect("split fragments should build")
}

fn normalized_schedule_set(
    entries: Vec<PlanarBooleanRawEdgeSplitScheduleEntry>,
) -> crate::workload_platform::planar_boolean_edge_splitting::micro_interval_normalization::PlanarBooleanIntervalSubdivisionNormalizedScheduleSet{
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
}

fn endpoint_noop_entry() -> PlanarBooleanRawEdgeSplitScheduleEntry {
    PlanarBooleanRawEdgeSplitScheduleEntry::new(
        "endpoint noop".to_string(),
        "source edge".to_string(),
        "carrier".to_string(),
        "candidate:endpoint noop".to_string(),
        "event:endpoint".to_string(),
        Some("parameter-fact:endpoint noop".to_string()),
        0.0,
        None,
        "local frame".to_string(),
        "precision basis".to_string(),
        PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(
            PlanarBooleanPointSplitPosture::EndpointNoOp,
        ),
        vec!["segment-pair:event:endpoint".to_string()],
        vec!["predicate:event:endpoint".to_string()],
        vec!["event-group:event:endpoint".to_string()],
        PlanarBooleanRawPointEndpointAuthority {
            exact_endpoint_source_identity: Some("endpoint:start".to_string()),
            exact_projected_endpoint_fact_identity: Some("projection:start".to_string()),
            shared_endpoint_source_identities: Vec::new(),
            shared_endpoint_projection_fact_digests: Vec::new(),
        },
        None,
    )
}

fn reversed_interval_entry() -> PlanarBooleanRawEdgeSplitScheduleEntry {
    PlanarBooleanRawEdgeSplitScheduleEntry::new(
        "reversed interval".to_string(),
        "source edge".to_string(),
        "carrier".to_string(),
        "candidate:reversed interval".to_string(),
        "event:reversed interval".to_string(),
        None,
        0.9,
        Some([0.9, 0.1]),
        "local frame".to_string(),
        "precision basis".to_string(),
        PlanarBooleanRawEdgeSplitScheduleEntryKind::Interval,
        vec!["segment-pair:event:reversed interval".to_string()],
        vec!["predicate:event:reversed interval".to_string()],
        vec!["event-group:event:reversed interval".to_string()],
        PlanarBooleanRawPointEndpointAuthority::default(),
        Some(PlanarBooleanRawIntervalAuthority::new(
            PlanarBooleanIntervalEventKind::PartialOverlap,
            "source-interval:reversed interval".to_string(),
            [0.9, 0.1],
            PlanarBooleanSourceIntervalSense::Reversed,
            "normalized-interval:reversed interval".to_string(),
            [0.9, 0.1],
            "participation-row:reversed interval".to_string(),
        )),
    )
}

fn fragment_ranges(fragments: &PlanarBooleanSplitEdgeFragmentSet) -> Vec<[f64; 2]> {
    fragments
        .fragments()
        .map(|fragment| fragment.parameter_range())
        .collect()
}

fn fragment_identities(fragments: &PlanarBooleanSplitEdgeFragmentSet) -> Vec<String> {
    fragments
        .fragments()
        .map(|fragment| fragment.fragment_identity().to_string())
        .collect()
}
