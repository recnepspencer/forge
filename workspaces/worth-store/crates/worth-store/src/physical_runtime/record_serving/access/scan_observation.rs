use super::super::{access::manifest_routing::ManifestRangeCursor, RecordScanDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordScanError {
    pub(in crate::physical_runtime::record_serving) denial: RecordScanDenial,
    pub(in crate::physical_runtime::record_serving) observation: RecordScanCounterSnapshot,
}

impl RecordScanError {
    pub const fn denial(self) -> RecordScanDenial {
        self.denial
    }
    pub const fn observation(self) -> RecordScanCounterSnapshot {
        self.observation
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RecordScanCounterSnapshot {
    pub(in crate::physical_runtime::record_serving) records: u64,
    pub(in crate::physical_runtime::record_serving) payload_bytes: u64,
    pub(in crate::physical_runtime::record_serving) manifest_blocks: u64,
    pub(in crate::physical_runtime::record_serving) manifest_bytes: u64,
    pub(in crate::physical_runtime::record_serving) manifest_comparisons: u64,
    pub(in crate::physical_runtime::record_serving) transfer_count: u64,
    pub(in crate::physical_runtime::record_serving) peak_transfer_width: u64,
    pub(in crate::physical_runtime::record_serving) explicit_copy_count: u64,
    pub(in crate::physical_runtime::record_serving) copied_bytes: u64,
    pub(in crate::physical_runtime::record_serving) peak_scratch_bytes: u64,
    pub(in crate::physical_runtime::record_serving) frames: u64,
}

impl RecordScanCounterSnapshot {
    pub const fn records(self) -> u64 {
        self.records
    }
    pub const fn payload_bytes(self) -> u64 {
        self.payload_bytes
    }
    pub const fn manifest_blocks(self) -> u64 {
        self.manifest_blocks
    }
    pub const fn manifest_bytes(self) -> u64 {
        self.manifest_bytes
    }
    pub const fn manifest_comparisons(self) -> u64 {
        self.manifest_comparisons
    }
    pub const fn transfer_count(self) -> u64 {
        self.transfer_count
    }
    pub const fn peak_transfer_width(self) -> u64 {
        self.peak_transfer_width
    }
    pub const fn explicit_copy_count(self) -> u64 {
        self.explicit_copy_count
    }
    pub const fn copied_bytes(self) -> u64 {
        self.copied_bytes
    }
    pub const fn peak_scratch_bytes(self) -> u64 {
        self.peak_scratch_bytes
    }
    pub const fn frames_traversed(self) -> u64 {
        self.frames
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompletedRecordScan {
    pub(in crate::physical_runtime::record_serving) observation: RecordScanCounterSnapshot,
}

impl CompletedRecordScan {
    pub const fn observation(self) -> RecordScanCounterSnapshot {
        self.observation
    }
}

pub(in crate::physical_runtime::record_serving) const fn scan_error(
    denial: RecordScanDenial,
) -> RecordScanError {
    RecordScanError {
        denial,
        observation: RecordScanCounterSnapshot {
            records: 0,
            payload_bytes: 0,
            manifest_blocks: 0,
            manifest_bytes: 0,
            manifest_comparisons: 0,
            transfer_count: 0,
            peak_transfer_width: 0,
            explicit_copy_count: 0,
            copied_bytes: 0,
            peak_scratch_bytes: 0,
            frames: 0,
        },
    }
}

pub(in crate::physical_runtime::record_serving) fn manifest_error(
    cursor: &ManifestRangeCursor<'_>,
    denial: RecordScanDenial,
) -> RecordScanError {
    RecordScanError {
        denial,
        observation: manifest_snapshot(cursor.counters()),
    }
}

pub(in crate::physical_runtime::record_serving) fn manifest_snapshot(
    snapshot: super::super::access::manifest_routing::ManifestDiscoveryCounterSnapshot,
) -> RecordScanCounterSnapshot {
    RecordScanCounterSnapshot {
        records: 0,
        payload_bytes: 0,
        manifest_blocks: snapshot.blocks_read(),
        manifest_bytes: snapshot.bytes_read(),
        manifest_comparisons: snapshot.comparisons(),
        transfer_count: 0,
        peak_transfer_width: 0,
        explicit_copy_count: 0,
        copied_bytes: 0,
        peak_scratch_bytes: 0,
        frames: snapshot.blocks_read(),
    }
}
