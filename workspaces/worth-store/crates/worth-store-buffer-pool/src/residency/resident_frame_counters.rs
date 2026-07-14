use crate::{
    DirtyPageCounterSnapshot, EvictionCounterSnapshot, PinLifecycleCounterSnapshot,
    ResidentByteCount,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResidentFrameCounterSnapshot {
    resident_bytes: ResidentByteCount,
    hit_count: u64,
    miss_count: u64,
    frame_table_lookup_count: u64,
    pin_lifecycle: PinLifecycleCounterSnapshot,
    dirty_state: DirtyPageCounterSnapshot,
    eviction: EvictionCounterSnapshot,
}

impl ResidentFrameCounterSnapshot {
    pub const fn empty() -> Self {
        Self {
            resident_bytes: ResidentByteCount::from_observed_bytes(0),
            hit_count: 0,
            miss_count: 0,
            frame_table_lookup_count: 0,
            pin_lifecycle: PinLifecycleCounterSnapshot::empty(),
            dirty_state: DirtyPageCounterSnapshot::empty(),
            eviction: EvictionCounterSnapshot::empty(),
        }
    }

    pub(crate) fn with_resident_bytes(self, resident_bytes: u64) -> Self {
        Self {
            resident_bytes: ResidentByteCount::from_observed_bytes(resident_bytes),
            ..self
        }
    }

    pub(crate) const fn with_hit(self) -> Self {
        Self {
            hit_count: self.hit_count + 1,
            frame_table_lookup_count: self.frame_table_lookup_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_miss(self) -> Self {
        Self {
            miss_count: self.miss_count + 1,
            frame_table_lookup_count: self.frame_table_lookup_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_lookup(self) -> Self {
        Self {
            frame_table_lookup_count: self.frame_table_lookup_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_pin_lifecycle(
        self,
        pin_lifecycle: PinLifecycleCounterSnapshot,
    ) -> Self {
        Self {
            pin_lifecycle,
            ..self
        }
    }

    pub(crate) const fn with_dirty_state(self, dirty_state: DirtyPageCounterSnapshot) -> Self {
        Self {
            dirty_state,
            ..self
        }
    }

    pub(crate) const fn with_eviction(self, eviction: EvictionCounterSnapshot) -> Self {
        Self { eviction, ..self }
    }

    pub const fn resident_bytes(self) -> ResidentByteCount {
        self.resident_bytes
    }

    pub const fn hit_count(self) -> u64 {
        self.hit_count
    }

    pub const fn miss_count(self) -> u64 {
        self.miss_count
    }

    pub const fn frame_table_lookup_count(self) -> u64 {
        self.frame_table_lookup_count
    }

    pub const fn pin_lifecycle(self) -> PinLifecycleCounterSnapshot {
        self.pin_lifecycle
    }

    pub const fn dirty_state(self) -> DirtyPageCounterSnapshot {
        self.dirty_state
    }

    pub const fn eviction(self) -> EvictionCounterSnapshot {
        self.eviction
    }
}
