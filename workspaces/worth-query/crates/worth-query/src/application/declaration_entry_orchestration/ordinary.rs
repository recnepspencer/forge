use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};

use super::artifacts::{
    terminal_error_from_outcome, WorthQueryDeclarationEntryOrchestrationArtifactPolicy,
    WorthQueryDeclarationEntryOrchestrationExposureLevel,
    WorthQueryDeclarationEntryOrchestrationTerminalError,
};

#[cfg(test)]
pub(crate) fn worth_query_declaration_entry_orchestration_on_handle<
    D: WorthQueryDomainEntryMarker,
    C: crate::application::WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &crate::application::WorthQueryInstalledDomainDeclarationContext<D, C>,
    input: I,
) -> Result<
    crate::application::WorthQueryDeclarationEnvelope<D, I>,
    WorthQueryDeclarationEntryOrchestrationTerminalError<D, I>,
> {
    let lowered = super::lower::worth_query_lower_declaration_entry_orchestration_on_handle(
        handle,
        input,
        WorthQueryDeclarationEntryOrchestrationExposureLevel::Ordinary,
        WorthQueryDeclarationEntryOrchestrationArtifactPolicy::OrdinaryEnvelopeOnly,
    );
    terminal_error_from_outcome(lowered.outcome)
}
