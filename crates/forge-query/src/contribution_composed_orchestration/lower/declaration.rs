use crate::application::aspect_coverage_from_publication;
use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryAdmittedDeclarationProgression,
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext,
};
use crate::domain_capabilities::ForgeQueryDeclarationBoundContributionTarget;

use super::super::aspect::ForgeQueryContributionComposedIntentAspectRecord;
use super::super::mapping::{declaration_error_outcome, envelope_error_outcome};
use super::super::outcome::ForgeQueryContributionComposedOrchestrationOutcome;

pub(crate) struct DeclarationLowering<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    pub(crate) envelope: ForgeQueryDeclarationEnvelope<D, I>,
    pub(crate) target: ForgeQueryDeclarationBoundContributionTarget,
    pub(crate) declaration_aspect_record: ForgeQueryContributionComposedIntentAspectRecord,
}

pub(crate) fn lower_declaration<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    declaration_input: I,
) -> Result<DeclarationLowering<D, I>, ForgeQueryContributionComposedOrchestrationOutcome<D, I>> {
    let progressed = match handle.declare_review_and_progress(declaration_input) {
        Ok(progressed) => progressed,
        Err(error) => return Err(declaration_error_outcome(error)),
    };
    lower_progressed_declaration(handle, progressed)
}

pub(crate) fn lower_progressed_declaration<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
) -> Result<DeclarationLowering<D, I>, ForgeQueryContributionComposedOrchestrationOutcome<D, I>> {
    let target = ForgeQueryDeclarationBoundContributionTarget::for_canonical_declaration(
        progressed.canonical_declaration(),
    );
    let declaration_digest = format!(
        "{:?}",
        progressed.canonical_declaration().declaration_digest()
    );
    let progression_digest = progressed.progression_digest().to_string();
    let envelope = match handle.orchestrate_envelope_from_progressed(progressed) {
        Ok(envelope) => envelope,
        Err(error) => {
            return Err(envelope_error_outcome(
                error,
                &declaration_digest,
                &progression_digest,
            ))
        }
    };
    let declaration_aspect_record = ForgeQueryContributionComposedIntentAspectRecord::new(
        envelope.aspect_contract().clone(),
        aspect_coverage_from_publication(envelope.aspect_publication()),
    );
    Ok(DeclarationLowering {
        envelope,
        target,
        declaration_aspect_record,
    })
}
