use super::super::RecordByteLimit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordReadLimits {
    pub(in crate::physical_runtime::record_serving) maximum_payload: RecordByteLimit,
}

impl RecordReadLimits {
    pub const fn new(maximum_payload: RecordByteLimit) -> Self {
        Self { maximum_payload }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RecordReadObservation {
    pub(in crate::physical_runtime::record_serving) touched_segments: u64,
    pub(in crate::physical_runtime::record_serving) touched_pages: u64,
    pub(in crate::physical_runtime::record_serving) touched_extents: u64,
    pub(in crate::physical_runtime::record_serving) payload_bytes: u64,
    pub(in crate::physical_runtime::record_serving) requested_bytes: u64,
    pub(in crate::physical_runtime::record_serving) transfer_count: u64,
    pub(in crate::physical_runtime::record_serving) peak_transfer_width: u64,
    pub(in crate::physical_runtime::record_serving) explicit_copy_count: u64,
    pub(in crate::physical_runtime::record_serving) copied_bytes: u64,
    pub(in crate::physical_runtime::record_serving) generation_checks: u64,
    pub(in crate::physical_runtime::record_serving) generation_rejections: u64,
    pub(in crate::physical_runtime::record_serving) peak_scratch_bytes: u64,
    pub(in crate::physical_runtime::record_serving) manifest_blocks: u64,
    pub(in crate::physical_runtime::record_serving) manifest_comparisons: u64,
    pub(in crate::physical_runtime::record_serving) manifest_bytes: u64,
}

impl RecordReadObservation {
    pub const fn touched_segments(self) -> u64 {
        self.touched_segments
    }
    pub const fn touched_pages(self) -> u64 {
        self.touched_pages
    }
    pub const fn touched_extents(self) -> u64 {
        self.touched_extents
    }
    pub const fn payload_bytes(self) -> u64 {
        self.payload_bytes
    }
    pub const fn bytes_requested(self) -> u64 {
        self.requested_bytes
    }
    pub const fn bytes_completed(self) -> u64 {
        self.payload_bytes
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
    pub const fn generation_checks(self) -> u64 {
        self.generation_checks
    }
    pub const fn generation_rejections(self) -> u64 {
        self.generation_rejections
    }
    pub const fn peak_scratch_bytes(self) -> u64 {
        self.peak_scratch_bytes
    }
    pub const fn manifest_blocks(self) -> u64 {
        self.manifest_blocks
    }
    pub const fn manifest_comparisons(self) -> u64 {
        self.manifest_comparisons
    }
    pub const fn manifest_bytes(self) -> u64 {
        self.manifest_bytes
    }

    pub(in crate::physical_runtime::record_serving) fn observe_manifest(
        &mut self,
        snapshot: super::super::access::manifest_routing::ManifestDiscoveryCounterSnapshot,
    ) {
        self.manifest_blocks = self.manifest_blocks.saturating_add(snapshot.blocks_read());
        self.manifest_comparisons = self
            .manifest_comparisons
            .saturating_add(snapshot.comparisons());
        self.manifest_bytes = self.manifest_bytes.saturating_add(snapshot.bytes_read());
    }

    pub(in crate::physical_runtime::record_serving) fn observe_manifest_block(
        &mut self,
        bytes: usize,
    ) {
        self.manifest_blocks = self.manifest_blocks.saturating_add(1);
        self.manifest_bytes = self.manifest_bytes.saturating_add(bytes as u64);
    }

    pub(in crate::physical_runtime::record_serving) fn check_generation(
        &mut self,
        matches: bool,
    ) -> bool {
        self.generation_checks = self.generation_checks.saturating_add(1);
        if !matches {
            self.generation_rejections = self.generation_rejections.saturating_add(1);
        }
        matches
    }

    pub(in crate::physical_runtime::record_serving) fn observe_transfer(&mut self, bytes: usize) {
        self.transfer_count = self.transfer_count.saturating_add(1);
        self.peak_transfer_width = self.peak_transfer_width.max(bytes as u64);
    }

    pub(in crate::physical_runtime::record_serving) fn observe_copy(&mut self, bytes: usize) {
        if bytes != 0 {
            self.explicit_copy_count = self.explicit_copy_count.saturating_add(1);
            self.copied_bytes = self.copied_bytes.saturating_add(bytes as u64);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordReadError {
    denial: RecordReadDenial,
    observation: RecordReadObservation,
}

impl RecordReadError {
    pub(in crate::physical_runtime::record_serving) const fn new(
        denial: RecordReadDenial,
        observation: RecordReadObservation,
    ) -> Self {
        Self {
            denial,
            observation,
        }
    }
    pub const fn denial(self) -> RecordReadDenial {
        self.denial
    }
    pub const fn observation(self) -> RecordReadObservation {
        self.observation
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordReadDenial {
    StoreIdentityMismatch,
    RecordNotFound,
    CallerLimitExceeded,
    AccessLimitExceeded,
    ArtifactUnavailable,
    ArtifactDamaged,
    FormatMismatch,
    ResidencyUnavailable(worth_store_buffer_pool::PhysicalResidencyDenial),
    StalePlacement(StalePhysicalRecordPlacement),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StalePhysicalRecordPlacement {
    SegmentGeneration,
    SegmentMembership,
    PageGeneration,
    PageIdentity,
    SlotGeneration,
    ExtentMembership,
}
