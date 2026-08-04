use super::super::outcome::SubscriptionSupportCertificationLaneOutcome;
use super::expectations::{invalid_lane, require_rejection};
use crate::failure::StoreError;

pub(super) fn validate_oversized_payload(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_rejection(outcome)?;
    if outcome
        .counter_snapshot
        .support_payload_budget_rejection_count()
        == 0
    {
        return invalid_lane(
            outcome,
            "oversized payload lane must bind the support payload budget rejection counter",
        );
    }
    Ok(())
}

pub(super) fn validate_access_structure_debt(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_rejection(outcome)?;
    Ok(())
}

pub(super) fn validate_basic_rejection(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_rejection(outcome)?;
    Ok(())
}
