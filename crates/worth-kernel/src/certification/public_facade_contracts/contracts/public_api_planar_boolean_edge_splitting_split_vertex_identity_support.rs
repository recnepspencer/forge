#![allow(dead_code)]

use super::edge_splitting_interval_subdivision_support::build_endpoint_boundary_schedule_for_metaboss;
use super::metaboss_support::MetabossEventExtractionSubject;
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanIntervalSubdivisionNormalizedScheduleSet, PlanarBooleanMicroIntervalPolicy,
    PlanarBooleanSplitVertexCoalescenceReason, PlanarBooleanSplitVertexIdentitySet,
};

pub(crate) fn assert_split_vertex_identities_match_metaboss(
    subject: &MetabossEventExtractionSubject,
) {
    let interval_normalized = build_interval_subdivision_schedule_for_metaboss(subject);
    let split_vertices = interval_normalized
        .mint_split_vertex_identities()
        .expect("metaboss split vertices should mint from interval-subdivision schedules");

    assert_eq!(
        split_vertices.interval_subdivision_schedule_set_identity(),
        interval_normalized.schedule_set_identity()
    );
    assert_split_vertex_identity_counters_reconcile(&interval_normalized, &split_vertices);
    assert_split_vertex_rows_preserve_authority(&split_vertices);
    assert_split_vertex_coalescence_decisions_are_proof_backed(&split_vertices);
}

pub(crate) fn build_interval_subdivision_schedule_for_metaboss(
    subject: &MetabossEventExtractionSubject,
) -> PlanarBooleanIntervalSubdivisionNormalizedScheduleSet {
    build_endpoint_boundary_schedule_for_metaboss(subject)
        .normalize_overlap_interval_subdivisions(
            PlanarBooleanMicroIntervalPolicy::RequireExplicitDecision,
        )
        .expect("metaboss interval subdivisions should normalize before split vertex minting")
}

fn assert_split_vertex_identity_counters_reconcile(
    interval_normalized: &PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    split_vertices: &PlanarBooleanSplitVertexIdentitySet,
) {
    let expected_point_cuts = interval_normalized
        .schedules()
        .iter()
        .map(|schedule| schedule.fragment_cuts().len())
        .sum::<usize>();
    let expected_interval_endpoints = interval_normalized
        .schedules()
        .iter()
        .map(|schedule| schedule.interval_subdivisions().len().saturating_mul(2))
        .sum::<usize>();
    let expected_endpoint_decisions = interval_normalized
        .schedules()
        .iter()
        .map(|schedule| schedule.endpoint_contact_decisions().len())
        .sum::<usize>();

    assert_eq!(
        split_vertices.counters().schedules_inspected(),
        interval_normalized.schedules().len()
    );
    assert_eq!(
        split_vertices.counters().point_cuts_inspected(),
        expected_point_cuts
    );
    assert_eq!(
        split_vertices
            .counters()
            .interval_endpoint_candidates_inspected(),
        expected_interval_endpoints
    );
    assert_eq!(
        split_vertices
            .counters()
            .endpoint_contact_decisions_inspected(),
        expected_endpoint_decisions
    );
    assert_eq!(
        split_vertices.counters().split_vertices_minted(),
        split_vertices.vertices().count()
    );
    assert_eq!(
        split_vertices
            .counters()
            .coordinate_only_attempts_rejected(),
        0
    );
}

fn assert_split_vertex_rows_preserve_authority(
    split_vertices: &PlanarBooleanSplitVertexIdentitySet,
) {
    for vertex in split_vertices.vertices() {
        assert!(!vertex.split_vertex_identity().is_empty());
        assert!(!vertex.source_edge_identity().is_empty());
        assert!(!vertex.carrier_identity().is_empty());
        assert!(vertex.normalized_parameter().is_finite());
        assert!(!vertex.local_frame_identity().is_empty());
        assert!(!vertex.precision_basis_identity().is_empty());
        assert!(!vertex.coalescence_provenance().is_empty());
        if !vertex.point_cut_identities().is_empty() {
            assert!(!vertex.parameter_fact_identities().is_empty());
        }
        assert!(
            !vertex.point_cut_identities().is_empty()
                || !vertex.interval_subdivision_identities().is_empty()
        );
    }
}

fn assert_split_vertex_coalescence_decisions_are_proof_backed(
    split_vertices: &PlanarBooleanSplitVertexIdentitySet,
) {
    assert!(split_vertices
        .coalescence_decisions()
        .any(|decision| decision.reason()
            == PlanarBooleanSplitVertexCoalescenceReason::IntervalEndpointAndPointCut));
    for decision in split_vertices.coalescence_decisions() {
        assert!(!decision.decision_identity().is_empty());
        assert!(!decision.split_vertex_identity().is_empty());
        assert!(decision.input_identities().len() > 1);
        assert!(!decision.event_group_identities().is_empty());
    }
}
