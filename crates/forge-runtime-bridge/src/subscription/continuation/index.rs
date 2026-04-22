use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::super::{
    BridgeActiveSubscription, BridgeActiveSubscriptionIdentity, BridgeAdmittedSubscriptionIdentity,
    BridgeSubscriptionBasisIdentity, BridgeSubscriptionContinuationIndexIdentity,
    BridgeSubscriptionCounters,
};
use super::{
    BridgeSubscriptionContinuationCandidate, BridgeSubscriptionContinuationCandidateInput,
    BridgeSubscriptionContinuationIndexRejection, BridgeSubscriptionContinuationIndexRejectionKind,
};

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
