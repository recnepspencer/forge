use super::edge_splitting_raw_schedule_support::build_raw_edge_split_schedule_for_metaboss;
use super::metaboss_support::MetabossEventExtractionSubject;
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
    PlanarBooleanNormalizedEdgeSplitScheduleSet, PlanarBooleanPointSplitPosture,
    PlanarBooleanRawEdgeSplitScheduleEntryKind,
};

pub(crate) fn assert_endpoint_boundary_normalization_matches_metaboss(
    subject: &MetabossEventExtractionSubject,
) {
    let raw_proof = build_raw_edge_split_schedule_for_metaboss(subject);
    let ordered = raw_proof
        .raw
        .canonicalize_split_schedule_order()
        .expect("raw metaboss split schedules should canonicalize before endpoint normalization");
    let normalized = ordered
        .collapse_duplicate_split_points()
        .expect("metaboss split schedules should duplicate-normalize");
    let endpoint_normalized = normalized
        .normalize_endpoint_boundary_splits()
        .expect("metaboss endpoint boundary splits should normalize");

    assert_eq!(
        endpoint_normalized.normalized_schedule_set_identity(),
        normalized.schedule_set_identity()
    );
    assert_endpoint_boundary_counters_reconcile(&normalized, &endpoint_normalized);
    assert_endpoint_decisions_preserve_normalized_cut_authority(&endpoint_normalized);
    assert_fragment_cuts_exclude_boundary_noops(&endpoint_normalized);
    assert_retained_intervals_pass_through(&normalized, &endpoint_normalized);
}

fn assert_endpoint_boundary_counters_reconcile(
    normalized: &PlanarBooleanNormalizedEdgeSplitScheduleSet,
    endpoint_normalized: &PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
) {
    let expected_decisions = expected_endpoint_decision_count(normalized);
    let expected_fragment_cuts = normalized_point_cut_count(normalized) - expected_decisions;
    assert_eq!(
        endpoint_normalized.counters().normalized_schedules(),
        normalized.schedules().len()
    );
    assert_eq!(
        endpoint_normalized.counters().inspected_point_cuts(),
        normalized_point_cut_count(normalized)
    );
    assert_eq!(
        endpoint_normalized.counters().fragment_point_cuts(),
        expected_fragment_cuts
    );
    assert_eq!(
        endpoint_normalized.endpoint_contact_decisions().count(),
        expected_decisions
    );
    assert_eq!(
        endpoint_normalized.counters().endpoint_noop_decisions(),
        expected_endpoint_noop_decision_count(normalized)
    );
    assert_eq!(
        endpoint_normalized.counters().shared_endpoint_decisions(),
        expected_shared_endpoint_decision_count(normalized)
    );
    assert_eq!(
        endpoint_normalized
            .counters()
            .t_junction_boundary_decisions(),
        expected_t_junction_boundary_decision_count(normalized)
    );
    assert!(
        endpoint_normalized.counters().shared_endpoint_decisions() > 0,
        "metaboss must prove at least one real shared-endpoint decision"
    );
}

fn assert_endpoint_decisions_preserve_normalized_cut_authority(
    endpoint_normalized: &PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
) {
    for decision in endpoint_normalized.endpoint_contact_decisions() {
        assert!(!decision.decision_identity().is_empty());
        assert!(!decision.normalized_cut_identity().is_empty());
        assert!(!decision.duplicate_report_identity().is_empty());
        assert!(!decision.source_edge_identity().is_empty());
        assert!(!decision.carrier_identity().is_empty());
        assert!(!decision.source_endpoint_identity().is_empty());
        assert!(!decision.projected_endpoint_fact_identity().is_empty());
        assert!(!decision.provenance_entry_identities().is_empty());
        assert!(!decision.event_group_identities().is_empty());
        if decision.posture() == PlanarBooleanPointSplitPosture::SharedEndpoint {
            assert!(!decision.shared_endpoint_source_identities().is_empty());
            assert_eq!(
                decision.shared_endpoint_source_identities().len(),
                decision.shared_endpoint_projection_fact_digests().len()
            );
        }
    }
}

fn assert_fragment_cuts_exclude_boundary_noops(
    endpoint_normalized: &PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
) {
    for cut in endpoint_normalized
        .schedules()
        .iter()
        .flat_map(|schedule| schedule.fragment_cuts())
    {
        if let PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(posture) = cut.kind() {
            assert_ne!(posture, PlanarBooleanPointSplitPosture::EndpointNoOp);
            assert_ne!(posture, PlanarBooleanPointSplitPosture::SharedEndpoint);
            assert!(
                !is_boundary_parameter(cut.parameter()),
                "endpoint-boundary normalization must not leave any boundary point cut in fragment construction"
            );
        }
    }
}

fn assert_retained_intervals_pass_through(
    normalized: &PlanarBooleanNormalizedEdgeSplitScheduleSet,
    endpoint_normalized: &PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
) {
    let expected = normalized
        .schedules()
        .iter()
        .flat_map(|schedule| schedule.retained_interval_entry_identities())
        .count();
    assert_eq!(
        endpoint_normalized.counters().retained_interval_entries(),
        expected
    );
}

fn expected_endpoint_noop_decision_count(
    normalized: &PlanarBooleanNormalizedEdgeSplitScheduleSet,
) -> usize {
    expected_decision_count_for_posture(normalized, PlanarBooleanPointSplitPosture::EndpointNoOp)
}

fn expected_shared_endpoint_decision_count(
    normalized: &PlanarBooleanNormalizedEdgeSplitScheduleSet,
) -> usize {
    expected_decision_count_for_posture(normalized, PlanarBooleanPointSplitPosture::SharedEndpoint)
}

fn expected_t_junction_boundary_decision_count(
    normalized: &PlanarBooleanNormalizedEdgeSplitScheduleSet,
) -> usize {
    expected_decision_count_for_posture(
        normalized,
        PlanarBooleanPointSplitPosture::TJunctionPromotion,
    )
}

fn expected_decision_count_for_posture(
    normalized: &PlanarBooleanNormalizedEdgeSplitScheduleSet,
    expected_posture: PlanarBooleanPointSplitPosture,
) -> usize {
    normalized
        .schedules()
        .iter()
        .flat_map(|schedule| schedule.cuts())
        .filter(|cut| match cut.kind() {
            PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(posture) => {
                posture == expected_posture && is_boundary_parameter(cut.parameter())
            }
            _ => false,
        })
        .count()
}

fn expected_endpoint_decision_count(
    normalized: &PlanarBooleanNormalizedEdgeSplitScheduleSet,
) -> usize {
    normalized
        .schedules()
        .iter()
        .flat_map(|schedule| schedule.cuts())
        .filter(|cut| match cut.kind() {
            PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(
                PlanarBooleanPointSplitPosture::EndpointNoOp
                | PlanarBooleanPointSplitPosture::SharedEndpoint
                | PlanarBooleanPointSplitPosture::TJunctionPromotion,
            ) => is_boundary_parameter(cut.parameter()),
            _ => false,
        })
        .count()
}

fn normalized_point_cut_count(normalized: &PlanarBooleanNormalizedEdgeSplitScheduleSet) -> usize {
    normalized
        .schedules()
        .iter()
        .map(|schedule| schedule.cuts().len())
        .sum()
}

fn is_boundary_parameter(parameter: f64) -> bool {
    parameter == 0.0 || parameter == 1.0
}
