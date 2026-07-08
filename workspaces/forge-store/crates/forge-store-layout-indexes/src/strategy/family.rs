#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8LayoutStrategyFamily {
    BTree,
    Lsm,
    ChunkTree,
    ExactScan,
}

impl S8LayoutStrategyFamily {
    pub const fn is_baseline_family(self) -> bool {
        matches!(self, Self::BTree | Self::Lsm)
    }
}
