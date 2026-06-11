#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoplanarOverlapPolicy {
    ExtractContractsOnly,
}

impl CoplanarOverlapPolicy {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExtractContractsOnly => "extract-contracts-only",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoplanarOverlapBooleanResult {
    NotComputedInMilestoneSix,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoplanarOverlapImprintAction {
    NotAllowedInMilestoneSix,
}
