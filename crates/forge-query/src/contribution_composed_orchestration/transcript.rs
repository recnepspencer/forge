use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationEntryOrchestrationStage,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};
use crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts;

use super::aspect::ForgeQueryContributionComposedDeclarationAspectRecord;
use super::composition::ForgeQueryContributionComposedClassification;
use super::input::ForgeQueryContributionComposedOrchestrationInput;
use super::intent_result::ForgeQueryContributionComposedIntentResult;
use super::lower::{
    build_composed_artifact, lower_declaration, materialization_policy_label,
    process_contributions, request_descriptor, request_identity, stop_reason, DeclarationLowering,
};
use super::mapping::{
    composed_outcome, contribution_digest_from_outcome, linked_artifacts_for_envelope,
    linked_artifacts_from_outcome,
};
use super::outcome::{
    ForgeQueryContributionComposedOrchestrationChecked,
    ForgeQueryContributionComposedOrchestrationCheckedKind,
    ForgeQueryContributionComposedOrchestrationOutcome,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryContributionComposedDeclarationRecord {
    stage: ForgeQueryDeclarationEntryOrchestrationStage,
    declaration_digest: Option<String>,
    progression_digest: Option<String>,
    envelope_digest: Option<String>,
    route_plan_digest: Option<String>,
    aspect_record: Option<ForgeQueryContributionComposedDeclarationAspectRecord>,
}

impl ForgeQueryContributionComposedDeclarationRecord {
    fn from_linked_artifacts(
        stage: ForgeQueryDeclarationEntryOrchestrationStage,
        linked_artifacts: &ForgeQueryBindingLinkedArtifacts,
    ) -> Self {
        Self {
            stage,
            declaration_digest: linked_artifacts.declaration_digest().map(str::to_string),
            progression_digest: linked_artifacts.progression_digest().map(str::to_string),
            envelope_digest: linked_artifacts.envelope_digest().map(str::to_string),
            route_plan_digest: linked_artifacts.route_plan_digest().map(str::to_string),
            aspect_record: None,
        }
    }

    fn from_envelope<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
        envelope: &crate::application::ForgeQueryDeclarationEnvelope<D, I>,
        linked_artifacts: &ForgeQueryBindingLinkedArtifacts,
    ) -> Self {
        Self {
            stage: ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
            declaration_digest: linked_artifacts.declaration_digest().map(str::to_string),
            progression_digest: linked_artifacts.progression_digest().map(str::to_string),
            envelope_digest: linked_artifacts.envelope_digest().map(str::to_string),
            route_plan_digest: linked_artifacts.route_plan_digest().map(str::to_string),
            aspect_record: Some(ForgeQueryContributionComposedDeclarationAspectRecord::new(
                envelope.aspect_contract().clone(),
                envelope.aspect_publication().clone(),
            )),
        }
    }

    pub fn stage(&self) -> ForgeQueryDeclarationEntryOrchestrationStage {
        self.stage
    }

    pub fn declaration_digest(&self) -> Option<&str> {
        self.declaration_digest.as_deref()
    }

    pub fn progression_digest(&self) -> Option<&str> {
        self.progression_digest.as_deref()
    }

    pub fn envelope_digest(&self) -> Option<&str> {
        self.envelope_digest.as_deref()
    }

    pub fn route_plan_digest(&self) -> Option<&str> {
        self.route_plan_digest.as_deref()
    }

    pub fn aspect_record(&self) -> Option<&ForgeQueryContributionComposedDeclarationAspectRecord> {
        self.aspect_record.as_ref()
    }

    pub fn aspect_contract(
        &self,
    ) -> Option<&crate::application::ForgeQueryDeclarationAspectContract> {
        self.aspect_record
            .as_ref()
            .map(ForgeQueryContributionComposedDeclarationAspectRecord::contract)
    }

    pub fn aspect_publication(
        &self,
    ) -> Option<&crate::application::ForgeQueryDeclarationAspectPublication> {
        self.aspect_record
            .as_ref()
            .map(ForgeQueryContributionComposedDeclarationAspectRecord::publication)
    }
}

