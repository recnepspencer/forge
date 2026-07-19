use crate::application::{
    WorthQueryDeclarationEnvelope, WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
};
use crate::basis_lifecycle::BasisFamily;

use super::contract::{
    WorthQueryDeclarationSignalCompatibilityContract, WorthQueryDeclarationSignalExecutionFamily,
};

pub(crate) fn derive_signal_execution_family<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    envelope: &WorthQueryDeclarationEnvelope<D, I>,
    contract: &WorthQueryDeclarationSignalCompatibilityContract,
) -> WorthQueryDeclarationSignalExecutionFamily {
    if envelope
        .route_plan()
        .is_some_and(|plan| plan.route_count() > 1)
    {
        WorthQueryDeclarationSignalExecutionFamily::MixedDerivedExecution
    } else {
        contract.execution_family()
    }
}

pub(crate) fn derive_required_basis_families(
    contract: &WorthQueryDeclarationSignalCompatibilityContract,
) -> Vec<BasisFamily> {
    contract.required_basis_families().to_vec()
}
