#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalShortcutCounterName {
    LogicalDecodeAfterInvalidHeader,
    WholeStoreMaterializationAttempt,
    LegacyBackendPlatformClaimRejection,
}

impl PhysicalShortcutCounterName {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::LogicalDecodeAfterInvalidHeader => {
                "physical_logical_decode_after_invalid_header_count"
            }
            Self::WholeStoreMaterializationAttempt => {
                "physical_whole_store_materialization_attempt_count"
            }
            Self::LegacyBackendPlatformClaimRejection => {
                "physical_legacy_backend_platform_claim_rejection_count"
            }
        }
    }
}
