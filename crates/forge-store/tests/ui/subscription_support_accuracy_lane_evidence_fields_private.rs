use forge_store::{
    SubscriptionSupportAccuracyCertificationRowKind, SubscriptionSupportAccuracyLaneEvidence,
    SubscriptionSupportAccuracyLaneOutcome,
};

fn main() {
    let _evidence = SubscriptionSupportAccuracyLaneEvidence {
        row_kind: SubscriptionSupportAccuracyCertificationRowKind::CompatibilityDriftRejectsExactTrust,
        outcome: SubscriptionSupportAccuracyLaneOutcome::TypedRejection,
        failure_kind: None,
        recovery_posture: None,
        source_digest: String::new(),
        diagnostics_digest: String::new(),
        counter_digest: String::new(),
        evidence_digest: String::new(),
    };
}
