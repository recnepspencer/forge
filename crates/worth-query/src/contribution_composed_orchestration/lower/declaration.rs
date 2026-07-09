use crate::application::aspect_coverage_from_publication;
use crate::application::{
    WorthQueryAdmittedConfiguredDomainHandle, WorthQueryAdmittedDeclarationProgression,
    WorthQueryDeclarationEnvelope, WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext,
};
use crate::domain_capabilities::WorthQueryDeclarationBoundContributionTarget;
use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

use super::super::aspect::WorthQueryContributionComposedIntentAspectRecord;
use super::super::mapping::{declaration_error_outcome, envelope_error_outcome};
use super::super::outcome::WorthQueryContributionComposedOrchestrationOutcome;

pub(crate) struct DeclarationLowering<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    pub(crate) envelope: WorthQueryDeclarationEnvelope<D, I>,
    pub(crate) target: WorthQueryDeclarationBoundContributionTarget,
    pub(crate) declaration_aspect_record: WorthQueryContributionComposedIntentAspectRecord,
    #[allow(dead_code)]
    pub(crate) lowering_declaration_identity: WorthQueryEvidenceIdentity,
}

pub(crate) fn lower_declaration<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    declaration_input: I,
) -> Result<DeclarationLowering<D, I>, WorthQueryContributionComposedOrchestrationOutcome<D, I>> {
    let progressed = match handle.declare_review_and_progress(declaration_input) {
        Ok(progressed) => progressed,
        Err(error) => return Err(declaration_error_outcome(error)),
    };
    lower_progressed_declaration(handle, progressed)
}

pub(crate) fn lower_progressed_declaration<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
) -> Result<DeclarationLowering<D, I>, WorthQueryContributionComposedOrchestrationOutcome<D, I>> {
    let target = WorthQueryDeclarationBoundContributionTarget::for_canonical_declaration(
        progressed.canonical_declaration(),
    );
    let lowering_declaration_identity = compose_lowering_declaration_identity(&progressed);
    let envelope = match handle.orchestrate_envelope_from_progressed(progressed) {
        Ok(envelope) => envelope,
        Err(error) => return Err(envelope_error_outcome(error)),
    };
    let declaration_aspect_record = WorthQueryContributionComposedIntentAspectRecord::new(
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
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    progressed: &WorthQueryAdmittedDeclarationProgression<D, I>,
) -> WorthQueryEvidenceIdentity {
    let declaration_identity =
        worth_query_evidence_identity(WorthQueryEvidenceScope::DeclarationBridgeLoweringIdentity)
            .field_shape(
                WorthQueryEvidenceTag::new("identity_family"),
                "worth_query_contribution_composed_lowering_declaration_v1",
            )
            .field_shape(
                WorthQueryEvidenceTag::new("declaration_family"),
                progressed.declaration_family_key(),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("handle_identity"),
                progressed.canonical_declaration().handle_identity_digest(),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("declaration"),
                &format!(
                    "{:?}",
                    progressed.canonical_declaration().declaration_digest()
                ),
            )
            .seal();
    worth_query_evidence_identity(WorthQueryEvidenceScope::DeclarationBridgeLoweringIdentity)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_contribution_composed_lowering_progression_v1",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("declaration"),
            &declaration_identity,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("progression"),
            progressed.progression_digest(),
        )
        .seal()
}
