use forge_foundational::FoundationalProfileSet;
use forge_proof::TransitionOutcome;

use crate::application::{
    ForgeQueryDeclarationEntryContributionComposition, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};
use crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts;
use crate::domain_capabilities::{
    admit_eligible_domain_capability_contribution,
    evaluate_requested_domain_capability_contribution, materialize_domain_capability_summary,
    prepare_admitted_domain_capability_contribution_for_materialization,
    ForgeQueryDeclarationBoundContributionTarget, ForgeQueryDomainCapabilityPayload,
    ForgeQueryDomainCapabilityTargetBinding, ForgeQueryRequestedDomainCapabilityContribution,
};
use crate::identity::hash_parts;

use super::artifact::{
    ForgeQueryContributionComposedContribution, ForgeQueryContributionComposedSummary,
};
use super::input::{
    ForgeQueryContributionComposedMaterializationPolicy,
    ForgeQueryContributionComposedOrchestrationInput, ForgeQueryContributionIntent,
};
use super::mapping::contribution_outcome;
use super::outcome::{
    ForgeQueryContributionComposedOrchestrationCheckedKind,
    ForgeQueryContributionComposedOrchestrationOutcome,
};

pub(super) fn request_digest<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    input: &ForgeQueryContributionComposedOrchestrationInput<D, I>,
) -> String {
    let mut parts = vec![
        format!("family:{}", I::Family::semantic_family_key()),
        format!(
            "declaration_entries:{:?}",
            input.declaration_input().canonical_declaration_entries()
        ),
        format!("contribution_count:{}", input.contributions().len()),
    ];
    parts.extend(
        input
            .contributions()
            .iter()
            .enumerate()
            .map(|(index, value)| format!("intent:{index}:{}:{value:?}", intent_label(value),)),
    );
    parts.push(format!(
        "materialization:{:?}",
        input.materialization_policy()
    ));
    hash_parts(&parts)
}

pub(super) fn process_contributions<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    target: ForgeQueryDeclarationBoundContributionTarget,
    contributions: Vec<ForgeQueryContributionIntent>,
    materialization_policy: ForgeQueryContributionComposedMaterializationPolicy,
    linked_artifacts: ForgeQueryBindingLinkedArtifacts,
) -> Result<
    (
        ForgeQueryDeclarationEntryContributionComposition,
        Vec<ForgeQueryContributionComposedContribution>,
    ),
    ForgeQueryContributionComposedOrchestrationOutcome<D, I>,
