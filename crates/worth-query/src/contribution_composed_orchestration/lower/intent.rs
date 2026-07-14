use worth_foundational::FoundationalProfileSet;
use worth_proof::TransitionOutcome;

use crate::application::{
    WorthQueryDeclarationEntryContributionCategoryFamily, WorthQueryDeclarationInput,
    WorthQueryDomainEntryMarker,
};
use crate::binding_pipeline::WorthQueryBindingLinkedArtifacts;
use crate::domain_capabilities::{
    admit_eligible_domain_capability_contribution,
    evaluate_requested_domain_capability_contribution, materialize_domain_capability_summary,
    prepare_admitted_domain_capability_contribution_for_materialization,
    WorthQueryDeclarationBoundContributionTarget, WorthQueryDomainCapabilityPayload,
    WorthQueryRequestedDomainCapabilityContribution,
};
use crate::target_binding::WorthQueryBindingTargetWitness;

use super::super::artifact::{
    WorthQueryContributionComposedContribution, WorthQueryContributionComposedSummary,
};
use super::super::aspect::WorthQueryContributionComposedIntentAspectRecord;
use super::super::input::{
    WorthQueryContributionComposedMaterializationPolicy, WorthQueryContributionIntent,
};
use super::super::intent_result::{
    WorthQueryContributionComposedIntentClassification,
    WorthQueryContributionComposedIntentRequestDescriptor,
    WorthQueryContributionComposedIntentResult, WorthQueryContributionComposedIntentStageResult,
};
use super::support::{
    denied_result, evidence_from_admitted, failed_result, rebind_required_result,
    retained_after_admission_result, stale_result,
};

pub(crate) fn process_contributions<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    target: WorthQueryDeclarationBoundContributionTarget,
    declaration_aspect_record: WorthQueryContributionComposedIntentAspectRecord,
    contributions: Vec<WorthQueryContributionIntent>,
    materialization_policy: WorthQueryContributionComposedMaterializationPolicy,
    linked_artifacts: WorthQueryBindingLinkedArtifacts,
) -> Vec<WorthQueryContributionComposedIntentResult> {
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

fn process_intent<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
    order_index: usize,
    target: WorthQueryDeclarationBoundContributionTarget,
    declaration_aspect_record: WorthQueryContributionComposedIntentAspectRecord,
    intent: WorthQueryContributionIntent,
    materialization_profile: Option<&FoundationalProfileSet>,
    linked_artifacts: WorthQueryBindingLinkedArtifacts,
) -> WorthQueryContributionComposedIntentResult {
    match intent {
        WorthQueryContributionIntent::Admission(value) => process_requested::<D, I, _>(
            order_index,
            WorthQueryDeclarationEntryContributionCategoryFamily::Admission,
            declaration_aspect_record,
            value.bind_to_declaration_target(target),
            materialization_profile,
            linked_artifacts,
        ),
        WorthQueryContributionIntent::Support(value) => process_requested::<D, I, _>(
            order_index,
            WorthQueryDeclarationEntryContributionCategoryFamily::SupportTraceability,
            declaration_aspect_record,
            value.bind_to_declaration_target(target),
            materialization_profile,
            linked_artifacts,
        ),
        WorthQueryContributionIntent::Explanation(value) => process_requested::<D, I, _>(
            order_index,
            WorthQueryDeclarationEntryContributionCategoryFamily::ExplanationInspection,
            declaration_aspect_record,
            value.bind_to_declaration_target(target),
            materialization_profile,
            linked_artifacts,
        ),
        WorthQueryContributionIntent::Workflow(value) => process_requested::<D, I, _>(
            order_index,
            WorthQueryDeclarationEntryContributionCategoryFamily::WorkflowPreview,
            declaration_aspect_record,
            value.bind_to_declaration_target(target),
            materialization_profile,
            linked_artifacts,
        ),
        WorthQueryContributionIntent::Continuity(value) => process_requested::<D, I, _>(
            order_index,
            WorthQueryDeclarationEntryContributionCategoryFamily::ContinuityLineage,
            declaration_aspect_record,
            value.bind_to_declaration_target(target),
            materialization_profile,
            linked_artifacts,
        ),
    }
}

