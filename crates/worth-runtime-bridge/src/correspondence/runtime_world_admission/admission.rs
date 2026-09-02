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

    Ok(AdmittedRuntimeWorldCorrespondenceBasis::from_installed(
        installed,
        AuthorityWitness::from_authority_marker(BridgeRuntimeWorldAdmissionAuthorityMarker::seal()),
    ))
}
