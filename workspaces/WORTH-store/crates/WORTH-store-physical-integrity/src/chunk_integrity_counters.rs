#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkIntegrityCounters {
    protected_window_reads: u32,
    streaming_windows_planned: u32,
    inspected_bytes: u64,
    chunk_header_checks: u32,
    chunk_payload_checks: u32,
    chunk_boundary_checks: u32,
    extent_boundary_checks: u32,
    skipped_whole_object_reads: u32,
}

impl ChunkIntegrityCounters {
    pub(crate) fn start(object_bytes: u64, window_bytes: u64, inspected_bytes: u64) -> Self {
        Self {
            protected_window_reads: 1,
            streaming_windows_planned: planned_windows(object_bytes, window_bytes),
            inspected_bytes,
            chunk_header_checks: 0,
            chunk_payload_checks: 0,
            chunk_boundary_checks: 0,
            extent_boundary_checks: 0,
            skipped_whole_object_reads: 0,
        }
    }

    pub(crate) const fn with_chunk_header_check(mut self) -> Self {
        self.chunk_header_checks += 1;
        self
    }

    pub(crate) const fn with_chunk_payload_check(mut self) -> Self {
        self.chunk_payload_checks += 1;
        self
    }

    pub(crate) const fn with_chunk_boundary_check(mut self) -> Self {
        self.chunk_boundary_checks += 1;
        self
    }

    pub(crate) const fn with_extent_boundary_check(mut self) -> Self {
        self.extent_boundary_checks += 1;
        self
    }

    pub(crate) const fn with_skipped_whole_object_read(mut self) -> Self {
        self.skipped_whole_object_reads += 1;
        self
    }

    pub const fn protected_window_reads(self) -> u32 {
        self.protected_window_reads
    }

    pub const fn streaming_windows_planned(self) -> u32 {
        self.streaming_windows_planned
    }

    pub const fn inspected_bytes(self) -> u64 {
        self.inspected_bytes
    }

    pub const fn chunk_header_checks(self) -> u32 {
        self.chunk_header_checks
    }

    pub const fn chunk_payload_checks(self) -> u32 {
        self.chunk_payload_checks
    }

    pub const fn chunk_boundary_checks(self) -> u32 {
        self.chunk_boundary_checks
    }

    pub const fn extent_boundary_checks(self) -> u32 {
        self.extent_boundary_checks
    }

    pub const fn skipped_whole_object_reads(self) -> u32 {
        self.skipped_whole_object_reads
    }
}

fn planned_windows(object_bytes: u64, window_bytes: u64) -> u32 {
    object_bytes
        .div_ceil(window_bytes)
        .try_into()
        .unwrap_or(u32::MAX)
}
