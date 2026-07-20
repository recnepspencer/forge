use crate::application::{
    WorthQueryDeclarationInput, WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
    WorthQueryInstalledDomainDeclarationContext,
};
use crate::ordinary_outcome::WorthQueryOrdinaryOutcome;
use crate::signal_compatibility_orchestration::{
    orchestrate_signal_compatibility_on_handle,
    ordinary_outcome_from_signal_compatibility_orchestration_checked,
    WorthQuerySignalCompatibilityOrchestration, WorthQuerySignalCompatibilityOrchestrationChecked,
    WorthQuerySignalCompatibilityOrchestrationInput,
    WorthQuerySignalCompatibilityOrchestrationOutcome,
    WorthQuerySignalCompatibilityOrchestrationTranscript,
};

impl<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>>
    WorthQueryInstalledDomainDeclarationContext<D, C>
{
    pub fn orchestrate_signal_compatibility<I: WorthQueryDeclarationInput<D>>(
        &self,
        input: WorthQuerySignalCompatibilityOrchestrationInput<D, I>,
    ) -> WorthQuerySignalCompatibilityOrchestrationOutcome<D, I> {
        self.orchestrate_signal_compatibility_checked(input)
            .into_outcome()
    }

    pub fn orchestrate_signal_compatibility_outcome<I: WorthQueryDeclarationInput<D>>(
        &self,
        input: WorthQuerySignalCompatibilityOrchestrationInput<D, I>,
    ) -> WorthQueryOrdinaryOutcome<WorthQuerySignalCompatibilityOrchestration<D, I>> {
        ordinary_outcome_from_signal_compatibility_orchestration_checked(
            self.orchestrate_signal_compatibility_checked(input),
        )
    }

    pub fn orchestrate_signal_compatibility_checked<I: WorthQueryDeclarationInput<D>>(
        &self,
        input: WorthQuerySignalCompatibilityOrchestrationInput<D, I>,
    ) -> WorthQuerySignalCompatibilityOrchestrationChecked<D, I> {
        self.orchestrate_signal_compatibility_proof(input)
            .into_checked()
    }

    pub fn orchestrate_signal_compatibility_proof<I: WorthQueryDeclarationInput<D>>(
        &self,
        input: WorthQuerySignalCompatibilityOrchestrationInput<D, I>,
    ) -> WorthQuerySignalCompatibilityOrchestrationTranscript<D, I> {
        orchestrate_signal_compatibility_on_handle(self, input)
    }
}
