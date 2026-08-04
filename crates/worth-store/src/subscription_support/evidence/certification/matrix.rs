use super::lane::{
    SubscriptionSupportCertificationLaneKind, SubscriptionSupportCertificationMatrixStatus,
};
use super::outcome::SubscriptionSupportCertificationLaneOutcome;
use super::validation::validate_lane_semantics;
use crate::failure::{StoreError, StoreErrorKind};
use crate::subscription_support::SubscriptionResumeClassification;
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportCertificationMatrix {
    lane_outcomes: Vec<SubscriptionSupportCertificationLaneOutcome>,
    status: SubscriptionSupportCertificationMatrixStatus,
}

impl SubscriptionSupportCertificationMatrix {
    pub fn from_lane_outcomes(
        mut lane_outcomes: Vec<SubscriptionSupportCertificationLaneOutcome>,
    ) -> Result<Self, StoreError> {
        lane_outcomes.sort_by_key(SubscriptionSupportCertificationLaneOutcome::lane);
        let mut seen = BTreeSet::new();
        for outcome in &lane_outcomes {
            if !seen.insert(outcome.lane()) {
                return Err(StoreError::new(
                    StoreErrorKind::SubscriptionSupportClassificationViolation,
                    "subscription-support certification matrix received a duplicate lane",
                ));
            }
            validate_lane_semantics(outcome)?;
        }
        for required in SubscriptionSupportCertificationLaneKind::phase_5b_required() {
            if !seen.contains(required) {
                return Err(StoreError::new(
                    StoreErrorKind::SubscriptionSupportClassificationViolation,
                    "subscription-support certification matrix is missing a required Phase 5B lane",
                ));
            }
        }
        let phase_6a_complete = SubscriptionSupportCertificationLaneKind::phase_6a_required()
            .iter()
            .all(|required| seen.contains(required));
        Ok(Self {
            lane_outcomes,
            status: if phase_6a_complete {
                SubscriptionSupportCertificationMatrixStatus::Phase6AOperationalParticipationComplete
            } else {
                SubscriptionSupportCertificationMatrixStatus::Phase5BComplete
            },
        })
    }

    pub fn lane_outcomes(&self) -> &[SubscriptionSupportCertificationLaneOutcome] {
        &self.lane_outcomes
    }

    pub fn status(&self) -> SubscriptionSupportCertificationMatrixStatus {
        self.status
    }

    pub(super) fn truth_digests(&self) -> Vec<&str> {
        self.lane_outcomes
            .iter()
            .map(|outcome| outcome.truth_digest.as_str())
            .collect()
    }

    pub(super) fn artifact_digests(&self) -> Vec<&str> {
        self.lane_outcomes
            .iter()
            .map(|outcome| outcome.artifact_digest.as_str())
            .collect()
    }

    pub(super) fn subscription_support_digests(&self) -> Vec<&str> {
        self.lane_outcomes
            .iter()
            .map(|outcome| outcome.subscription_support_digest.as_str())
            .collect()
    }

    pub(super) fn replay_digests(&self) -> Vec<&str> {
        self.lane_outcomes
            .iter()
            .map(|outcome| outcome.replay_digest.as_str())
            .collect()
    }

    pub(super) fn diagnostics_digests(&self) -> Vec<&str> {
        self.lane_outcomes
            .iter()
            .map(|outcome| outcome.diagnostics_digest.as_str())
            .collect()
    }

    pub(super) fn failure_digests(&self) -> Vec<&str> {
        self.lane_outcomes
            .iter()
            .filter(|outcome| {
                outcome.classification != Some(SubscriptionResumeClassification::Exact)
            })
            .map(|outcome| outcome.truth_digest.as_str())
            .collect()
    }

    pub(super) fn counter_digests(&self) -> Vec<&str> {
        self.lane_outcomes
            .iter()
            .map(|outcome| outcome.counter_digest.as_str())
            .collect()
    }
}
