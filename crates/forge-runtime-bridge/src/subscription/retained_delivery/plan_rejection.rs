use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::super::BridgeSubscriptionCounters;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionDeliveryReplayPlanRejectionKind {
    EmptyRetainedWindowSet,
    ActiveSubscriptionMismatch,
    AdmittedSubscriptionMismatch,
    BasisMismatch,
    DeliveryFamilyMismatch,
    RetainedWindowReplayReadinessBlocked,
    RetainedWindowNotAfterCheckpoint,
    RetainedWindowSequenceAmbiguous,
}

impl BridgeSubscriptionDeliveryReplayPlanRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyRetainedWindowSet => "empty_retained_window_set",
            Self::ActiveSubscriptionMismatch => "active_subscription_mismatch",
            Self::AdmittedSubscriptionMismatch => "admitted_subscription_mismatch",
            Self::BasisMismatch => "basis_mismatch",
            Self::DeliveryFamilyMismatch => "delivery_family_mismatch",
            Self::RetainedWindowReplayReadinessBlocked => {
                "retained_window_replay_readiness_blocked"
            }
            Self::RetainedWindowNotAfterCheckpoint => "retained_window_not_after_checkpoint",
            Self::RetainedWindowSequenceAmbiguous => "retained_window_sequence_ambiguous",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionDeliveryReplayPlanRejection {
    rejection_kind: BridgeSubscriptionDeliveryReplayPlanRejectionKind,
    rejection_context: Arc<str>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionDeliveryReplayPlanRejection {
    pub(super) fn new(
        rejection_kind: BridgeSubscriptionDeliveryReplayPlanRejectionKind,
        rejection_context: impl Into<Arc<str>>,
    ) -> Self {
        let rejection_context = rejection_context.into();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-delivery-replay-plan-rejection|kind={}|context={}",
            rejection_kind.as_str(),
            rejection_context.as_ref()
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            rejection_context,
            counters: BridgeSubscriptionCounters::from_delivery_replay_plan_rejection(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-delivery-replay-plan-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionDeliveryReplayPlanRejectionKind {
        self.rejection_kind
    }

    pub fn rejection_context(&self) -> &str {
        self.rejection_context.as_ref()
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
