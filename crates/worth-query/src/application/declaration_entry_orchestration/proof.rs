use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};

use super::artifacts::{
    WorthQueryDeclarationEntryOrchestrationArtifactPolicy,
    WorthQueryDeclarationEntryOrchestrationExposureLevel,
    WorthQueryDeclarationEntryOrchestrationTranscript,
};

pub(crate) fn worth_query_declaration_entry_orchestration_proof_on_handle<
    D: WorthQueryDomainEntryMarker,
    C: crate::application::WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &crate::application::WorthQueryInstalledDomainDeclarationContext<D, C>,
    input: I,
) -> WorthQueryDeclarationEntryOrchestrationTranscript<D, I> {
    let lowered = super::lower::worth_query_lower_declaration_entry_orchestration_on_handle(
        handle,
        input,
        WorthQueryDeclarationEntryOrchestrationExposureLevel::ProofVisible,
        WorthQueryDeclarationEntryOrchestrationArtifactPolicy::ProofVisibleTranscript,
    );
    WorthQueryDeclarationEntryOrchestrationTranscript::new(
        lowered.plan,
        lowered.outcome,
        lowered.step_records,
    )
}
