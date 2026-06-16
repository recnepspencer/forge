use super::edge_splitting_raw_schedule_support::build_raw_edge_split_schedule_for_metaboss;
use super::metaboss_support::MetabossEventExtractionSubject;
use std::collections::BTreeMap;
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
    PlanarBooleanIntervalSubdivisionNormalizedScheduleSet, PlanarBooleanMicroIntervalPolicy,
};

pub(crate) fn assert_interval_subdivision_normalization_matches_metaboss(
    subject: &MetabossEventExtractionSubject,
) {
    let endpoint_normalized = build_endpoint_boundary_schedule_for_metaboss(subject);
    let interval_normalized = endpoint_normalized
        .normalize_overlap_interval_subdivisions(
            PlanarBooleanMicroIntervalPolicy::RequireExplicitDecision,
        )
        .expect("metaboss interval subdivisions should normalize from endpoint-boundary schedules");

    assert_eq!(
        interval_normalized.endpoint_boundary_schedule_set_identity(),
        endpoint_normalized.schedule_set_identity()
    );
    assert_interval_subdivision_counters_reconcile(&endpoint_normalized, &interval_normalized);
    assert_interval_subdivisions_consume_retained_interval_rows(
        &endpoint_normalized,
        &interval_normalized,
    );
}

pub(crate) fn build_endpoint_boundary_schedule_for_metaboss(
    subject: &MetabossEventExtractionSubject,
) -> PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet {
    build_raw_edge_split_schedule_for_metaboss(subject)
        .raw
        .canonicalize_split_schedule_order()
        .expect("raw metaboss split schedules should canonicalize before interval normalization")
        .collapse_duplicate_split_points()
        .expect("metaboss split schedules should duplicate-normalize before interval normalization")
        .normalize_endpoint_boundary_splits()
        .expect(
            "metaboss endpoint-boundary schedules should normalize before interval normalization",
        )
}

fn assert_interval_subdivision_counters_reconcile(
    endpoint_normalized: &PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
    interval_normalized: &PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
) {
    let retained_rows = retained_interval_row_count(endpoint_normalized);
    assert_eq!(
        interval_normalized.counters().normalized_schedules(),
        endpoint_normalized.schedules().len()
    );
    assert_eq!(
        interval_normalized
            .counters()
            .retained_interval_rows_inspected(),
        retained_rows
    );
    assert_eq!(
        interval_normalized
            .counters()
            .fragment_point_cuts_retained(),
        endpoint_normalized
            .schedules()
            .iter()
            .map(|schedule| schedule.fragment_cuts().len())
            .sum::<usize>()
    );
    assert_eq!(
        interval_normalized
            .counters()
            .endpoint_contact_decisions_retained(),
        endpoint_normalized.endpoint_contact_decisions().count()
    );
    assert_eq!(interval_normalized.counters().micro_intervals_admitted(), 0);
    assert_eq!(
        interval_normalized
            .counters()
            .micro_intervals_policy_required(),
        0
    );
}

fn assert_interval_subdivisions_consume_retained_interval_rows(
    endpoint_normalized: &PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
    interval_normalized: &PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
) {
    let expected = retained_interval_source_sense_by_identity(endpoint_normalized);
    let mut consumed = BTreeMap::<String, usize>::new();
    for subdivision in interval_normalized
        .schedules()
        .iter()
        .flat_map(|schedule| schedule.interval_subdivisions())
    {
        assert!(!subdivision.subdivision_identity().is_empty());
        assert!(!subdivision.normalized_interval_identity().is_empty());
        assert!(!subdivision.local_frame_identity().is_empty());
        assert!(!subdivision.precision_basis_identity().is_empty());
        for provenance_identity in subdivision.provenance_entry_identities() {
            let expected_sense = expected
                .get(provenance_identity)
                .expect("subdivision provenance must come from endpoint retained interval row");
            assert_eq!(subdivision.source_sense(), *expected_sense);
            *consumed.entry(provenance_identity.clone()).or_default() += 1;
        }
    }
    assert_eq!(consumed.len(), expected.len());
    assert!(
        interval_normalized
            .counters()
            .opposite_sense_rows_preserved()
            > 0
    );
}

fn retained_interval_source_sense_by_identity(
    endpoint_normalized: &PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
) -> BTreeMap<String, worth_spatial::facade::planar_boolean_events::PlanarBooleanSourceIntervalSense>
{
    endpoint_normalized
        .schedules()
        .iter()
        .flat_map(|schedule| schedule.retained_interval_entries())
        .map(|entry| (entry.entry_identity().to_string(), entry.source_sense()))
        .collect()
}

fn retained_interval_row_count(
    endpoint_normalized: &PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
) -> usize {
    endpoint_normalized
        .schedules()
        .iter()
        .map(|schedule| schedule.retained_interval_entries().len())
        .sum()
}
