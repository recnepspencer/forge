use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    AdmittedMergeHistoryContract, LoweredMergeHistoryPacketSet, PublishedMergeContinuityArtifact,
    PublishedMergeExplanationArtifact, PublishedMergeRemapArtifact, ReducedMergeRoutingArtifact,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeReplayCertificationBundle {
    contract: AdmittedMergeHistoryContract,
    lowered_packet_set: LoweredMergeHistoryPacketSet,
    reduced_routing_artifact: ReducedMergeRoutingArtifact,
    continuity_artifact: Option<PublishedMergeContinuityArtifact>,
    remap_artifact: Option<PublishedMergeRemapArtifact>,
    explanation_artifact: PublishedMergeExplanationArtifact,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl MergeReplayCertificationBundle {
    pub(crate) fn new(
        contract: AdmittedMergeHistoryContract,
        lowered_packet_set: LoweredMergeHistoryPacketSet,
        reduced_routing_artifact: ReducedMergeRoutingArtifact,
        continuity_artifact: Option<PublishedMergeContinuityArtifact>,
        remap_artifact: Option<PublishedMergeRemapArtifact>,
        explanation_artifact: PublishedMergeExplanationArtifact,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "merge-replay-certification-bundle|contract={}|lowered={}|reduced={}|continuity={}|remap={}|explanation={}",
            contract.digest(),
            lowered_packet_set.digest(),
            reduced_routing_artifact.digest(),
            continuity_artifact
                .as_ref()
                .map(PublishedMergeContinuityArtifact::digest)
                .unwrap_or("none"),
            remap_artifact
                .as_ref()
                .map(PublishedMergeRemapArtifact::digest)
                .unwrap_or("none"),
            explanation_artifact.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            contract,
            lowered_packet_set,
            reduced_routing_artifact,
            continuity_artifact,
            remap_artifact,
            explanation_artifact,
            canonical_basis,
            digest: Arc::from(format!(
                "merge-replay-certification-bundle:sha256:{digest:x}"
            )),
        }
    }

    pub fn contract(&self) -> &AdmittedMergeHistoryContract {
        &self.contract
    }

    pub fn lowered_packet_set(&self) -> &LoweredMergeHistoryPacketSet {
        &self.lowered_packet_set
    }

    pub fn reduced_routing_artifact(&self) -> &ReducedMergeRoutingArtifact {
        &self.reduced_routing_artifact
    }

    pub fn continuity_artifact(&self) -> Option<&PublishedMergeContinuityArtifact> {
        self.continuity_artifact.as_ref()
    }

    pub fn remap_artifact(&self) -> Option<&PublishedMergeRemapArtifact> {
        self.remap_artifact.as_ref()
    }

    pub fn explanation_artifact(&self) -> &PublishedMergeExplanationArtifact {
        &self.explanation_artifact
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }

    pub(crate) fn with_replay_request(mut self) -> Self {
        let reduced_counters = self
            .reduced_routing_artifact
            .counters()
            .with_replay_request();
        let explanation_counters = self.explanation_artifact.counters().with_replay_request();
        self.reduced_routing_artifact = self
            .reduced_routing_artifact
            .clone()
            .with_counters(reduced_counters);
        self.explanation_artifact = self
            .explanation_artifact
            .clone()
            .with_counters(explanation_counters);
        self
    }

    pub(crate) fn with_replay_mismatch(mut self) -> Self {
        let reduced_counters = self
            .reduced_routing_artifact
            .counters()
            .with_replay_mismatch();
        let explanation_counters = self.explanation_artifact.counters().with_replay_mismatch();
        self.reduced_routing_artifact = self
            .reduced_routing_artifact
            .clone()
            .with_counters(reduced_counters);
        self.explanation_artifact = self
            .explanation_artifact
            .clone()
            .with_counters(explanation_counters);
        self
    }
}
