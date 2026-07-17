use std::collections::{BTreeMap, BTreeSet};

use crate::{ProtectedReferenceRange, ReclaimCandidateSet, RootEpoch};

use super::{HazardLeaseCounterSnapshot, HazardLeaseGeneration, HazardLeaseKind, HazardLeaseSlot};

type LeaseKey = (HazardLeaseSlot, HazardLeaseGeneration);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HazardLeaseEpochIndexSnapshot {
    buckets: BTreeMap<u64, HazardLeaseEpochBucket>,
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
    buckets: BTreeMap<u64, HazardLeaseEpochBucket>,
    total_live_entries: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct HazardLeaseEpochBucket {
    entries: BTreeMap<LeaseKey, HazardLeaseEpochIndexEntry>,
    range_handles: BTreeMap<ProtectedReferenceRange, BTreeSet<LeaseKey>>,
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
        let inserted = self
            .buckets
            .entry(entry.root_epoch.get())
            .or_default()
            .insert(entry);
        self.total_live_entries += usize::from(inserted);
    }

    pub(crate) fn remove(
        &mut self,
        root_epoch: RootEpoch,
        slot: HazardLeaseSlot,
        generation: HazardLeaseGeneration,
    ) -> bool {
        let epoch = root_epoch.get();
        let Some(bucket) = self.buckets.get_mut(&epoch) else {
            return false;
        };
        if !bucket.remove((slot, generation)) {
            return false;
        }
        self.total_live_entries -= 1;
        if bucket.is_empty() {
            self.buckets.remove(&epoch);
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
    fn insert(&mut self, entry: HazardLeaseEpochIndexEntry) -> bool {
        let key = (entry.slot, entry.generation);
        if self.entries.contains_key(&key) {
            return false;
        }
        for range in entry.ranges.iter().copied() {
            self.range_handles.entry(range).or_default().insert(key);
        }
        self.entries.insert(key, entry);
        true
    }

    fn remove(&mut self, key: LeaseKey) -> bool {
        let Some(entry) = self.entries.remove(&key) else {
            return false;
        };
        for range in entry.ranges {
            let remove_bucket = self.range_handles.get_mut(&range).is_some_and(|handles| {
                handles.remove(&key);
                handles.is_empty()
            });
            if remove_bucket {
                self.range_handles.remove(&range);
            }
        }
        true
    }

    fn first_overlap(
        &self,
        candidates: &ReclaimCandidateSet,
        mut counters: HazardLeaseCounterSnapshot,
    ) -> HazardLeaseOverlapScan {
        let mut first_overlap = None;
        let mut touched_entries = BTreeSet::new();
        for (range, handles) in &self.range_handles {
            counters = counters.with_range_bucket_lookup();
            if !candidates
                .candidate_ranges()
                .iter()
                .any(|candidate| range.intersects(*candidate))
            {
                continue;
            }
            for key in handles {
                if !touched_entries.insert(*key) {
                    continue;
                }
                let entry = self
                    .entries
                    .get(key)
                    .expect("range handle always resolves to a canonical lease entry");
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
            hazard_entries_touched: touched_entries.len() as u64,
        }
    }

    fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl HazardLeaseEpochIndexSnapshot {
    pub(crate) fn first_overlap(&self, candidates: &ReclaimCandidateSet) -> HazardLeaseOverlapScan {
        let Some(bucket) = self.buckets.get(&candidates.root_epoch().get()) else {
            return HazardLeaseOverlapScan {
                counters: self.counters,
                first_overlap: None,
                epoch_buckets_touched: 0,
                hazard_entries_touched: 0,
            };
        };
        let mut overlap = bucket.first_overlap(candidates, self.counters);
        overlap.epoch_buckets_touched = 1;
        overlap
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
