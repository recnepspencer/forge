use crate::BackgroundWorkClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackgroundEnvelopeRequest {
    work_class: BackgroundWorkClass,
    resident_frames: u32,
    resident_bytes: u64,
    bounded_pin_pages: u32,
    indefinite_pin_pages: u32,
    allocation_bytes: u64,
    copied_bytes: u64,
    whole_object_bytes: Option<u64>,
    streaming_object_bytes: u64,
    streaming_window_bytes: u64,
}

impl BackgroundEnvelopeRequest {
    pub const fn recovery_planning() -> BackgroundEnvelopeRequestBuilder {
        BackgroundEnvelopeRequestBuilder::new(BackgroundWorkClass::RecoveryPlanning)
    }

    pub const fn compaction_planning() -> BackgroundEnvelopeRequestBuilder {
        BackgroundEnvelopeRequestBuilder::new(BackgroundWorkClass::CompactionPlanning)
    }

    pub const fn scrub_planning() -> BackgroundEnvelopeRequestBuilder {
        BackgroundEnvelopeRequestBuilder::new(BackgroundWorkClass::ScrubPlanning)
    }

    pub const fn import_export() -> BackgroundEnvelopeRequestBuilder {
        BackgroundEnvelopeRequestBuilder::new(BackgroundWorkClass::ImportExport)
    }

    pub const fn large_record_streaming() -> BackgroundEnvelopeRequestBuilder {
        BackgroundEnvelopeRequestBuilder::new(BackgroundWorkClass::LargeRecordStreaming)
    }

    pub const fn work_class(self) -> BackgroundWorkClass {
        self.work_class
    }

    pub const fn resident_frames(self) -> u32 {
        self.resident_frames
    }

    pub const fn resident_bytes(self) -> u64 {
        self.resident_bytes
    }

    pub const fn bounded_pin_pages(self) -> u32 {
        self.bounded_pin_pages
    }

    pub const fn indefinite_pin_pages(self) -> u32 {
        self.indefinite_pin_pages
    }

    pub const fn allocation_bytes(self) -> u64 {
        self.allocation_bytes
    }

    pub const fn copied_bytes(self) -> u64 {
        self.copied_bytes
    }

    pub const fn whole_object_bytes(self) -> Option<u64> {
        self.whole_object_bytes
    }

    pub const fn streaming_object_bytes(self) -> u64 {
        self.streaming_object_bytes
    }

    pub const fn streaming_window_bytes(self) -> u64 {
        self.streaming_window_bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackgroundEnvelopeRequestBuilder {
    request: BackgroundEnvelopeRequest,
}

impl BackgroundEnvelopeRequestBuilder {
    pub const fn new(work_class: BackgroundWorkClass) -> Self {
        Self {
            request: BackgroundEnvelopeRequest {
                work_class,
                resident_frames: 0,
                resident_bytes: 0,
                bounded_pin_pages: 0,
                indefinite_pin_pages: 0,
                allocation_bytes: 0,
                copied_bytes: 0,
                whole_object_bytes: None,
                streaming_object_bytes: 0,
                streaming_window_bytes: 0,
            },
        }
    }

    pub const fn resident_frames(mut self, resident_frames: u32) -> Self {
        self.request.resident_frames = resident_frames;
        self
    }

    pub const fn resident_bytes(mut self, bytes: u64) -> Self {
        self.request.resident_bytes = bytes;
        self
    }

    pub const fn pin_pages_for_bounded_step(mut self, pinned_pages: u32) -> Self {
        self.request.bounded_pin_pages = pinned_pages;
        self
    }

    pub const fn pin_indefinitely(mut self, pinned_pages: u32) -> Self {
        self.request.indefinite_pin_pages = pinned_pages;
        self
    }

    pub const fn allocation_bytes(mut self, bytes: u64) -> Self {
        self.request.allocation_bytes = bytes;
        self
    }

    pub const fn copied_bytes(mut self, bytes: u64) -> Self {
        self.request.copied_bytes = bytes;
        self
    }

    pub const fn whole_object_memory_bytes(mut self, bytes: u64) -> Self {
        self.request.whole_object_bytes = Some(bytes);
        self
    }

    pub const fn streaming_window(mut self, object_bytes: u64, window_bytes: u64) -> Self {
        self.request.streaming_object_bytes = object_bytes;
        self.request.streaming_window_bytes = window_bytes;
        self
    }

    pub const fn finish(self) -> BackgroundEnvelopeRequest {
        self.request
    }
}
