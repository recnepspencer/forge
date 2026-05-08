use crate::boundary::types::SignalRuntime;

#[test]
fn diagnostics_boundary_exposes_phase7_product_guidance_certification() {
    let runtime = SignalRuntime::new().unwrap();

    let package = runtime
        .diagnostics()
        .worker_phase7_product_guidance_for_test()
        .unwrap();

    assert_eq!(
        package.certification_family,
        "workerPhase7ProductGuidanceCertification"
    );
    assert_eq!(
        package.recommended_default_posture,
        "workerFirstRuntimeOwnedGraph"
    );
    assert!(package.compatibility_guidance_rules.iter().any(|rule| {
        rule.posture == "mainThreadHostedCallbackLane"
            && rule.semantic_authority == "workerRuntimeAfterForgeProofReadmission"
    }));
    assert!(!package.hidden_fallback_allowed);
}
