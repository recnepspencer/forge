use std::collections::BTreeMap;

use super::edge_splitting_raw_schedule_support::build_raw_edge_split_schedule_for_metaboss;
use super::metaboss_support::MetabossEventExtractionSubject;
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanNormalizedEdgeSplitScheduleSet, PlanarBooleanRawEdgeSplitScheduleEntry,
    PlanarBooleanRawEdgeSplitScheduleEntryKind, PlanarBooleanRawEdgeSplitScheduleSet,
};

pub(crate) fn assert_normalized_edge_split_schedule_matches_metaboss(
    subject: &MetabossEventExtractionSubject,
) {
    let raw_proof = build_raw_edge_split_schedule_for_metaboss(subject);
    let ordered = raw_proof
        .raw
        .canonicalize_split_schedule_order()
        .expect("raw metaboss split schedules should canonicalize before normalization");
    let normalized = ordered
        .collapse_duplicate_split_points()
        .expect("metaboss ordered split schedules should normalize duplicate point cuts");

    assert_eq!(
        normalized.ordered_schedule_set_identity(),
        ordered.schedule_set_identity()
    );
    assert_eq!(
        normalized.counters().normalized_schedules(),
        ordered.schedules().len()
    );
    assert_normalized_counters_reconcile(&raw_proof.raw, &normalized);
    assert_normalized_schedules_preserve_ordered_authority(&normalized);
    assert_normalized_cuts_retain_raw_point_provenance(&raw_proof.raw, &normalized);
    assert_retained_interval_entries_match_raw_schedule(&raw_proof.raw, &normalized);
}

fn assert_normalized_counters_reconcile(
    raw: &PlanarBooleanRawEdgeSplitScheduleSet,
    normalized: &PlanarBooleanNormalizedEdgeSplitScheduleSet,
) {
    let expected_raw_points = raw_point_entry_count(raw);
    let expected_normalized_points = normalized_point_cut_count(normalized);
    let expected_retained_intervals = raw_interval_entry_count(raw);
    assert_eq!(normalized.counters().raw_point_cuts(), expected_raw_points);
    assert_eq!(
        normalized.counters().normalized_point_cuts(),
        expected_normalized_points
    );
    assert_eq!(
        normalized.counters().duplicate_reports_collapsed(),
        expected_raw_points - expected_normalized_points
    );
    assert_eq!(
        normalized.counters().provenance_rows_retained(),
        normalized_provenance_row_count(normalized)
    );
    assert_eq!(
        normalized.counters().provenance_rows_retained(),
        expected_raw_points
    );
    assert_eq!(
        normalized_provenance_row_count(normalized),
        expected_raw_points
    );
    assert_eq!(
        normalized.counters().retained_interval_entries(),
        expected_retained_intervals
    );
}

fn assert_normalized_schedules_preserve_ordered_authority(
    normalized: &PlanarBooleanNormalizedEdgeSplitScheduleSet,
) {
    for schedule in normalized.schedules() {
        assert!(!schedule.schedule_identity().is_empty());
        assert!(!schedule.ordered_schedule_identity().is_empty());
        for cut in schedule.cuts() {
            assert_eq!(cut.source_edge_identity(), schedule.source_edge_identity());
            assert_eq!(cut.carrier_identity(), schedule.carrier_identity());
            assert!(!cut.cut_identity().is_empty());
            assert!(!cut.duplicate_report_identity().is_empty());
            assert_eq!(cut.parameter_bits(), canonical_bits(cut.parameter()));
            assert!(!cut.local_frame_identity().is_empty());
            assert!(!cut.precision_basis_identity().is_empty());
        }
    }
}

