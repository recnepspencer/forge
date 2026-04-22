use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::super::{BridgeSubscriptionContinuationIndexIdentity, BridgeSubscriptionCounters};
use super::BridgeSubscriptionContinuationIndex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionContinuationRejectionKind {
    ActiveSubscriptionMismatch,
    CandidateSlotOutOfRange,
    Unsupported,
    Ambiguous,
    AuthorityDenied,
    BranchLeak,
    UnchangedRequiresExactlyOneChild,
    ReplaceRequiresExactlyOneChild,
    SplitRequiresMultipleChildren,
    MergeLikeRequiresAtLeastOneChild,
    BranchLocalRequiresExactlyOneChild,
}

impl BridgeSubscriptionContinuationRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveSubscriptionMismatch => "active_subscription_mismatch",
            Self::CandidateSlotOutOfRange => "candidate_slot_out_of_range",
            Self::Unsupported => "unsupported",
            Self::Ambiguous => "ambiguous",
            Self::AuthorityDenied => "authority_denied",
            Self::BranchLeak => "branch_leak",
            Self::UnchangedRequiresExactlyOneChild => "unchanged_requires_exactly_one_child",
            Self::ReplaceRequiresExactlyOneChild => "replace_requires_exactly_one_child",
            Self::SplitRequiresMultipleChildren => "split_requires_multiple_children",
            Self::MergeLikeRequiresAtLeastOneChild => "merge_like_requires_at_least_one_child",
            Self::BranchLocalRequiresExactlyOneChild => "branch_local_requires_exactly_one_child",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionContinuationRejection {
    rejection_kind: BridgeSubscriptionContinuationRejectionKind,
    continuation_index_identity: BridgeSubscriptionContinuationIndexIdentity,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionContinuationRejection {
    pub(super) fn new(
        rejection_kind: BridgeSubscriptionContinuationRejectionKind,
        continuation_index: &BridgeSubscriptionContinuationIndex,
    ) -> Self {
        Self::new_with_lookup(rejection_kind, continuation_index, false)
    }

    pub(super) fn new_with_lookup(
        rejection_kind: BridgeSubscriptionContinuationRejectionKind,
        continuation_index: &BridgeSubscriptionContinuationIndex,
        candidate_index_lookup: bool,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-continuation-rejection|kind={}|index={}",
            rejection_kind.as_str(),
            continuation_index.continuation_index_identity().as_str()
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            continuation_index_identity: continuation_index.continuation_index_identity().clone(),
            counters: BridgeSubscriptionCounters::from_continuation_rejection(
                candidate_index_lookup,
                matches!(
                    rejection_kind,
                    BridgeSubscriptionContinuationRejectionKind::BranchLeak
                ),
            ),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-continuation-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionContinuationRejectionKind {
        self.rejection_kind
    }

    pub fn continuation_index_identity(&self) -> &BridgeSubscriptionContinuationIndexIdentity {
        &self.continuation_index_identity
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
