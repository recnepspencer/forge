use std::collections::BTreeMap;

use super::edge_splitting_raw_schedule_support::build_raw_edge_split_schedule_for_metaboss;
use super::metaboss_support::MetabossEventExtractionSubject;
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanOrderedEdgeSplitScheduleSet, PlanarBooleanPointSplitPosture,
    PlanarBooleanRawEdgeSplitScheduleEntryKind, PlanarBooleanRawEdgeSplitScheduleSet,
};

pub(crate) fn assert_ordered_edge_split_schedule_matches_metaboss(
    subject: &MetabossEventExtractionSubject,
) {
    let raw_proof = build_raw_edge_split_schedule_for_metaboss(subject);
    let ordered = raw_proof
        .raw
        .canonicalize_split_schedule_order()
        .expect("raw metaboss split schedules should canonicalize before normalization");

    assert_eq!(
        ordered.raw_schedule_set_identity(),
        raw_proof.raw.schedule_set_identity()
    );
    assert_eq!(
        ordered.counters().ordered_schedules(),
        raw_proof.raw.schedules().len()
    );
    assert_eq!(
        ordered.counters().ordered_entries(),
        raw_entry_count(&raw_proof.raw)
    );
    assert_ordered_schedules_preserve_raw_authority(&raw_proof.raw, &ordered);
    assert_ordered_entries_retain_raw_multiplicity(&raw_proof.raw, &ordered);
    assert_order_keys_are_explicit_and_sorted(&ordered);
}

fn assert_ordered_schedules_preserve_raw_authority(
    raw: &PlanarBooleanRawEdgeSplitScheduleSet,
    ordered: &PlanarBooleanOrderedEdgeSplitScheduleSet,
) {
    let raw_by_identity = raw
        .schedules()
        .iter()
        .map(|schedule| (schedule.schedule_identity(), schedule))
        .collect::<BTreeMap<_, _>>();
    for schedule in ordered.schedules() {
        let raw_schedule = raw_by_identity
            .get(schedule.raw_schedule_identity())
            .expect("ordered schedule must bind a raw schedule");
        assert_eq!(
            schedule.source_edge_identity(),
            raw_schedule.source_edge_identity()
        );
        assert_eq!(schedule.carrier_identity(), raw_schedule.carrier_identity());
        assert!(!schedule.schedule_identity().is_empty());
        assert!(!schedule.order_digest().is_empty());
    }
}

fn assert_ordered_entries_retain_raw_multiplicity(
    raw: &PlanarBooleanRawEdgeSplitScheduleSet,
    ordered: &PlanarBooleanOrderedEdgeSplitScheduleSet,
) {
    let mut expected = raw
        .schedules()
        .iter()
        .flat_map(|schedule| schedule.entries())
        .map(|entry| entry.entry_identity().to_string())
        .collect::<Vec<_>>();
    let mut observed = ordered
        .schedules()
        .iter()
        .flat_map(|schedule| schedule.ordered_entries())
        .map(|entry| entry.raw_entry().entry_identity().to_string())
        .collect::<Vec<_>>();
    expected.sort();
    observed.sort();
    assert_eq!(observed, expected);
}

fn assert_order_keys_are_explicit_and_sorted(ordered: &PlanarBooleanOrderedEdgeSplitScheduleSet) {
    for schedule in ordered.schedules() {
        assert!(schedule.ordered_entries().windows(2).all(|pair| {
            pair[0].order_key() <= pair[1].order_key()
                && pair[0].order_ordinal() < pair[1].order_ordinal()
        }));
        for entry in schedule.ordered_entries() {
            let key = entry.order_key();
            assert_eq!(key.source_edge_identity(), schedule.source_edge_identity());
            assert_eq!(key.carrier_identity(), entry.raw_entry().carrier_identity());
            assert_eq!(key.event_identity(), entry.raw_entry().event_identity());
            assert_eq!(
                key.event_group_identities(),
                entry.raw_entry().event_group_identities()
            );
            assert!(!key.event_identity().is_empty());
            assert!(!key.candidate_identity().is_empty());
            assert_eq!(
                key.entry_kind_rank(),
                expected_entry_kind_rank(entry.raw_entry().kind())
            );
            assert!(entry.raw_entry().parameter().is_finite());
        }
    }
}

fn expected_entry_kind_rank(kind: PlanarBooleanRawEdgeSplitScheduleEntryKind) -> u8 {
    match kind {
        PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(posture) => match posture {
            PlanarBooleanPointSplitPosture::InteriorSplit
            | PlanarBooleanPointSplitPosture::TJunctionPromotion => 0,
            PlanarBooleanPointSplitPosture::EndpointNoOp
            | PlanarBooleanPointSplitPosture::SharedEndpoint => 1,
        },
        PlanarBooleanRawEdgeSplitScheduleEntryKind::Interval => 2,
    }
}

fn raw_entry_count(raw: &PlanarBooleanRawEdgeSplitScheduleSet) -> usize {
    raw.schedules()
        .iter()
        .map(|schedule| schedule.entries().len())
        .sum()
}