pub struct ForgeQueryContributionComposedOrchestrationTranscript<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    request_descriptor: String,
    request_digest: String,
    materialization_policy: &'static str,
    declaration: ForgeQueryContributionComposedDeclarationRecord,
    outcome: ForgeQueryContributionComposedOrchestrationOutcome<D, I>,
    linked_artifacts: ForgeQueryBindingLinkedArtifacts,
    contribution_digest: Option<String>,
    composition_classification: Option<ForgeQueryContributionComposedClassification>,
    intent_results: Vec<ForgeQueryContributionComposedIntentResult>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryContributionComposedOrchestrationTranscript<D, I>
{
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        request_descriptor: String,
        request_digest: String,
        materialization_policy: &'static str,
        declaration: ForgeQueryContributionComposedDeclarationRecord,
        outcome: ForgeQueryContributionComposedOrchestrationOutcome<D, I>,
        linked_artifacts: ForgeQueryBindingLinkedArtifacts,
        contribution_digest: Option<String>,
        composition_classification: Option<ForgeQueryContributionComposedClassification>,
        intent_results: Vec<ForgeQueryContributionComposedIntentResult>,
    ) -> Self {
        Self {
            request_descriptor,
            request_digest,
            materialization_policy,
            declaration,
            outcome,
            linked_artifacts,
            contribution_digest,
            composition_classification,
            intent_results,
        }
    }

    pub fn request_descriptor(&self) -> &str {
        &self.request_descriptor
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn materialization_policy(&self) -> &str {
        self.materialization_policy
    }

    pub fn declaration(&self) -> &ForgeQueryContributionComposedDeclarationRecord {
        &self.declaration
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

    pub fn composition_classification(
        &self,
    ) -> Option<ForgeQueryContributionComposedClassification> {
        self.composition_classification
    }

    pub fn intent_results(&self) -> &[ForgeQueryContributionComposedIntentResult] {
        &self.intent_results
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
    let request_descriptor_value = request_descriptor(&input);
    let request_digest_value = request_identity(&input).as_str().to_string();
    let materialization_policy = materialization_policy_label(input.materialization_policy());
    let (declaration_input, contributions, materialization_policy_value) = input.into_parts();
    if contributions.is_empty() {
        return empty_contribution_transcript(
            request_descriptor_value,
            request_digest_value,
            materialization_policy,
        );
    }
    let declaration = match lower_declaration(handle, declaration_input) {
        Ok(value) => value,
        Err(outcome) => {
            let linked = linked_artifacts_from_outcome(&outcome);
            let contribution_digest = contribution_digest_from_outcome(&outcome);
            return ForgeQueryContributionComposedOrchestrationTranscript::new(
                request_descriptor_value,
                request_digest_value,
                materialization_policy,
                ForgeQueryContributionComposedDeclarationRecord::from_linked_artifacts(
                    declaration_stage(&outcome),
                    &linked,
                ),
                outcome,
                linked.clone(),
                contribution_digest,
                None,
                Vec::new(),
            );
        }
    };
    let linked = linked_artifacts_for_envelope(&declaration.envelope);
    let intent_results = process_contributions::<D, I>(
        declaration.target.clone(),
        declaration.declaration_aspect_record.clone(),
        contributions,
        materialization_policy_value,
        linked.clone(),
    );
    finalize_transcript(
        request_descriptor_value,
        request_digest_value,
        materialization_policy,
        declaration,
        linked,
        intent_results,
    )
}

fn finalize_transcript<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    request_descriptor: String,
    request_digest: String,
    materialization_policy: &'static str,
    declaration: DeclarationLowering<D, I>,
    linked: ForgeQueryBindingLinkedArtifacts,
    intent_results: Vec<ForgeQueryContributionComposedIntentResult>,
) -> ForgeQueryContributionComposedOrchestrationTranscript<D, I> {
    let declaration_record = ForgeQueryContributionComposedDeclarationRecord::from_envelope(
        &declaration.envelope,
        &linked,
    );
    match build_composed_artifact(declaration.envelope, intent_results.clone()) {
        Ok(composed) => {
            let composition_classification = Some(composed.classification());
            let contribution_digest = Some(composed.composition_for_reporting().to_string());
            ForgeQueryContributionComposedOrchestrationTranscript::new(
                request_descriptor,
                request_digest,
                materialization_policy,
                declaration_record,
                ForgeQueryContributionComposedOrchestrationOutcome::Bound(composed),
                linked,
                contribution_digest,
                composition_classification,
                intent_results,
            )
        }
        Err((stop, contribution_digest)) => {
            let (kind, reason) = stop_outcome_kind(stop, &intent_results);
            let outcome = composed_outcome(
                kind,
                stop,
                ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
                reason,
                linked.clone(),
                Some(contribution_digest.clone()),
                declaration_record.aspect_record().cloned(),
                primary_intent_descriptor(&intent_results).cloned(),
            );
            ForgeQueryContributionComposedOrchestrationTranscript::new(
                request_descriptor,
                request_digest,
                materialization_policy,
                declaration_record,
                outcome,
                linked,
                Some(contribution_digest),
                None,
                intent_results,
            )
        }
    }
}

fn empty_contribution_transcript<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    request_descriptor: String,
    request_digest: String,
    materialization_policy: &'static str,
) -> ForgeQueryContributionComposedOrchestrationTranscript<D, I> {
    let outcome = super::mapping::composed_outcome(
        ForgeQueryContributionComposedOrchestrationCheckedKind::Unsupported,
        super::composition::ForgeQueryContributionComposedStop::Unsupported,
        ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
        "contribution-composed orchestration requires at least one contribution intent",
        ForgeQueryBindingLinkedArtifacts::new(),
        None,
        None,
        None,
    );
    ForgeQueryContributionComposedOrchestrationTranscript::new(
        request_descriptor,
        request_digest,
        materialization_policy,
        ForgeQueryContributionComposedDeclarationRecord::from_linked_artifacts(
            ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            &ForgeQueryBindingLinkedArtifacts::new(),
        ),
        outcome,
        ForgeQueryBindingLinkedArtifacts::new(),
        None,
        None,
        Vec::new(),
    )
}

