use crate::application::{
    forge_query_checked_declaration_entry_orchestration_on_handle,
    forge_query_declaration_entry_orchestration_on_handle,
    forge_query_declaration_entry_orchestration_proof_on_handle,
    ForgeQueryDeclarationEntryOrchestrationChecked, ForgeQueryDeclarationEntryOrchestrationProof,
    ForgeQueryDeclarationEntryOrchestrationTerminalError, ForgeQueryDeclarationEnvelope,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};

use super::ForgeQueryAdmittedConfiguredDomainHandle;
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

    pub fn orchestrate_declaration_entry_checked<I>(
        &self,
        input: I,
    ) -> ForgeQueryDeclarationEntryOrchestrationChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_checked_declaration_entry_orchestration_on_handle(self, input)
    }

    pub fn orchestrate_declaration_entry_proof<I>(
        &self,
        input: I,
    ) -> ForgeQueryDeclarationEntryOrchestrationProof<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_declaration_entry_orchestration_proof_on_handle(self, input)
    }
}
