use std::collections::{BTreeMap, VecDeque};

use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::proof::invalidation::progression::ReadyInvalidationBatch;
use crate::data::telemetry::InvalidationPerformedCounter;

use super::deduplication::{same_canonical_work, ReadyWorkKey};

pub(crate) struct ReadyQueueEntry {
    pub(crate) task_index: usize,
    pub(crate) ready: ReadyInvalidationBatch,
}

pub(crate) struct ReadyInvalidationQueue {
    order: VecDeque<ReadyWorkKey>,
    entries: BTreeMap<ReadyWorkKey, ReadyQueueEntry>,
}

impl ReadyInvalidationQueue {
    pub(crate) fn new() -> Self {
        Self {
            order: VecDeque::new(),
            entries: BTreeMap::new(),
        }
    }

    pub(crate) fn insert(
        &mut self,
        graph: &mut SignalGraph,
        entry: ReadyQueueEntry,
    ) -> Result<bool, SignalError> {
        let key = ReadyWorkKey::from_ready(&entry.ready);
        if let Some(existing) = self.entries.get(&key) {
            if !same_canonical_work(&existing.ready, &entry.ready) {
                return Err(SignalError::invalid_input(
                    "same-epoch invalidation dedup encountered different causal authority",
                ));
            }
            let telemetry = &mut graph.telemetry_mut().invalidation;
            telemetry.work_items_admitted += 1;
            telemetry.work_items_merged += 1;
            telemetry.ready_work_deduplicated += 1;
            let observed = graph.invalidation_performed_counter_state();
            observed.add(InvalidationPerformedCounter::WorkItemsAdmitted, 1);
            observed.add(InvalidationPerformedCounter::WorkItemsMerged, 1);
            return Ok(false);
        }
        if self.entries.is_empty() {
            graph
                .invalidation_performed_counter_state()
                .add(InvalidationPerformedCounter::BatchLocalAllocations, 1);
        }
        self.order.push_back(key);
        self.entries.insert(key, entry);
        let width = self.entries.len() as u64;
        let telemetry = &mut graph.telemetry_mut().invalidation;
        telemetry.work_items_admitted += 1;
        telemetry.ready_items_enqueued += 1;
        telemetry.maximum_ready_frontier_width = telemetry.maximum_ready_frontier_width.max(width);
        telemetry.retained_ready_frontier_width = width;
        let observed = graph.invalidation_performed_counter_state();
        observed.add(InvalidationPerformedCounter::WorkItemsAdmitted, 1);
        observed.add(InvalidationPerformedCounter::ReadyItemsEnqueued, 1);
        observed.record_max(
            InvalidationPerformedCounter::MaximumReadyFrontierWidth,
            width,
        );
        observed.record_max(InvalidationPerformedCounter::PeakBatchMemoryItems, width);
        observed.set(
            InvalidationPerformedCounter::RetainedReadyFrontierWidth,
            width,
        );
        Ok(true)
    }

    pub(crate) fn pop(
        &mut self,
        graph: &mut SignalGraph,
    ) -> Result<Option<ReadyQueueEntry>, SignalError> {
        let Some(key) = self.order.pop_front() else {
            return Ok(None);
        };
        let entry = self.entries.remove(&key).ok_or_else(|| {
            SignalError::internal("ready invalidation queue order drifted from stored entries")
        })?;
        let telemetry = &mut graph.telemetry_mut().invalidation;
        telemetry.ready_items_popped += 1;
        telemetry.retained_ready_frontier_width = self.entries.len() as u64;
        let observed = graph.invalidation_performed_counter_state();
        observed.add(InvalidationPerformedCounter::ReadyItemsPopped, 1);
        observed.set(
            InvalidationPerformedCounter::RetainedReadyFrontierWidth,
            self.entries.len() as u64,
        );
        Ok(Some(entry))
    }
}
