#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S7ColdPlacementState {
    HotAvailable,
    ColdAvailable,
    ColdFetchRequired,
    ColdFetchInProgress,
    ColdUnavailable,
    ColdStale,
    ColdScopeDenied,
    ColdRebindRequired,
}

impl S7ColdPlacementState {
    pub const fn permits_immediate_publication(self) -> bool {
        matches!(self, Self::HotAvailable | Self::ColdAvailable)
    }

    pub const fn permits_movement(self) -> bool {
        matches!(self, Self::HotAvailable | Self::ColdAvailable)
    }
}
