use super::super::outcome::SubscriptionSupportCertificationLaneOutcome;
use super::expectations::{
    invalid_lane, require_classification, require_cost_surface, require_no_primary_cause,
};
use crate::failure::StoreError;
use crate::subscription_support::SubscriptionResumeClassification::{
    Degraded, Exact, RebuildRequired,
};
use crate::subscription_support::SubscriptionSupportDensityClass;
use crate::subscription_support::SubscriptionSupportPlanFamily;

pub(super) fn validate_maintenance_rebuild(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, RebuildRequired)?;
    require_no_primary_cause(outcome)?;
    if outcome
        .counter_snapshot
        .support_maintenance_rebuild_debt_count()
        == 0
    {
        return invalid_lane(
            outcome,
            "maintenance rebuild lane must bind rebuild descriptor counter",
        );
    }
    Ok(())
}

pub(super) fn validate_maintenance_refresh(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, Exact)?;
    require_no_primary_cause(outcome)?;
    if outcome.counter_snapshot.support_maintenance_refresh_count() == 0 {
        return invalid_lane(
            outcome,
            "maintenance refresh lane must bind refresh counter",
        );
    }
    Ok(())
}

pub(super) fn validate_maintenance_compatibility(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, Exact)?;
    require_no_primary_cause(outcome)?;
    if outcome
        .counter_snapshot
        .support_maintenance_compatibility_migration_count()
        == 0
    {
        return invalid_lane(
            outcome,
            "maintenance compatibility-migration lane must bind migration counter",
        );
    }
    Ok(())
}

pub(super) fn validate_maintenance_degradation(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, Degraded)?;
    require_no_primary_cause(outcome)?;
    if outcome
        .counter_snapshot
        .support_maintenance_degradation_recovery_count()
        == 0
    {
        return invalid_lane(
            outcome,
            "maintenance degradation-recovery lane must bind degradation counter",
        );
    }
    Ok(())
}

pub(super) fn validate_maintenance_interrupted(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    match outcome.classification {
        Some(Exact | Degraded | RebuildRequired) => {}
        _ => {
            return invalid_lane(
                outcome,
                "maintenance interrupted-restart lane must preserve the recovered work posture",
            );
        }
    }
    require_no_primary_cause(outcome)?;
    if outcome
        .counter_snapshot
        .support_maintenance_interrupted_restart_recovery_count()
        == 0
    {
        return invalid_lane(
            outcome,
            "maintenance interrupted-restart lane must bind restart-recovery counter",
        );
    }
    Ok(())
}

pub(super) fn validate_maintenance_delay(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    match outcome.classification {
        Some(Exact | Degraded | RebuildRequired) => {}
        _ => {
            return invalid_lane(
                outcome,
                "maintenance delayed lane must preserve the admitted maintenance posture",
            );
        }
    }
    require_no_primary_cause(outcome)?;
    let cost_surface = require_cost_surface(outcome)?;
    if cost_surface.plan_family() != SubscriptionSupportPlanFamily::MaintenanceParticipationPlan
        || cost_surface.density_class() != SubscriptionSupportDensityClass::MaintenanceKeyBatch
        || cost_surface.allocation_scope()
            != crate::SubscriptionSupportAllocationScope::OperatorReport
        || cost_surface.scanned_support_rows() == 0
        || outcome.counter_snapshot.support_maintenance_delay_count() == 0
    {
        return invalid_lane(
            outcome,
            "maintenance delayed lane must bind delayed debt reporting through operator-report cost surface",
        );
    }
    Ok(())
}

pub(super) fn validate_maintenance_coalesced(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, RebuildRequired)?;
    require_no_primary_cause(outcome)?;
    if outcome
        .counter_snapshot
        .support_maintenance_rebuild_debt_count()
        == 0
    {
        return invalid_lane(
            outcome,
            "maintenance coalesced rebuild lane must still bind rebuild admission",
        );
    }
    if outcome
        .counter_snapshot
        .support_maintenance_coalesced_duplicate_count()
        == 0
    {
        return invalid_lane(
            outcome,
            "maintenance coalesced lane must bind duplicate-coalescing counter",
        );
    }
    Ok(())
}
