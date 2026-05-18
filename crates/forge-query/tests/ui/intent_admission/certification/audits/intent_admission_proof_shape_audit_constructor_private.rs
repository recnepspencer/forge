use forge_query::facade::{
    ForgeQueryIntentAdmissionProofShapeAudit, ForgeQueryIntentDecisionTraceStage,
};

fn main() {
    let _ = ForgeQueryIntentAdmissionProofShapeAudit {
        admitted_phase_progression: vec![ForgeQueryIntentDecisionTraceStage::RawIntent],
        advisory_phase_progression: vec![ForgeQueryIntentDecisionTraceStage::RawIntent],
        violation_phase_progression: vec![ForgeQueryIntentDecisionTraceStage::RawIntent],
        decision_phase_progression_digest: String::new(),
        decision_proof_shape_digest: String::new(),
    };
}
