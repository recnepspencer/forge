use super::super::outcome::SubscriptionSupportCertificationLaneOutcome;
use super::expectations::{invalid_lane, require_classification, require_no_primary_cause};
use crate::failure::StoreError;
use crate::subscription_support::SubscriptionResumeClassification::{
    Degraded, Exact, NotResumable,
};

pub(super) fn validate_portability_full(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, Exact)?;
    require_no_primary_cause(outcome)?;
    if outcome.counter_snapshot.support_portability_plan_count() == 0
        || outcome
            .counter_snapshot
            .support_replication_inclusion_count()
            == 0
    {
        return invalid_lane(
            outcome,
            "full-scope portability lane must bind plan and inclusion counters",
        );
    }
    Ok(())
}

pub(super) fn validate_portability_partial(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, Degraded)?;
    require_no_primary_cause(outcome)?;
    if outcome
        .counter_snapshot
        .support_replication_omission_count()
        == 0
        || outcome
            .counter_snapshot
            .support_portability_omitted_support_count()
            == 0
    {
        return invalid_lane(
            outcome,
            "partial omission portability lane must bind omission counters",
        );
    }
    Ok(())
}

pub(super) fn validate_portability_import(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, Exact)?;
    require_no_primary_cause(outcome)?;
    if outcome.counter_snapshot.support_import_admission_count() == 0 {
        return invalid_lane(
            outcome,
            "import-admitted portability lane must bind import admission counter",
        );
    }
    Ok(())
}

pub(super) fn validate_portability_missing_basis(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_classification(outcome, NotResumable)?;
    require_no_primary_cause(outcome)?;
    if outcome.counter_snapshot.support_import_admission_count() == 0
        || outcome
            .counter_snapshot
            .support_portability_required_basis_count()
            == 0
    {
        return invalid_lane(
            outcome,
            "missing-basis import lane must bind import admission and required-basis counters",
        );
    }
    Ok(())
}
