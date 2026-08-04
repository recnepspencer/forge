/// Exact WAL artifact interval belonging to one append work identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalWalAppendScope {
    segment: u64,
    generation: u64,
    offset: u64,
    byte_count: u64,
    disposition: PhysicalWalFrameWriteDisposition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhysicalWalFrameWriteDisposition {
    CreateSegment,
    AppendExistingSegment,
}

impl PhysicalWalAppendScope {
    pub(in crate::physical_runtime) const fn new(
        segment: u64,
        generation: u64,
        offset: u64,
        byte_count: u64,
        disposition: PhysicalWalFrameWriteDisposition,
    ) -> Option<Self> {
        if segment == 0
            || generation == 0
            || byte_count == 0
            || offset.checked_add(byte_count).is_none()
            || matches!(disposition, PhysicalWalFrameWriteDisposition::CreateSegment) && offset != 0
            || matches!(
                disposition,
                PhysicalWalFrameWriteDisposition::AppendExistingSegment
            ) && offset == 0
        {
            return None;
        }
        Some(Self {
            segment,
            generation,
            offset,
            byte_count,
            disposition,
        })
    }

    pub const fn segment(self) -> u64 {
        self.segment
    }

    pub const fn generation(self) -> u64 {
        self.generation
    }

    pub const fn offset(self) -> u64 {
        self.offset
    }

    pub const fn byte_count(self) -> u64 {
        self.byte_count
    }

    pub const fn disposition(self) -> PhysicalWalFrameWriteDisposition {
        self.disposition
    }
}
