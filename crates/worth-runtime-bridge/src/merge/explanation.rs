use std::sync::Arc;

use sha2::Digest;

use super::{
    BridgeMergeCounters, BridgeMergeDenialClass, BridgeMergePrecedenceStage,
    BridgeMergeRoutingOutcomeClass, LoweredMergeHistoryPacketSet, MergeDecisionLogEntry,
    PublishedMergeContinuityArtifact, PublishedMergeRemapArtifact, ReducedMergeRoutingArtifact,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedMergeExplanationArtifact {
    contract_identity: Arc<str>,
    lowered_digest: Arc<str>,
    reduced_digest: Arc<str>,
    continuity_digest: Option<Arc<str>>,
    remap_digest: Option<Arc<str>>,
    outcome_class: BridgeMergeRoutingOutcomeClass,
    blocked_stage: Option<BridgeMergePrecedenceStage>,
    denial_class: Option<BridgeMergeDenialClass>,
    decision_log: Arc<[MergeDecisionLogEntry]>,
    counters: BridgeMergeCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl PublishedMergeExplanationArtifact {
    pub(crate) fn from_merge_result(
        lowered_packet_set: &LoweredMergeHistoryPacketSet,
        reduced_routing_artifact: &ReducedMergeRoutingArtifact,
        continuity_artifact: Option<&PublishedMergeContinuityArtifact>,
        remap_artifact: Option<&PublishedMergeRemapArtifact>,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "published-merge-explanation-artifact|contract={}|lowered={}|reduced={}|continuity={}|remap={}|outcome:{:?}|blocked-stage={:?}|denial={:?}|decisions={}",
            lowered_packet_set.contract().contract_identity().as_str(),
            lowered_packet_set.digest(),
            reduced_routing_artifact.digest(),
            continuity_artifact.map(PublishedMergeContinuityArtifact::digest).unwrap_or("none"),
            remap_artifact.map(PublishedMergeRemapArtifact::digest).unwrap_or("none"),
            reduced_routing_artifact.outcome_class(),
            lowered_packet_set.blocked_stage(),
            lowered_packet_set.denial_class(),
            lowered_packet_set
                .decision_log()
                .iter()
                .map(|entry| format!("{:?}:{:?}:{}", entry.stage(), entry.decision_class(), entry.detail()))
                .collect::<Vec<_>>()
                .join("|"),
        ));
        let digest = sha2::Sha256::digest(canonical_basis.as_bytes());

        Self {
            contract_identity: Arc::from(
                lowered_packet_set.contract().contract_identity().as_str(),
            ),
            lowered_digest: Arc::from(lowered_packet_set.digest()),
            reduced_digest: Arc::from(reduced_routing_artifact.digest()),
            continuity_digest: continuity_artifact.map(|artifact| Arc::from(artifact.digest())),
            remap_digest: remap_artifact.map(|artifact| Arc::from(artifact.digest())),
            outcome_class: reduced_routing_artifact.outcome_class(),
            blocked_stage: lowered_packet_set.blocked_stage(),
            denial_class: lowered_packet_set.denial_class(),
            decision_log: Arc::from(lowered_packet_set.decision_log().to_vec()),
            counters: *reduced_routing_artifact.counters(),
            canonical_basis,
            digest: Arc::from(format!(
                "published-merge-explanation-artifact:sha256:{digest:x}"
            )),
        }
    }

    pub fn contract_identity(&self) -> &str {
        self.contract_identity.as_ref()
    }

    pub fn lowered_digest(&self) -> &str {
        self.lowered_digest.as_ref()
    }

    pub fn reduced_digest(&self) -> &str {
        self.reduced_digest.as_ref()
    }

    pub fn continuity_digest(&self) -> Option<&str> {
        self.continuity_digest.as_deref()
    }

    pub fn remap_digest(&self) -> Option<&str> {
        self.remap_digest.as_deref()
    }

    pub fn outcome_class(&self) -> BridgeMergeRoutingOutcomeClass {
        self.outcome_class
    }

    pub fn blocked_stage(&self) -> Option<BridgeMergePrecedenceStage> {
        self.blocked_stage
    }

    pub fn denial_class(&self) -> Option<BridgeMergeDenialClass> {
        self.denial_class
    }

    pub fn decision_log(&self) -> &[MergeDecisionLogEntry] {
        &self.decision_log
    }

    pub fn counters(&self) -> &BridgeMergeCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }

    pub(crate) fn with_counters(mut self, counters: BridgeMergeCounters) -> Self {
        self.counters = counters;
        self
    }
}