fn process_requested<D, I, P>(
    order_index: usize,
    category_family: WorthQueryDeclarationEntryContributionCategoryFamily,
    declaration_aspect_record: WorthQueryContributionComposedIntentAspectRecord,
    requested: WorthQueryRequestedDomainCapabilityContribution<
        P,
        WorthQueryDeclarationBoundContributionTarget,
    >,
    materialization_profile: Option<&FoundationalProfileSet>,
    _linked_artifacts: WorthQueryBindingLinkedArtifacts,
) -> WorthQueryContributionComposedIntentResult
where
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
    P: WorthQueryDomainCapabilityPayload,
    (P, WorthQueryDeclarationBoundContributionTarget):
        crate::domain_capabilities::AllowedContributionBinding<
            P,
            WorthQueryDeclarationBoundContributionTarget,
        >,
{
    let request = WorthQueryContributionComposedIntentRequestDescriptor::new(
        order_index,
        category_family,
        requested.payload().request_identity().clone(),
        requested.payload().target().binding_identity(),
        requested.payload().payload().semantic_code(),
        requested.payload().payload().detail(),
        declaration_aspect_record,
    );
    let evaluation = WorthQueryContributionComposedIntentStageResult::succeeded(
        "contribution request evaluated",
        Some(requested.requested_identity()),
    );
    let eligible = match evaluate_requested_domain_capability_contribution(requested) {
        TransitionOutcome::Success(value) => value,
        TransitionOutcome::Denied(value) => {
            return denied_result(
                request,
                WorthQueryContributionComposedIntentStageResult::denied(value.message()),
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
    let _eligibility = WorthQueryContributionComposedIntentStageResult::succeeded(
        "contribution request is eligible for admission",
        Some(eligible.eligibility_identity()),
    );
    let admitted = match admit_eligible_domain_capability_contribution(eligible) {
        TransitionOutcome::Success(value) => value,
        TransitionOutcome::Denied(value) => {
            return WorthQueryContributionComposedIntentResult::new(
                request,
                evaluation,
                WorthQueryContributionComposedIntentStageResult::denied(value.message()),
                WorthQueryContributionComposedIntentStageResult::not_attempted(),
                WorthQueryContributionComposedIntentClassification::Denied,
                None,
            )
        }
        TransitionOutcome::Stale(value) => {
            return WorthQueryContributionComposedIntentResult::new(
                request,
                evaluation,
                WorthQueryContributionComposedIntentStageResult::stale(format!(
                    "target {} no longer matches current {}",
                    value.bound_target_for_reporting(),
                    value.current_target_for_reporting()
                )),
                WorthQueryContributionComposedIntentStageResult::not_attempted(),
                WorthQueryContributionComposedIntentClassification::Stale,
                None,
            )
        }
        TransitionOutcome::RebindRequired(value) => {
            return WorthQueryContributionComposedIntentResult::new(
                request,
                evaluation,
                WorthQueryContributionComposedIntentStageResult::rebind_required(format!(
                    "target {} requires rebind to {}",
                    value.bound_target_for_reporting(),
                    value.current_target_for_reporting()
                )),
                WorthQueryContributionComposedIntentStageResult::not_attempted(),
                WorthQueryContributionComposedIntentClassification::RebindRequired,
                None,
            )
        }
        TransitionOutcome::Failed(value) => {
            return WorthQueryContributionComposedIntentResult::new(
                request,
                evaluation,
                WorthQueryContributionComposedIntentStageResult::failed(value.message()),
                WorthQueryContributionComposedIntentStageResult::not_attempted(),
                WorthQueryContributionComposedIntentClassification::Failed,
                None,
            )
        }
        TransitionOutcome::Deferred(never) => match never {},
    };
    let evidence = evidence_from_admitted(&admitted);
    let contribution_category = admitted.payload().payload().category();
    let semantic_posture = admitted.payload().payload().semantic_posture();
    let contribution = WorthQueryContributionComposedContribution::new(
        evidence,
        contribution_category,
        semantic_posture,
        request.request_digest().to_string(),
        None,
    );
    let admitted_digest = admitted.admitted_identity();
    let admission_stage = WorthQueryContributionComposedIntentStageResult::succeeded(
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
                    WorthQueryContributionComposedIntentStageResult::denied(value.message()),
                    contribution,
                )
            }
            TransitionOutcome::Stale(value) => {
                return retained_after_admission_result(
                    request,
                    evaluation,
                    admission_stage,
                    WorthQueryContributionComposedIntentStageResult::stale(format!(
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
                    WorthQueryContributionComposedIntentStageResult::rebind_required(format!(
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
                    WorthQueryContributionComposedIntentStageResult::failed(value.message()),
                    contribution,
                )
            }
            TransitionOutcome::Deferred(never) => match never {},
        };
        let ready_identity = ready.materialization_ready_identity();
        return match materialize_domain_capability_summary(ready, profile) {
            Ok(value) => {
                let summary = WorthQueryContributionComposedSummary::new(
                    ready_identity.as_str().to_string(),
                    value.outcome_kind(),
                    format!("{:?}", value.primary_code()),
                    value.required_row_count(),
                    value.standard_row_count(),
                    value.forensic_row_count(),
                );
                let contribution = WorthQueryContributionComposedContribution::new(
                    contribution.evidence().clone(),
                    contribution.contribution_category(),
                    contribution.semantic_posture(),
                    contribution.request_digest().to_string(),
                    Some(summary),
                );
                WorthQueryContributionComposedIntentResult::new(
                    request,
                    evaluation,
                    admission_stage,
                    WorthQueryContributionComposedIntentStageResult::succeeded(
                        "contribution summary materialized",
                        Some(ready_identity),
                    ),
                    WorthQueryContributionComposedIntentClassification::Admitted,
                    Some(contribution),
                )
            }
            Err(value) => retained_after_admission_result(
                request,
                evaluation,
                admission_stage,
                WorthQueryContributionComposedIntentStageResult::unsupported(format!("{value:?}")),
                contribution,
            ),
        };
    }
    WorthQueryContributionComposedIntentResult::new(
        request,
        evaluation,
        admission_stage,
        WorthQueryContributionComposedIntentStageResult::not_attempted(),
        WorthQueryContributionComposedIntentClassification::Admitted,
        Some(contribution),
    )
}

fn materialization_profile(
    policy: &WorthQueryContributionComposedMaterializationPolicy,
) -> Option<&FoundationalProfileSet> {
    match policy {
        WorthQueryContributionComposedMaterializationPolicy::None => None,
        WorthQueryContributionComposedMaterializationPolicy::Summary(value) => Some(value),
    }
}
