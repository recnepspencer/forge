use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryAdmittedDeclarationProgression,
    ForgeQueryDeclarationEntryOrchestrationArtifactPolicy,
    ForgeQueryDeclarationEntryOrchestrationExposureLevel,
    ForgeQueryDeclarationEntryOrchestrationProduct, ForgeQueryDeclarationEnvelope,
    ForgeQueryDeclarationEnvelopeChecked, ForgeQueryDeclarationEnvelopeTerminalError,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationRouteIntent, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext,
};

use super::common::{envelope_orchestration_identity, envelope_terminal_from_checked};
use super::transcript::ForgeQueryDeclarationEnvelopeOrchestrationTranscript;
use crate::application::{
    forge_query_lower_declaration_entry_product_orchestration_from_progressed_on_handle,
    ForgeQueryDeclarationEntryProductChecked,
};

pub(crate) fn forge_query_declaration_envelope_orchestration_from_progressed_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    route_intent: Option<ForgeQueryDeclarationRouteIntent>,
) -> Result<ForgeQueryDeclarationEnvelope<D, I>, ForgeQueryDeclarationEnvelopeTerminalError<D, I>> {
    match forge_query_checked_declaration_envelope_orchestration_from_progressed_on_handle(
        handle,
        progressed,
        route_intent,
    ) {
        ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => Ok(envelope),
        other => Err(envelope_terminal_from_checked(other)),
    }
}

pub(crate) fn forge_query_checked_declaration_envelope_orchestration_from_progressed_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    route_intent: Option<ForgeQueryDeclarationRouteIntent>,
) -> ForgeQueryDeclarationEnvelopeChecked<D, I> {
    match forge_query_lower_declaration_entry_product_orchestration_from_progressed_on_handle(
        handle,
        progressed,
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Checked,
        ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::CheckedOutcomeOnly,
        ForgeQueryDeclarationEntryOrchestrationProduct::Envelope,
        route_intent,
    )
    .checked
    {
        ForgeQueryDeclarationEntryProductChecked::Envelope(checked) => checked,
        _ => panic!("envelope orchestration must project the envelope product"),
    }
}

pub(crate) fn forge_query_declaration_envelope_orchestration_from_progressed_proof_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    route_intent: Option<ForgeQueryDeclarationRouteIntent>,
) -> ForgeQueryDeclarationEnvelopeOrchestrationTranscript<D, I> {
    let lowered =
        forge_query_lower_declaration_entry_product_orchestration_from_progressed_on_handle(
            handle,
            progressed,
            ForgeQueryDeclarationEntryOrchestrationExposureLevel::ProofVisible,
            ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::ProofVisibleTranscript,
            ForgeQueryDeclarationEntryOrchestrationProduct::Envelope,
            route_intent,
        );
    let checked = match lowered.checked {
        ForgeQueryDeclarationEntryProductChecked::Envelope(checked) => checked,
        _ => panic!("envelope orchestration proof must project the envelope product"),
    };
    let outcome_identity = envelope_orchestration_identity(&checked);
    ForgeQueryDeclarationEnvelopeOrchestrationTranscript::new(
        lowered.plan,
        checked,
        lowered.step_records,
        outcome_identity,
    )
}
