use super::*;

#[test]
fn runtime_floor_certification_proof_shape_audit_freezes_shared_phase_progressions() {
    let bundle = certify_intent_admission_runtime_floor();
    let audit = bundle.proof_shape_audit();

    assert_eq!(
        audit.admitted_phase_progression(),
        &[
            ForgeQueryIntentDecisionTraceStage::RawIntent,
            ForgeQueryIntentDecisionTraceStage::Eligibility,
            ForgeQueryIntentDecisionTraceStage::AdmittedDecision,
            ForgeQueryIntentDecisionTraceStage::ExecutionHandoff,
            ForgeQueryIntentDecisionTraceStage::ExecutionOutcome,
        ]
    );
    assert_eq!(
        audit.advisory_phase_progression(),
        &[
            ForgeQueryIntentDecisionTraceStage::RawIntent,
            ForgeQueryIntentDecisionTraceStage::Eligibility,
            ForgeQueryIntentDecisionTraceStage::AdvisoryStop,
        ]
    );
    assert_eq!(
        audit.violation_phase_progression(),
        &[
            ForgeQueryIntentDecisionTraceStage::RawIntent,
            ForgeQueryIntentDecisionTraceStage::Eligibility,
            ForgeQueryIntentDecisionTraceStage::ViolationStop,
        ]
    );
}
