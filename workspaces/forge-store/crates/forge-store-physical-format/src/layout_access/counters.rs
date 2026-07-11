#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PhysicalLayoutAccessCounterSnapshot {
    point_lookups: u16,
    range_lookups: u16,
    page_touches: u16,
    index_probes: u16,
    key_comparisons: u16,
    range_steps: u16,
    bytes_read: u64,
}

impl PhysicalLayoutAccessCounterSnapshot {
    pub const fn point(bytes_read: u64, page_touches: u16, index_probes: u16) -> Self {
        Self {
            point_lookups: 1,
            range_lookups: 0,
            page_touches,
            index_probes,
            key_comparisons: index_probes,
            range_steps: 0,
            bytes_read,
        }
    }

    pub const fn range(
        bytes_read: u64,
        page_touches: u16,
        index_probes: u16,
        range_steps: u16,
    ) -> Self {
        Self {
            point_lookups: 0,
            range_lookups: 1,
            page_touches,
            index_probes,
            key_comparisons: index_probes,
            range_steps,
            bytes_read,
        }
    }

    pub const fn point_lookups(self) -> u16 {
        self.point_lookups
    }

    pub const fn range_lookups(self) -> u16 {
        self.range_lookups
    }

    pub const fn page_touches(self) -> u16 {
        self.page_touches
    }

    pub const fn index_probes(self) -> u16 {
        self.index_probes
    }

    pub const fn key_comparisons(self) -> u16 {
        self.key_comparisons
    }

    pub const fn range_steps(self) -> u16 {
        self.range_steps
    }

    pub const fn bytes_read(self) -> u64 {
        self.bytes_read
    }
}
