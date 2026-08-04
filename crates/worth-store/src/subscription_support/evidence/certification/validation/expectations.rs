use super::super::outcome::SubscriptionSupportCertificationLaneOutcome;
use crate::failure::{StoreError, StoreErrorKind};
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportDriftCause,
    SubscriptionSupportResultCostSurface,
};

pub(super) fn require_classification(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
    expected: SubscriptionResumeClassification,
) -> Result<(), StoreError> {
    if outcome.classification != Some(expected) {
        return invalid_lane(outcome, "certification lane has the wrong classification");
    }
    Ok(())
}

pub(super) fn require_rejection(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    if outcome.classification.is_some() || outcome.primary_cause.is_some() {
        return invalid_lane(
            outcome,
            "typed rejection lane must not carry resume classification evidence",
        );
    }
    Ok(())
}

pub(super) fn require_no_primary_cause(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    if outcome.primary_cause.is_some() || !outcome.suppressed_causes.is_empty() {
        return invalid_lane(
            outcome,
            "clean certification lane must not carry drift causes",
        );
    }
    Ok(())
}

pub(super) fn require_primary_cause(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
    expected: SubscriptionSupportDriftCause,
) -> Result<(), StoreError> {
    if outcome.primary_cause != Some(expected) {
        return invalid_lane(
            outcome,
            "certification lane has the wrong primary drift cause",
        );
    }
    Ok(())
}

pub(super) fn require_suppressed_causes(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
    expected: &[SubscriptionSupportDriftCause],
) -> Result<(), StoreError> {
    if outcome.suppressed_causes != expected {
        return invalid_lane(
            outcome,
            "certification lane has the wrong suppressed drift causes",
        );
    }
    Ok(())
}

pub(super) fn require_cost_surface(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<SubscriptionSupportResultCostSurface, StoreError> {
    outcome.cost_surface.ok_or_else(|| {
        lane_error(
            outcome,
            "certification lane must carry a result cost surface",
        )
    })
}

pub(super) fn invalid_lane<T>(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
    message: &'static str,
) -> Result<T, StoreError> {
    Err(lane_error(outcome, message))
}

pub(super) fn lane_error(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
    message: &'static str,
) -> StoreError {
    StoreError::new(
        StoreErrorKind::SubscriptionSupportClassificationViolation,
        format!("{message}: {:?}", outcome.lane()),
    )
}
