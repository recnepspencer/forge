use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryAdmittedDeclarationProgression,
    ForgeQueryDeclarationEntryOrchestrationStage, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};
use crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts;

use super::composition::ForgeQueryContributionComposedStop;
use super::input::{
    ForgeQueryContributionComposedMaterializationPolicy, ForgeQueryContributionIntent,
};
use super::intent_result::{
    ForgeQueryContributionComposedIntentRequestDescriptor,
    ForgeQueryContributionComposedIntentResult,
};
use super::lower::{
    build_composed_artifact, lower_progressed_declaration, process_contributions, stop_reason,
};
use super::mapping::{composed_outcome, linked_artifacts_for_envelope};
use super::outcome::{
    ForgeQueryContributionComposedOrchestrationChecked,
    ForgeQueryContributionComposedOrchestrationCheckedKind,
};

pub(crate) fn orchestrate_progressed_declaration_with_contributions_checked_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    contributions: Vec<ForgeQueryContributionIntent>,
    materialization_policy: ForgeQueryContributionComposedMaterializationPolicy,
) -> ForgeQueryContributionComposedOrchestrationChecked<D, I> {
    if contributions.is_empty() {
        return composed_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::Unsupported,
            ForgeQueryContributionComposedStop::Unsupported,
            ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            "contribution-composed orchestration requires at least one contribution intent",
            ForgeQueryBindingLinkedArtifacts::new(),
            None,
            None,
            None,
        );
    }

    let declaration = match lower_progressed_declaration(handle, progressed) {
        Ok(value) => value,
        Err(outcome) => return outcome,
    };
    let linked = linked_artifacts_for_envelope(&declaration.envelope);
    let intent_results = process_contributions::<D, I>(
        declaration.target.clone(),
        declaration.declaration_aspect_record.clone(),
        contributions,
        materialization_policy,
        linked.clone(),
    );
    finalize_checked(declaration.envelope, linked, intent_results)
}

fn finalize_checked<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    envelope: crate::application::ForgeQueryDeclarationEnvelope<D, I>,
    linked: ForgeQueryBindingLinkedArtifacts,
    intent_results: Vec<ForgeQueryContributionComposedIntentResult>,
) -> ForgeQueryContributionComposedOrchestrationChecked<D, I> {
    let declaration_aspect_record =
        super::ForgeQueryContributionComposedDeclarationAspectRecord::new(
            envelope.aspect_contract().clone(),
            envelope.aspect_publication().clone(),
        );
    match build_composed_artifact(envelope, intent_results.clone()) {
        Ok(composed) => ForgeQueryContributionComposedOrchestrationChecked::Bound(composed),
        Err((stop, contribution_digest)) => {
            let kind = stop_kind(stop);
            composed_outcome(
                kind,
                stop,
                ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
                stop_reason(stop, &intent_results),
                linked,
                Some(contribution_digest),
                Some(declaration_aspect_record),
                primary_intent_descriptor(&intent_results).cloned(),
            )
        }
    }
}

fn primary_intent_descriptor(
    intent_results: &[ForgeQueryContributionComposedIntentResult],
) -> Option<&ForgeQueryContributionComposedIntentRequestDescriptor> {
    intent_results
        .iter()
        .find(|value| !value.is_admitted())
        .or_else(|| intent_results.first())
        .map(ForgeQueryContributionComposedIntentResult::request)
}

fn stop_kind(
    stop: ForgeQueryContributionComposedStop,
) -> ForgeQueryContributionComposedOrchestrationCheckedKind {
    match stop {
        ForgeQueryContributionComposedStop::Deferred => {
            ForgeQueryContributionComposedOrchestrationCheckedKind::Deferred
        }
        ForgeQueryContributionComposedStop::DeclarationDenied => {
            ForgeQueryContributionComposedOrchestrationCheckedKind::DeclarationDenied
        }
        ForgeQueryContributionComposedStop::ContributionDenied => {
            ForgeQueryContributionComposedOrchestrationCheckedKind::ContributionDenied
        }
        ForgeQueryContributionComposedStop::Stale => {
            ForgeQueryContributionComposedOrchestrationCheckedKind::Stale
        }
        ForgeQueryContributionComposedStop::RebindRequired => {
            ForgeQueryContributionComposedOrchestrationCheckedKind::RebindRequired
        }
        ForgeQueryContributionComposedStop::Unsupported => {
            ForgeQueryContributionComposedOrchestrationCheckedKind::Unsupported
        }
        ForgeQueryContributionComposedStop::Failed => {
            ForgeQueryContributionComposedOrchestrationCheckedKind::Failed
        }
    }
}