fn assert_normalized_cuts_retain_raw_point_provenance(
    raw: &PlanarBooleanRawEdgeSplitScheduleSet,
    normalized: &PlanarBooleanNormalizedEdgeSplitScheduleSet,
) {
    let mut raw_entries_by_identity = raw_point_entry_multimap(raw);
    for cut in normalized
        .schedules()
        .iter()
        .flat_map(|schedule| schedule.cuts())
    {
        assert!(!cut.provenance_entry_identities().is_empty());
        let mut expected_events = Vec::new();
        let mut expected_event_groups = Vec::new();
        let mut expected_segment_pairs = Vec::new();
        let mut expected_predicate_receipts = Vec::new();
        for entry_identity in cut.provenance_entry_identities() {
            let entries = raw_entries_by_identity
                .get_mut(entry_identity.as_str())
                .expect("normalized cut provenance must point at a raw entry");
            let entry = entries
                .pop()
                .expect("normalized cut provenance must retain raw entry multiplicity");
            assert_eq!(cut.source_edge_identity(), entry.source_edge_identity());
            assert_eq!(cut.carrier_identity(), entry.carrier_identity());
            assert_eq!(cut.parameter_bits(), canonical_bits(entry.parameter()));
            assert_eq!(cut.kind(), entry.kind());
            assert_eq!(cut.local_frame_identity(), entry.local_frame_identity());
            assert_eq!(
                cut.precision_basis_identity(),
                entry.precision_basis_identity()
            );
            expected_events.push(entry.event_identity().to_string());
            expected_event_groups.extend(entry.event_group_identities().iter().cloned());
            expected_segment_pairs.extend(entry.segment_pair_identities().iter().cloned());
            expected_predicate_receipts
                .extend(entry.predicate_receipt_identities().iter().cloned());
        }
        let expected_events = canonical_values(expected_events);
        let expected_event_groups = canonical_values(expected_event_groups);
        let expected_segment_pairs = canonical_values(expected_segment_pairs);
        let expected_predicate_receipts = canonical_values(expected_predicate_receipts);
        assert_eq!(cut.event_identities(), expected_events.as_slice());
        assert_eq!(
            cut.event_group_identities(),
            expected_event_groups.as_slice()
        );
        assert_eq!(
            cut.segment_pair_identities(),
            expected_segment_pairs.as_slice()
        );
        assert_eq!(
            cut.predicate_receipt_identities(),
            expected_predicate_receipts.as_slice()
        );
    }
    assert!(
        raw_entries_by_identity.values().all(Vec::is_empty),
        "normalized cut provenance must consume every raw point row exactly once"
    );
}

fn assert_retained_interval_entries_match_raw_schedule(
    raw: &PlanarBooleanRawEdgeSplitScheduleSet,
    normalized: &PlanarBooleanNormalizedEdgeSplitScheduleSet,
) {
    let mut expected = raw
        .schedules()
        .iter()
        .flat_map(|schedule| schedule.entries())
        .filter(|entry| {
            matches!(
                entry.kind(),
                PlanarBooleanRawEdgeSplitScheduleEntryKind::Interval
            )
        })
        .map(|entry| entry.entry_identity().to_string())
        .collect::<Vec<_>>();
    let mut observed = normalized
        .schedules()
        .iter()
        .flat_map(|schedule| schedule.retained_interval_entry_identities())
        .cloned()
        .collect::<Vec<_>>();
    expected.sort();
    observed.sort();
    assert_eq!(observed, expected);
}

fn raw_point_entry_multimap(
    raw: &PlanarBooleanRawEdgeSplitScheduleSet,
) -> BTreeMap<&str, Vec<&PlanarBooleanRawEdgeSplitScheduleEntry>> {
    let mut entries_by_identity = BTreeMap::new();
    for entry in raw
        .schedules()
        .iter()
        .flat_map(|schedule| schedule.entries())
        .filter(|entry| {
            matches!(
                entry.kind(),
                PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(_)
            )
        })
    {
        entries_by_identity
            .entry(entry.entry_identity())
            .or_insert_with(Vec::new)
            .push(entry);
    }
    entries_by_identity
}

fn raw_point_entry_count(raw: &PlanarBooleanRawEdgeSplitScheduleSet) -> usize {
    raw.schedules()
        .iter()
        .flat_map(|schedule| schedule.entries())
        .filter(|entry| {
            matches!(
                entry.kind(),
                PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(_)
            )
        })
        .count()
}

fn raw_interval_entry_count(raw: &PlanarBooleanRawEdgeSplitScheduleSet) -> usize {
    raw.schedules()
        .iter()
        .flat_map(|schedule| schedule.entries())
        .filter(|entry| {
            matches!(
                entry.kind(),
                PlanarBooleanRawEdgeSplitScheduleEntryKind::Interval
            )
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

fn normalized_provenance_row_count(
    normalized: &PlanarBooleanNormalizedEdgeSplitScheduleSet,
) -> usize {
    normalized
        .schedules()
        .iter()
        .flat_map(|schedule| schedule.cuts())
        .map(|cut| cut.provenance_entry_identities().len())
        .sum()
}

fn canonical_bits(parameter: f64) -> u64 {
    if parameter == 0.0 {
        0.0f64.to_bits()
    } else {
        parameter.to_bits()
    }
}

fn canonical_values(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values.dedup();
    values
}
