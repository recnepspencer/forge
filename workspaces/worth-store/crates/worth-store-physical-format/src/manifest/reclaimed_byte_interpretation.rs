#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReclaimedByteInterpretation {
    PhysicalZeros,
    LogicalHole,
    UnavailableBytes,
    TierFetchRequired,
    PlatformGradeDenied,
    NonObservableReclaimedStorage,
}

impl ReclaimedByteInterpretation {
    pub const fn is_platform_grade_denial(self) -> bool {
        matches!(self, Self::PlatformGradeDenied)
    }

    pub const fn requires_later_tier_fetch(self) -> bool {
        matches!(self, Self::TierFetchRequired)
    }
}
