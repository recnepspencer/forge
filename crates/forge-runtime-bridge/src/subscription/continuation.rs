use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeActiveSubscription, BridgeActiveSubscriptionIdentity, BridgeAdmittedSubscriptionIdentity,
    BridgeSubscriptionBasisIdentity, BridgeSubscriptionContinuationCandidateIdentity,
    BridgeSubscriptionContinuationChildIdentity, BridgeSubscriptionContinuationDecisionIdentity,
    BridgeSubscriptionContinuationIndexIdentity, BridgeSubscriptionCounters,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionContinuationKind {
    Unchanged,
    OneToOneReplace,
    OneToManySplit,
    MergeLikeContinue,
    BranchLocalContinue,
    RejectedUnsupported,
    RejectedAmbiguous,
    RejectedAuthorityDenied,
    RejectedBranchLeak,
}

impl BridgeSubscriptionContinuationKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unchanged => "unchanged",
            Self::OneToOneReplace => "one_to_one_replace",
            Self::OneToManySplit => "one_to_many_split",
            Self::MergeLikeContinue => "merge_like_continue",
            Self::BranchLocalContinue => "branch_local_continue",
            Self::RejectedUnsupported => "rejected_unsupported",
            Self::RejectedAmbiguous => "rejected_ambiguous",
            Self::RejectedAuthorityDenied => "rejected_authority_denied",
            Self::RejectedBranchLeak => "rejected_branch_leak",
        }
    }

    const fn is_rejected(self) -> bool {
        matches!(
            self,
            Self::RejectedUnsupported
                | Self::RejectedAmbiguous
                | Self::RejectedAuthorityDenied
                | Self::RejectedBranchLeak
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionContinuationCandidateInput {
    continuation_kind: BridgeSubscriptionContinuationKind,
    authority_digest: Arc<str>,
    locality_key: Arc<str>,
    child_basis_digests: Arc<[Arc<str>]>,
}

impl BridgeSubscriptionContinuationCandidateInput {
    pub fn unchanged(
        authority_digest: impl Into<Arc<str>>,
        locality_key: impl Into<Arc<str>>,
        child_basis_digest: impl Into<Arc<str>>,
    ) -> Self {
        Self::new(
            BridgeSubscriptionContinuationKind::Unchanged,
            authority_digest,
            locality_key,
            vec![child_basis_digest.into()],
        )
    }

    pub fn one_to_one_replace(
        authority_digest: impl Into<Arc<str>>,
        locality_key: impl Into<Arc<str>>,
        child_basis_digest: impl Into<Arc<str>>,
    ) -> Self {
        Self::new(
            BridgeSubscriptionContinuationKind::OneToOneReplace,
            authority_digest,
            locality_key,
            vec![child_basis_digest.into()],
        )
    }

    pub fn one_to_many_split(
        authority_digest: impl Into<Arc<str>>,
        locality_key: impl Into<Arc<str>>,
        child_basis_digests: Vec<Arc<str>>,
    ) -> Self {
        Self::new(
            BridgeSubscriptionContinuationKind::OneToManySplit,
            authority_digest,
            locality_key,
            child_basis_digests,
        )
    }

    pub fn merge_like_continue(
        authority_digest: impl Into<Arc<str>>,
        locality_key: impl Into<Arc<str>>,
        child_basis_digest: impl Into<Arc<str>>,
    ) -> Self {
        Self::new(
            BridgeSubscriptionContinuationKind::MergeLikeContinue,
            authority_digest,
            locality_key,
            vec![child_basis_digest.into()],
        )
    }

    pub fn branch_local_continue(
        authority_digest: impl Into<Arc<str>>,
        locality_key: impl Into<Arc<str>>,
        child_basis_digest: impl Into<Arc<str>>,
    ) -> Self {
        Self::new(
            BridgeSubscriptionContinuationKind::BranchLocalContinue,
            authority_digest,
            locality_key,
            vec![child_basis_digest.into()],
        )
    }

    pub fn rejected_unsupported(
        authority_digest: impl Into<Arc<str>>,
        locality_key: impl Into<Arc<str>>,
    ) -> Self {
        Self::new(
            BridgeSubscriptionContinuationKind::RejectedUnsupported,
            authority_digest,
            locality_key,
            Vec::new(),
        )
    }

    pub fn rejected_ambiguous(
        authority_digest: impl Into<Arc<str>>,
        locality_key: impl Into<Arc<str>>,
    ) -> Self {
        Self::new(
            BridgeSubscriptionContinuationKind::RejectedAmbiguous,
            authority_digest,
            locality_key,
            Vec::new(),
        )
    }

    pub fn rejected_authority_denied(
        authority_digest: impl Into<Arc<str>>,
        locality_key: impl Into<Arc<str>>,
    ) -> Self {
        Self::new(
            BridgeSubscriptionContinuationKind::RejectedAuthorityDenied,
            authority_digest,
            locality_key,
            Vec::new(),
        )
    }

    pub fn rejected_branch_leak(
        authority_digest: impl Into<Arc<str>>,
        locality_key: impl Into<Arc<str>>,
    ) -> Self {
        Self::new(
            BridgeSubscriptionContinuationKind::RejectedBranchLeak,
            authority_digest,
            locality_key,
            Vec::new(),
        )
    }

    fn new(
        continuation_kind: BridgeSubscriptionContinuationKind,
        authority_digest: impl Into<Arc<str>>,
        locality_key: impl Into<Arc<str>>,
        child_basis_digests: Vec<Arc<str>>,
    ) -> Self {
        Self {
            continuation_kind,
            authority_digest: authority_digest.into(),
            locality_key: locality_key.into(),
            child_basis_digests: child_basis_digests.into(),
        }
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

    pub fn child_basis_digests(&self) -> &[Arc<str>] {
        &self.child_basis_digests
    }

    fn is_rejected(&self) -> bool {
        self.continuation_kind.is_rejected()
    }

    fn canonical_input_basis(&self) -> String {
        format!(
            "kind={}|authority={}|locality={}|children={}",
            self.continuation_kind.as_str(),
            self.authority_digest.as_ref(),
            self.locality_key.as_ref(),
            self.child_basis_digests
                .iter()
                .map(|digest| digest.as_ref())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionContinuationIndexRejectionKind {
    EmptyCandidateIndex,
    CandidateMissingAuthorityDigest,
    CandidateMissingLocalityKey,
    CandidateMissingChildBasis,
}

impl BridgeSubscriptionContinuationIndexRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyCandidateIndex => "empty_candidate_index",
            Self::CandidateMissingAuthorityDigest => "candidate_missing_authority_digest",
            Self::CandidateMissingLocalityKey => "candidate_missing_locality_key",
            Self::CandidateMissingChildBasis => "candidate_missing_child_basis",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionContinuationIndexRejection {
    rejection_kind: BridgeSubscriptionContinuationIndexRejectionKind,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionContinuationIndexRejection {
    fn new(rejection_kind: BridgeSubscriptionContinuationIndexRejectionKind) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-continuation-index-rejection|kind={}",
            rejection_kind.as_str()
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            counters: BridgeSubscriptionCounters::from_continuation_rejection(false, false),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-continuation-index-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionContinuationIndexRejectionKind {
        self.rejection_kind
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionContinuationCandidate {
    continuation_candidate_identity: BridgeSubscriptionContinuationCandidateIdentity,
    candidate_slot: usize,
    continuation_kind: BridgeSubscriptionContinuationKind,
    authority_digest: Arc<str>,
    locality_key: Arc<str>,
    child_basis_digests: Arc<[Arc<str>]>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionContinuationCandidate {
    fn new(
        continuation_index_identity: &BridgeSubscriptionContinuationIndexIdentity,
        candidate_slot: usize,
        input: BridgeSubscriptionContinuationCandidateInput,
    ) -> Self {
        let child_basis = input
            .child_basis_digests()
            .iter()
            .map(|digest| digest.as_ref())
            .collect::<Vec<_>>()
            .join(",");
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-continuation-candidate|index={}|slot={}|kind={}|authority={}|locality={}|children={}",
            continuation_index_identity.as_str(),
            candidate_slot,
            input.continuation_kind().as_str(),
            input.authority_digest(),
            input.locality_key(),
            child_basis,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            continuation_candidate_identity: BridgeSubscriptionContinuationCandidateIdentity::new(
                format!("bridge-subscription-continuation-candidate-id:sha256:{digest:x}"),
            ),
            candidate_slot,
            continuation_kind: input.continuation_kind,
            authority_digest: input.authority_digest,
            locality_key: input.locality_key,
            child_basis_digests: input.child_basis_digests,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-continuation-candidate:sha256:{digest:x}"
            )),
        }
    }

    pub fn continuation_candidate_identity(
        &self,
    ) -> &BridgeSubscriptionContinuationCandidateIdentity {
        &self.continuation_candidate_identity
    }

    pub fn candidate_slot(&self) -> usize {
        self.candidate_slot
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

    pub fn child_basis_digests(&self) -> &[Arc<str>] {
        &self.child_basis_digests
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionContinuationIndex {
    continuation_index_identity: BridgeSubscriptionContinuationIndexIdentity,
    active_subscription_identity: BridgeActiveSubscriptionIdentity,
    admitted_subscription_identity: BridgeAdmittedSubscriptionIdentity,
    basis_identity: BridgeSubscriptionBasisIdentity,
    candidates: Arc<[BridgeSubscriptionContinuationCandidate]>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionContinuationIndex {
    pub(crate) fn build(
        active_subscription: &BridgeActiveSubscription,
        candidate_inputs: Vec<BridgeSubscriptionContinuationCandidateInput>,
    ) -> Result<Self, BridgeSubscriptionContinuationIndexRejection> {
        if candidate_inputs.is_empty() {
            return Err(BridgeSubscriptionContinuationIndexRejection::new(
                BridgeSubscriptionContinuationIndexRejectionKind::EmptyCandidateIndex,
            ));
        }
        for input in &candidate_inputs {
            if input.authority_digest().is_empty() {
                return Err(BridgeSubscriptionContinuationIndexRejection::new(
                    BridgeSubscriptionContinuationIndexRejectionKind::CandidateMissingAuthorityDigest,
                ));
            }
            if input.locality_key().is_empty() {
                return Err(BridgeSubscriptionContinuationIndexRejection::new(
                    BridgeSubscriptionContinuationIndexRejectionKind::CandidateMissingLocalityKey,
                ));
            }
            if !input.is_rejected() && input.child_basis_digests().is_empty()
                || input
                    .child_basis_digests()
                    .iter()
                    .any(|digest| digest.is_empty())
            {
                return Err(BridgeSubscriptionContinuationIndexRejection::new(
                    BridgeSubscriptionContinuationIndexRejectionKind::CandidateMissingChildBasis,
                ));
            }
        }
        let admitted = active_subscription.activation_ready().admitted();
        let candidate_input_basis = candidate_inputs
            .iter()
            .enumerate()
            .map(|(slot, input)| format!("{slot}:{}", input.canonical_input_basis()))
            .collect::<Vec<_>>()
            .join(",");
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-continuation-index|active={}|admitted={}|basis={}|candidates={}",
            active_subscription.active_subscription_identity().as_str(),
            admitted.admitted_subscription_identity().as_str(),
            admitted.basis_binding().basis_identity().as_str(),
            candidate_input_basis,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        let continuation_index_identity = BridgeSubscriptionContinuationIndexIdentity::new(
            format!("bridge-subscription-continuation-index-id:sha256:{digest:x}"),
        );
        let candidates = candidate_inputs
            .into_iter()
            .enumerate()
            .map(|(slot, input)| {
                BridgeSubscriptionContinuationCandidate::new(
                    &continuation_index_identity,
                    slot,
                    input,
                )
            })
            .collect::<Vec<_>>();
        Ok(Self {
            continuation_index_identity,
            active_subscription_identity: active_subscription
                .active_subscription_identity()
                .clone(),
            admitted_subscription_identity: admitted.admitted_subscription_identity().clone(),
            basis_identity: admitted.basis_binding().basis_identity().clone(),
            counters: BridgeSubscriptionCounters::from_continuation_index(candidates.len()),
            candidates: candidates.into(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-continuation-index:sha256:{digest:x}"
            )),
        })
    }

    pub fn continuation_index_identity(&self) -> &BridgeSubscriptionContinuationIndexIdentity {
        &self.continuation_index_identity
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

    pub fn candidates(&self) -> &[BridgeSubscriptionContinuationCandidate] {
        &self.candidates
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

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
    fn new(
        rejection_kind: BridgeSubscriptionContinuationRejectionKind,
        continuation_index: &BridgeSubscriptionContinuationIndex,
    ) -> Self {
        Self::new_with_lookup(rejection_kind, continuation_index, false)
    }

    fn new_with_lookup(
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionContinuationChild {
    continuation_child_identity: BridgeSubscriptionContinuationChildIdentity,
    child_slot: usize,
    child_basis_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionContinuationChild {
    fn new(
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
