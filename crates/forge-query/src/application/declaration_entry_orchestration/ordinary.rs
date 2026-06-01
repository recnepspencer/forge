use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};

use super::artifacts::{
    terminal_error_from_outcome, ForgeQueryDeclarationEntryOrchestrationArtifactPolicy,
    ForgeQueryDeclarationEntryOrchestrationExposureLevel,
    ForgeQueryDeclarationEntryOrchestrationTerminalError,
};

pub(crate) fn forge_query_declaration_entry_orchestration_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    input: I,
) -> Result<
    crate::application::ForgeQueryDeclarationEnvelope<D, I>,
    ForgeQueryDeclarationEntryOrchestrationTerminalError<D, I>,
> {
    let lowered = super::lower::forge_query_lower_declaration_entry_orchestration_on_handle(
        handle,
        input,
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Ordinary,
        ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::OrdinaryEnvelopeOnly,
    );
    terminal_error_from_outcome(lowered.outcome)
}
