use worth_store_physical_format::{PhysicalCheckpointIdentity, PhysicalCheckpointSource};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalCheckpointProgressPhase {
    Admitted,
    CandidateCreation,
    Capture,
    CandidateSynchronization,
    CandidateCleanup,
    PublicationReplacement,
    NamespaceSynchronization,
    Terminal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalCheckpointProgress {
    identity: PhysicalCheckpointIdentity,
    source: PhysicalCheckpointSource,
    phase: PhysicalCheckpointProgressPhase,
    captured_dirty_frames: u64,
    encoded_bytes: u64,
    current_capture_bytes: u64,
    peak_capture_bytes: u64,
    cancellation_requested: bool,
}

impl PhysicalCheckpointProgress {
    pub(super) const fn admitted(source: PhysicalCheckpointSource) -> Self {
        Self {
            identity: source.identity(),
            source,
            phase: PhysicalCheckpointProgressPhase::Admitted,
            captured_dirty_frames: 0,
            encoded_bytes: 0,
            current_capture_bytes: 0,
            peak_capture_bytes: 0,
            cancellation_requested: false,
        }
    }

    pub const fn identity(self) -> PhysicalCheckpointIdentity {
        self.identity
    }

    pub const fn source(self) -> PhysicalCheckpointSource {
        self.source
    }

    pub const fn phase(self) -> PhysicalCheckpointProgressPhase {
        self.phase
    }

    pub const fn captured_dirty_frames(self) -> u64 {
        self.captured_dirty_frames
    }

    pub const fn encoded_bytes(self) -> u64 {
        self.encoded_bytes
    }

    pub const fn current_capture_bytes(self) -> u64 {
        self.current_capture_bytes
    }

    pub const fn peak_capture_bytes(self) -> u64 {
        self.peak_capture_bytes
    }

    pub const fn cancellation_requested(self) -> bool {
        self.cancellation_requested
    }

    pub(super) fn enter(&mut self, phase: PhysicalCheckpointProgressPhase) {
        self.phase = phase;
    }

    pub(super) fn record_capture(&mut self, dirty_frames: u64, encoded_bytes: u64) {
        self.captured_dirty_frames = dirty_frames;
        self.encoded_bytes = encoded_bytes;
    }

    pub(super) fn begin_capture_allocation(&mut self, bytes: u64) {
        self.current_capture_bytes = bytes;
        self.peak_capture_bytes = self.peak_capture_bytes.max(bytes);
    }

    pub(super) fn end_capture_allocation(&mut self) {
        self.current_capture_bytes = 0;
    }

    pub(super) fn request_cancellation(&mut self) {
        self.cancellation_requested = true;
    }
}
