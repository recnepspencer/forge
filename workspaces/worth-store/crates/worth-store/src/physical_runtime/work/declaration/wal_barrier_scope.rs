/// Exact WAL member interval whose admitted durability barrier is physical work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalWalBarrierScope {
    member: [u8; 32],
    segment: u64,
    generation: u64,
    lsn_start: u64,
    lsn_end_exclusive: u64,
    append_offset: u64,
    append_byte_count: u64,
}

impl PhysicalWalBarrierScope {
    #[allow(clippy::too_many_arguments)]
    pub(in crate::physical_runtime) fn new(
        member: [u8; 32],
        segment: u64,
        generation: u64,
        lsn_start: u64,
        lsn_end_exclusive: u64,
        append_offset: u64,
        append_byte_count: u64,
    ) -> Option<Self> {
        if member == [0; 32]
            || segment == 0
            || generation == 0
            || lsn_end_exclusive <= lsn_start
            || append_byte_count == 0
            || append_offset.checked_add(append_byte_count).is_none()
        {
            return None;
        }
        Some(Self {
            member,
            segment,
            generation,
            lsn_start,
            lsn_end_exclusive,
            append_offset,
            append_byte_count,
        })
    }

    pub const fn member(self) -> [u8; 32] {
        self.member
    }

    pub const fn segment(self) -> u64 {
        self.segment
    }

    pub const fn generation(self) -> u64 {
        self.generation
    }

    pub const fn lsn_start(self) -> u64 {
        self.lsn_start
    }

    pub const fn lsn_end_exclusive(self) -> u64 {
        self.lsn_end_exclusive
    }

    pub const fn append_offset(self) -> u64 {
        self.append_offset
    }

    pub const fn append_byte_count(self) -> u64 {
        self.append_byte_count
    }
}
