use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::super::BridgeSubscriptionCounters;

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
    pub(super) fn new(rejection_kind: BridgeSubscriptionContinuationIndexRejectionKind) -> Self {
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
