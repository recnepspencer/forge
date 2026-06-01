use crate::application::{
    forge_query_checked_declaration_entry_orchestration_on_handle,
    forge_query_declaration_entry_orchestration_on_handle,
    forge_query_declaration_entry_orchestration_proof_on_handle,
    ForgeQueryDeclarationEntryOrchestrationOutcome,
    ForgeQueryDeclarationEntryOrchestrationTerminalError,
    ForgeQueryDeclarationEntryOrchestrationTranscript, ForgeQueryDeclarationEnvelope,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};
use crate::ordinary_outcome::{
    ordinary_outcome_from_orchestration_terminal, ForgeQueryOrdinaryOutcome,
};

use super::super::ForgeQueryAdmittedConfiguredDomainHandle;
use crate::application::ForgeQueryDomainOperatingContext;

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn orchestrate_declaration_entry<I>(
        &self,
        input: I,
    ) -> Result<
        ForgeQueryDeclarationEnvelope<D, I>,
        ForgeQueryDeclarationEntryOrchestrationTerminalError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_declaration_entry_orchestration_on_handle(self, input)
    }

    pub fn orchestrate_declaration_entry_outcome<I>(
        &self,
        input: I,
    ) -> ForgeQueryOrdinaryOutcome<ForgeQueryDeclarationEnvelope<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        match forge_query_declaration_entry_orchestration_on_handle(self, input) {
            Ok(envelope) => ForgeQueryOrdinaryOutcome::Bound(envelope),
            Err(terminal) => ordinary_outcome_from_orchestration_terminal(terminal),
        }
    }

    pub fn orchestrate_declaration_entry_checked<I>(
        &self,
        input: I,
    ) -> ForgeQueryDeclarationEntryOrchestrationOutcome<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_checked_declaration_entry_orchestration_on_handle(self, input)
    }

    pub fn orchestrate_declaration_entry_proof<I>(
        &self,
        input: I,
    ) -> ForgeQueryDeclarationEntryOrchestrationTranscript<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_declaration_entry_orchestration_proof_on_handle(self, input)
    }
}
