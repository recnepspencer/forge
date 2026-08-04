use super::super::outcome::SubscriptionSupportCertificationLaneOutcome;
use super::expectations::{
    invalid_lane, require_classification, require_cost_surface, require_no_primary_cause,
    require_primary_cause,
};
use crate::failure::StoreError;
use crate::subscription_support::SubscriptionResumeClassification::{
    Degraded, Exact, NotResumable, RebuildRequired,
};
use crate::subscription_support::SubscriptionSupportDensityClass;
use crate::subscription_support::SubscriptionSupportDriftCause::{
    SubscriptionSupportDigestMismatch, SubscriptionSupportPlacementUnavailable,
    SubscriptionSupportSessionMemoryMissing,
};
use crate::subscription_support::SubscriptionSupportPlanFamily;

pub(super) fn validate_exact_resume(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, Exact)?;
    require_no_primary_cause(outcome)?;
    Ok(())
}

pub(super) fn validate_result_cost_surface(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, Exact)?;
    let cost_surface = require_cost_surface(outcome)?;
    if cost_surface.plan_family() != SubscriptionSupportPlanFamily::ExactResumeClassificationPlan
        || cost_surface.density_class()
            != SubscriptionSupportDensityClass::SparseIdentityClassification
        || cost_surface.scanned_support_rows() == 0
    {
        return invalid_lane(
            outcome,
            "exact cost surface must bind exact sparse direct-lookup work",
        );
    }
    Ok(())
}

pub(super) fn validate_restart_shard(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, Exact)?;
    if require_cost_surface(outcome)?.restart_shards_touched() != 1 {
        return invalid_lane(
            outcome,
            "restart reconstruction must touch exactly one shard",
        );
    }
    Ok(())
}

pub(super) fn validate_degraded_recoverable(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, Degraded)?;
    require_no_primary_cause(outcome)?;
    Ok(())
}

pub(super) fn validate_missing_support_rebuild(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, RebuildRequired)?;
    require_primary_cause(outcome, SubscriptionSupportDigestMismatch)?;
    if outcome
        .counter_snapshot
        .support_maintenance_rebuild_debt_count()
        == 0
    {
        return invalid_lane(
            outcome,
            "rebuild-required missing support lane must publish a maintenance rebuild descriptor",
        );
    }
    Ok(())
}

pub(super) fn validate_session_memory_loss(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, NotResumable)?;
    require_primary_cause(outcome, SubscriptionSupportSessionMemoryMissing)?;
    Ok(())
}

pub(super) fn validate_tier_recall(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, Exact)?;
    require_primary_cause(outcome, SubscriptionSupportPlacementUnavailable)?;
    Ok(())
}
