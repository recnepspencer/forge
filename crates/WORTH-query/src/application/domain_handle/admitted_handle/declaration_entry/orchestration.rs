use crate::application::{
    worth_query_checked_declaration_entry_orchestration_on_handle,
    worth_query_declaration_entry_orchestration_on_handle,
    worth_query_declaration_entry_orchestration_proof_on_handle,
    WorthQueryDeclarationEntryOrchestrationOutcome,
    WorthQueryDeclarationEntryOrchestrationTerminalError,
    WorthQueryDeclarationEntryOrchestrationTranscript, WorthQueryDeclarationEnvelope,
    WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
};
use crate::ordinary_outcome::{
    ordinary_outcome_from_orchestration_terminal, WorthQueryOrdinaryOutcome,
};

use super::super::WorthQueryAdmittedConfiguredDomainHandle;
use crate::application::WorthQueryDomainOperatingContext;

impl<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>>
    WorthQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn orchestrate_declaration_entry<I>(
        &self,
        input: I,
    ) -> Result<
        WorthQueryDeclarationEnvelope<D, I>,
        WorthQueryDeclarationEntryOrchestrationTerminalError<D, I>,
    >
    where
        I: WorthQueryDeclarationInput<D>,
    {
        worth_query_declaration_entry_orchestration_on_handle(self, input)
    }

    pub fn orchestrate_declaration_entry_outcome<I>(
        &self,
        input: I,
    ) -> WorthQueryOrdinaryOutcome<WorthQueryDeclarationEnvelope<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        match worth_query_declaration_entry_orchestration_on_handle(self, input) {
            Ok(envelope) => WorthQueryOrdinaryOutcome::Bound(envelope),
            Err(terminal) => ordinary_outcome_from_orchestration_terminal(terminal),
        }
    }

    pub fn orchestrate_declaration_entry_checked<I>(
        &self,
        input: I,
    ) -> WorthQueryDeclarationEntryOrchestrationOutcome<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        worth_query_checked_declaration_entry_orchestration_on_handle(self, input)
    }

    pub fn orchestrate_declaration_entry_proof<I>(
        &self,
        input: I,
    ) -> WorthQueryDeclarationEntryOrchestrationTranscript<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        worth_query_declaration_entry_orchestration_proof_on_handle(self, input)
    }
}
