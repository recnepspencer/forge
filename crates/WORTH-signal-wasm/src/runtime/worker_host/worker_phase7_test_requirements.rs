use serde::Serialize;

use crate::boundary::errors::WORTHSignalJsError;

use super::canonical_worker_certification_digest;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerPhase7TestRequirementsCertificationPackage {
    pub certification_family: &'static str,
    pub test_requirements_status: &'static str,
    pub required_proof_family_count: u64,
    pub covered_proof_family_count: u64,
    pub final_closeout_pending_count: u64,
    pub proof_families: Vec<WorkerPhase7ProofFamilyRequirement>,
    pub acceptance_artifacts: Vec<&'static str>,
    pub proof_family_digest: String,
    pub acceptance_artifact_digest: String,
    pub certification_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerPhase7ProofFamilyRequirement {
    pub proof_family: &'static str,
    pub readiness: &'static str,
    pub runtime_test_surface: &'static str,
    pub boundary_test_surface: &'static str,
    pub certification_surface: &'static str,
    pub hostile_requirement: &'static str,
}

pub fn certify_worker_phase7_test_requirements(
) -> Result<WorkerPhase7TestRequirementsCertificationPackage, WORTHSignalJsError> {
    WorkerPhase7TestRequirementsCertificationPackage::from_catalog(
        required_proof_family_requirements(),
        required_acceptance_artifacts(),
    )
}

impl WorkerPhase7TestRequirementsCertificationPackage {
    pub(crate) fn from_catalog(
        proof_families: Vec<WorkerPhase7ProofFamilyRequirement>,
        acceptance_artifacts: Vec<&'static str>,
    ) -> Result<Self, WORTHSignalJsError> {
        reject_missing_proof_families(proof_families.as_slice())?;
        reject_duplicate_proof_families(proof_families.as_slice())?;
        reject_weak_proof_family_rows(proof_families.as_slice())?;
        reject_missing_acceptance_artifacts(acceptance_artifacts.as_slice())?;
        reject_duplicate_acceptance_artifacts(acceptance_artifacts.as_slice())?;

        let proof_family_digest = canonical_worker_certification_digest(&(
            "workerPhase7RequiredProofFamilies",
            &proof_families,
        ))?;
        let acceptance_artifact_digest = canonical_worker_certification_digest(&(
            "workerPhase7AcceptanceArtifacts",
            &acceptance_artifacts,
        ))?;
        let final_closeout_pending_count = proof_families
            .iter()
            .filter(|proof_family| proof_family.readiness != "ClosedByCanonicalCertification")
            .count() as u64;
        let certification_digest = canonical_worker_certification_digest(&(
            "workerPhase7TestRequirementsCertification",
            proof_family_digest.as_str(),
            acceptance_artifact_digest.as_str(),
            final_closeout_pending_count,
        ))?;

        Ok(Self {
            certification_family: "workerPhase7TestRequirementsCertification",
            test_requirements_status: "FinalCloseoutCertified",
            required_proof_family_count: required_proof_family_names().len() as u64,
            covered_proof_family_count: proof_families.len() as u64,
            final_closeout_pending_count,
            proof_families,
            acceptance_artifacts,
            proof_family_digest,
            acceptance_artifact_digest,
            certification_digest,
        })
    }
}

pub(crate) fn required_proof_family_requirements() -> Vec<WorkerPhase7ProofFamilyRequirement> {
    vec![
        proof_family(
            "The Worker Compatibility Truth Equivalence Test",
            "runtime/tests/worker_runtime/compatibility/worker_compatibility_certification.rs",
            "boundary/tests/worker_runtime_bootstrap.rs",
            "SignalDiagnostics.workerRuntimeCompatibilityCertification",
            "worker-first and compatibility truth must converge under hostile branch, observation, diagnostics, async, and isolation probes",
        ),
        proof_family(
            "The Mixed Placement Graph Isolation Test",
            "runtime/tests/worker_runtime/compatibility/worker_compatibility_certification.rs",
            "runtime/tests/worker_runtime/placement/worker_placement.rs",
            "WorkerRuntimeNonHostIsolationReport",
            "main-thread-hosted or unavailable work must not collapse unrelated worker-owned regions",
        ),
        proof_family(
            "The Host Capability Worker Bridge Parity Test",
            "boundary/tests/worker_host_boundary/host_capability_ingress.rs",
            "boundary/tests/worker_host_boundary/main_thread_host_bridge_certification.rs",
            "SignalWorkerRuntime.certifyMainThreadHostBridge",
            "host capability ingress must be typed, coalesced, ambient-read denied, and parity-bound",
        ),
        proof_family(
            "The Browser History Worker Admission Parity Test",
            "boundary/tests/worker_host_boundary/browser_history_ingress.rs",
            "boundary/tests/worker_host_boundary/main_thread_host_bridge_certification.rs",
            "SignalWorkerRuntime.admitBrowserHistoryIngress",
            "browser facts must enter as typed route continuity, not ambient worker reads",
        ),
        proof_family(
            "The Main-Thread Host Effect Boundary Test",
            "boundary/tests/worker_host_boundary/host_effect_boundary.rs",
            "boundary/tests/worker_host_boundary/main_thread_host_bridge_certification.rs",
            "SignalWorkerRuntime.issueHostEffectRequest/admitHostEffectAcknowledgement",
            "host effects must remain non-authoritative until worth-proof readmission",
        ),
        proof_family(
            "The Callback Placement Eligibility And Denial Test",
            "runtime/tests/worker_runtime/placement/worker_placement.rs",
            "runtime/tests/worker_runtime/placement/main_thread_hosted_callback_execution.rs",
            "SignalDiagnostics.workerCallbackPlacementEligibility",
            "callback placement must distinguish executable, hosted, denied, and unavailable paths",
        ),
        proof_family(
            "The Worker Ineligible Node Does Not Collapse Graph Breadth Test",
            "runtime/tests/worker_runtime/compatibility/worker_compatibility_certification.rs",
            "runtime/tests/worker_runtime/placement/worker_placement.rs",
            "WorkerRuntimeNonHostIsolationReport",
            "one ineligible node must produce a broad-work denial artifact instead of pinning unrelated breadth",
        ),
        proof_family(
            "The Observation And Output Delivery Boundary Test",
            "runtime/tests/worker_runtime/observation_delivery",
            "boundary/tests/worker_runtime_bootstrap.rs",
            "SignalWorkerRuntime.certifyWorkerPhase5Closeout",
            "committed public delivery must be packetized by delivery breadth and remain rollback-safe",
        ),
        proof_family(
            "The Diagnostics Summary Cost Honesty Test",
            "runtime/tests/worker_runtime/diagnostics_history_read.rs",
            "boundary/worker_diagnostics_history_read.rs",
            "SignalWorkerRuntime.certifyWorkerDiagnosticsSummaryRead",
            "summary reads must stay summary-only with zero rich reconstruction",
        ),
        proof_family(
            "The Worker Replay Restore Capability Honesty Test",
            "runtime/tests/worker_runtime/replay_restore_capability.rs",
            "runtime/tests/worker_runtime/replay_checkpoint_retained_history.rs",
            "SignalWorkerRuntime.certifyWorkerPhase6Closeout",
            "history must preserve capability posture, exact restore artifacts, and retained replay evidence",
        ),
        proof_family(
            "The Import Export Callback Unavailability Test",
            "runtime/tests/worker_runtime/import_export_callback_unavailability.rs",
            "boundary/tests/worker_phase6_closeout.rs",
            "SignalWorkerRuntime.certifyWorkerImportExportCallbackUnavailability",
            "callback-bearing exports must deny portable import unless explicit reattachment evidence exists",
        ),
        proof_family(
            "The Worker Bridge Boundedness Test",
            "runtime/tests/worker_runtime/phase7/performance_contracts.rs",
            "boundary/tests/phase7/performance_contracts.rs",
            "SignalDiagnostics.workerPhase7PerformanceContracts",
            "worker bridge cost must be named by counters, cost bases, allocation posture, and failure modes",
        ),
        proof_family(
            "The UI Freeze Surface Denial Test",
            "runtime/tests/worker_runtime/phase7/performance_contracts.rs",
            "runtime/tests/worker_runtime/observation_delivery/worker_output_delivery.rs",
            "SignalDiagnostics.workerPhase7PerformanceContracts",
            "broad public delivery and serialization must be visible as explicit delivery breadth, not hidden UI-thread work",
        ),
    ]
}

pub(crate) fn required_acceptance_artifacts() -> Vec<&'static str> {
    vec![
        "placementClassification",
        "workerRuntimeIdentity",
        "transactionEnvelopes",
        "hostCapabilityUpdateEnvelopes",
        "browserHistoryEventEnvelopes",
        "hostEffectRequestAndAcknowledgementEnvelopes",
        "committedOutputDeliveryPackets",
        "committedObservationDeliveryPackets",
        "diagnosticsHistoryReadEnvelopes",
        "fallbackAndDenialClassifications",
        "capabilityAvailabilityAndReattachmentPosture",
        "replayRestoreImportExportCapabilityArtifacts",
        "compatibilityModeAndWorkerFirstTruthDigests",
        "boundaryPerformanceEnvelopes",
        "bridgeAllocationPosture",
        "mainThreadBroadWorkDenialArtifacts",
    ]
}

