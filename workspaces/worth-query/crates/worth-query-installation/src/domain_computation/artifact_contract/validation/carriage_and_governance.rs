use crate::domain_computation::{
    WorthQueryArtifactLifecycleContract, WorthQueryPortableArtifactContract,
};

use super::{
    portable_text, WorthQueryArtifactContractValidationDenial,
    WorthQueryArtifactContractValidationDenialKind as Kind,
};

pub(super) fn validate(
    contract: &WorthQueryPortableArtifactContract,
) -> Result<(), WorthQueryArtifactContractValidationDenial> {
    if contract.lifecycle == WorthQueryArtifactLifecycleContract::ReconstructibleAsAuthoritative {
        return Err(denial(contract, Kind::DerivedReconstructionClaimsAuthority));
    }
    if !contract.carriage.is_coherent() {
        return Err(denial(contract, Kind::InvalidCarriageContract));
    }
    if !contract.counters.names_are_distinct() {
        return Err(denial(contract, Kind::InvalidStructuralCounterContract));
    }
    if contract.producer_roles.is_empty()
        || contract.consumer_roles.is_empty()
        || contract
            .producer_roles
            .iter()
            .chain(&contract.consumer_roles)
            .any(|role| !portable_text(role))
    {
        return Err(denial(contract, Kind::InvalidStageRole));
    }
    if contract.governance.audiences().is_empty()
        || contract
            .governance
            .audiences()
            .iter()
            .any(|audience| !portable_text(audience))
    {
        return Err(denial(contract, Kind::InvalidGovernanceContract));
    }
    Ok(())
}

fn denial(
    contract: &WorthQueryPortableArtifactContract,
    kind: Kind,
) -> WorthQueryArtifactContractValidationDenial {
    WorthQueryArtifactContractValidationDenial::new(kind, contract.family.as_str())
}
