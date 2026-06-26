use crate::{
    AllocationAdmission, AllocationCounterSnapshot, AllocationScope, BoundedCopyRecordView,
    RecordCopyCounterSnapshot, ResidentFrameCounterSnapshot, ResidentFrameTable,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferPoolCounterSnapshot {
    resident_memory: ResidentFrameCounterSnapshot,
    allocation: AllocationCounterSnapshot,
    copy_materialization: RecordCopyCounterSnapshot,
}

impl BufferPoolCounterSnapshot {
    pub(crate) fn from_executed_store_counters(
        resident_memory: ResidentFrameCounterSnapshot,
        allocation: AllocationCounterSnapshot,
        copy_materialization: RecordCopyCounterSnapshot,
    ) -> Result<Self, BufferPoolEvidenceSourceDenial> {
        let snapshot = Self {
            resident_memory,
            allocation,
            copy_materialization,
        };
        if snapshot.is_empty() {
            return Err(BufferPoolEvidenceSourceDenial::NoExecutedStoreCounters);
        }
        Ok(snapshot)
    }

    pub const fn resident_memory(self) -> ResidentFrameCounterSnapshot {
        self.resident_memory
    }

    pub const fn allocation(self) -> AllocationCounterSnapshot {
        self.allocation
    }

    pub const fn copy_materialization(self) -> RecordCopyCounterSnapshot {
        self.copy_materialization
    }

    pub fn is_empty(self) -> bool {
        resident_memory_counter_total(self.resident_memory) == 0
            && allocation_counter_total(self.allocation) == 0
            && copy_counter_total(self.copy_materialization) == 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferPoolExecutedEvidenceSource {
    counters: BufferPoolCounterSnapshot,
}

impl BufferPoolExecutedEvidenceSource {
    pub fn from_store_execution(
        resident_frames: &ResidentFrameTable,
        allocation: &AllocationAdmission,
        bounded_copy: &BoundedCopyRecordView,
    ) -> Result<Self, BufferPoolEvidenceSourceDenial> {
        Self::from_counters(BufferPoolCounterSnapshot::from_executed_store_counters(
            resident_frames.counters(),
            allocation.counters(),
            bounded_copy.counters(),
        )?)
    }

    pub(crate) fn from_counters(
        counters: BufferPoolCounterSnapshot,
    ) -> Result<Self, BufferPoolEvidenceSourceDenial> {
        if counters.is_empty() {
            return Err(BufferPoolEvidenceSourceDenial::NoExecutedStoreCounters);
        }
        Ok(Self { counters })
    }

    pub const fn counters(self) -> BufferPoolCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferPoolEvidenceSourceDenial {
    NoExecutedStoreCounters,
}

fn resident_memory_counter_total(counters: ResidentFrameCounterSnapshot) -> u64 {
    counters.resident_bytes().as_bytes()
        + counters.hit_count()
        + counters.miss_count()
        + counters.frame_table_lookup_count()
        + counters.pin_lifecycle().pin_attempt_count()
        + counters.pin_lifecycle().successful_pin_count()
        + counters.pin_lifecycle().explicit_unpin_count()
        + counters.pin_lifecycle().defensive_drop_count()
        + counters.pin_lifecycle().leaked_pin_count()
        + counters.pin_lifecycle().denied_protected_mutation_count()
        + counters.pin_lifecycle().active_pinned_pages()
        + counters.dirty_state().dirty_pages().as_pages() as u64
        + counters.eviction().plan_attempt_count()
        + counters.eviction().resident_frame_scan_count()
        + counters.eviction().candidate_count()
        + counters.eviction().receipt_count()
}

fn allocation_counter_total(counters: AllocationCounterSnapshot) -> u64 {
    let mut total = counters.fixed_metadata_bytes()
        + counters.fixed_metadata_exemption_count() as u64
        + counters.fixed_metadata_denied_bytes();
    for scope in AllocationScope::ALL {
        let scope = counters.scope(scope);
        total += scope.requested_bytes()
            + scope.admitted_bytes()
            + scope.allocated_bytes()
            + scope.copied_bytes()
            + scope.denied_bytes()
            + scope.denial_count() as u64;
    }
    total
}

fn copy_counter_total(counters: RecordCopyCounterSnapshot) -> u64 {
    counters.zero_copy_admission_attempt_count()
        + counters.zero_copy_admission_count()
        + counters.bounded_copy_attempt_count()
        + counters.bounded_copy_count()
        + counters.copied_bytes()
        + counters.materialized_bytes()
        + counters.cow_fallback_count()
        + counters.denied_before_view_construction_count()
        + counters.dirty_mutation_conflict_denial_count()
        + counters.publication_conflict_denial_count()
}
