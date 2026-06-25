#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PageRecordCounterSnapshot {
    page_read_count: u32,
    page_write_count: u32,
    frame_decode_count: u32,
    record_locate_count: u32,
    slot_lookup_count: u32,
    page_local_scan_count: u32,
    record_payload_view_count: u32,
}

impl PageRecordCounterSnapshot {
    pub const fn for_append(page_local_scan_count: u32) -> Self {
        Self {
            page_read_count: 1,
            page_write_count: 0,
            frame_decode_count: 0,
            record_locate_count: 0,
            slot_lookup_count: 0,
            page_local_scan_count,
            record_payload_view_count: 0,
        }
    }

    pub const fn with_page_write(mut self) -> Self {
        self.page_write_count += 1;
        self
    }

    pub const fn for_locate_attempt() -> Self {
        Self {
            page_read_count: 1,
            page_write_count: 0,
            frame_decode_count: 0,
            record_locate_count: 1,
            slot_lookup_count: 0,
            page_local_scan_count: 0,
            record_payload_view_count: 0,
        }
    }

    pub const fn with_slot_lookup(mut self) -> Self {
        self.slot_lookup_count += 1;
        self
    }

    pub const fn with_frame_decode(mut self) -> Self {
        self.frame_decode_count += 1;
        self
    }

    pub const fn with_record_payload_view(mut self) -> Self {
        self.record_payload_view_count += 1;
        self
    }

    pub const fn merge(self, other: Self) -> Self {
        Self {
            page_read_count: self.page_read_count + other.page_read_count,
            page_write_count: self.page_write_count + other.page_write_count,
            frame_decode_count: self.frame_decode_count + other.frame_decode_count,
            record_locate_count: self.record_locate_count + other.record_locate_count,
            slot_lookup_count: self.slot_lookup_count + other.slot_lookup_count,
            page_local_scan_count: self.page_local_scan_count + other.page_local_scan_count,
            record_payload_view_count: self.record_payload_view_count
                + other.record_payload_view_count,
        }
    }

    pub const fn page_read_count(self) -> u32 {
        self.page_read_count
    }

    pub const fn page_write_count(self) -> u32 {
        self.page_write_count
    }

    pub const fn frame_decode_count(self) -> u32 {
        self.frame_decode_count
    }

    pub const fn record_locate_count(self) -> u32 {
        self.record_locate_count
    }

    pub const fn slot_lookup_count(self) -> u32 {
        self.slot_lookup_count
    }

    pub const fn page_local_scan_count(self) -> u32 {
        self.page_local_scan_count
    }

    pub const fn record_payload_view_count(self) -> u32 {
        self.record_payload_view_count
    }
}
