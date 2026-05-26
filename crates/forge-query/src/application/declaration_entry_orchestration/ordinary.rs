use crate::application::{
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};

use super::checked::ForgeQueryDeclarationEntryOrchestrationChecked;
use super::refusal::ForgeQueryDeclarationEntryOrchestrationTerminalError;

pub(crate) fn forge_query_declaration_entry_orchestration_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    input: I,
) -> Result<
    ForgeQueryDeclarationEnvelope<D, I>,
    ForgeQueryDeclarationEntryOrchestrationTerminalError<D, I>,
> {
    match super::checked::forge_query_checked_declaration_entry_orchestration_on_handle(
        handle, input,
    ) {
        ForgeQueryDeclarationEntryOrchestrationChecked::Enveloped(envelope) => Ok(envelope),
        ForgeQueryDeclarationEntryOrchestrationChecked::Deferred(outcome) => {
            Err(ForgeQueryDeclarationEntryOrchestrationTerminalError::Deferred(outcome))
        }
        ForgeQueryDeclarationEntryOrchestrationChecked::Denied(outcome) => {
            Err(ForgeQueryDeclarationEntryOrchestrationTerminalError::Denied(outcome))
        }
        ForgeQueryDeclarationEntryOrchestrationChecked::Stale(outcome) => Err(
            ForgeQueryDeclarationEntryOrchestrationTerminalError::Stale(outcome),
        ),
        ForgeQueryDeclarationEntryOrchestrationChecked::RebindRequired(outcome) => {
            Err(ForgeQueryDeclarationEntryOrchestrationTerminalError::RebindRequired(outcome))
        }
        ForgeQueryDeclarationEntryOrchestrationChecked::Failed(outcome) => {
            Err(ForgeQueryDeclarationEntryOrchestrationTerminalError::Failed(outcome))
        }
        ForgeQueryDeclarationEntryOrchestrationChecked::Refused(outcome) => {
            Err(ForgeQueryDeclarationEntryOrchestrationTerminalError::Refused(outcome))
        }
    }
}
