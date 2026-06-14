use crate::application::aspect_coverage_from_publication;
use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryAdmittedDeclarationProgression,
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext,
};
use crate::domain_capabilities::ForgeQueryDeclarationBoundContributionTarget;
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

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
    pub(crate) lowering_declaration_identity: ForgeQueryEvidenceIdentity,
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
    let lowering_declaration_identity = compose_lowering_declaration_identity(&progressed);
    let envelope = match handle.orchestrate_envelope_from_progressed(progressed) {
        Ok(envelope) => envelope,
        Err(error) => return Err(envelope_error_outcome(error)),
    };
    let declaration_aspect_record = ForgeQueryContributionComposedIntentAspectRecord::new(
        envelope.aspect_contract().clone(),
        aspect_coverage_from_publication(envelope.aspect_publication()),
    );
    Ok(DeclarationLowering {
        envelope,
        target,
        declaration_aspect_record,
        lowering_declaration_identity,
    })
}

pub(crate) fn compose_lowering_declaration_identity<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    progressed: &ForgeQueryAdmittedDeclarationProgression<D, I>,
) -> ForgeQueryEvidenceIdentity {
    let declaration_identity = forge_query_evidence_identity(
        ForgeQueryEvidenceScope::DeclarationBridgeLoweringIdentity,
    )
    .field_shape(
        ForgeQueryEvidenceTag::new("identity_family"),
        "forge_query_contribution_composed_lowering_declaration_v1",
    )
    .field_shape(
        ForgeQueryEvidenceTag::new("declaration_family"),
        progressed.declaration_family_key(),
    )
    .field_shape(
        ForgeQueryEvidenceTag::new("handle_identity"),
        progressed.canonical_declaration().handle_identity_digest(),
    )
    .field_shape(
        ForgeQueryEvidenceTag::new("declaration"),
        &format!(
            "{:?}",
            progressed.canonical_declaration().declaration_digest()
        ),
    )
    .seal();
    forge_query_evidence_identity(ForgeQueryEvidenceScope::DeclarationBridgeLoweringIdentity)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_contribution_composed_lowering_progression_v1",
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("declaration"),
            &declaration_identity,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("progression"),
            progressed.progression_digest(),
        )
        .seal()
}
