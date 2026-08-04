use super::super::outcome::SubscriptionSupportCertificationLaneOutcome;
use super::expectations::{
    invalid_lane, require_classification, require_no_primary_cause, require_rejection,
};
use crate::failure::StoreError;
use crate::subscription_support::SubscriptionResumeClassification::{Exact, RebuildRequired};

pub(super) fn validate_retention_exact(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, Exact)?;
    require_no_primary_cause(outcome)?;
    if outcome.counter_snapshot.support_retained_family_count() == 0 {
        return invalid_lane(
            outcome,
            "retained support lane must bind retained-family counter",
        );
    }
    Ok(())
}

pub(super) fn validate_retention_compacted(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, Exact)?;
    require_no_primary_cause(outcome)?;
    if outcome.counter_snapshot.support_compacted_basis_count() == 0 {
        return invalid_lane(
            outcome,
            "compacted support lane must bind compacted-basis counter",
        );
    }
    Ok(())
}

pub(super) fn validate_retention_reclaimed(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, RebuildRequired)?;
    require_no_primary_cause(outcome)?;
    if outcome.counter_snapshot.support_reclaimed_family_count() == 0
        || outcome.counter_snapshot.support_reclaim_consequence_count() == 0
    {
        return invalid_lane(
            outcome,
            "reclaimed rebuildable lane must bind reclaim and consequence counters",
        );
    }
    Ok(())
}

pub(super) fn validate_retention_expired(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_rejection(outcome)?;
    if outcome.counter_snapshot.support_expired_family_count() == 0
        || outcome.counter_snapshot.support_policy_expiration_count() == 0
    {
        return invalid_lane(
            outcome,
            "expired support lane must bind expiration and policy counters",
        );
    }
    Ok(())
}