fn primary_intent_descriptor(
    intent_results: &[ForgeQueryContributionComposedIntentResult],
) -> Option<&super::intent_result::ForgeQueryContributionComposedIntentRequestDescriptor> {
    intent_results
        .iter()
        .find(|value| !value.is_admitted())
        .or_else(|| intent_results.first())
        .map(ForgeQueryContributionComposedIntentResult::request)
}

fn declaration_stage<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    outcome: &ForgeQueryContributionComposedOrchestrationOutcome<D, I>,
) -> ForgeQueryDeclarationEntryOrchestrationStage {
    match outcome {
        ForgeQueryContributionComposedOrchestrationOutcome::Bound(_) => {
            ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed
        }
        ForgeQueryContributionComposedOrchestrationOutcome::Deferred(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::DeclarationDenied(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::ContributionDenied(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::Stale(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::RebindRequired(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::Unsupported(value)
        | ForgeQueryContributionComposedOrchestrationOutcome::Failed(value) => value.stop_stage(),
    }
}

fn stop_outcome_kind(
    stop: super::composition::ForgeQueryContributionComposedStop,
    intent_results: &[ForgeQueryContributionComposedIntentResult],
) -> (
    ForgeQueryContributionComposedOrchestrationCheckedKind,
    String,
) {
    let kind = match stop {
        super::composition::ForgeQueryContributionComposedStop::Deferred => {
            ForgeQueryContributionComposedOrchestrationCheckedKind::Deferred
        }
        super::composition::ForgeQueryContributionComposedStop::DeclarationDenied => {
            ForgeQueryContributionComposedOrchestrationCheckedKind::DeclarationDenied
        }
        super::composition::ForgeQueryContributionComposedStop::ContributionDenied => {
            ForgeQueryContributionComposedOrchestrationCheckedKind::ContributionDenied
        }
        super::composition::ForgeQueryContributionComposedStop::Stale => {
            ForgeQueryContributionComposedOrchestrationCheckedKind::Stale
        }
        super::composition::ForgeQueryContributionComposedStop::RebindRequired => {
            ForgeQueryContributionComposedOrchestrationCheckedKind::RebindRequired
        }
        super::composition::ForgeQueryContributionComposedStop::Unsupported => {
            ForgeQueryContributionComposedOrchestrationCheckedKind::Unsupported
        }
        super::composition::ForgeQueryContributionComposedStop::Failed => {
            ForgeQueryContributionComposedOrchestrationCheckedKind::Failed
        }
    };
    (kind, stop_reason(stop, intent_results))
}
