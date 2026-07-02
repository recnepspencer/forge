use crate::{ProtectedReferenceRange, ReclaimCandidateSet, RootEpoch};

use super::{HazardLeaseCounterSnapshot, HazardLeaseGeneration, HazardLeaseKind, HazardLeaseSlot};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HazardLeaseEpochIndexSnapshot {
    buckets: Vec<HazardLeaseEpochBucket>,
    counters: HazardLeaseCounterSnapshot,
    total_live_entries: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HazardLeaseOverlap {
    slot: HazardLeaseSlot,
    generation: HazardLeaseGeneration,
    kind: HazardLeaseKind,
    overlapping_ranges: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct HazardLeaseEpochIndexEntry {
    slot: HazardLeaseSlot,
    generation: HazardLeaseGeneration,
    kind: HazardLeaseKind,
    root_epoch: RootEpoch,
    ranges: Vec<ProtectedReferenceRange>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct HazardLeaseEpochIndex {
    buckets: Vec<HazardLeaseEpochBucket>,
    total_live_entries: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HazardLeaseEpochBucket {
    root_epoch: RootEpoch,
    range_buckets: Vec<HazardLeaseRangeBucket>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HazardLeaseRangeBucket {
    range: ProtectedReferenceRange,
    entries: Vec<HazardLeaseEpochIndexEntry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HazardLeaseOverlapScan {
    pub counters: HazardLeaseCounterSnapshot,
    pub first_overlap: Option<HazardLeaseOverlap>,
    pub epoch_buckets_touched: u64,
    pub hazard_entries_touched: u64,
}

impl HazardLeaseEpochIndex {
    pub(crate) fn insert(&mut self, entry: HazardLeaseEpochIndexEntry) {
        let Some(epoch_bucket) = self
            .buckets
            .iter_mut()
            .find(|bucket| bucket.root_epoch.has_same_epoch_value(entry.root_epoch))
        else {
            self.buckets.push(HazardLeaseEpochBucket::from_entry(entry));
            self.total_live_entries += 1;
            return;
        };
        epoch_bucket.insert(entry);
        self.total_live_entries += 1;
    }

    pub(crate) fn remove(
        &mut self,
        root_epoch: RootEpoch,
        slot: HazardLeaseSlot,
        generation: HazardLeaseGeneration,
    ) -> bool {
        let Some(bucket_index) = self
            .buckets
            .iter()
            .position(|bucket| bucket.root_epoch.has_same_epoch_value(root_epoch))
        else {
            return false;
        };
        let removed = self.buckets[bucket_index].remove(slot, generation);
        if !removed {
            return false;
        }
        self.total_live_entries -= 1;
        if self.buckets[bucket_index].is_empty() {
            self.buckets.swap_remove(bucket_index);
        }
        true
    }

    pub(crate) fn snapshot(
        &self,
        counters: HazardLeaseCounterSnapshot,
    ) -> HazardLeaseEpochIndexSnapshot {
        HazardLeaseEpochIndexSnapshot {
            buckets: self.buckets.clone(),
            counters,
            total_live_entries: self.total_live_entries,
        }
    }
}

impl HazardLeaseEpochBucket {
    fn from_entry(entry: HazardLeaseEpochIndexEntry) -> Self {
        let mut bucket = Self {
            root_epoch: entry.root_epoch,
            range_buckets: Vec::new(),
        };
        bucket.insert(entry);
        bucket
    }

    fn insert(&mut self, entry: HazardLeaseEpochIndexEntry) {
        for range in entry.ranges.iter().copied() {
            let Some(range_bucket) = self
                .range_buckets
                .iter_mut()
                .find(|bucket| bucket.range == range)
            else {
                self.range_buckets.push(HazardLeaseRangeBucket {
                    range,
                    entries: vec![entry.clone()],
                });
                continue;
            };
            range_bucket.entries.push(entry.clone());
        }
    }

    fn remove(&mut self, slot: HazardLeaseSlot, generation: HazardLeaseGeneration) -> bool {
        let mut removed = false;
        let mut bucket_index = 0;
        while bucket_index < self.range_buckets.len() {
            let bucket = &mut self.range_buckets[bucket_index];
            let before = bucket.entries.len();
            bucket
                .entries
                .retain(|entry| entry.slot != slot || entry.generation != generation);
            removed |= bucket.entries.len() != before;
            if bucket.entries.is_empty() {
                self.range_buckets.swap_remove(bucket_index);
            } else {
                bucket_index += 1;
            }
        }
        removed
    }

    fn first_overlap(
        &self,
        candidates: &ReclaimCandidateSet,
        mut counters: HazardLeaseCounterSnapshot,
    ) -> HazardLeaseOverlapScan {
        let mut first_overlap = None;
        let mut hazard_entries_touched = 0;
        let mut touched_entries = Vec::new();
        for range_bucket in &self.range_buckets {
            counters = counters.with_range_bucket_lookup();
            if !candidates
                .candidate_ranges()
                .iter()
                .any(|candidate| range_bucket.range.intersects(*candidate))
            {
                continue;
            }
            for entry in &range_bucket.entries {
                if touched_entries
                    .iter()
                    .any(|touched| *touched == (entry.slot, entry.generation))
                {
                    continue;
                }
                touched_entries.push((entry.slot, entry.generation));
                hazard_entries_touched += 1;
                let intersection = candidates.ranges().bounded_intersection(&entry.ranges);
                counters = counters.with_lookup(
                    entry.ranges.len() as u64,
                    intersection.range_comparisons(),
                    intersection.overlapping_ranges(),
                );
                if first_overlap.is_none() && intersection.overlapping_ranges() > 0 {
                    first_overlap = Some(HazardLeaseOverlap {
                        slot: entry.slot,
                        generation: entry.generation,
                        kind: entry.kind,
                        overlapping_ranges: intersection.overlapping_ranges(),
                    });
                }
            }
        }
        HazardLeaseOverlapScan {
            counters,
            first_overlap,
            epoch_buckets_touched: 0,
            hazard_entries_touched,
        }
    }

    fn is_empty(&self) -> bool {
        self.range_buckets.is_empty()
    }
}

impl HazardLeaseEpochIndexSnapshot {
    pub(crate) fn first_overlap(&self, candidates: &ReclaimCandidateSet) -> HazardLeaseOverlapScan {
        let mut counters = self.counters;
        let mut first_overlap = None;
        let mut epoch_buckets_touched = 0;
        let mut hazard_entries_touched = 0;
        for bucket in &self.buckets {
            if !bucket
                .root_epoch
                .has_same_epoch_value(candidates.root_epoch())
            {
                continue;
            }
            epoch_buckets_touched += 1;
            let overlap = bucket.first_overlap(candidates, counters);
            counters = overlap.counters;
            hazard_entries_touched += overlap.hazard_entries_touched;
            if first_overlap.is_none() {
                first_overlap = overlap.first_overlap;
            }
        }
        HazardLeaseOverlapScan {
            counters,
            first_overlap,
            epoch_buckets_touched,
            hazard_entries_touched,
        }
    }

    pub const fn counters(&self) -> HazardLeaseCounterSnapshot {
        self.counters
    }

    pub fn live_entries(&self) -> usize {
        self.total_live_entries
    }
}

impl HazardLeaseEpochIndexEntry {
    pub(crate) fn new(
        slot: HazardLeaseSlot,
        generation: HazardLeaseGeneration,
        kind: HazardLeaseKind,
        root_epoch: RootEpoch,
        ranges: Vec<ProtectedReferenceRange>,
    ) -> Self {
        Self {
            slot,
            generation,
            kind,
            root_epoch,
            ranges,
        }
    }
}

impl HazardLeaseOverlap {
    pub const fn slot(self) -> HazardLeaseSlot {
        self.slot
    }

    pub const fn generation(self) -> HazardLeaseGeneration {
        self.generation
    }

    pub const fn kind(self) -> HazardLeaseKind {
        self.kind
    }

    pub const fn overlapping_ranges(self) -> u64 {
        self.overlapping_ranges
    }
}
