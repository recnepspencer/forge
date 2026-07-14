use worth_query::facade::runtime::WorthQueryIntentDecisionTraceStage;
use worth_query::facade::certification::WorthQueryIntentAdmissionProofShapeAudit;

fn main() {
    let _ = WorthQueryIntentAdmissionProofShapeAudit {
        admitted_phase_progression: vec![WorthQueryIntentDecisionTraceStage::RawIntent],
        advisory_phase_progression: vec![WorthQueryIntentDecisionTraceStage::RawIntent],
        violation_phase_progression: vec![WorthQueryIntentDecisionTraceStage::RawIntent],
        decision_phase_progression_digest: String::new(),
        decision_proof_shape_digest: String::new(),
    };
}
