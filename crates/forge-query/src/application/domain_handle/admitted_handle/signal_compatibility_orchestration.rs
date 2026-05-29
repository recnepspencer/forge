use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};
use crate::ordinary_outcome::ForgeQueryOrdinaryOutcome;
use crate::signal_compatibility_orchestration::{
    orchestrate_signal_compatibility_on_handle,
    ordinary_outcome_from_signal_compatibility_orchestration_checked,
    ForgeQuerySignalCompatibilityOrchestration, ForgeQuerySignalCompatibilityOrchestrationChecked,
    ForgeQuerySignalCompatibilityOrchestrationInput,
    ForgeQuerySignalCompatibilityOrchestrationOutcome,
    ForgeQuerySignalCompatibilityOrchestrationTranscript,
};

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn orchestrate_signal_compatibility<I: ForgeQueryDeclarationInput<D>>(
        &self,
        input: ForgeQuerySignalCompatibilityOrchestrationInput<D, I>,
    ) -> ForgeQuerySignalCompatibilityOrchestrationOutcome<D, I> {
        self.orchestrate_signal_compatibility_checked(input)
            .into_outcome()
    }

    pub fn orchestrate_signal_compatibility_outcome<I: ForgeQueryDeclarationInput<D>>(
        &self,
        input: ForgeQuerySignalCompatibilityOrchestrationInput<D, I>,
    ) -> ForgeQueryOrdinaryOutcome<ForgeQuerySignalCompatibilityOrchestration<D, I>> {
        ordinary_outcome_from_signal_compatibility_orchestration_checked(
            self.orchestrate_signal_compatibility_checked(input),
        )
    }

    pub fn orchestrate_signal_compatibility_checked<I: ForgeQueryDeclarationInput<D>>(
        &self,
        input: ForgeQuerySignalCompatibilityOrchestrationInput<D, I>,
    ) -> ForgeQuerySignalCompatibilityOrchestrationChecked<D, I> {
        self.orchestrate_signal_compatibility_proof(input)
            .into_checked()
    }

    pub fn orchestrate_signal_compatibility_proof<I: ForgeQueryDeclarationInput<D>>(
        &self,
        input: ForgeQuerySignalCompatibilityOrchestrationInput<D, I>,
    ) -> ForgeQuerySignalCompatibilityOrchestrationTranscript<D, I> {
        orchestrate_signal_compatibility_on_handle(self, input)
    }
}
