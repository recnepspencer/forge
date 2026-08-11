use super::super::{
    classification_error, SubscriptionSupportFamilyId, SubscriptionSupportOperationalVerdict,
};
use super::evidence_validation::require_non_empty;
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportCompatibilityDecision {
    evidence: SubscriptionSupportCompatibilityDecisionEvidence,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
enum SubscriptionSupportCompatibilityDecisionEvidence {
    ExactCompatibleMigration {
        classifier_equivalence_digest: String,
    },
    DegradedCompatibility {
        drift_reason: String,
    },
    OldReaderRejected {
        reader_version: u16,
        required_minimum_version: u16,
    },
    UnknownFamilyRejected {
        family_id: SubscriptionSupportFamilyId,
    },
    VersionSkewRejected {
        skew_reason: String,
    },
}

#[allow(dead_code)]
impl SubscriptionSupportCompatibilityDecision {
    pub(crate) fn exact_compatible_migration(
        classifier_equivalence_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportCompatibilityDecisionEvidence::ExactCompatibleMigration {
                classifier_equivalence_digest: require_non_empty(
                    "classifier equivalence",
                    classifier_equivalence_digest,
                )?,
            }
            .into(),
        )
    }

    pub(crate) fn degraded_compatibility(
        drift_reason: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportCompatibilityDecisionEvidence::DegradedCompatibility {
                drift_reason: require_non_empty("compatibility drift reason", drift_reason)?,
            }
            .into(),
        )
    }

    pub(crate) fn old_reader_rejected(
        reader_version: u16,
        required_minimum_version: u16,
    ) -> Result<Self, StoreError> {
        if reader_version == 0 || required_minimum_version == 0 {
            return Err(classification_error(
                "subscription-support old-reader rejection requires non-zero versions",
            ));
        }
        if reader_version >= required_minimum_version {
            return Err(classification_error(
                "subscription-support old-reader rejection requires a reader below the admitted window",
            ));
        }
        Ok(
            SubscriptionSupportCompatibilityDecisionEvidence::OldReaderRejected {
                reader_version,
                required_minimum_version,
            }
            .into(),
        )
    }

    pub(crate) fn unknown_family_rejected(family_id: SubscriptionSupportFamilyId) -> Self {
        SubscriptionSupportCompatibilityDecisionEvidence::UnknownFamilyRejected { family_id }.into()
    }

    pub(crate) fn version_skew_rejected(
        skew_reason: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportCompatibilityDecisionEvidence::VersionSkewRejected {
                skew_reason: require_non_empty("version-skew rejection reason", skew_reason)?,
            }
            .into(),
        )
    }

    pub fn verdict(&self) -> SubscriptionSupportOperationalVerdict {
        match &self.evidence {
            SubscriptionSupportCompatibilityDecisionEvidence::ExactCompatibleMigration {
                ..
            } => SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            SubscriptionSupportCompatibilityDecisionEvidence::DegradedCompatibility { .. } => {
                SubscriptionSupportOperationalVerdict::DegradedResumePreserved
            }
            SubscriptionSupportCompatibilityDecisionEvidence::OldReaderRejected { .. }
            | SubscriptionSupportCompatibilityDecisionEvidence::UnknownFamilyRejected { .. }
            | SubscriptionSupportCompatibilityDecisionEvidence::VersionSkewRejected { .. } => {
                SubscriptionSupportOperationalVerdict::RejectedByPolicy
            }
        }
    }

    pub fn kind(&self) -> SubscriptionSupportCompatibilityDecisionKind {
        match &self.evidence {
            SubscriptionSupportCompatibilityDecisionEvidence::ExactCompatibleMigration {
                ..
            } => SubscriptionSupportCompatibilityDecisionKind::ExactCompatibleMigration,
            SubscriptionSupportCompatibilityDecisionEvidence::DegradedCompatibility { .. } => {
                SubscriptionSupportCompatibilityDecisionKind::DegradedCompatibility
            }
            SubscriptionSupportCompatibilityDecisionEvidence::OldReaderRejected { .. } => {
                SubscriptionSupportCompatibilityDecisionKind::OldReaderRejected
            }
            SubscriptionSupportCompatibilityDecisionEvidence::UnknownFamilyRejected { .. } => {
                SubscriptionSupportCompatibilityDecisionKind::UnknownFamilyRejected
            }
            SubscriptionSupportCompatibilityDecisionEvidence::VersionSkewRejected { .. } => {
                SubscriptionSupportCompatibilityDecisionKind::VersionSkewRejected
            }
        }
    }

    pub(super) fn classifier_equivalence_digest(&self) -> Option<&str> {
        match &self.evidence {
            SubscriptionSupportCompatibilityDecisionEvidence::ExactCompatibleMigration {
                classifier_equivalence_digest,
            } => Some(classifier_equivalence_digest),
            _ => None,
        }
    }

    pub(super) fn drift_reason(&self) -> Option<&str> {
        match &self.evidence {
            SubscriptionSupportCompatibilityDecisionEvidence::DegradedCompatibility {
                drift_reason,
            }
            | SubscriptionSupportCompatibilityDecisionEvidence::VersionSkewRejected {
                skew_reason: drift_reason,
            } => Some(drift_reason),
            SubscriptionSupportCompatibilityDecisionEvidence::OldReaderRejected {
                reader_version,
                required_minimum_version,
            } => Some(if reader_version < required_minimum_version {
                "reader below admitted support manifest window"
            } else {
                "invalid old-reader compatibility rejection"
            }),
            SubscriptionSupportCompatibilityDecisionEvidence::UnknownFamilyRejected { .. } => {
                Some("unknown subscription-support family")
            }
            _ => None,
        }
    }
}

impl From<SubscriptionSupportCompatibilityDecisionEvidence>
    for SubscriptionSupportCompatibilityDecision
{
    fn from(evidence: SubscriptionSupportCompatibilityDecisionEvidence) -> Self {
        Self { evidence }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SubscriptionSupportCompatibilityDecisionKind {
    ExactCompatibleMigration,
    DegradedCompatibility,
    OldReaderRejected,
    UnknownFamilyRejected,
    VersionSkewRejected,
}
