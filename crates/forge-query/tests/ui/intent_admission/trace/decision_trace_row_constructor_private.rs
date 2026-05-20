use forge_query::facade::{
    ForgeQueryIntentDecisionTraceEvidence, ForgeQueryIntentDecisionTraceEvidenceOwner,
    ForgeQueryIntentDecisionTraceRow, ForgeQueryIntentDecisionTraceStage,
};

fn main() {
    let _forged = ForgeQueryIntentDecisionTraceRow {
        stage: ForgeQueryIntentDecisionTraceStage::RawIntent,
        cause: "raw_intent_authored",
        detail: String::new(),
        evidence_owner: ForgeQueryIntentDecisionTraceEvidenceOwner::QueryIntentAuthoring,
        evidence: ForgeQueryIntentDecisionTraceEvidence::Request {
            request_digest: String::new(),
        },
        artifact_digest: String::new(),
        row_digest: String::new(),
    };
}
