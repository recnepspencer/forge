use super::*;
use crate::intent_admission::ForgeQueryIntentDecisionTraceEnvelope;
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
    snapshot_token: String,
    execution_provenance: ForgeQueryIntentExecutionProvenance,
    decision_trace_envelope: ForgeQueryIntentDecisionTraceEnvelope,
    failure_digest: String,
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
        let snapshot_token = execution.mutation_receipt().snapshot_token.clone();
        let invariant_evidence = execution.invariant_evidence().to_vec();
        let invariant_evidence_digest_part = invariant_evidence.join("|");
        let failure_digest = hash_parts(&[
            "forge_query_intent_execution_failure_evidence_v1".to_string(),
            format!("intent:{}", declaration.name()),
            format!("stage:{stage}"),
            format!("message:{message}"),
            format!("strategy:{}", declaration.strategy_name()),
            format!("version:{}", declaration.strategy_version()),
            format!("returned-strategy:{}", execution.strategy_identity()),
            format!("returned-version:{}", execution.strategy_version()),
            format!(
                "returned-descriptor:{}",
                execution.strategy_descriptor_digest()
            ),
            format!("input:{}", declaration.input_digest()),
            format!("source:{}", declaration.source_lane().as_str()),
            format!("target:{}", declaration.target_lane()),
            format!("execution-kind:{}", execution.execution_kind().as_str()),
            format!("attempt:{}", execution.outcome_digest()),
            format!("invariants:{invariant_evidence_digest_part}"),
            format!("snapshot:{snapshot_token}"),
            format!(
                "execution-provenance:{}",
                execution_provenance.execution_provenance_chain_digest()
            ),
            format!("decision-trace:{}", decision_trace_envelope.trace_digest()),
        ]);
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
            snapshot_token,
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

    pub fn snapshot_token(&self) -> &str {
        &self.snapshot_token
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
        &self.failure_digest
    }
}
