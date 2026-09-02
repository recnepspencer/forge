use crate::facade::RuntimeBridge;
use worth_proof::AuthorityWitness;

use super::super::BridgeInstalledSemanticCorrespondence;
use super::RuntimeWorldCorrespondenceInspectionLedger;
use super::{AdmittedRuntimeWorldCorrespondenceBasis, RuntimeWorldCorrespondenceAdmissionDenial};

worth_proof::authority_marker!(pub(crate) BridgeRuntimeWorldAdmissionAuthorityMarker);

pub(crate) fn admit_installed_basis(
    runtime: &RuntimeBridge,
    installed: &BridgeInstalledSemanticCorrespondence,
    inspection: &RuntimeWorldCorrespondenceInspectionLedger,
) -> Result<AdmittedRuntimeWorldCorrespondenceBasis, RuntimeWorldCorrespondenceAdmissionDenial> {
    let actual_runtime_key = installed.basis().bridge_runtime_key;
    if actual_runtime_key != runtime.signal_runtime_key {
        return Err(
            RuntimeWorldCorrespondenceAdmissionDenial::ForeignBridgeRuntime {
                expected_runtime_key: runtime.signal_runtime_key,
                actual_runtime_key,
            },
        );
    }

    ensure_current_installation(
        runtime,
        installed.dependency(),
        installed.basis().source_installation_generation(),
        inspection,
    )?;
    Ok(AdmittedRuntimeWorldCorrespondenceBasis::from_installed(
        installed,
        AuthorityWitness::from_authority_marker(BridgeRuntimeWorldAdmissionAuthorityMarker::seal()),
    ))
}

pub(crate) fn compare_current_basis(
    runtime: &RuntimeBridge,
    admitted: &AdmittedRuntimeWorldCorrespondenceBasis,
    inspection: &RuntimeWorldCorrespondenceInspectionLedger,
) -> Result<(), RuntimeWorldCorrespondenceAdmissionDenial> {
    let actual_runtime_key = admitted.basis().bridge_runtime_key;
    if actual_runtime_key != runtime.signal_runtime_key {
        return Err(
            RuntimeWorldCorrespondenceAdmissionDenial::ForeignBridgeRuntime {
                expected_runtime_key: runtime.signal_runtime_key,
                actual_runtime_key,
            },
        );
    }
    ensure_current_installation(
        runtime,
        admitted.dependency(),
        admitted.source_installation_generation(),
        inspection,
    )
}

fn ensure_current_installation(
    runtime: &RuntimeBridge,
    dependency: &super::super::BridgeSemanticDependencyCandidate,
    actual_generation: u64,
    inspection: &RuntimeWorldCorrespondenceInspectionLedger,
) -> Result<(), RuntimeWorldCorrespondenceAdmissionDenial> {
    let Some(expected_generation) = runtime
        .semantic_dependency_registry
        .current_source_installation_generation(dependency, inspection)
    else {
        return Err(RuntimeWorldCorrespondenceAdmissionDenial::InstalledCorrespondenceNotCurrent);
    };
    if expected_generation != actual_generation {
        return Err(
            RuntimeWorldCorrespondenceAdmissionDenial::InstalledGenerationDrift {
                expected_generation,
                actual_generation,
            },
        );
    }
    Ok(())
}
