use sha2::{Digest, Sha256};
use worth_store_authority::FenceProof;

use crate::{
    DivergentReplicaHistoryReport, ReplicaHistoryClassification, ReplicaRecoveryFrontier,
    ReplicationPeerId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplicaPromotionIntent {
    candidate_peer: ReplicationPeerId,
    current_frontier: ReplicaRecoveryFrontier,
}

impl ReplicaPromotionIntent {
    pub fn candidate_peer(&self) -> &ReplicationPeerId {
        &self.candidate_peer
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplicaPromotionCandidate {
    intent: ReplicaPromotionIntent,
    history: DivergentReplicaHistoryReport,
    acknowledged_data_loss_lsn: u64,
}

impl ReplicaPromotionCandidate {
    pub const fn acknowledged_data_loss_lsn(&self) -> u64 {
        self.acknowledged_data_loss_lsn
    }

    pub const fn frontier(&self) -> ReplicaRecoveryFrontier {
        self.history.observation().frontier()
    }

    pub const fn history(&self) -> &DivergentReplicaHistoryReport {
        &self.history
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweredReplicaPromotionPlan {
    candidate: ReplicaPromotionCandidate,
    fingerprint: [u8; 32],
}

impl LoweredReplicaPromotionPlan {
    pub const fn fingerprint(&self) -> [u8; 32] {
        self.fingerprint
    }

    pub const fn candidate(&self) -> &ReplicaPromotionCandidate {
        &self.candidate
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplicaPromotionDenial {
    CandidatePeerMismatch,
    DivergentHistory,
    PartialMedia,
    FenceAuthorityMismatch,
    FenceEpochNotAdvanced,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplicaPromotionReceipt {
    plan_fingerprint: [u8; 32],
    promoted_peer: ReplicationPeerId,
    promoted_frontier: ReplicaRecoveryFrontier,
    acknowledged_data_loss_lsn: u64,
    fence_identity: [u8; 32],
    promoted_epoch: worth_store_authority::PromotedAuthorityEpoch,
    durable_target_identity: [u8; 32],
}

impl ReplicaPromotionReceipt {
    pub const fn plan_fingerprint(&self) -> [u8; 32] {
        self.plan_fingerprint
    }

    pub const fn promoted_frontier(&self) -> ReplicaRecoveryFrontier {
        self.promoted_frontier
    }

    pub const fn acknowledged_data_loss_lsn(&self) -> u64 {
        self.acknowledged_data_loss_lsn
    }

    pub fn promoted_peer(&self) -> &ReplicationPeerId {
        &self.promoted_peer
    }

    pub const fn promoted_epoch(&self) -> worth_store_authority::PromotedAuthorityEpoch {
        self.promoted_epoch
    }

    pub const fn fence_identity(&self) -> [u8; 32] {
        self.fence_identity
    }

    pub const fn durable_target_identity(&self) -> [u8; 32] {
        self.durable_target_identity
    }

    pub fn receipt_identity(&self) -> [u8; 32] {
        let mut digest = Sha256::new();
        digest.update(b"worth-store-replica-promotion-receipt-v1");
        digest.update(self.plan_fingerprint);
        digest.update(self.promoted_peer.as_str().as_bytes());
        digest.update(self.promoted_frontier.observed_lsn().to_be_bytes());
        digest.update(self.promoted_frontier.durable_lsn().to_be_bytes());
        digest.update(
            self.promoted_frontier
                .client_acknowledged_lsn()
                .to_be_bytes(),
        );
        digest.update(
            self.promoted_frontier
                .replication_acknowledged_lsn()
                .to_be_bytes(),
        );
        digest.update(self.acknowledged_data_loss_lsn.to_be_bytes());
        digest.update(self.fence_identity);
        digest.update(self.promoted_epoch.get().to_be_bytes());
        digest.update(self.durable_target_identity);
        digest.finalize().into()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ReplicaPromotionOwner;

impl ReplicaPromotionOwner {
    pub fn intent(
        candidate_peer: ReplicationPeerId,
        current_frontier: ReplicaRecoveryFrontier,
    ) -> ReplicaPromotionIntent {
        ReplicaPromotionIntent {
            candidate_peer,
            current_frontier,
        }
    }

    pub fn resolve_candidate(
        intent: ReplicaPromotionIntent,
        history: DivergentReplicaHistoryReport,
    ) -> Result<ReplicaPromotionCandidate, ReplicaPromotionDenial> {
        if history.observation().peer() != &intent.candidate_peer {
            return Err(ReplicaPromotionDenial::CandidatePeerMismatch);
        }
        match history.classification() {
            ReplicaHistoryClassification::Divergent => {
                return Err(ReplicaPromotionDenial::DivergentHistory)
            }
            ReplicaHistoryClassification::PartialContinuation => {
                return Err(ReplicaPromotionDenial::PartialMedia)
            }
            ReplicaHistoryClassification::AttestedContinuation
            | ReplicaHistoryClassification::ReplayDerivedContinuation => {}
        }
        let acknowledged_data_loss_lsn = history
            .observation()
            .frontier()
            .acknowledged_data_loss_from(intent.current_frontier);
        Ok(ReplicaPromotionCandidate {
            intent,
            history,
            acknowledged_data_loss_lsn,
        })
    }

    pub fn lower(candidate: ReplicaPromotionCandidate) -> LoweredReplicaPromotionPlan {
        let fingerprint = promotion_fingerprint(&candidate);
        LoweredReplicaPromotionPlan {
            candidate,
            fingerprint,
        }
    }

    pub fn record_fenced_promotion(
        plan: LoweredReplicaPromotionPlan,
        fence: FenceProof,
    ) -> Result<ReplicaPromotionReceipt, ReplicaPromotionDenial> {
        let frontier = plan.candidate.frontier();
        if fence.promoted_epoch().get() <= frontier.authority_epoch() {
            return Err(ReplicaPromotionDenial::FenceEpochNotAdvanced);
        }
        Ok(ReplicaPromotionReceipt {
            plan_fingerprint: plan.fingerprint,
            promoted_peer: plan.candidate.intent.candidate_peer,
            promoted_frontier: frontier,
            acknowledged_data_loss_lsn: plan.candidate.acknowledged_data_loss_lsn,
            fence_identity: fence.fence_identity(),
            promoted_epoch: fence.promoted_epoch(),
            durable_target_identity: plan
                .candidate
                .history
                .observation()
                .durable_media_identity(),
        })
    }
}

fn promotion_fingerprint(candidate: &ReplicaPromotionCandidate) -> [u8; 32] {
    let frontier = candidate.frontier();
    let mut digest = Sha256::new();
    digest.update(b"worth-store-replica-promotion-plan-v1");
    digest.update(candidate.intent.candidate_peer.as_str().as_bytes());
    digest.update(frontier.observed_lsn().to_be_bytes());
    digest.update(frontier.durable_lsn().to_be_bytes());
    digest.update(frontier.client_acknowledged_lsn().to_be_bytes());
    digest.update(frontier.replication_acknowledged_lsn().to_be_bytes());
    digest.update(frontier.authority_epoch().to_be_bytes());
    digest.update(candidate.acknowledged_data_loss_lsn.to_be_bytes());
    digest.update(candidate.history.observation().durable_media_identity());
    digest.finalize().into()
}
