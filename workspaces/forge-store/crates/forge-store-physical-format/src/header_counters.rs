#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PhysicalHeaderDecodeCounterSnapshot {
    header_decode_attempt_count: u32,
    page_header_decode_count: u32,
    frame_header_decode_count: u32,
    unknown_kind_denial_count: u32,
    unsupported_version_denial_count: u32,
    length_mismatch_denial_count: u32,
    reserved_field_denial_count: u32,
    logical_decode_after_invalid_header_count: u32,
}

impl PhysicalHeaderDecodeCounterSnapshot {
    pub const fn for_page_header_attempt() -> Self {
        Self {
            header_decode_attempt_count: 1,
            page_header_decode_count: 1,
            frame_header_decode_count: 0,
            unknown_kind_denial_count: 0,
            unsupported_version_denial_count: 0,
            length_mismatch_denial_count: 0,
            reserved_field_denial_count: 0,
            logical_decode_after_invalid_header_count: 0,
        }
    }

    pub const fn for_frame_header_attempt() -> Self {
        Self {
            header_decode_attempt_count: 1,
            page_header_decode_count: 0,
            frame_header_decode_count: 1,
            unknown_kind_denial_count: 0,
            unsupported_version_denial_count: 0,
            length_mismatch_denial_count: 0,
            reserved_field_denial_count: 0,
            logical_decode_after_invalid_header_count: 0,
        }
    }

    pub const fn with_unknown_kind_denial(mut self) -> Self {
        self.unknown_kind_denial_count = 1;
        self
    }

    pub const fn with_unsupported_version_denial(mut self) -> Self {
        self.unsupported_version_denial_count = 1;
        self
    }

    pub const fn with_length_mismatch_denial(mut self) -> Self {
        self.length_mismatch_denial_count = 1;
        self
    }

    pub const fn with_reserved_field_denial(mut self) -> Self {
        self.reserved_field_denial_count = 1;
        self
    }

    pub const fn header_decode_attempt_count(self) -> u32 {
        self.header_decode_attempt_count
    }

    pub const fn page_header_decode_count(self) -> u32 {
        self.page_header_decode_count
    }

    pub const fn frame_header_decode_count(self) -> u32 {
        self.frame_header_decode_count
    }

    pub const fn unknown_kind_denial_count(self) -> u32 {
        self.unknown_kind_denial_count
    }

    pub const fn unsupported_version_denial_count(self) -> u32 {
        self.unsupported_version_denial_count
    }

    pub const fn length_mismatch_denial_count(self) -> u32 {
        self.length_mismatch_denial_count
    }

    pub const fn reserved_field_denial_count(self) -> u32 {
        self.reserved_field_denial_count
    }

    pub const fn logical_decode_after_invalid_header_count(self) -> u32 {
        self.logical_decode_after_invalid_header_count
    }
}
