use worth_store_physical_backend::{MediaCounterSnapshot, MediaOperationRole};
use worth_store_physical_format::PersistedRecordIdentity;

use super::super::PhysicalRecordId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime::record_serving) struct PublicationObservation {
    pub(in crate::physical_runtime::record_serving) records: u64,
    pub(in crate::physical_runtime::record_serving) logical_bytes: u64,
    pub(in crate::physical_runtime::record_serving) completed_bytes: u64,
    pub(in crate::physical_runtime::record_serving) segment_artifacts: u64,
    pub(in crate::physical_runtime::record_serving) extent_artifacts: u64,
    pub(in crate::physical_runtime::record_serving) transfer_count: u64,
    pub(in crate::physical_runtime::record_serving) peak_transfer_width: u64,
    pub(in crate::physical_runtime::record_serving) explicit_copy_count: u64,
    pub(in crate::physical_runtime::record_serving) copied_bytes: u64,
    pub(in crate::physical_runtime::record_serving) peak_scratch_bytes: u64,
    pub(in crate::physical_runtime::record_serving) manifest_blocks_read: u64,
    pub(in crate::physical_runtime::record_serving) manifest_comparisons: u64,
    pub(in crate::physical_runtime::record_serving) manifest_bytes_read: u64,
}

impl PublicationObservation {
    pub(in crate::physical_runtime::record_serving) fn observe_transfer(&mut self, bytes: usize) {
        self.peak_transfer_width = self.peak_transfer_width.max(bytes as u64);
    }

    pub(in crate::physical_runtime::record_serving) fn observe_copy(&mut self, bytes: usize) {
        self.explicit_copy_count = self.explicit_copy_count.saturating_add(1);
        self.copied_bytes = self.copied_bytes.saturating_add(bytes as u64);
    }

    pub(in crate::physical_runtime::record_serving) fn observe_scratch(&mut self, bytes: usize) {
        self.peak_scratch_bytes = self.peak_scratch_bytes.max(bytes as u64);
    }

    pub(in crate::physical_runtime::record_serving) fn complete(
        &mut self,
        before: MediaCounterSnapshot,
        after: MediaCounterSnapshot,
    ) {
        self.completed_bytes = self.logical_bytes;
        self.transfer_count = transfer_attempts(after).saturating_sub(transfer_attempts(before));
    }
}

const fn transfer_attempts(counters: MediaCounterSnapshot) -> u64 {
    counters
        .attempts_for(MediaOperationRole::PositionedRead)
        .saturating_add(counters.attempts_for(MediaOperationRole::PositionedWrite))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordAppendObservation {
    value: PublicationObservation,
}

impl RecordAppendObservation {
    pub const fn records(self) -> u64 {
        self.value.records
    }
    pub const fn logical_bytes(self) -> u64 {
        self.value.logical_bytes
    }
    pub const fn segment_artifacts(self) -> u64 {
        self.value.segment_artifacts
    }
    pub const fn extent_artifacts(self) -> u64 {
        self.value.extent_artifacts
    }
    pub const fn bytes_requested(self) -> u64 {
        self.value.logical_bytes
    }
    pub const fn bytes_completed(self) -> u64 {
        self.value.completed_bytes
    }
    pub const fn transfer_count(self) -> u64 {
        self.value.transfer_count
    }
    pub const fn peak_transfer_width(self) -> u64 {
        self.value.peak_transfer_width
    }
    pub const fn explicit_copy_count(self) -> u64 {
        self.value.explicit_copy_count
    }
    pub const fn copied_bytes(self) -> u64 {
        self.value.copied_bytes
    }
    pub const fn peak_scratch_bytes(self) -> u64 {
        self.value.peak_scratch_bytes
    }
    pub const fn manifest_blocks_read(self) -> u64 {
        self.value.manifest_blocks_read
    }
    pub const fn manifest_comparisons(self) -> u64 {
        self.value.manifest_comparisons
    }
    pub const fn manifest_bytes_read(self) -> u64 {
        self.value.manifest_bytes_read
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedRecordBatch {
    record_ids: Vec<PhysicalRecordId>,
    root_generation: u64,
    publication_identity: u64,
    observation: RecordAppendObservation,
    counters_before: MediaCounterSnapshot,
    counters_after: MediaCounterSnapshot,
}

impl PublishedRecordBatch {
    pub fn record_ids(&self) -> &[PhysicalRecordId] {
        &self.record_ids
    }
    pub fn record_id(&self, index: usize) -> Option<PhysicalRecordId> {
        self.record_ids.get(index).copied()
    }
    pub const fn root_generation(&self) -> u64 {
        self.root_generation
    }
    pub const fn publication_identity(&self) -> u64 {
        self.publication_identity
    }
    pub const fn observation(&self) -> RecordAppendObservation {
        self.observation
    }
    pub const fn media_counters_before(&self) -> MediaCounterSnapshot {
        self.counters_before
    }
    pub const fn media_counters_after(&self) -> MediaCounterSnapshot {
        self.counters_after
    }
    pub(in crate::physical_runtime::record_serving) fn from_publication(
        records: Vec<PersistedRecordIdentity>,
        root_generation: u64,
        publication_identity: u64,
        observation: PublicationObservation,
        counters_before: MediaCounterSnapshot,
        counters_after: MediaCounterSnapshot,
    ) -> Self {
        Self {
            record_ids: records
                .into_iter()
                .map(PhysicalRecordId::from_persisted)
                .collect(),
            root_generation,
            publication_identity,
            observation: RecordAppendObservation { value: observation },
            counters_before,
            counters_after,
        }
    }
}