> {
    let processed = contributions
        .into_iter()
        .map(|value| {
            process_intent::<D, I>(
                target.clone(),
                value,
                materialization_profile(&materialization_policy),
                linked_artifacts.clone(),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let composition = ForgeQueryDeclarationEntryContributionComposition::new(
        processed
            .iter()
            .map(|value| value.evidence().clone())
            .collect(),
    );
    Ok((composition, processed))
}

fn process_intent<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    target: ForgeQueryDeclarationBoundContributionTarget,
    intent: ForgeQueryContributionIntent,
    materialization_profile: Option<&FoundationalProfileSet>,
    linked_artifacts: ForgeQueryBindingLinkedArtifacts,
) -> Result<
    ForgeQueryContributionComposedContribution,
    ForgeQueryContributionComposedOrchestrationOutcome<D, I>,
> {
    match intent {
        ForgeQueryContributionIntent::Admission(value) => process_requested::<D, I, _>(
            value.bind_to_declaration_target(target),
            materialization_profile,
            linked_artifacts,
        ),
        ForgeQueryContributionIntent::Support(value) => process_requested::<D, I, _>(
            value.bind_to_declaration_target(target),
            materialization_profile,
            linked_artifacts,
        ),
        ForgeQueryContributionIntent::Explanation(value) => process_requested::<D, I, _>(
            value.bind_to_declaration_target(target),
            materialization_profile,
            linked_artifacts,
        ),
        ForgeQueryContributionIntent::Workflow(value) => process_requested::<D, I, _>(
            value.bind_to_declaration_target(target),
            materialization_profile,
            linked_artifacts,
        ),
    }
}

fn process_requested<D, I, P>(
    requested: ForgeQueryRequestedDomainCapabilityContribution<
        P,
        ForgeQueryDeclarationBoundContributionTarget,
    >,
    materialization_profile: Option<&FoundationalProfileSet>,
    linked_artifacts: ForgeQueryBindingLinkedArtifacts,
) -> Result<
    ForgeQueryContributionComposedContribution,
    ForgeQueryContributionComposedOrchestrationOutcome<D, I>,
>
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
    let request_digest = requested.payload().request_digest().to_string();
    let eligible = match evaluate_requested_domain_capability_contribution(requested) {
        TransitionOutcome::Success(value) => value,
        TransitionOutcome::Denied(value) => return Err(contribution_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::ContributionDenied,
            crate::application::ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            value.message().to_string(),
            linked_artifacts,
            None,
        )),
        TransitionOutcome::Stale(value) => return Err(contribution_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::Stale,
            crate::application::ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            format!(
                "domain capability contribution is stale for category {}",
                value.category()
            ),
            linked_artifacts,
            None,
        )),
        TransitionOutcome::RebindRequired(value) => return Err(contribution_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::RebindRequired,
            crate::application::ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            format!(
                "domain capability contribution requires rebind for category {}",
                value.category()
            ),
            linked_artifacts,
            None,
        )),
        TransitionOutcome::Failed(value) => return Err(contribution_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::Failed,
            crate::application::ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            value.message().to_string(),
            linked_artifacts,
            None,
        )),
        TransitionOutcome::Deferred(never) => match never {},
    };
    let admitted = match admit_eligible_domain_capability_contribution(eligible) {
        TransitionOutcome::Success(value) => value,
        TransitionOutcome::Denied(value) => return Err(contribution_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::ContributionDenied,
            crate::application::ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            value.message().to_string(),
            linked_artifacts,
            None,
        )),
        TransitionOutcome::Stale(value) => return Err(contribution_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::Stale,
            crate::application::ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            format!(
                "domain capability contribution is stale for category {}",
                value.category()
            ),
            linked_artifacts,
            None,
        )),
        TransitionOutcome::RebindRequired(value) => return Err(contribution_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::RebindRequired,
            crate::application::ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            format!(
                "domain capability contribution requires rebind for category {}",
                value.category()
            ),
            linked_artifacts,
            None,
        )),
        TransitionOutcome::Failed(value) => return Err(contribution_outcome(
            ForgeQueryContributionComposedOrchestrationCheckedKind::Failed,
            crate::application::ForgeQueryDeclarationEntryOrchestrationStage::DeclarationReviewed,
            value.message().to_string(),
            linked_artifacts,
            None,
        )),
        TransitionOutcome::Deferred(never) => match never {},
    };
    let evidence = evidence_from_admitted(&admitted);
    let contribution_category = admitted.payload().payload().category();
    let semantic_posture = admitted.payload().payload().semantic_posture();
    let contribution_digest = Some(evidence.evidence_digest().to_string());
    let summary = if let Some(profile) = materialization_profile.cloned() {
        let current_target = admitted.payload().target().clone();
        let ready = match prepare_admitted_domain_capability_contribution_for_materialization(
            admitted,
            current_target,
        ) {
            TransitionOutcome::Success(value) => value,
            TransitionOutcome::Denied(value) => {
                return Err(contribution_outcome(
                    ForgeQueryContributionComposedOrchestrationCheckedKind::ContributionDenied,
                    crate::application::ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
                    value.message().to_string(),
                    linked_artifacts,
                    contribution_digest,
                ))
            }
            TransitionOutcome::Stale(value) => {
                return Err(contribution_outcome(
                    ForgeQueryContributionComposedOrchestrationCheckedKind::Stale,
                    crate::application::ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
                    format!(
                        "domain capability contribution is stale for category {}",
                        value.category()
                    ),
                    linked_artifacts,
                    contribution_digest,
                ))
            }
            TransitionOutcome::RebindRequired(value) => {
                return Err(contribution_outcome(
                    ForgeQueryContributionComposedOrchestrationCheckedKind::RebindRequired,
                    crate::application::ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
                    format!(
                        "domain capability contribution requires rebind for category {}",
                        value.category()
                    ),
                    linked_artifacts,
                    contribution_digest,
                ))
            }
            TransitionOutcome::Failed(value) => {
                return Err(contribution_outcome(
                    ForgeQueryContributionComposedOrchestrationCheckedKind::Failed,
                    crate::application::ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
                    value.message().to_string(),
                    linked_artifacts,
                    contribution_digest,
                ))
            }
            TransitionOutcome::Deferred(never) => match never {},
        };
        let materialization_ready_digest = ready.materialization_ready_digest().to_string();
        match materialize_domain_capability_summary(ready, profile) {
            Ok(value) => Some(ForgeQueryContributionComposedSummary::new(
                materialization_ready_digest,
                value.outcome_kind(),
                format!("{:?}", value.primary_code()),
                value.required_row_count(),
                value.standard_row_count(),
                value.forensic_row_count(),
            )),
            Err(value) => {
                return Err(contribution_outcome(
                    ForgeQueryContributionComposedOrchestrationCheckedKind::Failed,
                    crate::application::ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
                    format!("{value:?}"),
                    linked_artifacts,
                    contribution_digest,
                ))
            }
        }
    } else {
        None
    };
    Ok(ForgeQueryContributionComposedContribution::new(
        evidence,
        contribution_category,
        semantic_posture,
        request_digest,
        summary,
    ))
}

