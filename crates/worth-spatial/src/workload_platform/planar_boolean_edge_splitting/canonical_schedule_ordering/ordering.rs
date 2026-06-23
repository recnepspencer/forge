use crate::workload_platform::planar_boolean_edge_splitting::canonical_parameter::canonical_parameter_bits;
use crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::{
    PlanarBooleanRawEdgeSplitSchedule, PlanarBooleanRawEdgeSplitScheduleEntry,
    PlanarBooleanRawEdgeSplitScheduleSet,
};

use super::counters::PlanarBooleanOrderedEdgeSplitScheduleCounters;
use super::denial::PlanarBooleanOrderedEdgeSplitScheduleDenial;
use super::identity::{
    ordered_entry_identity, ordered_schedule_identity, ordered_schedule_set_identity,
    schedule_order_digest,
};
use super::order_key::PlanarBooleanSplitScheduleOrderKey;
use super::ordered_schedule::{
    PlanarBooleanOrderedEdgeSplitSchedule, PlanarBooleanOrderedEdgeSplitScheduleEntry,
    PlanarBooleanOrderedEdgeSplitScheduleSet,
};
use super::validation::validate_ordered_schedule_set;

impl PlanarBooleanRawEdgeSplitScheduleSet {
    pub fn canonicalize_split_schedule_order(
        &self,
    ) -> Result<PlanarBooleanOrderedEdgeSplitScheduleSet, PlanarBooleanOrderedEdgeSplitScheduleDenial>
    {
        let mut schedules = build_ordered_schedules(self)?;
        sort_schedules_by_split_schedule_authority(&mut schedules);
        let ordered_entries = schedules
            .iter()
            .map(|schedule| schedule.ordered_entries().len())
            .sum();
        let equal_parameter_ties = schedules
            .iter()
            .map(|schedule| count_equal_parameter_ties(schedule.ordered_entries()))
            .sum();
        let counters = PlanarBooleanOrderedEdgeSplitScheduleCounters::new(
            schedules.len(),
            ordered_entries,
            equal_parameter_ties,
        );
        let set_identity = ordered_schedule_set_identity(self.schedule_set_identity(), &schedules);
        let ordered = PlanarBooleanOrderedEdgeSplitScheduleSet::new(
            set_identity,
            self.schedule_set_identity().to_string(),
            schedules,
            counters,
        );
        validate_ordered_schedule_set(self, &ordered)?;
        Ok(ordered)
    }
}

fn build_ordered_schedules(
    raw: &PlanarBooleanRawEdgeSplitScheduleSet,
) -> Result<Vec<PlanarBooleanOrderedEdgeSplitSchedule>, PlanarBooleanOrderedEdgeSplitScheduleDenial>
{
    let mut schedules = Vec::with_capacity(raw.schedules().len());
    for raw_schedule in raw.schedules() {
        schedules.push(build_ordered_schedule(raw_schedule)?);
    }
    Ok(schedules)
}

fn build_ordered_schedule(
    raw_schedule: &PlanarBooleanRawEdgeSplitSchedule,
) -> Result<PlanarBooleanOrderedEdgeSplitSchedule, PlanarBooleanOrderedEdgeSplitScheduleDenial> {
    let mut keyed_entries = build_order_keyed_entries(raw_schedule)?;
    sort_entries_by_split_schedule_order_key(&mut keyed_entries);
    let ordered_entries = assign_order_ordinals(keyed_entries);
    let order_digest = schedule_order_digest(raw_schedule.schedule_identity(), &ordered_entries);
    let schedule_identity =
        ordered_schedule_identity(raw_schedule.schedule_identity(), &order_digest);
    Ok(PlanarBooleanOrderedEdgeSplitSchedule::new(
        schedule_identity,
        raw_schedule.schedule_identity().to_string(),
        raw_schedule.source_edge_identity().to_string(),
        raw_schedule.carrier_identity().to_string(),
        order_digest,
        ordered_entries,
    ))
}

fn build_order_keyed_entries(
    raw_schedule: &PlanarBooleanRawEdgeSplitSchedule,
) -> Result<Vec<OrderKeyedRawEntry>, PlanarBooleanOrderedEdgeSplitScheduleDenial> {
    let mut keyed_entries = Vec::with_capacity(raw_schedule.entries().len());
    for raw_entry in raw_schedule.entries() {
        keyed_entries.push((
            PlanarBooleanSplitScheduleOrderKey::from_entry(raw_entry)?,
            raw_entry.clone(),
        ));
    }
    Ok(keyed_entries)
}

fn sort_entries_by_split_schedule_order_key(keyed_entries: &mut [OrderKeyedRawEntry]) {
    keyed_entries.sort_by(|left, right| left.0.cmp(&right.0));
}

fn sort_schedules_by_split_schedule_authority(
    schedules: &mut [PlanarBooleanOrderedEdgeSplitSchedule],
) {
    schedules.sort_by(|left, right| {
        left.source_edge_identity()
            .cmp(right.source_edge_identity())
            .then_with(|| left.carrier_identity().cmp(right.carrier_identity()))
            .then_with(|| {
                left.raw_schedule_identity()
                    .cmp(right.raw_schedule_identity())
            })
    });
}

fn assign_order_ordinals(
    keyed_entries: Vec<OrderKeyedRawEntry>,
) -> Vec<PlanarBooleanOrderedEdgeSplitScheduleEntry> {
    let mut ordered_entries = Vec::with_capacity(keyed_entries.len());
    for (ordinal, (order_key, raw_entry)) in keyed_entries.into_iter().enumerate() {
        let identity = ordered_entry_identity(raw_entry.entry_identity(), ordinal);
        ordered_entries.push(PlanarBooleanOrderedEdgeSplitScheduleEntry::new(
            identity, raw_entry, order_key, ordinal,
        ));
    }
    ordered_entries
}

fn count_equal_parameter_ties(
    ordered_entries: &[PlanarBooleanOrderedEdgeSplitScheduleEntry],
) -> usize {
    ordered_entries
        .windows(2)
        .filter(|pair| {
            canonical_parameter_bits(pair[0].raw_entry().parameter())
                == canonical_parameter_bits(pair[1].raw_entry().parameter())
        })
        .count()
}

type OrderKeyedRawEntry = (
    PlanarBooleanSplitScheduleOrderKey,
    PlanarBooleanRawEdgeSplitScheduleEntry,
);
