#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWalObservation {
    segment: u64,
    generation: u64,
    appended_frames: u64,
    appended_bytes: u64,
    valid_prefix_bytes: u64,
    last_lsn_end: Option<u64>,
    sealed_for_inspection: bool,
}

impl PhysicalWalObservation {
    pub(in crate::physical_runtime) const fn new(
        segment: u64,
        generation: u64,
        appended_frames: u64,
        appended_bytes: u64,
        valid_prefix_bytes: u64,
        last_lsn_end: Option<u64>,
        sealed_for_inspection: bool,
    ) -> Self {
        Self {
            segment,
            generation,
            appended_frames,
            appended_bytes,
            valid_prefix_bytes,
            last_lsn_end,
            sealed_for_inspection,
        }
    }

    pub const fn segment(self) -> u64 {
        self.segment
    }

    pub const fn generation(self) -> u64 {
        self.generation
    }

    pub const fn appended_frames(self) -> u64 {
        self.appended_frames
    }

    pub const fn appended_bytes(self) -> u64 {
        self.appended_bytes
    }

    pub const fn valid_prefix_bytes(self) -> u64 {
        self.valid_prefix_bytes
    }

    pub const fn last_lsn_end(self) -> Option<u64> {
        self.last_lsn_end
    }

    pub const fn sealed_for_inspection(self) -> bool {
        self.sealed_for_inspection
    }
}
