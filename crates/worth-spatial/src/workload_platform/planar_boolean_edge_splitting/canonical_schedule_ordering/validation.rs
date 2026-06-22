use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::{
    PlanarBooleanRawEdgeSplitSchedule, PlanarBooleanRawEdgeSplitScheduleSet,
};

use super::denial::{
    PlanarBooleanOrderedEdgeSplitScheduleDenial, PlanarBooleanOrderedEdgeSplitScheduleDenialKind,
};
use super::ordered_schedule::{
    PlanarBooleanOrderedEdgeSplitSchedule, PlanarBooleanOrderedEdgeSplitScheduleSet,
};

pub(super) fn validate_ordered_schedule_set(
    raw: &PlanarBooleanRawEdgeSplitScheduleSet,
    ordered: &PlanarBooleanOrderedEdgeSplitScheduleSet,
) -> Result<(), PlanarBooleanOrderedEdgeSplitScheduleDenial> {
    if ordered.raw_schedule_set_identity() != raw.schedule_set_identity() {
        return invariant_denial(
            raw.schedule_set_identity(),
            "ordered set must preserve raw set identity",
        );
    }
    if ordered.schedules().len() != raw.schedules().len() {
        return invariant_denial(
            raw.schedule_set_identity(),
            "ordered schedule count must match raw schedule count",
        );
    }
    if ordered.counters().ordered_entries() != raw_total_entries(raw) {
        return invariant_denial(
            raw.schedule_set_identity(),
            "ordered entry count must match raw entry count",
        );
    }

    let raw_by_identity = raw_schedule_map(raw);
    for schedule in ordered.schedules() {
        let Some(raw_schedule) = raw_by_identity.get(schedule.raw_schedule_identity()) else {
            return invariant_denial(
                schedule.raw_schedule_identity(),
                "ordered schedule must point at a raw schedule",
            );
        };
        validate_ordered_schedule(raw_schedule, schedule)?;
    }
    Ok(())
}

fn validate_ordered_schedule(
    raw: &PlanarBooleanRawEdgeSplitSchedule,
    ordered: &PlanarBooleanOrderedEdgeSplitSchedule,
) -> Result<(), PlanarBooleanOrderedEdgeSplitScheduleDenial> {
    if ordered.source_edge_identity() != raw.source_edge_identity()
        || ordered.carrier_identity() != raw.carrier_identity()
    {
        return invariant_denial(
            raw.schedule_identity(),
            "ordered schedule must preserve source-edge carrier authority",
        );
    }
    if ordered.ordered_entries().len() != raw.entries().len() {
        return invariant_denial(
            raw.schedule_identity(),
            "ordered schedule must preserve raw entry multiplicity",
        );
    }
    let mut raw_entry_counts = raw_entry_count_map(raw);
    for entry in ordered.ordered_entries() {
        if entry.order_key().source_edge_identity() != ordered.source_edge_identity()
            || entry.order_key().carrier_identity() != entry.raw_entry().carrier_identity()
            || entry.order_key().event_identity() != entry.raw_entry().event_identity()
            || entry.order_key().event_group_identities()
                != entry.raw_entry().event_group_identities()
        {
            return invariant_denial(
                entry.ordered_entry_identity(),
                "order key must preserve raw source-edge carrier event basis",
            );
        }
        let Some(count) = raw_entry_counts.get_mut(entry.raw_entry().entry_identity()) else {
            return invariant_denial(
                entry.ordered_entry_identity(),
                "ordered entry must point at a raw entry",
            );
        };
        *count -= 1;
    }
    if raw_entry_counts.values().any(|count| *count != 0) {
        return invariant_denial(
            raw.schedule_identity(),
            "ordered entries must consume every raw entry exactly once",
        );
    }
    if !ordered
        .ordered_entries()
        .windows(2)
        .all(|pair| pair[0].order_key() <= pair[1].order_key())
    {
        return invariant_denial(
            raw.schedule_identity(),
            "ordered entries must be sorted by the explicit split schedule order key",
        );
    }
    Ok(())
}

fn raw_schedule_map(
    raw: &PlanarBooleanRawEdgeSplitScheduleSet,
) -> BTreeMap<&str, &PlanarBooleanRawEdgeSplitSchedule> {
    raw.schedules()
        .iter()
        .map(|schedule| (schedule.schedule_identity(), schedule))
        .collect()
}

fn raw_entry_count_map(raw: &PlanarBooleanRawEdgeSplitSchedule) -> BTreeMap<&str, isize> {
    let mut counts = BTreeMap::new();
    for entry in raw.entries() {
        *counts.entry(entry.entry_identity()).or_insert(0) += 1;
    }
    counts
}

fn raw_total_entries(raw: &PlanarBooleanRawEdgeSplitScheduleSet) -> usize {
    raw.schedules()
        .iter()
        .map(|schedule| schedule.entries().len())
        .sum()
}

fn invariant_denial<T>(
    evidence_identity: impl Into<String>,
    human_reason: &'static str,
) -> Result<T, PlanarBooleanOrderedEdgeSplitScheduleDenial> {
    Err(PlanarBooleanOrderedEdgeSplitScheduleDenial::new(
        PlanarBooleanOrderedEdgeSplitScheduleDenialKind::OrderedScheduleInvariantMismatch,
        evidence_identity,
        human_reason,
    ))
}
