use super::admission::ForgeQueryIntentAdmissionDenial;
use super::*;
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::intent_admission::ForgeQueryIntentDecisionTraceEnvelope;
use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use crate::runtime::ForgeQueryIntentConsumerInspection;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentDenialEvidence {
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
    execution_provenance: Option<ForgeQueryIntentExecutionProvenance>,
    decision_trace_envelope: Option<ForgeQueryIntentDecisionTraceEnvelope>,
    denial_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryIntentDenialEvidence {
    pub(in crate::runtime) fn new(
        declaration: &ForgeQueryIntentDeclaration,
        denial: &ForgeQueryIntentAdmissionDenial,
        execution: Option<&ForgeQueryIntentExecution>,
    ) -> Self {
        Self::new_with_trace(declaration, denial, execution, None, None)
    }

    pub(in crate::runtime) fn new_with_trace(
        declaration: &ForgeQueryIntentDeclaration,
        denial: &ForgeQueryIntentAdmissionDenial,
        execution: Option<&ForgeQueryIntentExecution>,
        execution_provenance: Option<ForgeQueryIntentExecutionProvenance>,
        decision_trace_envelope: Option<ForgeQueryIntentDecisionTraceEnvelope>,
    ) -> Self {
        let execution_kind = execution.map(ForgeQueryIntentExecution::execution_kind);
        let returned_strategy_identity =
            execution.map(|execution| execution.strategy_identity.clone());
        let returned_strategy_version =
            execution.map(|execution| execution.strategy_version.clone());
        let returned_strategy_descriptor_digest =
            execution.map(|execution| execution.strategy_descriptor_digest.clone());
        let attempt_digest = execution.map(|execution| execution.outcome_digest.clone());
        let invariant_evidence = execution
            .map(|execution| execution.invariant_evidence.clone())
            .unwrap_or_default();
        let snapshot_identity =
            execution.map(|execution| execution.mutation_receipt().snapshot_identity.clone());
        let snapshot_evidence_identity = snapshot_identity
            .as_ref()
            .map(ForgeQuerySnapshotIdentity::evidence_identity);
        let denial_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::IntentDenialEvidence)
                .field_shape(
                    ForgeQueryEvidenceTag::new("intent_name"),
                    declaration.name(),
                )
                .field_shape(ForgeQueryEvidenceTag::new("stage"), denial.stage())
                .field_value(ForgeQueryEvidenceTag::new("message"), denial.message())
                .field_shape(
                    ForgeQueryEvidenceTag::new("strategy_identity"),
                    declaration.strategy_name(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("strategy_version"),
                    declaration.strategy_version(),
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("returned_strategy_identity"),
                    returned_strategy_identity.as_deref(),
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("returned_strategy_version"),
                    returned_strategy_version.as_deref(),
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("returned_strategy_descriptor_digest"),
                    returned_strategy_descriptor_digest.as_deref(),
                )
                .field_identity(
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
                .optional_shape(
                    ForgeQueryEvidenceTag::new("execution_kind"),
                    execution_kind.map(ForgeQueryIntentExecutionKind::as_str),
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("attempt_digest"),
                    attempt_digest.as_deref(),
                )
                .field_identity_sequence(
                    ForgeQueryEvidenceTag::new("invariant_evidence"),
                    invariant_evidence.iter().map(String::as_str),
                )
                .optional_evidence_identity(
                    ForgeQueryEvidenceTag::new("snapshot_identity"),
                    snapshot_evidence_identity.as_ref(),
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("execution_provenance"),
                    execution_provenance.as_ref().map(
                        ForgeQueryIntentExecutionProvenance::execution_provenance_chain_digest,
                    ),
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("decision_trace_digest"),
                    decision_trace_envelope
                        .as_ref()
                        .map(ForgeQueryIntentDecisionTraceEnvelope::trace_digest),
                )
                .seal();
        Self {
            intent_name: declaration.name().to_string(),
            stage: denial.stage(),
            message: denial.message().to_string(),
            strategy_identity: declaration.strategy_name().to_string(),
            strategy_version: declaration.strategy_version().to_string(),
            returned_strategy_identity,
            returned_strategy_version,
            returned_strategy_descriptor_digest,
            canonical_input_digest: declaration.input_digest().to_string(),
            source_lane: declaration.source_lane(),
            target_lane: declaration.target_lane(),
            execution_kind,
            attempt_digest,
            invariant_evidence,
            snapshot_identity,
            execution_provenance,
            decision_trace_envelope,
            denial_digest,
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

    pub fn execution_provenance(&self) -> Option<&ForgeQueryIntentExecutionProvenance> {
        self.execution_provenance.as_ref()
    }

    pub fn decision_trace_envelope(&self) -> Option<&ForgeQueryIntentDecisionTraceEnvelope> {
        self.decision_trace_envelope.as_ref()
    }

    pub fn consumer_inspection(&self) -> ForgeQueryIntentConsumerInspection<'_> {
        ForgeQueryIntentConsumerInspection::from_denial(self)
    }

    pub fn denial_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.denial_digest
    }
}
