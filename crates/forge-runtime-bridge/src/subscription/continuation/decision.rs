use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::super::{
    BridgeActiveSubscription, BridgeActiveSubscriptionIdentity, BridgeAdmittedSubscriptionIdentity,
    BridgeSubscriptionBasisIdentity, BridgeSubscriptionContinuationCandidateIdentity,
    BridgeSubscriptionContinuationChildIdentity, BridgeSubscriptionContinuationDecisionIdentity,
    BridgeSubscriptionContinuationIndexIdentity, BridgeSubscriptionCounters,
};
use super::{
    BridgeSubscriptionContinuationIndex, BridgeSubscriptionContinuationKind,
    BridgeSubscriptionContinuationRejection, BridgeSubscriptionContinuationRejectionKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionContinuationChild {
    continuation_child_identity: BridgeSubscriptionContinuationChildIdentity,
    child_slot: usize,
    child_basis_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionContinuationChild {
    pub(super) fn new(
        decision_identity: &BridgeSubscriptionContinuationDecisionIdentity,
        child_slot: usize,
        child_basis_digest: Arc<str>,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-continuation-child|decision={}|slot={}|basis={}",
            decision_identity.as_str(),
            child_slot,
            child_basis_digest.as_ref(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            continuation_child_identity: BridgeSubscriptionContinuationChildIdentity::new(format!(
                "bridge-subscription-continuation-child-id:sha256:{digest:x}"
            )),
            child_slot,
            child_basis_digest,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-continuation-child:sha256:{digest:x}"
            )),
        }
    }

    pub fn continuation_child_identity(&self) -> &BridgeSubscriptionContinuationChildIdentity {
        &self.continuation_child_identity
    }

    pub fn child_slot(&self) -> usize {
        self.child_slot
    }

    pub fn child_basis_digest(&self) -> &str {
        self.child_basis_digest.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionContinuationDecision {
    continuation_decision_identity: BridgeSubscriptionContinuationDecisionIdentity,
    continuation_index_identity: BridgeSubscriptionContinuationIndexIdentity,
    continuation_candidate_identity: BridgeSubscriptionContinuationCandidateIdentity,
    active_subscription_identity: BridgeActiveSubscriptionIdentity,
    admitted_subscription_identity: BridgeAdmittedSubscriptionIdentity,
    basis_identity: BridgeSubscriptionBasisIdentity,
    continuation_kind: BridgeSubscriptionContinuationKind,
    authority_digest: Arc<str>,
    locality_key: Arc<str>,
    children: Arc<[BridgeSubscriptionContinuationChild]>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionContinuationDecision {
    pub(crate) fn plan(
        active_subscription: &BridgeActiveSubscription,
        continuation_index: &BridgeSubscriptionContinuationIndex,
        candidate_slot: usize,
    ) -> Result<Self, BridgeSubscriptionContinuationRejection> {
        if active_subscription.active_subscription_identity()
            != continuation_index.active_subscription_identity()
        {
            return Err(BridgeSubscriptionContinuationRejection::new(
                BridgeSubscriptionContinuationRejectionKind::ActiveSubscriptionMismatch,
                continuation_index,
            ));
        }
        let Some(candidate) = continuation_index.candidates().get(candidate_slot) else {
            return Err(BridgeSubscriptionContinuationRejection::new_with_lookup(
                BridgeSubscriptionContinuationRejectionKind::CandidateSlotOutOfRange,
                continuation_index,
                true,
            ));
        };
        let child_count = candidate.child_basis_digests().len();
        match candidate.continuation_kind() {
            BridgeSubscriptionContinuationKind::RejectedUnsupported => {
                return Err(BridgeSubscriptionContinuationRejection::new_with_lookup(
                    BridgeSubscriptionContinuationRejectionKind::Unsupported,
                    continuation_index,
                    true,
                ));
            }
            BridgeSubscriptionContinuationKind::RejectedAmbiguous => {
                return Err(BridgeSubscriptionContinuationRejection::new_with_lookup(
                    BridgeSubscriptionContinuationRejectionKind::Ambiguous,
                    continuation_index,
                    true,
                ));
            }
            BridgeSubscriptionContinuationKind::RejectedAuthorityDenied => {
                return Err(BridgeSubscriptionContinuationRejection::new_with_lookup(
                    BridgeSubscriptionContinuationRejectionKind::AuthorityDenied,
                    continuation_index,
                    true,
                ));
            }
            BridgeSubscriptionContinuationKind::RejectedBranchLeak => {
                return Err(BridgeSubscriptionContinuationRejection::new_with_lookup(
                    BridgeSubscriptionContinuationRejectionKind::BranchLeak,
                    continuation_index,
                    true,
                ));
            }
            BridgeSubscriptionContinuationKind::Unchanged if child_count != 1 => {
                return Err(BridgeSubscriptionContinuationRejection::new_with_lookup(
                    BridgeSubscriptionContinuationRejectionKind::UnchangedRequiresExactlyOneChild,
                    continuation_index,
                    true,
                ));
            }
            BridgeSubscriptionContinuationKind::OneToOneReplace if child_count != 1 => {
                return Err(BridgeSubscriptionContinuationRejection::new_with_lookup(
                    BridgeSubscriptionContinuationRejectionKind::ReplaceRequiresExactlyOneChild,
                    continuation_index,
                    true,
                ));
            }
            BridgeSubscriptionContinuationKind::OneToManySplit if child_count < 2 => {
                return Err(BridgeSubscriptionContinuationRejection::new_with_lookup(
                    BridgeSubscriptionContinuationRejectionKind::SplitRequiresMultipleChildren,
                    continuation_index,
                    true,
                ));
            }
            BridgeSubscriptionContinuationKind::MergeLikeContinue if child_count < 1 => {
                return Err(BridgeSubscriptionContinuationRejection::new_with_lookup(
                    BridgeSubscriptionContinuationRejectionKind::MergeLikeRequiresAtLeastOneChild,
                    continuation_index,
                    true,
                ));
            }
            BridgeSubscriptionContinuationKind::BranchLocalContinue if child_count != 1 => {
                return Err(BridgeSubscriptionContinuationRejection::new_with_lookup(
                    BridgeSubscriptionContinuationRejectionKind::BranchLocalRequiresExactlyOneChild,
                    continuation_index,
                    true,
                ));
            }
            _ => {}
        }

        let child_basis = candidate
            .child_basis_digests()
            .iter()
            .map(|digest| digest.as_ref())
            .collect::<Vec<_>>()
            .join(",");
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-continuation-decision|index={}|candidate={}|active={}|admitted={}|basis={}|kind={}|authority={}|locality={}|children={}",
            continuation_index.continuation_index_identity().as_str(),
            candidate.continuation_candidate_identity().as_str(),
            continuation_index.active_subscription_identity().as_str(),
            continuation_index.admitted_subscription_identity().as_str(),
            continuation_index.basis_identity().as_str(),
            candidate.continuation_kind().as_str(),
            candidate.authority_digest(),
            candidate.locality_key(),
            child_basis,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        let continuation_decision_identity = BridgeSubscriptionContinuationDecisionIdentity::new(
            format!("bridge-subscription-continuation-decision-id:sha256:{digest:x}"),
        );
        let children = candidate
            .child_basis_digests()
            .iter()
            .enumerate()
            .map(|(child_slot, child_basis_digest)| {
                BridgeSubscriptionContinuationChild::new(
                    &continuation_decision_identity,
                    child_slot,
                    child_basis_digest.clone(),
                )
            })
            .collect::<Vec<_>>();
        Ok(Self {
            continuation_decision_identity,
            continuation_index_identity: continuation_index.continuation_index_identity().clone(),
            continuation_candidate_identity: candidate.continuation_candidate_identity().clone(),
            active_subscription_identity: continuation_index.active_subscription_identity().clone(),
            admitted_subscription_identity: continuation_index
                .admitted_subscription_identity()
                .clone(),
            basis_identity: continuation_index.basis_identity().clone(),
            continuation_kind: candidate.continuation_kind(),
            authority_digest: Arc::from(candidate.authority_digest().to_owned()),
            locality_key: Arc::from(candidate.locality_key().to_owned()),
            counters: BridgeSubscriptionCounters::from_continuation_decision(children.len()),
            children: children.into(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-continuation-decision:sha256:{digest:x}"
            )),
        })
    }

    pub fn continuation_decision_identity(
        &self,
    ) -> &BridgeSubscriptionContinuationDecisionIdentity {
        &self.continuation_decision_identity
    }

    pub fn continuation_index_identity(&self) -> &BridgeSubscriptionContinuationIndexIdentity {
        &self.continuation_index_identity
    }

    pub fn continuation_candidate_identity(
        &self,
    ) -> &BridgeSubscriptionContinuationCandidateIdentity {
        &self.continuation_candidate_identity
    }

    pub fn active_subscription_identity(&self) -> &BridgeActiveSubscriptionIdentity {
        &self.active_subscription_identity
    }

    pub fn admitted_subscription_identity(&self) -> &BridgeAdmittedSubscriptionIdentity {
        &self.admitted_subscription_identity
    }

    pub fn basis_identity(&self) -> &BridgeSubscriptionBasisIdentity {
        &self.basis_identity
    }

    pub fn continuation_kind(&self) -> BridgeSubscriptionContinuationKind {
        self.continuation_kind
    }

    pub fn authority_digest(&self) -> &str {
        self.authority_digest.as_ref()
    }

    pub fn locality_key(&self) -> &str {
        self.locality_key.as_ref()
    }

    pub fn children(&self) -> &[BridgeSubscriptionContinuationChild] {
        &self.children
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
