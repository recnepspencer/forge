#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum RecordAllocationClass {
    InlinePage = 1,
    Extent = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordFreeSpaceManifestEntry {
    class: RecordAllocationClass,
    owner: u64,
    first_unallocated: u64,
    unallocated_count: u64,
    generation: u64,
}

impl RecordFreeSpaceManifestEntry {
    pub fn new(
        class: RecordAllocationClass,
        owner: u64,
        first_unallocated: u64,
        unallocated_count: u64,
        generation: u64,
    ) -> Option<Self> {
        (owner > 0 && first_unallocated > 0 && unallocated_count > 0 && generation > 0).then_some(
            Self {
                class,
                owner,
                first_unallocated,
                unallocated_count,
                generation,
            },
        )
    }
    pub const fn class(self) -> RecordAllocationClass {
        self.class
    }
    pub const fn owner(self) -> u64 {
        self.owner
    }
    pub const fn first_unallocated(self) -> u64 {
        self.first_unallocated
    }
    pub const fn unallocated_count(self) -> u64 {
        self.unallocated_count
    }
    pub const fn generation(self) -> u64 {
        self.generation
    }
}
