#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ExtentRecordCounterSnapshot {
    extent_read_count: u32,
    extent_write_count: u32,
    extent_header_decode_count: u32,
    extent_membership_check_count: u32,
    extent_length_check_count: u32,
    extent_locate_count: u32,
    extent_payload_view_count: u32,
    moved_slot_misuse_rejection_count: u32,
}

impl ExtentRecordCounterSnapshot {
    pub const fn for_append_attempt() -> Self {
        Self::zero()
    }

    pub const fn with_extent_write(mut self) -> Self {
        self.extent_write_count += 1;
        self
    }

    pub const fn for_locate_attempt() -> Self {
        Self {
            extent_read_count: 1,
            extent_locate_count: 1,
            ..Self::zero()
        }
    }

    pub const fn with_membership_check(mut self) -> Self {
        self.extent_membership_check_count += 1;
        self
    }

    pub const fn with_length_check(mut self) -> Self {
        self.extent_length_check_count += 1;
        self
    }

    pub const fn with_header_decode(mut self) -> Self {
        self.extent_header_decode_count += 1;
        self
    }

    pub const fn with_payload_view(mut self) -> Self {
        self.extent_payload_view_count += 1;
        self
    }

    pub const fn with_moved_slot_misuse_rejection(mut self) -> Self {
        self.moved_slot_misuse_rejection_count += 1;
        self
    }

    pub const fn extent_read_count(self) -> u32 {
        self.extent_read_count
    }

    pub const fn extent_write_count(self) -> u32 {
        self.extent_write_count
    }

    pub const fn extent_header_decode_count(self) -> u32 {
        self.extent_header_decode_count
    }

    pub const fn extent_membership_check_count(self) -> u32 {
        self.extent_membership_check_count
    }

    pub const fn extent_length_check_count(self) -> u32 {
        self.extent_length_check_count
    }

    pub const fn extent_locate_count(self) -> u32 {
        self.extent_locate_count
    }

    pub const fn extent_payload_view_count(self) -> u32 {
        self.extent_payload_view_count
    }

    pub const fn moved_slot_misuse_rejection_count(self) -> u32 {
        self.moved_slot_misuse_rejection_count
    }

    const fn zero() -> Self {
        Self {
            extent_read_count: 0,
            extent_write_count: 0,
            extent_header_decode_count: 0,
            extent_membership_check_count: 0,
            extent_length_check_count: 0,
            extent_locate_count: 0,
            extent_payload_view_count: 0,
            moved_slot_misuse_rejection_count: 0,
        }
    }
}
