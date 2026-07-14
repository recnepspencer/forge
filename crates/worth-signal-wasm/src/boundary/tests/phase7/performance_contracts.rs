use crate::boundary::types::SignalRuntime;

#[test]
fn diagnostics_boundary_exposes_phase7_performance_contract_certification() {
    let runtime = SignalRuntime::new().unwrap();

    let package = runtime
        .diagnostics()
        .worker_phase7_performance_contracts_for_test()
        .unwrap();

    assert_eq!(
        package.certification_family,
        "workerPhase7PerformanceContractCertification"
    );
    assert_eq!(package.covered_counter_count, 27);
    assert_eq!(package.covered_complexity_contract_count, 14);
    assert!(package
        .prohibited_failure_modes
        .iter()
        .any(|mode| mode.mode == "UIFreezeBySerialization"));
    assert_eq!(
        package
            .bridge_allocation_posture
            .serialization_allocation_counter,
        "bridgeSerializationAllocationCount"
    );
    assert!(package.certification_digest.len() == 64);
}
