#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalSpeculativeWorkKind {
    ReadAhead,
    Prefetch,
    WriteBehind,
}

impl PhysicalSpeculativeWorkKind {
    pub(crate) const fn index(self) -> usize {
        match self {
            Self::ReadAhead => 0,
            Self::Prefetch => 1,
            Self::WriteBehind => 2,
        }
    }
}