fn proof_family(
    proof_family: &'static str,
    runtime_test_surface: &'static str,
    boundary_test_surface: &'static str,
    certification_surface: &'static str,
    hostile_requirement: &'static str,
) -> WorkerPhase7ProofFamilyRequirement {
    WorkerPhase7ProofFamilyRequirement {
        proof_family,
        readiness: "ClosedByCanonicalCertification",
        runtime_test_surface,
        boundary_test_surface,
        certification_surface,
        hostile_requirement,
    }
}

fn required_proof_family_names() -> Vec<&'static str> {
    vec![
        "The Worker Compatibility Truth Equivalence Test",
        "The Mixed Placement Graph Isolation Test",
        "The Host Capability Worker Bridge Parity Test",
        "The Browser History Worker Admission Parity Test",
        "The Main-Thread Host Effect Boundary Test",
        "The Callback Placement Eligibility And Denial Test",
        "The Worker Ineligible Node Does Not Collapse Graph Breadth Test",
        "The Observation And Output Delivery Boundary Test",
        "The Diagnostics Summary Cost Honesty Test",
        "The Worker Replay Restore Capability Honesty Test",
        "The Import Export Callback Unavailability Test",
        "The Worker Bridge Boundedness Test",
        "The UI Freeze Surface Denial Test",
    ]
}

