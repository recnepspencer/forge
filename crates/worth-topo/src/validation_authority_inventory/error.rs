use std::fmt;

use super::source_authority::WorthValidationAuthoritySource;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorthValidationAuthorityInventoryError {
    DuplicateSource(String),
    MissingRequiredSource(String),
    MissingRemovalTrigger(String),
    MissingOwner(String),
    CertificationOnlyWithoutCap(String),
    SourceFirewallViolation(String),
    SourceDiscoveryFailure(String),
    UnclassifiedDiscoveredSource(String),
    MilestoneEightSeedClaimsValidatorSelection(String),
}

impl WorthValidationAuthorityInventoryError {
    pub(super) fn duplicate_source(source: WorthValidationAuthoritySource) -> Self {
        Self::DuplicateSource(source.stable_key())
    }

    pub(super) fn missing_required_source(source: WorthValidationAuthoritySource) -> Self {
        Self::MissingRequiredSource(source.stable_key())
    }

    pub(super) fn missing_removal_trigger(source: WorthValidationAuthoritySource) -> Self {
        Self::MissingRemovalTrigger(source.stable_key())
    }

    pub(super) fn missing_owner(source: WorthValidationAuthoritySource) -> Self {
        Self::MissingOwner(source.stable_key())
    }

    pub(super) fn certification_only_without_cap(source: WorthValidationAuthoritySource) -> Self {
        Self::CertificationOnlyWithoutCap(source.stable_key())
    }
}

impl fmt::Display for WorthValidationAuthorityInventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateSource(source) => write!(f, "duplicate authority source `{source}`"),
            Self::MissingRequiredSource(source) => {
                write!(f, "missing required authority source `{source}`")
            }
            Self::MissingRemovalTrigger(source) => {
                write!(f, "authority source `{source}` has no removal trigger")
            }
            Self::MissingOwner(source) => write!(f, "authority source `{source}` has no owner"),
            Self::CertificationOnlyWithoutCap(source) => write!(
                f,
                "authority source `{source}` allows certification-only comparison without cap"
            ),
            Self::SourceFirewallViolation(reason) => {
                write!(f, "validation authority source firewall failed: {reason}")
            }
            Self::SourceDiscoveryFailure(reason) => {
                write!(f, "validation authority source discovery failed: {reason}")
            }
            Self::UnclassifiedDiscoveredSource(source) => {
                write!(
                    f,
                    "unclassified discovered validation authority source `{source}`"
                )
            }
            Self::MilestoneEightSeedClaimsValidatorSelection(seed) => write!(
                f,
                "milestone eight seed `{seed}` cannot claim validator selection authority"
            ),
        }
    }
}

impl std::error::Error for WorthValidationAuthorityInventoryError {}
