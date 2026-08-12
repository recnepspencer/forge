use crate::{RecoveryMemoryAllocation, RedoExecutionReceipt};

use super::{source_discovery::RecoveryWorkBudgetEvidence, RecoveryStoreFootprint};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryCounterSnapshot {
    replayed_frames: usize,
    skipped_frames: usize,
    validated_checkpoints: u64,
    scanned_segments: usize,
    page_redos: usize,
    memory_envelope_bytes: u64,
    memory_envelope_frames: u32,
    allocation_bytes: u64,
    total_store_pages: u64,
    residue_rejections: usize,
    forbidden_full_store_scans: u64,
}

impl RecoveryCounterSnapshot {
    pub(crate) fn from_execution(
        execution: &RedoExecutionReceipt,
        evidence: RecoveryWorkBudgetEvidence,
        memory_allocation: &RecoveryMemoryAllocation<'_>,
        store_footprint: RecoveryStoreFootprint,
    ) -> Self {
        let memory = memory_allocation.counters();
        Self {
            replayed_frames: execution.planned_frame_count(),
            skipped_frames: execution.skipped_frames().len(),
            validated_checkpoints: evidence.validated_checkpoints(),
            scanned_segments: evidence.scanned_segments(),
            page_redos: execution.applied_frame_count(),
            memory_envelope_bytes: memory.resident_bytes_admitted(),
            memory_envelope_frames: memory.resident_frames_admitted(),
            allocation_bytes: memory.allocation_bytes_allocated(),
            total_store_pages: store_footprint.total_store_pages(),
            residue_rejections: evidence.residue_rejections(),
            forbidden_full_store_scans: evidence.forbidden_full_store_scans(),
        }
    }

    pub const fn replayed_frames(self) -> usize {
        self.replayed_frames
    }

    pub const fn skipped_frames(self) -> usize {
        self.skipped_frames
    }

    pub const fn validated_checkpoints(self) -> u64 {
        self.validated_checkpoints
    }

    pub const fn scanned_segments(self) -> usize {
        self.scanned_segments
    }

    pub const fn page_redos(self) -> usize {
        self.page_redos
    }

    pub const fn memory_envelope_bytes(self) -> u64 {
        self.memory_envelope_bytes
    }

    pub const fn memory_envelope_frames(self) -> u32 {
        self.memory_envelope_frames
    }

    pub const fn allocation_bytes(self) -> u64 {
        self.allocation_bytes
    }

    pub const fn total_store_pages(self) -> u64 {
        self.total_store_pages
    }

    pub const fn residue_rejections(self) -> usize {
        self.residue_rejections
    }

    pub const fn forbidden_full_store_scans(self) -> u64 {
        self.forbidden_full_store_scans
    }
}
