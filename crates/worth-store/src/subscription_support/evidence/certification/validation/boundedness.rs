use super::super::outcome::SubscriptionSupportCertificationLaneOutcome;
use super::expectations::{invalid_lane, require_classification, require_cost_surface};
use crate::failure::StoreError;
use crate::subscription_support::SubscriptionResumeClassification::{
    Exact, NotResumable, RebuildRequired,
};
use crate::subscription_support::SubscriptionSupportDensityClass;
use crate::subscription_support::SubscriptionSupportPlanFamily;

pub(super) fn validate_batch_classification_debt(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, NotResumable)?;
    if require_cost_surface(outcome)?.density_class()
        != SubscriptionSupportDensityClass::FamilyBatchClassificationDebt
    {
        return invalid_lane(
            outcome,
            "batch debt lane must carry family-batch debt density",
        );
    }
    Ok(())
}

pub(super) fn validate_family_local_bounded(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, Exact)?;
    let cost_surface = require_cost_surface(outcome)?;
    if cost_surface.plan_family() != SubscriptionSupportPlanFamily::RetentionParticipationPlan
        || cost_surface.density_class() != SubscriptionSupportDensityClass::FamilyLocalBatch
        || cost_surface.allocation_scope()
            != crate::SubscriptionSupportAllocationScope::FamilyLocalBatch
        || cost_surface.scanned_support_rows() == 0
        || outcome.counter_snapshot.support_retention_plan_count() == 0
        || outcome
            .counter_snapshot
            .support_retention_affected_entries()
            != cost_surface.scanned_support_rows()
    {
        return invalid_lane(
            outcome,
            "family-local bounded lane must bind retention family-local breadth exactly",
        );
    }
    Ok(())
}

pub(super) fn validate_basis_local_bounded(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, Exact)?;
    let cost_surface = require_cost_surface(outcome)?;
    if cost_surface.plan_family() != SubscriptionSupportPlanFamily::CompatibilityParticipationPlan
        || cost_surface.density_class() != SubscriptionSupportDensityClass::BasisLocalBatch
        || cost_surface.allocation_scope() != crate::SubscriptionSupportAllocationScope::ActionLocal
        || cost_surface.scanned_support_rows() == 0
        || outcome.counter_snapshot.support_compatibility_plan_count() == 0
        || outcome.counter_snapshot.support_manifest_admission_count() == 0
        || outcome
            .counter_snapshot
            .support_compatibility_affected_entries()
            != cost_surface.scanned_support_rows()
    {
        return invalid_lane(
            outcome,
            "basis-local bounded lane must bind compatibility basis-local breadth exactly",
        );
    }
    Ok(())
}

pub(super) fn validate_portability_scope_bounded(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, Exact)?;
    let cost_surface = require_cost_surface(outcome)?;
    if cost_surface.plan_family() != SubscriptionSupportPlanFamily::PortabilityParticipationPlan
        || cost_surface.density_class() != SubscriptionSupportDensityClass::PortabilityScopeBatch
        || cost_surface.allocation_scope()
            != crate::SubscriptionSupportAllocationScope::PortabilityManifest
        || cost_surface.scanned_support_rows() == 0
        || outcome.counter_snapshot.support_portability_plan_count() == 0
        || outcome
            .counter_snapshot
            .support_portability_manifest_entries()
            != cost_surface.scanned_support_rows()
    {
        return invalid_lane(
            outcome,
            "portability bounded lane must bind portability manifest breadth exactly",
        );
    }
    Ok(())
}

pub(super) fn validate_maintenance_key_bounded(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, RebuildRequired)?;
    let cost_surface = require_cost_surface(outcome)?;
    if cost_surface.plan_family() != SubscriptionSupportPlanFamily::MaintenanceParticipationPlan
        || cost_surface.density_class() != SubscriptionSupportDensityClass::MaintenanceKeyBatch
        || cost_surface.allocation_scope()
            != crate::SubscriptionSupportAllocationScope::FamilyLocalBatch
        || cost_surface.scanned_support_rows() == 0
        || outcome
            .counter_snapshot
            .support_maintenance_descriptor_count()
            != cost_surface.scanned_support_rows()
    {
        return invalid_lane(
            outcome,
            "maintenance bounded lane must bind maintenance-key breadth exactly",
        );
    }
    Ok(())
}
