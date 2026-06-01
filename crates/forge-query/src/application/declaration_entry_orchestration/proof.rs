use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};

use super::artifacts::{
    ForgeQueryDeclarationEntryOrchestrationArtifactPolicy,
    ForgeQueryDeclarationEntryOrchestrationExposureLevel,
    ForgeQueryDeclarationEntryOrchestrationTranscript,
};

pub(crate) fn forge_query_declaration_entry_orchestration_proof_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    input: I,
) -> ForgeQueryDeclarationEntryOrchestrationTranscript<D, I> {
    let lowered = super::lower::forge_query_lower_declaration_entry_orchestration_on_handle(
        handle,
        input,
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::ProofVisible,
        ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::ProofVisibleTranscript,
    );
    ForgeQueryDeclarationEntryOrchestrationTranscript::new(
        lowered.plan,
        lowered.outcome,
        lowered.step_records,
    )
}
