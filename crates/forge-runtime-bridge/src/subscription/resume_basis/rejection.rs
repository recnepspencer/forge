use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::subscription::BridgeSubscriptionCounters;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionResumeBasisRejectionKind {
    ActiveSubscriptionMismatch,
    AdmittedSubscriptionMismatch,
    BasisMismatch,
    CostProfileMismatch,
    ConsumerContractMismatch,
    DeliveryBasisMissing,
    DeliveryBasisMismatch,
    TemporalBasisMissing,
    InflightAsyncBasisMissing,
    InflightAsyncGenerationMissing,
    RetentionTruncated,
    CrossBranchResumeRejected,
}

impl BridgeSubscriptionResumeBasisRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveSubscriptionMismatch => "active_subscription_mismatch",
            Self::AdmittedSubscriptionMismatch => "admitted_subscription_mismatch",
            Self::BasisMismatch => "basis_mismatch",
            Self::CostProfileMismatch => "cost_profile_mismatch",
            Self::ConsumerContractMismatch => "consumer_contract_mismatch",
            Self::DeliveryBasisMissing => "delivery_basis_missing",
            Self::DeliveryBasisMismatch => "delivery_basis_mismatch",
            Self::TemporalBasisMissing => "temporal_basis_missing",
            Self::InflightAsyncBasisMissing => "inflight_async_basis_missing",
            Self::InflightAsyncGenerationMissing => "inflight_async_generation_missing",
            Self::RetentionTruncated => "retention_truncated",
            Self::CrossBranchResumeRejected => "cross_branch_resume_rejected",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionResumeBasisRejection {
    rejection_kind: BridgeSubscriptionResumeBasisRejectionKind,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionResumeBasisRejection {
    pub(crate) fn new(rejection_kind: BridgeSubscriptionResumeBasisRejectionKind) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-resume-basis-rejection|kind={}",
            rejection_kind.as_str()
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            counters: BridgeSubscriptionCounters::from_resume_basis_rejection(
                matches!(
                    rejection_kind,
                    BridgeSubscriptionResumeBasisRejectionKind::CrossBranchResumeRejected
                ),
                matches!(
                    rejection_kind,
                    BridgeSubscriptionResumeBasisRejectionKind::DeliveryBasisMismatch
                        | BridgeSubscriptionResumeBasisRejectionKind::DeliveryBasisMissing
                ),
                matches!(
                    rejection_kind,
                    BridgeSubscriptionResumeBasisRejectionKind::InflightAsyncGenerationMissing
                ),
                matches!(
                    rejection_kind,
                    BridgeSubscriptionResumeBasisRejectionKind::RetentionTruncated
                ),
            ),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-resume-basis-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionResumeBasisRejectionKind {
        self.rejection_kind
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
