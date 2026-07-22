use super::{
    DirtyPhysicalFrame, OperationAllocationGrant, OperationAllocationScope,
    PhysicalCandidateBatchReservation, PhysicalCandidateFrameReservation, PhysicalFrameLease,
    PhysicalFrameLoadError, PhysicalResidencyCounters, PhysicalResidencyDenial,
    PhysicalResidencyLimits, PhysicalResidencyShutdown, PhysicalWritebackClaim,
    SpeculativeResidencyGrant,
};
use std::{
    collections::HashMap,
    sync::{Arc, Condvar, Mutex, MutexGuard},
};
use worth_store_physical_format::{store_namespace::StableStoreIdentity, RecordFrameCoordinate};

mod eviction_order;
mod frame_admission;
mod identity_transition;
mod operation_accounting;
mod pin_lifecycle;
mod public_api;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalFrameKey {
    store: StableStoreIdentity,
    coordinate: RecordFrameCoordinate,
}

impl PhysicalFrameKey {
    pub const fn new(store: StableStoreIdentity, coordinate: RecordFrameCoordinate) -> Self {
        Self { store, coordinate }
    }
    pub const fn store(self) -> StableStoreIdentity {
        self.store
    }
    pub const fn coordinate(self) -> RecordFrameCoordinate {
        self.coordinate
    }
}

#[derive(Debug, Clone)]
pub struct PhysicalResidencyPool {
    pub(crate) inner: Arc<PoolInner>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalResidencyIncarnation(u64);

impl PhysicalResidencyIncarnation {
    fn next() -> Option<Self> {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        NEXT.fetch_update(
            std::sync::atomic::Ordering::Relaxed,
            std::sync::atomic::Ordering::Relaxed,
            |current| current.checked_add(1),
        )
        .ok()
        .map(Self)
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug)]
pub(crate) struct PoolInner {
    store: StableStoreIdentity,
    incarnation: PhysicalResidencyIncarnation,
    limits: PhysicalResidencyLimits,
    metadata_bytes: u64,
    state: Mutex<PoolState>,
    changed: Condvar,
}

#[derive(Debug)]
struct PoolState {
    frames: HashMap<RecordFrameCoordinate, FrameEntry>,
    counters: PhysicalResidencyCounters,
    evictable_head: Option<RecordFrameCoordinate>,
    evictable_tail: Option<RecordFrameCoordinate>,
    loading_frames: u32,
    candidate_publication_active: bool,
    accepting: bool,
    closed: bool,
}

#[derive(Debug)]
struct FrameEntry {
    state: FrameState,
    origin: FrameOrigin,
    pins: u32,
    dirty: bool,
    writeback_claimed: bool,
    bytes: u64,
    older_evictable: Option<RecordFrameCoordinate>,
    newer_evictable: Option<RecordFrameCoordinate>,
}

impl FrameEntry {
    fn is_evictable(&self) -> bool {
        self.pins == 0
            && !self.dirty
            && !self.writeback_claimed
            && matches!(&self.state, FrameState::Resident(_))
    }
}

#[derive(Debug)]
enum FrameState {
    Loading,
    CandidateReserved,
    Resident(Arc<Vec<u8>>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FrameOrigin {
    Fault,
    Candidate,
}

impl PoolInner {
    pub(crate) const fn store_identity(&self) -> StableStoreIdentity {
        self.store
    }

    pub(crate) const fn incarnation(&self) -> PhysicalResidencyIncarnation {
        self.incarnation
    }

    fn lock(&self) -> MutexGuard<'_, PoolState> {
        self.state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn validate_key(&self, key: PhysicalFrameKey) -> Result<(), PhysicalResidencyDenial> {
        if key.store != self.store {
            return Err(PhysicalResidencyDenial::WrongStore);
        }
        if key.coordinate.length() as u64 > self.limits.resident_bytes() {
            return Err(PhysicalResidencyDenial::FrameLargerThanResidentBudget);
        }
        Ok(())
    }

    fn record_source_load(&self) {
        self.lock().counters.source_loads += 1;
    }

    pub(crate) fn record_denial(&self, denial: PhysicalResidencyDenial) -> PhysicalResidencyDenial {
        Self::deny(&mut self.lock(), denial)
    }

    fn observe_admitted_peak(&self, state: &mut PoolState) {
        let admitted = self
            .metadata_bytes
            .saturating_add(state.counters.resident_bytes)
            .saturating_add(state.counters.active_operation_bytes);
        state.counters.peak_admitted_bytes = state.counters.peak_admitted_bytes.max(admitted);
    }

    fn deny(state: &mut PoolState, denial: PhysicalResidencyDenial) -> PhysicalResidencyDenial {
        if !state.closed {
            state.counters.denials += 1;
        }
        denial
    }
}

struct LoadingReservation {
    owner: Arc<PoolInner>,
    key: PhysicalFrameKey,
    armed: bool,
}

impl LoadingReservation {
    fn new(owner: Arc<PoolInner>, key: PhysicalFrameKey) -> Self {
        Self {
            owner,
            key,
            armed: true,
        }
    }
    fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for LoadingReservation {
    fn drop(&mut self) {
        if self.armed {
            self.owner.cancel_loading(self.key);
        }
    }
}

impl PhysicalCandidateFrameReservation {
    pub(crate) fn new(owner: Arc<PoolInner>, key: PhysicalFrameKey) -> Self {
        Self {
            owner,
            key,
            armed: true,
        }
    }
}
