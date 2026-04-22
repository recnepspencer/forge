use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::super::{BridgeSubscriptionCounters, BridgeSubscriptionDuplicateReplayPolicyIdentity};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionDuplicateReplayPolicyKind {
    SuppressAcknowledgedMembers,
    RedeliverAcknowledgedMembersWhenIdempotent,
    RejectDuplicateReplay,
}

impl BridgeSubscriptionDuplicateReplayPolicyKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SuppressAcknowledgedMembers => "suppress_acknowledged_members",
            Self::RedeliverAcknowledgedMembersWhenIdempotent => {
                "redeliver_acknowledged_members_when_idempotent"
            }
            Self::RejectDuplicateReplay => "reject_duplicate_replay",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionDuplicateReplayPolicy {
    duplicate_replay_policy_identity: BridgeSubscriptionDuplicateReplayPolicyIdentity,
    policy_kind: BridgeSubscriptionDuplicateReplayPolicyKind,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionDuplicateReplayPolicy {
    pub(crate) fn select(policy_kind: BridgeSubscriptionDuplicateReplayPolicyKind) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-duplicate-replay-policy|kind={}",
            policy_kind.as_str()
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            duplicate_replay_policy_identity: BridgeSubscriptionDuplicateReplayPolicyIdentity::new(
                format!("bridge-subscription-duplicate-replay-policy-id:sha256:{digest:x}"),
            ),
            policy_kind,
            counters: BridgeSubscriptionCounters::from_duplicate_replay_policy_selection(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-duplicate-replay-policy:sha256:{digest:x}"
            )),
        }
    }

    pub fn duplicate_replay_policy_identity(
        &self,
    ) -> &BridgeSubscriptionDuplicateReplayPolicyIdentity {
        &self.duplicate_replay_policy_identity
    }

    pub fn policy_kind(&self) -> BridgeSubscriptionDuplicateReplayPolicyKind {
        self.policy_kind
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
