#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReclaimLaterHandoffPolicy {
    blob_lifecycle_claim: bool,
    compaction_claim: bool,
    tier_placement_claim: bool,
}

impl ReclaimLaterHandoffPolicy {
    pub const fn non_claim() -> Self {
        Self {
            blob_lifecycle_claim: false,
            compaction_claim: false,
            tier_placement_claim: false,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn claims_later_lifecycle_for_denial() -> Self {
        Self {
            blob_lifecycle_claim: true,
            compaction_claim: true,
            tier_placement_claim: true,
        }
    }

    pub const fn is_non_claim(self) -> bool {
        !self.blob_lifecycle_claim && !self.compaction_claim && !self.tier_placement_claim
    }
}
