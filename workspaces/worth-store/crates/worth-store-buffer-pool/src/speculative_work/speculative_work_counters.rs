#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpeculativeWorkCounterSnapshot {
    read_ahead_attempt_count: u64,
    read_ahead_admitted_count: u64,
    read_ahead_denied_count: u64,
    prefetch_attempt_count: u64,
    prefetch_admitted_count: u64,
    prefetch_denied_count: u64,
    write_behind_attempt_count: u64,
    write_behind_admitted_count: u64,
    write_behind_denied_count: u64,
    deferred_count: u64,
    resident_frames_requested: u64,
    dirty_pages_requested: u64,
    allocation_bytes_admitted: u64,
}

impl SpeculativeWorkCounterSnapshot {
    pub const fn empty() -> Self {
        Self {
            read_ahead_attempt_count: 0,
            read_ahead_admitted_count: 0,
            read_ahead_denied_count: 0,
            prefetch_attempt_count: 0,
            prefetch_admitted_count: 0,
            prefetch_denied_count: 0,
            write_behind_attempt_count: 0,
            write_behind_admitted_count: 0,
            write_behind_denied_count: 0,
            deferred_count: 0,
            resident_frames_requested: 0,
            dirty_pages_requested: 0,
            allocation_bytes_admitted: 0,
        }
    }

    pub(crate) const fn with_read_ahead_attempt(self) -> Self {
        Self {
            read_ahead_attempt_count: self.read_ahead_attempt_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_read_ahead_admitted(self, resident_frames: u32) -> Self {
        Self {
            read_ahead_admitted_count: self.read_ahead_admitted_count + 1,
            resident_frames_requested: self.resident_frames_requested + resident_frames as u64,
            ..self
        }
    }

    pub(crate) const fn with_read_ahead_denied(self) -> Self {
        Self {
            read_ahead_denied_count: self.read_ahead_denied_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_prefetch_attempt(self) -> Self {
        Self {
            prefetch_attempt_count: self.prefetch_attempt_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_prefetch_admitted(self, resident_frames: u32) -> Self {
        Self {
            prefetch_admitted_count: self.prefetch_admitted_count + 1,
            resident_frames_requested: self.resident_frames_requested + resident_frames as u64,
            ..self
        }
    }

    pub(crate) const fn with_prefetch_denied(self) -> Self {
        Self {
            prefetch_denied_count: self.prefetch_denied_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_write_behind_attempt(self) -> Self {
        Self {
            write_behind_attempt_count: self.write_behind_attempt_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_write_behind_admitted(self, dirty_pages: u32) -> Self {
        Self {
            write_behind_admitted_count: self.write_behind_admitted_count + 1,
            dirty_pages_requested: self.dirty_pages_requested + dirty_pages as u64,
            ..self
        }
    }

    pub(crate) const fn with_write_behind_denied(self) -> Self {
        Self {
            write_behind_denied_count: self.write_behind_denied_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_deferred(self) -> Self {
        Self {
            deferred_count: self.deferred_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_allocation_bytes_admitted(self, bytes: u64) -> Self {
        Self {
            allocation_bytes_admitted: self.allocation_bytes_admitted + bytes,
            ..self
        }
    }

    pub const fn read_ahead_attempt_count(self) -> u64 {
        self.read_ahead_attempt_count
    }

    pub const fn read_ahead_admitted_count(self) -> u64 {
        self.read_ahead_admitted_count
    }

    pub const fn read_ahead_denied_count(self) -> u64 {
        self.read_ahead_denied_count
    }

    pub const fn prefetch_attempt_count(self) -> u64 {
        self.prefetch_attempt_count
    }

    pub const fn prefetch_admitted_count(self) -> u64 {
        self.prefetch_admitted_count
    }

    pub const fn prefetch_denied_count(self) -> u64 {
        self.prefetch_denied_count
    }

    pub const fn write_behind_attempt_count(self) -> u64 {
        self.write_behind_attempt_count
    }

    pub const fn write_behind_admitted_count(self) -> u64 {
        self.write_behind_admitted_count
    }

    pub const fn write_behind_denied_count(self) -> u64 {
        self.write_behind_denied_count
    }

    pub const fn deferred_count(self) -> u64 {
        self.deferred_count
    }

    pub const fn resident_frames_requested(self) -> u64 {
        self.resident_frames_requested
    }

    pub const fn dirty_pages_requested(self) -> u64 {
        self.dirty_pages_requested
    }

    pub const fn allocation_bytes_admitted(self) -> u64 {
        self.allocation_bytes_admitted
    }
}
