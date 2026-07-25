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
    pub(in crate::physical_runtime::record_serving) physical_work_count: u64,
    pub(in crate::physical_runtime::record_serving) first_physical_work:
        Option<crate::physical_runtime::PhysicalWorkIdentity>,
    pub(in crate::physical_runtime::record_serving) last_physical_work:
        Option<crate::physical_runtime::PhysicalWorkIdentity>,
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
    pub const fn physical_work_count(self) -> u64 {
        self.physical_work_count
    }
    pub const fn first_physical_work(
        self,
    ) -> Option<crate::physical_runtime::PhysicalWorkIdentity> {
        self.first_physical_work
    }
    pub const fn last_physical_work(self) -> Option<crate::physical_runtime::PhysicalWorkIdentity> {
        self.last_physical_work
    }

    pub(in crate::physical_runtime::record_serving) fn observe_manifest_delta(
        &mut self,
        before: super::super::access::manifest_routing::ManifestDiscoveryCounterSnapshot,
        after: super::super::access::manifest_routing::ManifestDiscoveryCounterSnapshot,
    ) {
        let blocks = after.blocks_read().saturating_sub(before.blocks_read());
        let work = after.work_count().saturating_sub(before.work_count());
        self.manifest_blocks = self.manifest_blocks.saturating_add(blocks);
        self.manifest_bytes = self
            .manifest_bytes
            .saturating_add(after.bytes_read().saturating_sub(before.bytes_read()));
        self.manifest_comparisons = self
            .manifest_comparisons
            .saturating_add(after.comparisons().saturating_sub(before.comparisons()));
        self.frames = self.frames.saturating_add(blocks);
        self.physical_work_count = self.physical_work_count.saturating_add(work);
        self.first_physical_work = self.first_physical_work.or(after.first_work());
        if work != 0 {
            self.last_physical_work = after.last_work();
        }
    }

    pub(in crate::physical_runtime::record_serving) fn observe_record_read(
        &mut self,
        observation: super::super::RecordReadObservation,
    ) {
        self.payload_bytes = self
            .payload_bytes
            .saturating_add(observation.bytes_completed());
        self.frames = self.frames.saturating_add(
            observation
                .manifest_blocks()
                .saturating_add(observation.touched_pages())
                .saturating_add(observation.touched_extents()),
        );
        self.manifest_blocks = self
            .manifest_blocks
            .saturating_add(observation.manifest_blocks());
        self.manifest_bytes = self
            .manifest_bytes
            .saturating_add(observation.manifest_bytes());
        self.manifest_comparisons = self
            .manifest_comparisons
            .saturating_add(observation.manifest_comparisons());
        self.transfer_count = self
            .transfer_count
            .saturating_add(observation.transfer_count());
        self.peak_transfer_width = self
            .peak_transfer_width
            .max(observation.peak_transfer_width());
        self.explicit_copy_count = self
            .explicit_copy_count
            .saturating_add(observation.explicit_copy_count());
        self.copied_bytes = self.copied_bytes.saturating_add(observation.copied_bytes());
        self.peak_scratch_bytes = self
            .peak_scratch_bytes
            .max(observation.peak_scratch_bytes());
        self.physical_work_count = self
            .physical_work_count
            .saturating_add(observation.physical_work_count());
        self.first_physical_work = self
            .first_physical_work
            .or(observation.first_physical_work());
        self.last_physical_work = observation.last_physical_work().or(self.last_physical_work);
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
            physical_work_count: 0,
            first_physical_work: None,
            last_physical_work: None,
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
        physical_work_count: snapshot.work_count(),
        first_physical_work: snapshot.first_work(),
        last_physical_work: snapshot.last_work(),
    }
}
