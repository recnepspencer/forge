use forge_foundational::FoundationalProfileSet;
use forge_proof::TransitionOutcome;

use crate::application::{
    ForgeQueryDeclarationEntryContributionCategoryFamily, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker,
};
use crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts;
use crate::domain_capabilities::{
    admit_eligible_domain_capability_contribution,
    evaluate_requested_domain_capability_contribution, materialize_domain_capability_summary,
    prepare_admitted_domain_capability_contribution_for_materialization,
    ForgeQueryDeclarationBoundContributionTarget, ForgeQueryDomainCapabilityPayload,
    ForgeQueryRequestedDomainCapabilityContribution,
};
use crate::target_binding::ForgeQueryBindingTargetWitness;

use super::super::artifact::{
    ForgeQueryContributionComposedContribution, ForgeQueryContributionComposedSummary,
};
use super::super::aspect::ForgeQueryContributionComposedIntentAspectRecord;
use super::super::input::{
    ForgeQueryContributionComposedMaterializationPolicy, ForgeQueryContributionIntent,
};
use super::super::intent_result::{
    ForgeQueryContributionComposedIntentClassification,
    ForgeQueryContributionComposedIntentRequestDescriptor,
    ForgeQueryContributionComposedIntentResult, ForgeQueryContributionComposedIntentStageResult,
};
use super::support::{
    denied_result, evidence_from_admitted, failed_result, rebind_required_result,
    retained_after_admission_result, stale_result,
};

pub(crate) fn process_contributions<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    target: ForgeQueryDeclarationBoundContributionTarget,
    declaration_aspect_record: ForgeQueryContributionComposedIntentAspectRecord,
    contributions: Vec<ForgeQueryContributionIntent>,
    materialization_policy: ForgeQueryContributionComposedMaterializationPolicy,
    linked_artifacts: ForgeQueryBindingLinkedArtifacts,
) -> Vec<ForgeQueryContributionComposedIntentResult> {
    contributions
        .into_iter()
        .enumerate()
        .map(|(order_index, value)| {
            process_intent::<D, I>(
                order_index,
                target.clone(),
                declaration_aspect_record.clone(),
                value,
                materialization_profile(&materialization_policy),
                linked_artifacts.clone(),
            )
        })
        .collect()
}

fn process_intent<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    order_index: usize,
    target: ForgeQueryDeclarationBoundContributionTarget,
    declaration_aspect_record: ForgeQueryContributionComposedIntentAspectRecord,
    intent: ForgeQueryContributionIntent,
    materialization_profile: Option<&FoundationalProfileSet>,
    linked_artifacts: ForgeQueryBindingLinkedArtifacts,
) -> ForgeQueryContributionComposedIntentResult {
    match intent {
        ForgeQueryContributionIntent::Admission(value) => process_requested::<D, I, _>(
            order_index,
            ForgeQueryDeclarationEntryContributionCategoryFamily::Admission,
            declaration_aspect_record,
            value.bind_to_declaration_target(target),
            materialization_profile,
            linked_artifacts,
        ),
        ForgeQueryContributionIntent::Support(value) => process_requested::<D, I, _>(
            order_index,
            ForgeQueryDeclarationEntryContributionCategoryFamily::SupportTraceability,
            declaration_aspect_record,
            value.bind_to_declaration_target(target),
            materialization_profile,
            linked_artifacts,
        ),
        ForgeQueryContributionIntent::Explanation(value) => process_requested::<D, I, _>(
            order_index,
            ForgeQueryDeclarationEntryContributionCategoryFamily::ExplanationInspection,
            declaration_aspect_record,
            value.bind_to_declaration_target(target),
            materialization_profile,
            linked_artifacts,
        ),
        ForgeQueryContributionIntent::Workflow(value) => process_requested::<D, I, _>(
            order_index,
            ForgeQueryDeclarationEntryContributionCategoryFamily::WorkflowPreview,
            declaration_aspect_record,
            value.bind_to_declaration_target(target),
            materialization_profile,
            linked_artifacts,
        ),
        ForgeQueryContributionIntent::Continuity(value) => process_requested::<D, I, _>(
            order_index,
            ForgeQueryDeclarationEntryContributionCategoryFamily::ContinuityLineage,
            declaration_aspect_record,
            value.bind_to_declaration_target(target),
            materialization_profile,
            linked_artifacts,
        ),
    }
}

