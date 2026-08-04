use super::super::outcome::SubscriptionSupportCertificationLaneOutcome;
use super::expectations::{
    invalid_lane, require_classification, require_no_primary_cause, require_primary_cause,
    require_rejection,
};
use crate::failure::StoreError;
use crate::subscription_support::SubscriptionResumeClassification::{Degraded, Exact};
use crate::subscription_support::SubscriptionSupportDriftCause::SubscriptionSupportCompatibilityDrift;

pub(super) fn validate_exact_compatibility(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, Exact)?;
    require_no_primary_cause(outcome)?;
    if outcome
        .counter_snapshot
        .support_exact_compatible_migration_count()
        == 0
        || outcome
            .counter_snapshot
            .support_compatibility_receipt_binding_count()
            == 0
    {
        return invalid_lane(
            outcome,
            "exact support compatibility lane must bind exact migration and receipt counters",
        );
    }
    Ok(())
}

pub(super) fn validate_degraded_compatibility(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, Degraded)?;
    require_primary_cause(outcome, SubscriptionSupportCompatibilityDrift)?;
    if outcome
        .counter_snapshot
        .support_degraded_compatibility_count()
        == 0
    {
        return invalid_lane(
            outcome,
            "degraded support compatibility lane must bind degraded compatibility counter",
        );
    }
    Ok(())
}

pub(super) fn validate_compatibility_rejection(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_rejection(outcome)?;
    if outcome
        .counter_snapshot
        .support_version_skew_rejection_count()
        == 0
    {
        return invalid_lane(
            outcome,
            "support compatibility rejection lane must bind version-skew rejection counter",
        );
    }
    Ok(())
}
