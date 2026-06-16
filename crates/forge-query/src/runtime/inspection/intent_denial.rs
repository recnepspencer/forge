use super::super::{
    ForgeQueryAuthorityLane, ForgeQueryIntentDenialEvidence, ForgeQueryIntentExecutionKind,
    ForgeQueryIntentSourceLane,
};
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::memory_workspace::ForgeQuerySnapshotIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentDenialInspection {
    intent_name: String,
    stage: &'static str,
    message: String,
    strategy_identity: String,
    strategy_version: String,
    returned_strategy_identity: Option<String>,
    returned_strategy_version: Option<String>,
    returned_strategy_descriptor_digest: Option<String>,
    canonical_input_digest: String,
    source_lane: ForgeQueryIntentSourceLane,
    target_lane: ForgeQueryAuthorityLane,
    execution_kind: Option<ForgeQueryIntentExecutionKind>,
    attempt_digest: Option<String>,
    invariant_evidence: Vec<String>,
    snapshot_identity: Option<ForgeQuerySnapshotIdentity>,
    denial_digest: ForgeQueryEvidenceIdentity,
    inspection_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryIntentDenialInspection {
    pub(in crate::runtime) fn from_evidence(evidence: &ForgeQueryIntentDenialEvidence) -> Self {
        let returned_strategy_identity = evidence.returned_strategy_identity().map(str::to_string);
        let returned_strategy_version = evidence.returned_strategy_version().map(str::to_string);
        let returned_strategy_descriptor_digest = evidence
            .returned_strategy_descriptor_digest()
            .map(str::to_string);
        let attempt_digest = evidence.attempt_digest().map(str::to_string);
        let invariant_evidence = evidence.invariant_evidence().to_vec();
        let snapshot_identity = evidence.snapshot_identity().cloned();
        let snapshot_evidence_identity = evidence.snapshot_evidence_identity();
        let inspection_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::IntentDenialInspection)
                .field_shape(
                    ForgeQueryEvidenceTag::new("intent_name"),
                    evidence.intent_name(),
                )
                .field_shape(ForgeQueryEvidenceTag::new("stage"), evidence.stage())
                .field_value(ForgeQueryEvidenceTag::new("message"), evidence.message())
                .field_value(
                    ForgeQueryEvidenceTag::new("strategy_identity"),
                    evidence.strategy_identity(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("strategy_version"),
                    evidence.strategy_version(),
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("returned_strategy_identity"),
                    returned_strategy_identity.as_deref(),
                )
                .optional_shape(
                    ForgeQueryEvidenceTag::new("returned_strategy_version"),
                    returned_strategy_version.as_deref(),
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("returned_strategy_descriptor_digest"),
                    returned_strategy_descriptor_digest.as_deref(),
                )
                .field_value(
                    ForgeQueryEvidenceTag::new("canonical_input_digest"),
                    evidence.canonical_input_digest(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("source_lane"),
                    evidence.source_lane().as_str(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("target_lane"),
                    evidence.target_lane().as_str(),
                )
                .optional_shape(
                    ForgeQueryEvidenceTag::new("execution_kind"),
                    evidence
                        .execution_kind()
                        .map(ForgeQueryIntentExecutionKind::as_str),
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("attempt_digest"),
                    attempt_digest.as_deref(),
                )
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("invariant_evidence"),
                    invariant_evidence.iter().map(String::as_str),
                )
                .optional_evidence_identity(
                    ForgeQueryEvidenceTag::new("snapshot_identity"),
                    snapshot_evidence_identity.as_ref(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("denial_digest"),
                    evidence.denial_digest(),
                )
                .seal();
        Self {
            intent_name: evidence.intent_name().to_string(),
            stage: evidence.stage(),
            message: evidence.message().to_string(),
            strategy_identity: evidence.strategy_identity().to_string(),
            strategy_version: evidence.strategy_version().to_string(),
            returned_strategy_identity,
            returned_strategy_version,
            returned_strategy_descriptor_digest,
            canonical_input_digest: evidence.canonical_input_digest().to_string(),
            source_lane: evidence.source_lane(),
            target_lane: evidence.target_lane(),
            execution_kind: evidence.execution_kind(),
            attempt_digest,
            invariant_evidence,
            snapshot_identity,
            denial_digest: evidence.denial_digest().clone(),
            inspection_digest,
        }
    }

    pub fn intent_name(&self) -> &str {
        &self.intent_name
    }

    pub fn stage(&self) -> &'static str {
        self.stage
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn strategy_identity(&self) -> &str {
        &self.strategy_identity
    }

    pub fn strategy_version(&self) -> &str {
        &self.strategy_version
    }

    pub fn returned_strategy_identity(&self) -> Option<&str> {
        self.returned_strategy_identity.as_deref()
    }

    pub fn returned_strategy_version(&self) -> Option<&str> {
        self.returned_strategy_version.as_deref()
    }

    pub fn returned_strategy_descriptor_digest(&self) -> Option<&str> {
        self.returned_strategy_descriptor_digest.as_deref()
    }

    pub fn canonical_input_digest(&self) -> &str {
        &self.canonical_input_digest
    }

    pub fn source_lane(&self) -> ForgeQueryIntentSourceLane {
        self.source_lane
    }

    pub fn target_lane(&self) -> ForgeQueryAuthorityLane {
        self.target_lane
    }

    pub fn execution_kind(&self) -> Option<ForgeQueryIntentExecutionKind> {
        self.execution_kind
    }

    pub fn attempt_digest(&self) -> Option<&str> {
        self.attempt_digest.as_deref()
    }

    pub fn invariant_evidence(&self) -> &[String] {
        &self.invariant_evidence
    }

    pub fn snapshot_identity(&self) -> Option<&ForgeQuerySnapshotIdentity> {
        self.snapshot_identity.as_ref()
    }

    pub fn snapshot_evidence_identity(&self) -> Option<ForgeQueryEvidenceIdentity> {
        self.snapshot_identity
            .as_ref()
            .map(ForgeQuerySnapshotIdentity::evidence_identity)
    }

    pub fn denial_digest(&self) -> &str {
        self.denial_digest.as_str()
    }

    pub fn denial_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.denial_digest
    }

    pub fn inspection_digest(&self) -> &str {
        self.inspection_digest.as_str()
    }

    pub fn inspection_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.inspection_digest
    }
}
