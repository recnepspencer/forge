use crate::boundary::types::SignalRuntime;

#[test]
fn diagnostics_boundary_exposes_phase7_test_requirements_certification() {
    let runtime = SignalRuntime::new().unwrap();

    let package = runtime
        .diagnostics()
        .worker_phase7_test_requirements_for_test()
        .unwrap();

    assert_eq!(
        package.certification_family,
        "workerPhase7TestRequirementsCertification"
    );
    assert_eq!(package.covered_proof_family_count, 13);
    assert_eq!(package.final_closeout_pending_count, 0);
    assert!(package
        .proof_families
        .iter()
        .all(|row| row.readiness == "ClosedByCanonicalCertification"));
    assert!(package.proof_families.iter().any(|row| {
        row.proof_family == "The Diagnostics Summary Cost Honesty Test"
            && row.certification_surface
                == "SignalWorkerRuntime.certifyWorkerDiagnosticsSummaryRead"
    }));
}
