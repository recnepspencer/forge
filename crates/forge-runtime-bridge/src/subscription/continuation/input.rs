use std::sync::Arc;

use super::BridgeSubscriptionContinuationKind;

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

    pub(super) fn is_rejected(&self) -> bool {
        self.continuation_kind.is_rejected()
    }

    pub(super) fn canonical_input_basis(&self) -> String {
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
