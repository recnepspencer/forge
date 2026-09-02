use crate::facade::RuntimeBridge;
use worth_proof::AuthorityWitness;

use super::super::BridgeInstalledSemanticCorrespondence;
use super::{AdmittedRuntimeWorldCorrespondenceBasis, RuntimeWorldCorrespondenceAdmissionDenial};

worth_proof::authority_marker!(pub(crate) BridgeRuntimeWorldAdmissionAuthorityMarker);

pub(crate) fn admit_installed_basis(
    runtime: &RuntimeBridge,
    installed: &BridgeInstalledSemanticCorrespondence,
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

    ensure_current_installation(runtime, installed)?;
    Ok(AdmittedRuntimeWorldCorrespondenceBasis::from_installed(
        installed,
        AuthorityWitness::from_authority_marker(BridgeRuntimeWorldAdmissionAuthorityMarker::seal()),
    ))
}

pub(crate) fn compare_current_basis(
    runtime: &RuntimeBridge,
    admitted: &AdmittedRuntimeWorldCorrespondenceBasis,
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
    let Some(expected_generation) = runtime
        .semantic_dependency_registry
        .current_source_installation_generation(admitted.dependency())
    else {
        return Err(RuntimeWorldCorrespondenceAdmissionDenial::InstalledCorrespondenceNotCurrent);
    };
    let actual_generation = admitted.source_installation_generation();
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

fn ensure_current_installation(
    runtime: &RuntimeBridge,
    installed: &BridgeInstalledSemanticCorrespondence,
) -> Result<(), RuntimeWorldCorrespondenceAdmissionDenial> {
    let Some(expected_generation) = runtime
        .semantic_dependency_registry
        .current_source_installation_generation(installed.dependency())
    else {
        return Err(RuntimeWorldCorrespondenceAdmissionDenial::InstalledCorrespondenceNotCurrent);
    };
    let actual_generation = installed.basis().source_installation_generation();
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
