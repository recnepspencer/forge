use crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::PlanarBooleanRawEdgeSplitScheduleEntry;

use super::counters::PlanarBooleanOrderedEdgeSplitScheduleCounters;
use super::order_key::PlanarBooleanSplitScheduleOrderKey;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanOrderedEdgeSplitScheduleEntry {
    ordered_entry_identity: String,
    raw_entry: PlanarBooleanRawEdgeSplitScheduleEntry,
    order_key: PlanarBooleanSplitScheduleOrderKey,
    order_ordinal: usize,
}

impl PlanarBooleanOrderedEdgeSplitScheduleEntry {
    pub(crate) fn new(
        ordered_entry_identity: String,
        raw_entry: PlanarBooleanRawEdgeSplitScheduleEntry,
        order_key: PlanarBooleanSplitScheduleOrderKey,
        order_ordinal: usize,
    ) -> Self {
        Self {
            ordered_entry_identity,
            raw_entry,
            order_key,
            order_ordinal,
        }
    }

    pub fn ordered_entry_identity(&self) -> &str {
        &self.ordered_entry_identity
    }
    pub fn raw_entry(&self) -> &PlanarBooleanRawEdgeSplitScheduleEntry {
        &self.raw_entry
    }
    pub fn order_key(&self) -> &PlanarBooleanSplitScheduleOrderKey {
        &self.order_key
    }
    pub fn order_ordinal(&self) -> usize {
        self.order_ordinal
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanOrderedEdgeSplitSchedule {
    schedule_identity: String,
    raw_schedule_identity: String,
    source_edge_identity: String,
    carrier_identity: String,
    order_digest: String,
    ordered_entries: Vec<PlanarBooleanOrderedEdgeSplitScheduleEntry>,
}

impl PlanarBooleanOrderedEdgeSplitSchedule {
    pub(crate) fn new(
        schedule_identity: String,
        raw_schedule_identity: String,
        source_edge_identity: String,
        carrier_identity: String,
        order_digest: String,
        ordered_entries: Vec<PlanarBooleanOrderedEdgeSplitScheduleEntry>,
    ) -> Self {
        Self {
            schedule_identity,
            raw_schedule_identity,
            source_edge_identity,
            carrier_identity,
            order_digest,
            ordered_entries,
        }
    }

    pub fn schedule_identity(&self) -> &str {
        &self.schedule_identity
    }
    pub fn raw_schedule_identity(&self) -> &str {
        &self.raw_schedule_identity
    }
    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }
    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }
    pub fn order_digest(&self) -> &str {
        &self.order_digest
    }
    pub fn ordered_entries(&self) -> &[PlanarBooleanOrderedEdgeSplitScheduleEntry] {
        &self.ordered_entries
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanOrderedEdgeSplitScheduleSet {
    schedule_set_identity: String,
    raw_schedule_set_identity: String,
    schedules: Vec<PlanarBooleanOrderedEdgeSplitSchedule>,
    counters: PlanarBooleanOrderedEdgeSplitScheduleCounters,
}

impl PlanarBooleanOrderedEdgeSplitScheduleSet {
    pub(crate) fn new(
        schedule_set_identity: String,
        raw_schedule_set_identity: String,
        schedules: Vec<PlanarBooleanOrderedEdgeSplitSchedule>,
        counters: PlanarBooleanOrderedEdgeSplitScheduleCounters,
    ) -> Self {
        Self {
            schedule_set_identity,
            raw_schedule_set_identity,
            schedules,
            counters,
        }
    }

    pub fn schedule_set_identity(&self) -> &str {
        &self.schedule_set_identity
    }
    pub fn raw_schedule_set_identity(&self) -> &str {
        &self.raw_schedule_set_identity
    }
    pub fn schedules(&self) -> &[PlanarBooleanOrderedEdgeSplitSchedule] {
        &self.schedules
    }
    pub fn counters(&self) -> PlanarBooleanOrderedEdgeSplitScheduleCounters {
        self.counters
    }
}
