use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::error::{BridgeErrorContext, BridgeReplayError, BridgeReplayErrorKind};
use crate::identity::{BridgeIdentity, MergeRecordIdentityTag};
use crate::merge::{
    AdmittedMergeHistoryContract, BridgeMergeCounters, BridgeMergeDenialClass,
    BridgeMergePrecedenceStage, BridgeMergeRoutingOutcomeClass, LoweredMergeHistoryPacketSet,
    MergeDecisionLogEntry, MergeReplayCertificationBundle, PublishedMergeExplanationArtifact,
    ReducedMergeRoutingArtifact,
};

pub const BRIDGE_CANONICAL_MERGE_RECORD_SCHEMA_V1: &str = "worth-runtime-bridge.merge-record.v1";

pub type BridgeMergeRecordIdentity = BridgeIdentity<MergeRecordIdentityTag>;
pub type BridgeMergeReplaySummary = MergeReplayCertificationBundle;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeMergeRecord {
    record_identity: BridgeMergeRecordIdentity,
    contract: AdmittedMergeHistoryContract,
    bundle: MergeReplayCertificationBundle,
    counters: BridgeMergeCounters,
    canonical_basis: Arc<str>,
}

impl BridgeMergeRecord {
    pub(crate) fn new(bundle: MergeReplayCertificationBundle) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "merge-record|contract={}|lowered={}|reduced={}|continuity={}|remap={}|explanation={}",
            bundle.contract().digest(),
            bundle.lowered_packet_set().digest(),
            bundle.reduced_routing_artifact().digest(),
            bundle
                .continuity_artifact()
                .map(|artifact| artifact.digest())
                .unwrap_or("none"),
            bundle
                .remap_artifact()
                .map(|artifact| artifact.digest())
                .unwrap_or("none"),
            bundle.explanation_artifact().digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            record_identity: BridgeMergeRecordIdentity::admit_bridge_owned(format!(
                "merge-record:sha256:{digest:x}"
            )),
            contract: bundle.contract().clone(),
            counters: *bundle.reduced_routing_artifact().counters(),
            bundle,
            canonical_basis,
        }
    }

    pub fn record_identity(&self) -> &BridgeMergeRecordIdentity {
        &self.record_identity
    }

    pub fn contract(&self) -> &AdmittedMergeHistoryContract {
        &self.contract
    }

    pub fn bundle(&self) -> &MergeReplayCertificationBundle {
        &self.bundle
    }

    pub fn counters(&self) -> &BridgeMergeCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeCanonicalMergeRecord {
    schema_version: Arc<str>,
    record: BridgeMergeRecord,
}

impl BridgeCanonicalMergeRecord {
    pub(crate) fn new(record: BridgeMergeRecord) -> Self {
        Self {
            schema_version: Arc::from(BRIDGE_CANONICAL_MERGE_RECORD_SCHEMA_V1),
            record,
        }
    }

    pub fn schema_version(&self) -> &str {
        self.schema_version.as_ref()
    }

    pub fn record_identity(&self) -> &BridgeMergeRecordIdentity {
        self.record.record_identity()
    }

    pub fn contract(&self) -> &AdmittedMergeHistoryContract {
        self.record.contract()
    }

    pub fn bundle(&self) -> &MergeReplayCertificationBundle {
        self.record.bundle()
    }

    pub fn counters(&self) -> &BridgeMergeCounters {
        self.record.counters()
    }

    pub(crate) fn decode(&self) -> Result<BridgeMergeRecord, BridgeReplayError> {
        if self.schema_version() != BRIDGE_CANONICAL_MERGE_RECORD_SCHEMA_V1 {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::CanonicalArtifactCoherenceFailure,
                format!(
                    "Bridge canonical merge record schema `{}` is not supported; expected `{}`.",
                    self.schema_version(),
                    BRIDGE_CANONICAL_MERGE_RECORD_SCHEMA_V1
                ),
            )
            .with_context(BridgeErrorContext::default()));
        }

        Ok(self.record.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeMergeExplanation {
    record_identity: BridgeMergeRecordIdentity,
    contract_identity: Arc<str>,
    lowered_digest: Arc<str>,
    reduced_digest: Arc<str>,
    continuity_digest: Option<Arc<str>>,
    remap_digest: Option<Arc<str>>,
    explanation_digest: Arc<str>,
    outcome_class: BridgeMergeRoutingOutcomeClass,
    blocked_stage: Option<BridgeMergePrecedenceStage>,
    denial_class: Option<BridgeMergeDenialClass>,
    decision_log: Vec<MergeDecisionLogEntry>,
    counters: BridgeMergeCounters,
}

impl BridgeMergeExplanation {
    pub fn from_canonical_record(record: &BridgeCanonicalMergeRecord) -> Self {
        let explanation: &PublishedMergeExplanationArtifact =
            record.bundle().explanation_artifact();
        let lowered: &LoweredMergeHistoryPacketSet = record.bundle().lowered_packet_set();
        let reduced: &ReducedMergeRoutingArtifact = record.bundle().reduced_routing_artifact();

        Self {
            record_identity: record.record_identity().clone(),
            contract_identity: Arc::from(record.contract().contract_identity().as_str()),
            lowered_digest: Arc::from(lowered.digest()),
            reduced_digest: Arc::from(reduced.digest()),
            continuity_digest: record
                .bundle()
                .continuity_artifact()
                .map(|artifact| Arc::from(artifact.digest())),
            remap_digest: record
                .bundle()
                .remap_artifact()
                .map(|artifact| Arc::from(artifact.digest())),
            explanation_digest: Arc::from(explanation.digest()),
            outcome_class: explanation.outcome_class(),
            blocked_stage: explanation.blocked_stage(),
            denial_class: explanation.denial_class(),
            decision_log: explanation.decision_log().to_vec(),
            counters: *record.counters(),
        }
    }

    pub fn record_identity(&self) -> &BridgeMergeRecordIdentity {
        &self.record_identity
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

    pub fn explanation_digest(&self) -> &str {
        self.explanation_digest.as_ref()
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
}
