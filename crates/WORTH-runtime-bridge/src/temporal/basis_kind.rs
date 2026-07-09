#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeTemporalBasisKind {
    Authoritative,
    BranchHead,
    Historical,
    CdcCursor,
}

impl BridgeTemporalBasisKind {
    pub const fn canonical_label(self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
            Self::BranchHead => "branch-head",
            Self::Historical => "historical",
            Self::CdcCursor => "cdc-cursor",
        }
    }
}
