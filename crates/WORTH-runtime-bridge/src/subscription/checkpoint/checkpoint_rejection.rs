use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::super::BridgeSubscriptionCounters;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionCheckpointRejectionKind {
    ActiveSubscriptionMismatch,
    AdmittedSubscriptionMismatch,
    BasisMismatch,
    FanoutLayoutActiveSubscriptionMismatch,
    FanoutLayoutCostProfileMismatch,
    FanoutLayoutDeliveryFamilyMismatch,
}

impl BridgeSubscriptionCheckpointRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveSubscriptionMismatch => "active_subscription_mismatch",
            Self::AdmittedSubscriptionMismatch => "admitted_subscription_mismatch",
            Self::BasisMismatch => "basis_mismatch",
            Self::FanoutLayoutActiveSubscriptionMismatch => {
                "fanout_layout_active_subscription_mismatch"
            }
            Self::FanoutLayoutCostProfileMismatch => "fanout_layout_cost_profile_mismatch",
            Self::FanoutLayoutDeliveryFamilyMismatch => "fanout_layout_delivery_family_mismatch",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCheckpointRejection {
    rejection_kind: BridgeSubscriptionCheckpointRejectionKind,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCheckpointRejection {
    pub(super) fn new(rejection_kind: BridgeSubscriptionCheckpointRejectionKind) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-checkpoint-rejection|kind={}",
            rejection_kind.as_str()
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            counters: BridgeSubscriptionCounters::from_checkpoint_publication_rejection(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-checkpoint-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionCheckpointRejectionKind {
        self.rejection_kind
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }
}
