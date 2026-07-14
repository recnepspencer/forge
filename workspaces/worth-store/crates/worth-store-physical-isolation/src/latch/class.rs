#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalLatchClass {
    Root,
    Manifest,
    Segment,
    Extent,
    Page,
    FutureChunk,
}

impl PhysicalLatchClass {
    pub const fn hierarchy_rank(self) -> u8 {
        match self {
            Self::Root => 0,
            Self::Manifest => 1,
            Self::Segment => 2,
            Self::Extent => 3,
            Self::Page => 4,
            Self::FutureChunk => 5,
        }
    }
}
