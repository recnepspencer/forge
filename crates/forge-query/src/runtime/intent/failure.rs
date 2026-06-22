use super::*;
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::intent_admission::ForgeQueryIntentDecisionTraceEnvelope;
use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use crate::runtime::ForgeQueryIntentConsumerInspection;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentExecutionFailureEvidence {
    intent_name: String,
    stage: &'static str,
    message: String,
    strategy_identity: String,
    strategy_version: String,
    returned_strategy_identity: String,
    returned_strategy_version: String,
    returned_strategy_descriptor_digest: String,
    canonical_input_digest: String,
    source_lane: ForgeQueryIntentSourceLane,
    target_lane: ForgeQueryAuthorityLane,
    execution_kind: ForgeQueryIntentExecutionKind,
    attempt_digest: String,
    invariant_evidence: Vec<String>,
    snapshot_identity: ForgeQuerySnapshotIdentity,
    execution_provenance: ForgeQueryIntentExecutionProvenance,
    decision_trace_envelope: ForgeQueryIntentDecisionTraceEnvelope,
    failure_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryIntentExecutionFailureEvidence {
    pub(in crate::runtime) fn new(
        declaration: &ForgeQueryIntentDeclaration,
        stage: &'static str,
        message: String,
        execution: &ForgeQueryIntentExecution,
        execution_provenance: ForgeQueryIntentExecutionProvenance,
        decision_trace_envelope: ForgeQueryIntentDecisionTraceEnvelope,
    ) -> Self {
        let snapshot_identity = execution.mutation_receipt().snapshot_identity.clone();
        let snapshot_evidence_identity = snapshot_identity.evidence_identity();
        let invariant_evidence = execution.invariant_evidence().to_vec();
        let failure_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::IntentExecutionFailureEvidence)
                .field_shape(
                    ForgeQueryEvidenceTag::new("intent_name"),
                    declaration.name(),
                )
                .field_shape(ForgeQueryEvidenceTag::new("stage"), stage)
                .field_value(ForgeQueryEvidenceTag::new("message"), &message)
                .field_shape(
                    ForgeQueryEvidenceTag::new("strategy_identity"),
                    declaration.strategy_name(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("strategy_version"),
                    declaration.strategy_version(),
                )
                .field_value(
                    ForgeQueryEvidenceTag::new("returned_strategy_identity"),
                    execution.strategy_identity(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("returned_strategy_version"),
                    execution.strategy_version(),
                )
                .field_value(
                    ForgeQueryEvidenceTag::new("returned_strategy_descriptor_digest"),
                    execution.strategy_descriptor_digest(),
                )
                .field_value(
                    ForgeQueryEvidenceTag::new("canonical_input_digest"),
                    declaration.input_digest(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("source_lane"),
                    declaration.source_lane().as_str(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("target_lane"),
                    declaration.target_lane().as_str(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("execution_kind"),
                    execution.execution_kind().as_str(),
                )
                .field_value(
                    ForgeQueryEvidenceTag::new("attempt_digest"),
                    execution.outcome_digest(),
                )
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("invariant_evidence"),
                    invariant_evidence.iter().map(String::as_str),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("snapshot_identity"),
                    &snapshot_evidence_identity,
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("execution_provenance"),
                    execution_provenance.execution_provenance_chain_identity(),
                )
                .field_value(
                    ForgeQueryEvidenceTag::new("decision_trace_digest"),
                    decision_trace_envelope.trace_digest(),
                )
                .seal();
        Self {
            intent_name: declaration.name().to_string(),
            stage,
            message,
            strategy_identity: declaration.strategy_name().to_string(),
            strategy_version: declaration.strategy_version().to_string(),
            returned_strategy_identity: execution.strategy_identity().to_string(),
            returned_strategy_version: execution.strategy_version().to_string(),
            returned_strategy_descriptor_digest: execution.strategy_descriptor_digest().to_string(),
            canonical_input_digest: declaration.input_digest().to_string(),
            source_lane: declaration.source_lane(),
            target_lane: declaration.target_lane(),
            execution_kind: execution.execution_kind(),
            attempt_digest: execution.outcome_digest().to_string(),
            invariant_evidence,
            snapshot_identity,
            execution_provenance,
            decision_trace_envelope,
            failure_digest,
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

    pub fn returned_strategy_identity(&self) -> &str {
        &self.returned_strategy_identity
    }

    pub fn returned_strategy_version(&self) -> &str {
        &self.returned_strategy_version
    }

    pub fn returned_strategy_descriptor_digest(&self) -> &str {
        &self.returned_strategy_descriptor_digest
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

    pub fn execution_kind(&self) -> ForgeQueryIntentExecutionKind {
        self.execution_kind
    }

    pub fn attempt_digest(&self) -> &str {
        &self.attempt_digest
    }

    pub fn invariant_evidence(&self) -> &[String] {
        &self.invariant_evidence
    }

    pub fn snapshot_identity(&self) -> &ForgeQuerySnapshotIdentity {
        &self.snapshot_identity
    }

    pub fn snapshot_evidence_identity(&self) -> ForgeQueryEvidenceIdentity {
        self.snapshot_identity.evidence_identity()
    }

    pub fn execution_provenance(&self) -> &ForgeQueryIntentExecutionProvenance {
        &self.execution_provenance
    }

    pub fn decision_trace_envelope(&self) -> &ForgeQueryIntentDecisionTraceEnvelope {
        &self.decision_trace_envelope
    }

    pub fn consumer_inspection(&self) -> ForgeQueryIntentConsumerInspection<'_> {
        ForgeQueryIntentConsumerInspection::from_failure(self)
    }

    pub fn failure_digest(&self) -> &str {
        self.failure_digest.as_str()
    }

    pub fn failure_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.failure_digest
    }
}