fn process_requested<D, I, P>(
    order_index: usize,
    category_family: ForgeQueryDeclarationEntryContributionCategoryFamily,
    declaration_aspect_record: ForgeQueryContributionComposedIntentAspectRecord,
    requested: ForgeQueryRequestedDomainCapabilityContribution<
        P,
        ForgeQueryDeclarationBoundContributionTarget,
    >,
    materialization_profile: Option<&FoundationalProfileSet>,
    _linked_artifacts: ForgeQueryBindingLinkedArtifacts,
) -> ForgeQueryContributionComposedIntentResult
where
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
    P: ForgeQueryDomainCapabilityPayload,
    (P, ForgeQueryDeclarationBoundContributionTarget):
        crate::domain_capabilities::AllowedContributionBinding<
            P,
            ForgeQueryDeclarationBoundContributionTarget,
        >,
{
    let request = ForgeQueryContributionComposedIntentRequestDescriptor::new(
        order_index,
        category_family,
        requested.payload().request_identity().clone(),
        requested.payload().target().binding_identity(),
        requested.payload().payload().semantic_code(),
        requested.payload().payload().detail(),
        declaration_aspect_record,
    );
    let evaluation = ForgeQueryContributionComposedIntentStageResult::succeeded(
        "contribution request evaluated",
        Some(requested.requested_identity()),
    );
    let eligible = match evaluate_requested_domain_capability_contribution(requested) {
        TransitionOutcome::Success(value) => value,
        TransitionOutcome::Denied(value) => {
            return denied_result(
                request,
                ForgeQueryContributionComposedIntentStageResult::denied(value.message()),
            )
        }
        TransitionOutcome::Stale(value) => {
            return stale_result(
                request,
                format!(
                    "target {} no longer matches current {}",
                    value.bound_target_for_reporting(),
                    value.current_target_for_reporting()
                ),
            )
        }
        TransitionOutcome::RebindRequired(value) => {
            return rebind_required_result(
                request,
                format!(
                    "target {} requires rebind to {}",
                    value.bound_target_for_reporting(),
                    value.current_target_for_reporting()
                ),
            )
        }
        TransitionOutcome::Failed(value) => {
            return failed_result(request, value.message().to_string())
        }
        TransitionOutcome::Deferred(never) => match never {},
    };
    let _eligibility = ForgeQueryContributionComposedIntentStageResult::succeeded(
        "contribution request is eligible for admission",
        Some(eligible.eligibility_identity()),
    );
    let admitted = match admit_eligible_domain_capability_contribution(eligible) {
        TransitionOutcome::Success(value) => value,
        TransitionOutcome::Denied(value) => {
            return ForgeQueryContributionComposedIntentResult::new(
                request,
                evaluation,
                ForgeQueryContributionComposedIntentStageResult::denied(value.message()),
                ForgeQueryContributionComposedIntentStageResult::not_attempted(),
                ForgeQueryContributionComposedIntentClassification::Denied,
                None,
            )
        }
        TransitionOutcome::Stale(value) => {
            return ForgeQueryContributionComposedIntentResult::new(
                request,
                evaluation,
                ForgeQueryContributionComposedIntentStageResult::stale(format!(
                    "target {} no longer matches current {}",
                    value.bound_target_for_reporting(),
                    value.current_target_for_reporting()
                )),
                ForgeQueryContributionComposedIntentStageResult::not_attempted(),
                ForgeQueryContributionComposedIntentClassification::Stale,
                None,
            )
        }
        TransitionOutcome::RebindRequired(value) => {
            return ForgeQueryContributionComposedIntentResult::new(
                request,
                evaluation,
                ForgeQueryContributionComposedIntentStageResult::rebind_required(format!(
                    "target {} requires rebind to {}",
                    value.bound_target_for_reporting(),
                    value.current_target_for_reporting()
                )),
                ForgeQueryContributionComposedIntentStageResult::not_attempted(),
                ForgeQueryContributionComposedIntentClassification::RebindRequired,
                None,
            )
        }
        TransitionOutcome::Failed(value) => {
            return ForgeQueryContributionComposedIntentResult::new(
                request,
                evaluation,
                ForgeQueryContributionComposedIntentStageResult::failed(value.message()),
                ForgeQueryContributionComposedIntentStageResult::not_attempted(),
                ForgeQueryContributionComposedIntentClassification::Failed,
                None,
            )
        }
        TransitionOutcome::Deferred(never) => match never {},
    };
    let evidence = evidence_from_admitted(&admitted);
    let contribution_category = admitted.payload().payload().category();
    let semantic_posture = admitted.payload().payload().semantic_posture();
    let contribution = ForgeQueryContributionComposedContribution::new(
        evidence,
        contribution_category,
        semantic_posture,
        request.request_digest().to_string(),
        None,
    );
    let admitted_digest = admitted.admitted_identity();
    let admission_stage = ForgeQueryContributionComposedIntentStageResult::succeeded(
        "contribution admitted",
        Some(admitted_digest),
    );
    if let Some(profile) = materialization_profile.cloned() {
        let current_target = admitted.payload().target().clone();
        let ready = match prepare_admitted_domain_capability_contribution_for_materialization(
            admitted,
            current_target,
        ) {
            TransitionOutcome::Success(value) => value,
            TransitionOutcome::Denied(value) => {
                return retained_after_admission_result(
                    request,
                    evaluation,
                    admission_stage,
                    ForgeQueryContributionComposedIntentStageResult::denied(value.message()),
                    contribution,
                )
            }
            TransitionOutcome::Stale(value) => {
                return retained_after_admission_result(
                    request,
                    evaluation,
                    admission_stage,
                    ForgeQueryContributionComposedIntentStageResult::stale(format!(
                        "target {} no longer matches current {}",
                        value.bound_target_for_reporting(),
                        value.current_target_for_reporting()
                    )),
                    contribution,
                )
            }
            TransitionOutcome::RebindRequired(value) => {
                return retained_after_admission_result(
                    request,
                    evaluation,
                    admission_stage,
                    ForgeQueryContributionComposedIntentStageResult::rebind_required(format!(
                        "target {} requires rebind to {}",
                        value.bound_target_for_reporting(),
                        value.current_target_for_reporting()
                    )),
                    contribution,
                )
            }
            TransitionOutcome::Failed(value) => {
                return retained_after_admission_result(
                    request,
                    evaluation,
                    admission_stage,
                    ForgeQueryContributionComposedIntentStageResult::failed(value.message()),
                    contribution,
                )
            }
            TransitionOutcome::Deferred(never) => match never {},
        };
        let ready_identity = ready.materialization_ready_identity();
        return match materialize_domain_capability_summary(ready, profile) {
            Ok(value) => {
                let summary = ForgeQueryContributionComposedSummary::new(
                    ready_identity.as_str().to_string(),
                    value.outcome_kind(),
                    format!("{:?}", value.primary_code()),
                    value.required_row_count(),
                    value.standard_row_count(),
                    value.forensic_row_count(),
                );
                let contribution = ForgeQueryContributionComposedContribution::new(
                    contribution.evidence().clone(),
                    contribution.contribution_category(),
                    contribution.semantic_posture(),
                    contribution.request_digest().to_string(),
                    Some(summary),
                );
                ForgeQueryContributionComposedIntentResult::new(
                    request,
                    evaluation,
                    admission_stage,
                    ForgeQueryContributionComposedIntentStageResult::succeeded(
                        "contribution summary materialized",
                        Some(ready_identity),
                    ),
                    ForgeQueryContributionComposedIntentClassification::Admitted,
                    Some(contribution),
                )
            }
            Err(value) => retained_after_admission_result(
                request,
                evaluation,
                admission_stage,
                ForgeQueryContributionComposedIntentStageResult::unsupported(format!("{value:?}")),
                contribution,
            ),
        };
    }
    ForgeQueryContributionComposedIntentResult::new(
        request,
        evaluation,
        admission_stage,
        ForgeQueryContributionComposedIntentStageResult::not_attempted(),
        ForgeQueryContributionComposedIntentClassification::Admitted,
        Some(contribution),
    )
}

fn materialization_profile(
    policy: &ForgeQueryContributionComposedMaterializationPolicy,
) -> Option<&FoundationalProfileSet> {
    match policy {
        ForgeQueryContributionComposedMaterializationPolicy::None => None,
        ForgeQueryContributionComposedMaterializationPolicy::Summary(value) => Some(value),
    }
}
