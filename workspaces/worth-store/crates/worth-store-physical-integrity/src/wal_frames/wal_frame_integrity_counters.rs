#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WalFrameIntegrityCounters {
    protected_window_reads: u32,
    frame_header_checks: u32,
    payload_boundary_checks: u32,
    checksum_posture_checks: u32,
    tail_posture_checks: u32,
    checkpoint_adjacency_checks: u32,
    skipped_replay_attempts: u32,
}

impl WalFrameIntegrityCounters {
    pub const fn start() -> Self {
        Self {
            protected_window_reads: 1,
            frame_header_checks: 0,
            payload_boundary_checks: 0,
            checksum_posture_checks: 0,
            tail_posture_checks: 0,
            checkpoint_adjacency_checks: 0,
            skipped_replay_attempts: 0,
        }
    }

    pub const fn with_frame_header_check(mut self) -> Self {
        self.frame_header_checks += 1;
        self
    }

    pub const fn with_payload_boundary_check(mut self) -> Self {
        self.payload_boundary_checks += 1;
        self
    }

    pub const fn with_checksum_posture_check(mut self) -> Self {
        self.checksum_posture_checks += 1;
        self
    }

    pub const fn with_tail_posture_check(mut self) -> Self {
        self.tail_posture_checks += 1;
        self
    }

    pub const fn with_checkpoint_adjacency_check(mut self) -> Self {
        self.checkpoint_adjacency_checks += 1;
        self
    }

    pub const fn with_skipped_replay_attempt(mut self) -> Self {
        self.skipped_replay_attempts += 1;
        self
    }

    pub const fn protected_window_reads(self) -> u32 {
        self.protected_window_reads
    }

    pub const fn frame_header_checks(self) -> u32 {
        self.frame_header_checks
    }

    pub const fn payload_boundary_checks(self) -> u32 {
        self.payload_boundary_checks
    }

    pub const fn checksum_posture_checks(self) -> u32 {
        self.checksum_posture_checks
    }

    pub const fn tail_posture_checks(self) -> u32 {
        self.tail_posture_checks
    }

    pub const fn checkpoint_adjacency_checks(self) -> u32 {
        self.checkpoint_adjacency_checks
    }

    pub const fn skipped_replay_attempts(self) -> u32 {
        self.skipped_replay_attempts
    }
}
