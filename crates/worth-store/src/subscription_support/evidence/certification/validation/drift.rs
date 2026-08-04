use super::super::outcome::SubscriptionSupportCertificationLaneOutcome;
use super::expectations::{
    invalid_lane, require_classification, require_primary_cause, require_suppressed_causes,
};
use crate::failure::StoreError;
use crate::subscription_support::SubscriptionResumeClassification::NotResumable;
use crate::subscription_support::SubscriptionSupportDriftCause::{
    SubscriptionSupportBasisDrift, SubscriptionSupportCompatibilityDrift,
    SubscriptionSupportCursorDrift, SubscriptionSupportDigestMismatch,
    SubscriptionSupportFamilyMismatch,
};

pub(super) fn validate_missing_rebuild_basis(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, NotResumable)?;
    require_primary_cause(outcome, SubscriptionSupportDigestMismatch)?;
    if outcome
        .counter_snapshot
        .support_maintenance_rebuild_debt_count()
        != 0
    {
        return invalid_lane(
            outcome,
            "missing rebuild basis lane must not claim maintenance rebuild admission",
        );
    }
    Ok(())
}

pub(super) fn validate_basis_drift(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, NotResumable)?;
    require_primary_cause(outcome, SubscriptionSupportBasisDrift)?;
    Ok(())
}

pub(super) fn validate_cursor_drift(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, NotResumable)?;
    require_primary_cause(outcome, SubscriptionSupportCursorDrift)?;
    Ok(())
}

pub(super) fn validate_support_digest_drift(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, NotResumable)?;
    require_primary_cause(outcome, SubscriptionSupportDigestMismatch)?;
    Ok(())
}

pub(super) fn validate_compatibility_drift(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, NotResumable)?;
    require_primary_cause(outcome, SubscriptionSupportCompatibilityDrift)?;
    Ok(())
}

pub(super) fn validate_cross_family_reuse(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, NotResumable)?;
    require_primary_cause(outcome, SubscriptionSupportFamilyMismatch)?;
    Ok(())
}

pub(super) fn validate_basis_precedence(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, NotResumable)?;
    require_primary_cause(outcome, SubscriptionSupportBasisDrift)?;
    require_suppressed_causes(
        outcome,
        &[
            SubscriptionSupportCursorDrift,
            SubscriptionSupportDigestMismatch,
        ],
    )?;
    Ok(())
}

pub(super) fn validate_compatibility_precedence(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, NotResumable)?;
    require_primary_cause(outcome, SubscriptionSupportCompatibilityDrift)?;
    require_suppressed_causes(outcome, &[SubscriptionSupportDigestMismatch])?;
    Ok(())
}
