use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};
use crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts;

use super::artifact::ForgeQueryContributionComposedOrchestration;
use super::input::ForgeQueryContributionComposedOrchestrationInput;
use super::lower::{process_contributions, request_digest};
use super::mapping::{
    contribution_digest_from_outcome, declaration_error_outcome, envelope_error_outcome,
    linked_artifacts_for_envelope, linked_artifacts_from_outcome,
};
use super::outcome::{
    ForgeQueryContributionComposedOrchestrationChecked,
    ForgeQueryContributionComposedOrchestrationOutcome,
};

pub struct ForgeQueryContributionComposedOrchestrationTranscript<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    request_digest: String,
    outcome: ForgeQueryContributionComposedOrchestrationOutcome<D, I>,
    linked_artifacts: ForgeQueryBindingLinkedArtifacts,
    contribution_digest: Option<String>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryContributionComposedOrchestrationTranscript<D, I>
{
    pub(crate) fn new(
        request_digest: String,
        outcome: ForgeQueryContributionComposedOrchestrationOutcome<D, I>,
        linked_artifacts: ForgeQueryBindingLinkedArtifacts,
        contribution_digest: Option<String>,
    ) -> Self {
        Self {
            request_digest,
            outcome,
            linked_artifacts,
            contribution_digest,
        }
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn outcome(&self) -> &ForgeQueryContributionComposedOrchestrationOutcome<D, I> {
        &self.outcome
    }

    pub fn linked_artifacts(&self) -> &ForgeQueryBindingLinkedArtifacts {
        &self.linked_artifacts
    }

    pub fn contribution_digest(&self) -> Option<&str> {
        self.contribution_digest.as_deref()
    }

    pub fn into_checked(self) -> ForgeQueryContributionComposedOrchestrationChecked<D, I> {
        self.outcome
    }
}

pub(crate) fn orchestrate_declaration_with_contributions_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    input: ForgeQueryContributionComposedOrchestrationInput<D, I>,
) -> ForgeQueryContributionComposedOrchestrationTranscript<D, I> {
    let request_digest = request_digest(&input);
    let (declaration_input, contributions, materialization_policy) = input.into_parts();
    let progressed = match handle.declare_review_and_progress(declaration_input) {
        Ok(progressed) => progressed,
        Err(error) => {
            let outcome = declaration_error_outcome(error);
            let linked = linked_artifacts_from_outcome(&outcome);
            return ForgeQueryContributionComposedOrchestrationTranscript::new(
                request_digest,
                outcome,
                linked,
                None,
            );
        }
    };
    let declaration_target = crate::domain_capabilities::ForgeQueryDeclarationBoundContributionTarget::for_canonical_declaration(
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
            let outcome = envelope_error_outcome(error, &declaration_digest, &progression_digest);
            let linked = linked_artifacts_from_outcome(&outcome);
            let contribution_digest = contribution_digest_from_outcome(&outcome);
            return ForgeQueryContributionComposedOrchestrationTranscript::new(
                request_digest,
                outcome,
                linked,
                contribution_digest,
            );
        }
    };
    let linked = linked_artifacts_for_envelope(&envelope);
    let (contribution_composition, processed) = match process_contributions(
        declaration_target,
        contributions,
        materialization_policy,
        linked.clone(),
    ) {
        Ok(value) => value,
        Err(outcome) => {
            let linked = linked_artifacts_from_outcome(&outcome);
            let contribution_digest = contribution_digest_from_outcome(&outcome);
            return ForgeQueryContributionComposedOrchestrationTranscript::new(
                request_digest,
                outcome,
                linked,
                contribution_digest,
            );
        }
    };
    let composed = ForgeQueryContributionComposedOrchestration::new(
        envelope,
        contribution_composition,
        processed,
    );
    let contribution_digest = Some(
        composed
            .contribution_composition()
            .contribution_digest()
            .to_string(),
    );
    ForgeQueryContributionComposedOrchestrationTranscript::new(
        request_digest,
        ForgeQueryContributionComposedOrchestrationOutcome::Bound(composed),
        linked,
        contribution_digest,
    )
}
