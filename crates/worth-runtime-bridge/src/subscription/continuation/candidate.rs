use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::super::{
    BridgeSubscriptionContinuationCandidateIdentity, BridgeSubscriptionContinuationIndexIdentity,
};
use super::{BridgeSubscriptionContinuationCandidateInput, BridgeSubscriptionContinuationKind};

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
    pub(super) fn new(
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
            continuation_candidate_identity:
                BridgeSubscriptionContinuationCandidateIdentity::admit_bridge_owned(format!(
                    "bridge-subscription-continuation-candidate-id:sha256:{digest:x}"
                )),
            candidate_slot,
            continuation_kind: input.continuation_kind(),
            authority_digest: Arc::from(input.authority_digest().to_owned()),
            locality_key: Arc::from(input.locality_key().to_owned()),
            child_basis_digests: input.child_basis_digests().to_vec().into(),
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