fn intent_label(intent: &ForgeQueryContributionIntent) -> &'static str {
    match intent {
        ForgeQueryContributionIntent::Admission(_) => "admission",
        ForgeQueryContributionIntent::Support(_) => "support",
        ForgeQueryContributionIntent::Explanation(_) => "explanation",
        ForgeQueryContributionIntent::Workflow(_) => "workflow",
    }
}

fn materialization_profile(
    policy: &ForgeQueryContributionComposedMaterializationPolicy,
) -> Option<&FoundationalProfileSet> {
    match policy {
        ForgeQueryContributionComposedMaterializationPolicy::None => None,
        ForgeQueryContributionComposedMaterializationPolicy::Summary(value) => Some(value),
    }
}

fn evidence_from_admitted<P>(
    admitted: &crate::domain_capabilities::ForgeQueryAdmittedDomainCapabilityContribution<
        P,
        ForgeQueryDeclarationBoundContributionTarget,
    >,
) -> crate::application::ForgeQueryDeclarationEntryContributionEvidence
where
    P: ForgeQueryDomainCapabilityPayload,
{
    let payload = admitted.payload().payload();
    let record = crate::application::ForgeQueryDeclarationEntryContributionEvidenceRecord {
        target_family:
            crate::application::ForgeQueryDeclarationEntryContributionTargetFamily::DeclarationBound,
        target_digest: admitted.payload().target().target_digest().to_string(),
        target_binding_digest: admitted.payload().target().binding_digest().to_string(),
        evidence_digest: admitted.admitted_digest(),
        posture_label: payload.posture_label().to_string(),
        semantic_code: payload.semantic_code().to_string(),
        detail: payload.detail().to_string(),
        decision_stage: None,
    };
    match payload.category() {
        crate::domain_capabilities::ForgeQueryDomainCapabilityCategory::Admission => {
            crate::application::ForgeQueryDeclarationEntryContributionEvidence::Admission(record)
        }
        crate::domain_capabilities::ForgeQueryDomainCapabilityCategory::SupportTraceability => {
            crate::application::ForgeQueryDeclarationEntryContributionEvidence::SupportTraceability(
                record,
            )
        }
        crate::domain_capabilities::ForgeQueryDomainCapabilityCategory::ExplanationInspection => {
            crate::application::ForgeQueryDeclarationEntryContributionEvidence::ExplanationInspection(
                record,
            )
        }
        crate::domain_capabilities::ForgeQueryDomainCapabilityCategory::WorkflowPreview => {
            crate::application::ForgeQueryDeclarationEntryContributionEvidence::WorkflowPreview(
                record,
            )
        }
        crate::domain_capabilities::ForgeQueryDomainCapabilityCategory::ContinuityLineage => {
            crate::application::ForgeQueryDeclarationEntryContributionEvidence::ContinuityLineage(
                record,
            )
        }
        crate::domain_capabilities::ForgeQueryDomainCapabilityCategory::ConsequenceAftermath => {
            crate::application::ForgeQueryDeclarationEntryContributionEvidence::ConsequenceAftermath(
                record,
            )
        }
        crate::domain_capabilities::ForgeQueryDomainCapabilityCategory::InvariantCapability => {
            crate::application::ForgeQueryDeclarationEntryContributionEvidence::SupportTraceability(
                record,
            )
        }
    }
}
