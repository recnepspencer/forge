use crate::application::{
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};
use crate::basis_lifecycle::BasisFamily;

use super::contract::{
    ForgeQueryDeclarationSignalCompatibilityContract, ForgeQueryDeclarationSignalExecutionFamily,
};

pub(crate) fn derive_signal_execution_family<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    envelope: &ForgeQueryDeclarationEnvelope<D, I>,
    contract: &ForgeQueryDeclarationSignalCompatibilityContract,
) -> ForgeQueryDeclarationSignalExecutionFamily {
    if envelope
        .route_plan()
        .is_some_and(|plan| plan.route_count() > 1)
    {
        ForgeQueryDeclarationSignalExecutionFamily::MixedDerivedExecution
    } else {
        contract.execution_family()
    }
}

pub(crate) fn derive_required_basis_families(
    contract: &ForgeQueryDeclarationSignalCompatibilityContract,
) -> Vec<BasisFamily> {
    contract.required_basis_families().to_vec()
}
