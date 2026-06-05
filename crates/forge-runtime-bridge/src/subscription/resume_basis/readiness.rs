use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::subscription::{BridgeSubscriptionCounters, BridgeSubscriptionReplayReadinessIdentity};

use super::admission::AdmittedBridgeSubscriptionResumeBasis;
use super::basis::BridgeRetainedSubscriptionResumeBasis;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionReplayReadiness {
    replay_readiness_identity: BridgeSubscriptionReplayReadinessIdentity,
    admitted_resume_basis_identity: Arc<str>,
    retained_resume_basis_identity: Arc<str>,
    expected_next_canonical_sequence: usize,
    acknowledged_ordered_cause_sequence: Option<usize>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionReplayReadiness {
    pub(crate) fn prepare(admitted_resume_basis: &AdmittedBridgeSubscriptionResumeBasis) -> Self {
        let retained_basis: &BridgeRetainedSubscriptionResumeBasis =
            admitted_resume_basis.retained_basis();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-replay-readiness|admitted={}|retained={}|next-sequence={}|delivery-ack={}",
            admitted_resume_basis.admitted_resume_basis_identity().as_str(),
            retained_basis.digest(),
            retained_basis.expected_next_canonical_sequence(),
            retained_basis
                .delivery_resume_basis()
                .map(|basis| basis.acknowledged_ordered_cause_sequence().to_string())
                .unwrap_or_else(|| "-".to_owned()),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            replay_readiness_identity: BridgeSubscriptionReplayReadinessIdentity::new(format!(
                "bridge-subscription-replay-readiness-id:sha256:{digest:x}"
            )),
            admitted_resume_basis_identity: Arc::from(
                admitted_resume_basis
                    .admitted_resume_basis_identity()
                    .as_str()
                    .to_owned(),
            ),
            retained_resume_basis_identity: Arc::from(retained_basis.digest().to_owned()),
            expected_next_canonical_sequence: retained_basis.expected_next_canonical_sequence(),
            acknowledged_ordered_cause_sequence: retained_basis
                .delivery_resume_basis()
                .map(|basis| basis.acknowledged_ordered_cause_sequence()),
            counters: BridgeSubscriptionCounters::from_resume_replay_readiness(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-replay-readiness:sha256:{digest:x}"
            )),
        }
    }

    pub fn expected_next_canonical_sequence(&self) -> usize {
        self.expected_next_canonical_sequence
    }

    pub fn admitted_resume_basis_identity(&self) -> &str {
        self.admitted_resume_basis_identity.as_ref()
    }

    pub fn retained_resume_basis_digest(&self) -> &str {
        self.retained_resume_basis_identity.as_ref()
    }

    pub fn acknowledged_ordered_cause_sequence(&self) -> Option<usize> {
        self.acknowledged_ordered_cause_sequence
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
