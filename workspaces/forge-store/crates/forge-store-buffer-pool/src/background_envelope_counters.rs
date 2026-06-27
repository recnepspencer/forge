#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BackgroundEnvelopeCounterSnapshot {
    attempts: u32,
    admitted: u32,
    denied: u32,
    deferred: u32,
    resident_frames_requested: u32,
    resident_frames_admitted: u32,
    resident_bytes_requested: u64,
    resident_bytes_admitted: u64,
    pinned_pages_requested: u32,
    pinned_pages_admitted: u32,
    allocation_bytes_requested: u64,
    allocation_bytes_admitted: u64,
    allocation_bytes_allocated: u64,
    copied_bytes: u64,
    streaming_object_bytes: u64,
    streaming_window_bytes: u64,
    foreground_interference_denials: u32,
    whole_object_materialization_attempts: u32,
    indefinite_pin_denials: u32,
}

impl BackgroundEnvelopeCounterSnapshot {
    pub const fn attempts(self) -> u32 {
        self.attempts
    }

    pub const fn admitted(self) -> u32 {
        self.admitted
    }

    pub const fn denied(self) -> u32 {
        self.denied
    }

    pub const fn deferred(self) -> u32 {
        self.deferred
    }

    pub const fn resident_frames_requested(self) -> u32 {
        self.resident_frames_requested
    }

    pub const fn resident_frames_admitted(self) -> u32 {
        self.resident_frames_admitted
    }

    pub const fn resident_bytes_requested(self) -> u64 {
        self.resident_bytes_requested
    }

    pub const fn resident_bytes_admitted(self) -> u64 {
        self.resident_bytes_admitted
    }

    pub const fn pinned_pages_requested(self) -> u32 {
        self.pinned_pages_requested
    }

    pub const fn pinned_pages_admitted(self) -> u32 {
        self.pinned_pages_admitted
    }

    pub const fn allocation_bytes_requested(self) -> u64 {
        self.allocation_bytes_requested
    }

    pub const fn allocation_bytes_admitted(self) -> u64 {
        self.allocation_bytes_admitted
    }

    pub const fn allocation_bytes_allocated(self) -> u64 {
        self.allocation_bytes_allocated
    }

    pub const fn copied_bytes(self) -> u64 {
        self.copied_bytes
    }

    pub const fn streaming_object_bytes(self) -> u64 {
        self.streaming_object_bytes
    }

    pub const fn streaming_window_bytes(self) -> u64 {
        self.streaming_window_bytes
    }

    pub const fn foreground_interference_denials(self) -> u32 {
        self.foreground_interference_denials
    }

    pub const fn whole_object_materialization_attempts(self) -> u32 {
        self.whole_object_materialization_attempts
    }

    pub const fn indefinite_pin_denials(self) -> u32 {
        self.indefinite_pin_denials
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BackgroundEnvelopeCounters {
    snapshot: BackgroundEnvelopeCounterSnapshot,
}

impl BackgroundEnvelopeCounters {
    pub const fn new() -> Self {
        Self {
            snapshot: BackgroundEnvelopeCounterSnapshot {
                attempts: 0,
                admitted: 0,
                denied: 0,
                deferred: 0,
                resident_frames_requested: 0,
                resident_frames_admitted: 0,
                resident_bytes_requested: 0,
                resident_bytes_admitted: 0,
                pinned_pages_requested: 0,
                pinned_pages_admitted: 0,
                allocation_bytes_requested: 0,
                allocation_bytes_admitted: 0,
                allocation_bytes_allocated: 0,
                copied_bytes: 0,
                streaming_object_bytes: 0,
                streaming_window_bytes: 0,
                foreground_interference_denials: 0,
                whole_object_materialization_attempts: 0,
                indefinite_pin_denials: 0,
            },
        }
    }

    pub const fn snapshot(self) -> BackgroundEnvelopeCounterSnapshot {
        self.snapshot
    }

    pub fn record_attempt(
        &mut self,
        resident_frames: u32,
        resident_bytes: u64,
        pinned_pages: u32,
        allocation: u64,
    ) {
        self.snapshot.attempts += 1;
        self.snapshot.resident_frames_requested += resident_frames;
        self.snapshot.resident_bytes_requested += resident_bytes;
        self.snapshot.pinned_pages_requested += pinned_pages;
        self.snapshot.allocation_bytes_requested += allocation;
    }

    pub fn record_admitted(
        &mut self,
        resident_frames: u32,
        resident_bytes: u64,
        pinned_pages: u32,
        allocation: u64,
        copied: u64,
        stream_object: u64,
        stream_window: u64,
    ) {
        self.snapshot.admitted += 1;
        self.snapshot.resident_frames_admitted += resident_frames;
        self.snapshot.resident_bytes_admitted += resident_bytes;
        self.snapshot.pinned_pages_admitted += pinned_pages;
        self.snapshot.allocation_bytes_admitted += allocation;
        self.snapshot.allocation_bytes_allocated += allocation;
        self.snapshot.copied_bytes += copied;
        self.snapshot.streaming_object_bytes += stream_object;
        self.snapshot.streaming_window_bytes += stream_window;
    }

    pub fn record_denied(&mut self) {
        self.snapshot.denied += 1;
    }

    pub fn record_deferred(&mut self) {
        self.snapshot.deferred += 1;
    }

    pub fn record_foreground_interference(&mut self) {
        self.snapshot.foreground_interference_denials += 1;
    }

    pub fn record_whole_object_attempt(&mut self) {
        self.snapshot.whole_object_materialization_attempts += 1;
    }

    pub fn record_indefinite_pin_denial(&mut self) {
        self.snapshot.indefinite_pin_denials += 1;
    }
}
