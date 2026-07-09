use crate::runtime::tests::support::*;
use crate::runtime::worker_host::{
    certify_worker_phase7_test_requirements, required_acceptance_artifacts,
    required_proof_family_requirements, WorkerPhase7TestRequirementsCertificationPackage,
};

#[test]
fn worker_phase7_test_requirements_certify_required_proof_family_tracking() {
    let package = certify_worker_phase7_test_requirements().unwrap();

    assert_eq!(
        package.certification_family,
        "workerPhase7TestRequirementsCertification"
    );
    assert_eq!(package.test_requirements_status, "FinalCloseoutCertified");
    assert_eq!(package.required_proof_family_count, 13);
    assert_eq!(package.covered_proof_family_count, 13);
    assert_eq!(package.final_closeout_pending_count, 0);
    assert!(package
        .proof_families
        .iter()
        .all(|row| row.readiness == "ClosedByCanonicalCertification"));
    assert!(package.proof_families.iter().any(|row| {
        row.proof_family == "The Worker Bridge Boundedness Test"
            && row.certification_surface == "SignalDiagnostics.workerPhase7PerformanceContracts"
            && row.runtime_test_surface
                == "runtime/tests/worker_runtime/phase7/performance_contracts.rs"
            && row.boundary_test_surface == "boundary/tests/phase7/performance_contracts.rs"
    }));
    assert!(package
        .acceptance_artifacts
        .contains(&"bridgeAllocationPosture"));
    assert_digest_shape(&package.proof_family_digest);
    assert_digest_shape(&package.acceptance_artifact_digest);
    assert_digest_shape(&package.certification_digest);
}

#[test]
fn worker_phase7_test_requirements_reject_missing_required_proof_family() {
    let mut proof_families = required_proof_family_requirements();
    proof_families.retain(|row| row.proof_family != "The UI Freeze Surface Denial Test");

    let error = WorkerPhase7TestRequirementsCertificationPackage::from_catalog(
        proof_families,
        required_acceptance_artifacts(),
    )
    .unwrap_err();

    assert!(error.message.contains("UI Freeze Surface Denial"));
}

#[test]
fn worker_phase7_test_requirements_reject_duplicate_proof_family() {
    let mut proof_families = required_proof_family_requirements();
    proof_families.push(proof_families[0].clone());

    let error = WorkerPhase7TestRequirementsCertificationPackage::from_catalog(
        proof_families,
        required_acceptance_artifacts(),
    )
    .unwrap_err();

    assert!(error.message.contains("duplicate proof family"));
}

#[test]
fn worker_phase7_test_requirements_reject_weak_unbacked_row() {
    let mut proof_families = required_proof_family_requirements();
    proof_families[0].certification_surface = "";

    let error = WorkerPhase7TestRequirementsCertificationPackage::from_catalog(
        proof_families,
        required_acceptance_artifacts(),
    )
    .unwrap_err();

    assert!(error.message.contains("closed proof family status"));
}

#[test]
fn worker_phase7_test_requirements_reject_pending_closeout_row() {
    let mut proof_families = required_proof_family_requirements();
    proof_families[0].readiness = "CoveredPendingFinalCloseout";

    let error = WorkerPhase7TestRequirementsCertificationPackage::from_catalog(
        proof_families,
        required_acceptance_artifacts(),
    )
    .unwrap_err();

    assert!(error
        .message
        .contains("concrete test and certification surfaces"));
}

#[test]
fn worker_phase7_test_requirements_reject_missing_acceptance_artifact() {
    let mut artifacts = required_acceptance_artifacts();
    artifacts.retain(|artifact| *artifact != "mainThreadBroadWorkDenialArtifacts");

    let error = WorkerPhase7TestRequirementsCertificationPackage::from_catalog(
        required_proof_family_requirements(),
        artifacts,
    )
    .unwrap_err();

    assert!(error.message.contains("mainThreadBroadWorkDenialArtifacts"));
}