fn reject_missing_proof_families(
    proof_families: &[WorkerPhase7ProofFamilyRequirement],
) -> Result<(), WORTHSignalJsError> {
    for required in required_proof_family_names() {
        if !proof_families
            .iter()
            .any(|row| row.proof_family == required)
        {
            return Err(WORTHSignalJsError::invalid_input(format!(
                "worker Phase 7 test requirements require proof family {required}",
            )));
        }
    }
    Ok(())
}

fn reject_duplicate_proof_families(
    proof_families: &[WorkerPhase7ProofFamilyRequirement],
) -> Result<(), WORTHSignalJsError> {
    for (index, row) in proof_families.iter().enumerate() {
        if proof_families[(index + 1)..]
            .iter()
            .any(|candidate| candidate.proof_family == row.proof_family)
        {
            return Err(WORTHSignalJsError::invalid_input(format!(
                "worker Phase 7 test requirements duplicate proof family {}",
                row.proof_family,
            )));
        }
    }
    Ok(())
}

fn reject_weak_proof_family_rows(
    proof_families: &[WorkerPhase7ProofFamilyRequirement],
) -> Result<(), WORTHSignalJsError> {
    for row in proof_families {
        if row.readiness != "ClosedByCanonicalCertification"
            || row.runtime_test_surface.is_empty()
            || row.boundary_test_surface.is_empty()
            || row.certification_surface.is_empty()
            || row.hostile_requirement.is_empty()
        {
            return Err(WORTHSignalJsError::invalid_input(format!(
                "worker Phase 7 test requirements require closed proof family status with concrete test and certification surfaces for {}",
                row.proof_family,
            )));
        }
    }
    Ok(())
}

fn reject_missing_acceptance_artifacts(artifacts: &[&str]) -> Result<(), WORTHSignalJsError> {
    for required in required_acceptance_artifacts() {
        if !artifacts.contains(&required) {
            return Err(WORTHSignalJsError::invalid_input(format!(
                "worker Phase 7 test requirements require acceptance artifact {required}",
            )));
        }
    }
    Ok(())
}

fn reject_duplicate_acceptance_artifacts(artifacts: &[&str]) -> Result<(), WORTHSignalJsError> {
    for (index, artifact) in artifacts.iter().enumerate() {
        if artifacts[(index + 1)..].contains(artifact) {
            return Err(WORTHSignalJsError::invalid_input(format!(
                "worker Phase 7 test requirements duplicate acceptance artifact {artifact}",
            )));
        }
    }
    Ok(())
}
