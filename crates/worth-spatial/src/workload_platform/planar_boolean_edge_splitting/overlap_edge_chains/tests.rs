use super::*;
use crate::workload_platform::planar_boolean_edge_splitting::interval_parameter_admission::{
    AdmittedIntervalSplitCandidate, PlanarBooleanAdmittedIntervalSplitCandidateSet,
    PlanarBooleanSplitIntervalAdmissionCounters,
};
use crate::workload_platform::planar_boolean_edge_splitting::interval_split_candidates::{
    PlanarBooleanIntervalSplitCandidate, PlanarBooleanIntervalSplitCandidateInput,
};
use crate::workload_platform::planar_boolean_edge_splitting::proof_chain_support::{
    point_candidate, point_candidate_set,
};
use crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::PlanarBooleanRawEdgeSplitScheduleSet;
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanIntervalEventKind, PlanarBooleanPointEventKind, PlanarBooleanSourceIntervalSense,
};

mod malformed_index;

#[test]
fn overlap_edge_chain_preserves_partial_containment_and_identical_interval_kinds() {
    let chains = build_chains(vec![
        admitted_interval(
            "partial",
            "event:partial",
            "carrier:a",
            "source edge a",
            [0.2, 0.6],
            PlanarBooleanIntervalEventKind::PartialOverlap,
            PlanarBooleanSourceIntervalSense::Forward,
        ),
        admitted_interval(
            "contained",
            "event:contained",
            "carrier:b",
            "source edge b",
            [0.25, 0.75],
            PlanarBooleanIntervalEventKind::ContainmentOverlap,
            PlanarBooleanSourceIntervalSense::Forward,
        ),
        admitted_interval(
            "identical",
            "event:identical",
            "carrier:c",
            "source edge c",
            [0.1, 0.9],
            PlanarBooleanIntervalEventKind::IdenticalSameDirection,
            PlanarBooleanSourceIntervalSense::Forward,
        ),
    ]);

    assert_eq!(chains.counters().chains_emitted(), 3);
    assert_eq!(chains.counters().partial_overlap_chains(), 1);
    assert_eq!(chains.counters().different_parameterization_chains(), 1);
    assert_eq!(chains.counters().identical_parallel_chains(), 1);
    assert!(chains
        .chains()
        .iter()
        .any(|chain| chain.interval_event_kind()
            == PlanarBooleanIntervalEventKind::ContainmentOverlap));
}

#[test]
fn opposite_sense_overlap_chain_preserves_both_source_senses() {
    let chains = build_chains(vec![
        admitted_interval(
            "forward",
            "event:opposite",
            "carrier:a",
            "source edge a",
            [0.2, 0.8],
            PlanarBooleanIntervalEventKind::IdenticalAntiParallel,
            PlanarBooleanSourceIntervalSense::Forward,
        ),
        admitted_interval(
            "reversed",
            "event:opposite",
            "carrier:b",
            "source edge b",
            [0.2, 0.8],
            PlanarBooleanIntervalEventKind::IdenticalAntiParallel,
            PlanarBooleanSourceIntervalSense::Reversed,
        ),
    ]);

    let chain = chains
        .chains()
        .iter()
        .find(|chain| chain.interval_event_identity() == "event:opposite")
        .expect("opposite-sense chain should be present");
    assert_eq!(
        chain.source_senses(),
        &[
            PlanarBooleanSourceIntervalSense::Forward,
            PlanarBooleanSourceIntervalSense::Reversed
        ]
    );
    assert_eq!(chain.members().len(), 2);
    assert_eq!(chains.counters().opposite_sense_chains(), 1);
    assert_eq!(chains.counters().identical_antiparallel_chains(), 1);
}

#[test]
fn overlap_edge_chain_construction_does_not_emit_region_or_loop_products() {
    let chains = build_chains(vec![admitted_interval(
        "partial",
        "event:partial",
        "carrier:a",
        "source edge a",
        [0.25, 0.75],
        PlanarBooleanIntervalEventKind::PartialOverlap,
        PlanarBooleanSourceIntervalSense::Forward,
    )]);

    assert!(chains.certifies_prepared_overlap_chains());
    assert!(!chains.emits_topology_truth());
    assert_eq!(chains.counters().topology_products_emitted(), 0);
}

#[test]
fn overlap_edge_chain_preserves_multiple_fragments_for_one_interval_subdivision() {
    let chains = build_chains_with_point_cut(
        vec![admitted_interval(
            "partial",
            "event:partial",
            "carrier:a",
            "source edge a",
            [0.25, 0.75],
            PlanarBooleanIntervalEventKind::PartialOverlap,
            PlanarBooleanSourceIntervalSense::Forward,
        )],
        0.5,
    );

    let chain = &chains.chains()[0];
    assert_eq!(chain.members().len(), 2);
    assert!(chain.members().iter().any(|member| member.boundary_role()
        == PlanarBooleanOverlapChainBoundaryRole::OverlapStartBoundary));
    assert!(chain.members().iter().any(|member| member.boundary_role()
        == PlanarBooleanOverlapChainBoundaryRole::OverlapEndBoundary));
}

