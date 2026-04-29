use super::admission::ForgeQueryIntentAdmissionDenial;
use super::*;

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
    snapshot_token: Option<String>,
    denial_digest: String,
}

impl ForgeQueryIntentDenialEvidence {
    pub(in crate::runtime) fn new(
        declaration: &ForgeQueryIntentDeclaration,
        denial: &ForgeQueryIntentAdmissionDenial,
        execution: Option<&ForgeQueryIntentExecution>,
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
        let snapshot_token = execution.and_then(|execution| {
            let token = execution.mutation_receipt().snapshot_token.clone();
            (!token.is_empty()).then_some(token)
        });
        let invariant_evidence_digest_part = invariant_evidence.join("|");
        let denial_digest = hash_parts(&[
            "forge_query_intent_denial_evidence_v1".to_string(),
            format!("intent:{}", declaration.name()),
            format!("stage:{}", denial.stage()),
            format!("message:{}", denial.message()),
            format!("strategy:{}", declaration.strategy_name()),
            format!("version:{}", declaration.strategy_version()),
            format!(
                "returned-strategy:{}",
                returned_strategy_identity
                    .as_deref()
                    .unwrap_or("not-executed")
            ),
            format!(
                "returned-version:{}",
                returned_strategy_version
                    .as_deref()
                    .unwrap_or("not-executed")
            ),
            format!(
                "returned-descriptor:{}",
                returned_strategy_descriptor_digest
                    .as_deref()
                    .unwrap_or("not-executed")
            ),
            format!("input:{}", declaration.input_digest()),
            format!("source:{}", declaration.source_lane().as_str()),
            format!("target:{}", declaration.target_lane()),
            format!(
                "execution-kind:{}",
                execution_kind
                    .map(ForgeQueryIntentExecutionKind::as_str)
                    .unwrap_or("not-executed")
            ),
            format!("attempt:{}", attempt_digest.as_deref().unwrap_or("none")),
            format!("invariants:{invariant_evidence_digest_part}"),
            format!("snapshot:{}", snapshot_token.as_deref().unwrap_or("none")),
        ]);
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
            snapshot_token,
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

    pub fn snapshot_token(&self) -> Option<&str> {
        self.snapshot_token.as_deref()
    }

    pub fn denial_digest(&self) -> &str {
        &self.denial_digest
    }
}
