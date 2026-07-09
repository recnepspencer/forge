use crate::identity::hash_parts;

use super::{
    WorthQueryIntentDecisionTraceEvidence, WorthQueryIntentDecisionTraceEvidenceOwner,
    WorthQueryIntentDecisionTraceStage,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentDecisionTraceRow {
    stage: WorthQueryIntentDecisionTraceStage,
    cause: &'static str,
    detail: String,
    evidence_owner: WorthQueryIntentDecisionTraceEvidenceOwner,
    evidence: WorthQueryIntentDecisionTraceEvidence,
    artifact_digest: String,
    row_digest: String,
}

impl WorthQueryIntentDecisionTraceRow {
    pub(crate) fn new(
        stage: WorthQueryIntentDecisionTraceStage,
        cause: &'static str,
        detail: impl Into<String>,
        evidence: WorthQueryIntentDecisionTraceEvidence,
    ) -> Self {
        let detail = detail.into();
        let evidence_owner = evidence.owner();
        let artifact_digest = evidence.primary_digest().to_string();
        let row_digest = hash_parts(&[
            "worth_query_intent_decision_trace_row_v2".to_string(),
            format!("stage:{}", stage.as_str()),
            format!("cause:{cause}"),
            format!("detail:{detail}"),
            format!("evidence-owner:{}", evidence_owner.as_str()),
            format!("artifact:{artifact_digest}"),
        ]);
        Self {
            stage,
            cause,
            detail,
            evidence_owner,
            evidence,
            artifact_digest,
            row_digest,
        }
    }

    pub fn stage(&self) -> WorthQueryIntentDecisionTraceStage {
        self.stage
    }

    pub fn cause(&self) -> &'static str {
        self.cause
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn evidence_owner(&self) -> WorthQueryIntentDecisionTraceEvidenceOwner {
        self.evidence_owner
    }

    pub fn evidence(&self) -> &WorthQueryIntentDecisionTraceEvidence {
        &self.evidence
    }

    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
