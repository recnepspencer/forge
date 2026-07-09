use worth_query::facade::{
    WorthQueryIntentDecisionTraceEvidence, WorthQueryIntentDecisionTraceEvidenceOwner,
    WorthQueryIntentDecisionTraceRow, WorthQueryIntentDecisionTraceStage,
};

fn main() {
    let _worthd = WorthQueryIntentDecisionTraceRow {
        stage: WorthQueryIntentDecisionTraceStage::RawIntent,
        cause: "raw_intent_authored",
        detail: String::new(),
        evidence_owner: WorthQueryIntentDecisionTraceEvidenceOwner::QueryIntentAuthoring,
        evidence: WorthQueryIntentDecisionTraceEvidence::Request {
            request_digest: String::new(),
        },
        artifact_digest: String::new(),
        row_digest: String::new(),
    };
}