#[test]
fn overlap_edge_chain_rejects_foreign_fragment_set() {
    let intervals = interval_set(vec![admitted_interval(
        "partial",
        "event:partial",
        "carrier:a",
        "source edge a",
        [0.25, 0.75],
        PlanarBooleanIntervalEventKind::PartialOverlap,
        PlanarBooleanSourceIntervalSense::Forward,
    )]);
    let first = interval_normalized(intervals, None);
    let foreign = interval_normalized(
        interval_set(vec![admitted_interval(
            "foreign",
            "event:foreign",
            "carrier:b",
            "source edge b",
            [0.1, 0.9],
            PlanarBooleanIntervalEventKind::PartialOverlap,
            PlanarBooleanSourceIntervalSense::Forward,
        )]),
        None,
    );
    let foreign_vertices = foreign
        .mint_split_vertex_identities()
        .expect("foreign vertices should mint");
    let foreign_fragments = foreign
        .build_split_edge_fragments(&foreign_vertices)
        .expect("foreign fragments should build");

    let denial = first
        .build_overlap_edge_chains(&foreign_fragments)
        .expect_err("foreign fragment set must deny");

    assert_eq!(
        denial.denial_kind(),
        PlanarBooleanOverlapEdgeChainDenialKind::ForeignFragmentSet
    );
}

fn build_chains(
    intervals: Vec<AdmittedIntervalSplitCandidate>,
) -> PlanarBooleanOverlapEdgeChainSet {
    build_chains_with_optional_point_cut(intervals, None)
}

fn build_chains_with_point_cut(
    intervals: Vec<AdmittedIntervalSplitCandidate>,
    point_parameter: f64,
) -> PlanarBooleanOverlapEdgeChainSet {
    build_chains_with_optional_point_cut(intervals, Some(point_parameter))
}

fn build_chains_with_optional_point_cut(
    intervals: Vec<AdmittedIntervalSplitCandidate>,
    point_parameter: Option<f64>,
) -> PlanarBooleanOverlapEdgeChainSet {
    let normalized = interval_normalized(interval_set(intervals), point_parameter);
    let split_vertices = normalized
        .mint_split_vertex_identities()
        .expect("split vertices should mint");
    let fragments = normalized
        .build_split_edge_fragments(&split_vertices)
        .expect("split fragments should build");
    if point_parameter.is_some() {
        return normalized
            .build_overlap_edge_chains(&fragments)
            .expect("overlap chains should build with point subdivision");
    }
    normalized
        .build_overlap_edge_chains(&fragments)
        .expect("overlap chains should build")
}

fn interval_normalized(
    intervals: PlanarBooleanAdmittedIntervalSplitCandidateSet,
    point_parameter: Option<f64>,
) -> crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanIntervalSubdivisionNormalizedScheduleSet{
    let points = point_parameter
        .map(|parameter| {
            vec![point_candidate(
                "split",
                "event:split",
                "carrier:a",
                parameter,
                PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
            )]
        })
        .unwrap_or_default();
    let postures = point_candidate_set(points)
        .admit_parameter_domain()
        .expect("point parameter should admit")
        .classify_point_split_postures()
        .expect("point posture should classify");
    let raw = PlanarBooleanRawEdgeSplitScheduleSet::assemble_from_admitted_candidates(
        &postures, &intervals,
    )
    .expect("raw schedule should assemble");
    raw.canonicalize_split_schedule_order()
        .expect("raw schedule should order")
        .collapse_duplicate_split_points()
        .expect("point schedule should normalize")
        .normalize_endpoint_boundary_splits()
        .expect("endpoint boundaries should normalize")
        .normalize_overlap_interval_subdivisions(
            crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanMicroIntervalPolicy::DenyBelowTolerance,
        )
        .expect("interval subdivisions should normalize")
}

fn interval_set(
    intervals: Vec<AdmittedIntervalSplitCandidate>,
) -> PlanarBooleanAdmittedIntervalSplitCandidateSet {
    PlanarBooleanAdmittedIntervalSplitCandidateSet::new(
        "interval set".to_string(),
        "participation index".to_string(),
        intervals,
        PlanarBooleanSplitIntervalAdmissionCounters::new(0, 0, 0, 0, 0, 0),
    )
}

fn admitted_interval(
    label: &str,
    event_identity: &str,
    carrier_identity: &str,
    source_edge_identity: &str,
    range: [f64; 2],
    kind: PlanarBooleanIntervalEventKind,
    sense: PlanarBooleanSourceIntervalSense,
) -> AdmittedIntervalSplitCandidate {
    let candidate =
        PlanarBooleanIntervalSplitCandidate::new(PlanarBooleanIntervalSplitCandidateInput {
            candidate_identity: format!("candidate:{label}"),
            interval_event_identity: event_identity.to_string(),
            interval_event_kind: kind,
            carrier_identity: carrier_identity.to_string(),
            source_edge_identity: source_edge_identity.to_string(),
            segment_identity: format!("segment:{label}"),
            source_interval_identity: format!("source-interval:{label}"),
            source_parameter_range: range,
            source_sense: sense,
            normalized_interval_identity: format!("normalized-interval:{event_identity}"),
            normalized_parameter_range: range,
            local_frame_identity: "local frame".to_string(),
            precision_basis_identity: "precision basis".to_string(),
            participation_row_identity: format!("row:{label}"),
            event_group_identities: vec![format!("event-group:{event_identity}")],
        });
    AdmittedIntervalSplitCandidate::new(candidate, range)
}
