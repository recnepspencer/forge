/// Exact sealed WAL group interval whose admitted durability barrier is physical work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalWalBarrierScope {
    group: [u8; 32],
    membership: [u8; 32],
    member_count: u32,
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
        group: [u8; 32],
        membership: [u8; 32],
        member_count: u32,
        segment: u64,
        generation: u64,
        lsn_start: u64,
        lsn_end_exclusive: u64,
        append_offset: u64,
        append_byte_count: u64,
    ) -> Option<Self> {
        if group == [0; 32]
            || membership == [0; 32]
            || member_count == 0
            || segment == 0
            || generation == 0
            || lsn_end_exclusive <= lsn_start
            || append_byte_count == 0
            || append_offset.checked_add(append_byte_count).is_none()
        {
            return None;
        }
        Some(Self {
            group,
            membership,
            member_count,
            segment,
            generation,
            lsn_start,
            lsn_end_exclusive,
            append_offset,
            append_byte_count,
        })
    }

    pub const fn group(self) -> [u8; 32] {
        self.group
    }

    pub const fn membership(self) -> [u8; 32] {
        self.membership
    }

    pub const fn group_member_count(self) -> u32 {
        self.member_count
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
