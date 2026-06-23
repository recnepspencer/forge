use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_edge_splitting::canonical_schedule_ordering::PlanarBooleanOrderedEdgeSplitSchedule;
use crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::{
    PlanarBooleanRawEdgeSplitScheduleEntry, PlanarBooleanRawEdgeSplitScheduleEntryKind,
};

use super::denial::PlanarBooleanDuplicateSplitNormalizationDenial;
use super::duplicate_key::PlanarBooleanDuplicateSplitCutKey;
use super::retained_interval_entry::PlanarBooleanRetainedIntervalSplitEntry;

pub(super) struct DuplicateSplitGrouping<'a> {
    point_groups: Vec<Vec<&'a PlanarBooleanRawEdgeSplitScheduleEntry>>,
    retained_interval_entries: Vec<PlanarBooleanRetainedIntervalSplitEntry>,
    raw_point_cuts: usize,
    retained_interval_entry_count: usize,
}

impl<'a> DuplicateSplitGrouping<'a> {
    pub(super) fn from_ordered_schedule(
        schedule: &'a PlanarBooleanOrderedEdgeSplitSchedule,
    ) -> Result<Self, PlanarBooleanDuplicateSplitNormalizationDenial> {
        let mut point_groups_by_key = BTreeMap::<
            PlanarBooleanDuplicateSplitCutKey,
            Vec<&PlanarBooleanRawEdgeSplitScheduleEntry>,
        >::new();
        let mut retained_interval_entries = Vec::new();
        let mut raw_point_cuts = 0;
        for ordered_entry in schedule.ordered_entries() {
            let raw_entry = ordered_entry.raw_entry();
            match raw_entry.kind() {
                PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(_) => {
                    raw_point_cuts += 1;
                    if let Some(key) =
                        PlanarBooleanDuplicateSplitCutKey::from_point_entry(raw_entry)
                    {
                        point_groups_by_key.entry(key).or_default().push(raw_entry);
                    }
                }
                PlanarBooleanRawEdgeSplitScheduleEntryKind::Interval => {
                    retained_interval_entries.push(
                        PlanarBooleanRetainedIntervalSplitEntry::from_raw_interval_entry(
                            raw_entry,
                        )?,
                    );
                }
            }
        }
        let retained_interval_entry_count = retained_interval_entries.len();
        Ok(Self {
            point_groups: point_groups_by_key.into_values().collect(),
            retained_interval_entries,
            raw_point_cuts,
            retained_interval_entry_count,
        })
    }

    pub(super) fn point_groups(&self) -> &[Vec<&'a PlanarBooleanRawEdgeSplitScheduleEntry>] {
        &self.point_groups
    }

    pub(super) fn retained_interval_entries(&self) -> &[PlanarBooleanRetainedIntervalSplitEntry] {
        &self.retained_interval_entries
    }

    pub(super) fn raw_point_cuts(&self) -> usize {
        self.raw_point_cuts
    }

    pub(super) fn retained_interval_entry_count(&self) -> usize {
        self.retained_interval_entry_count
    }
}
