#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct OfflineInspectionCounters {
    backend_requested_bytes: u64,
    bytes_read: u64,
    peak_buffer_bytes: u64,
    peak_owned_allocation_bytes: u64,
    decoder_allocated_bytes: u64,
    file_touches: u64,
    chunk_touches: u64,
    checkpoint_revalidated_files: u64,
    checkpoint_revalidated_bytes: u64,
    checkpoint_rejections: u64,
}

pub(crate) struct OfflineInspectionCounterCheckpoint {
    pub(crate) backend_requested_bytes: u64,
    pub(crate) bytes_read: u64,
    pub(crate) peak_buffer_bytes: u64,
    pub(crate) peak_owned_allocation_bytes: u64,
    pub(crate) decoder_allocated_bytes: u64,
    pub(crate) file_touches: u64,
    pub(crate) chunk_touches: u64,
    pub(crate) checkpoint_revalidated_files: u64,
    pub(crate) checkpoint_revalidated_bytes: u64,
    pub(crate) checkpoint_rejections: u64,
}

impl OfflineInspectionCounters {
    pub(crate) fn with_allocations(
        buffer_bytes: usize,
        peak_owned_allocation_bytes: u64,
    ) -> Option<Self> {
        Some(Self {
            peak_buffer_bytes: u64::try_from(buffer_bytes).ok()?,
            peak_owned_allocation_bytes,
            ..Self::default()
        })
    }

    pub(crate) fn record_read(
        mut self,
        requested: usize,
        read: usize,
        first_file_touch: bool,
    ) -> Option<Self> {
        self.backend_requested_bytes = self
            .backend_requested_bytes
            .checked_add(u64::try_from(requested).ok()?)?;
        self.bytes_read = self.bytes_read.checked_add(u64::try_from(read).ok()?)?;
        self.file_touches = self.file_touches.checked_add(u64::from(first_file_touch))?;
        self.chunk_touches = self.chunk_touches.checked_add(1)?;
        Some(self)
    }

    pub(crate) const fn from_checkpoint(
        checkpoint: OfflineInspectionCounterCheckpoint,
    ) -> Option<Self> {
        if checkpoint.bytes_read > checkpoint.backend_requested_bytes
            || checkpoint.file_touches > checkpoint.chunk_touches
        {
            return None;
        }
        Some(Self {
            backend_requested_bytes: checkpoint.backend_requested_bytes,
            bytes_read: checkpoint.bytes_read,
            peak_buffer_bytes: checkpoint.peak_buffer_bytes,
            peak_owned_allocation_bytes: checkpoint.peak_owned_allocation_bytes,
            decoder_allocated_bytes: checkpoint.decoder_allocated_bytes,
            file_touches: checkpoint.file_touches,
            chunk_touches: checkpoint.chunk_touches,
            checkpoint_revalidated_files: checkpoint.checkpoint_revalidated_files,
            checkpoint_revalidated_bytes: checkpoint.checkpoint_revalidated_bytes,
            checkpoint_rejections: checkpoint.checkpoint_rejections,
        })
    }

    pub(crate) fn record_checkpoint_revalidation(mut self, files: u64, bytes: u64) -> Option<Self> {
        self.checkpoint_revalidated_files = self.checkpoint_revalidated_files.checked_add(files)?;
        self.checkpoint_revalidated_bytes = self.checkpoint_revalidated_bytes.checked_add(bytes)?;
        Some(self)
    }

    pub(crate) fn record_checkpoint_rejection(mut self) -> Option<Self> {
        self.checkpoint_rejections = self.checkpoint_rejections.checked_add(1)?;
        Some(self)
    }
    pub(crate) fn record_owned_allocation_peak(mut self, bytes: u64) -> Self {
        self.peak_owned_allocation_bytes = self.peak_owned_allocation_bytes.max(bytes);
        self
    }
    pub const fn backend_requested_bytes(self) -> u64 {
        self.backend_requested_bytes
    }
    pub const fn bytes_read(self) -> u64 {
        self.bytes_read
    }
    pub const fn peak_buffer_bytes(self) -> u64 {
        self.peak_buffer_bytes
    }
    pub const fn peak_owned_allocation_bytes(self) -> u64 {
        self.peak_owned_allocation_bytes
    }
    pub const fn decoder_allocated_bytes(self) -> u64 {
        self.decoder_allocated_bytes
    }
    pub const fn file_touches(self) -> u64 {
        self.file_touches
    }
    pub const fn chunk_touches(self) -> u64 {
        self.chunk_touches
    }
    pub const fn checkpoint_revalidated_files(self) -> u64 {
        self.checkpoint_revalidated_files
    }
    pub const fn checkpoint_revalidated_bytes(self) -> u64 {
        self.checkpoint_revalidated_bytes
    }
    pub const fn checkpoint_rejections(self) -> u64 {
        self.checkpoint_rejections
    }
}
