use forge_proof::TransitionOutcome;

use crate::domain_capabilities::denials::{
    ForgeQueryDomainCapabilityProgressionDenial, ForgeQueryDomainCapabilityProgressionDenialKind,
};
use crate::domain_capabilities::payloads::{
    ForgeQueryContinuityContributionPayload, ForgeQueryContinuityContributionPosture,
    ForgeQueryContinuityRuntimeSemantics,
};
use crate::domain_capabilities::targets::{
    ForgeQueryAdmittedPlanBoundContributionTarget, ForgeQueryDomainCapabilityTargetBinding,
};
use crate::domain_capabilities::{
    ForgeQueryCanonicalContinuityArtifact, ForgeQueryDomainCapabilityTransitionOutcome,
    ForgeQueryMaterializationReadyContinuityContribution,
};
use crate::runtime::{ForgeQueryContinuityMutationEvidence, ForgeQueryContinuityMutationIntent};

pub fn materialize_canonical_continuity_artifact<T>(
    contribution: ForgeQueryMaterializationReadyContinuityContribution<T>,
) -> ForgeQueryCanonicalContinuityArtifact<T>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    super::artifacts::materialize_domain_capability_canonical_runtime_artifact(contribution)
}

pub fn materialize_runtime_continuity_evidence(
    contribution: ForgeQueryMaterializationReadyContinuityContribution<
        ForgeQueryAdmittedPlanBoundContributionTarget,
    >,
) -> ForgeQueryDomainCapabilityTransitionOutcome<ForgeQueryContinuityMutationEvidence> {
    let domain_contribution = contribution.payload();
    let payload = domain_contribution.payload();
    let binding_digest = domain_contribution.target().binding_digest().to_string();
    let Some(runtime_semantics) = payload.runtime_semantics() else {
        return TransitionOutcome::Denied(missing_runtime_semantics_denial(
            payload,
            domain_contribution.request_digest(),
        ));
    };
    if !runtime_semantics_match_posture(runtime_semantics, payload.posture()) {
        return TransitionOutcome::Denied(inconsistent_runtime_semantics_denial(
            payload,
            runtime_semantics,
            domain_contribution.request_digest(),
        ));
    }
    let Some(intent) = continuity_intent_from_runtime_semantics(runtime_semantics, payload) else {
        return TransitionOutcome::Denied(unsupported_posture_denial(
            payload,
            domain_contribution.request_digest(),
        ));
    };

    TransitionOutcome::Success(ForgeQueryContinuityMutationEvidence::from_intent(
        &intent,
        Some(&binding_digest),
        None,
        None,
    ))
}

fn runtime_semantics_match_posture(
    runtime_semantics: &ForgeQueryContinuityRuntimeSemantics,
    posture: ForgeQueryContinuityContributionPosture,
) -> bool {
    match posture {
        ForgeQueryContinuityContributionPosture::Preserved => {
            runtime_semantics.family()
                == crate::runtime::ForgeQueryContinuityMutationFamily::RebindExistingTarget
                && runtime_semantics.outcome_class()
                    == crate::runtime::ForgeQueryContinuityMutationOutcomeClass::ContinuesAsSingleSuccessor
        }
        ForgeQueryContinuityContributionPosture::Split => {
            runtime_semantics.family()
                == crate::runtime::ForgeQueryContinuityMutationFamily::SplitExistingTarget
                && runtime_semantics.outcome_class()
                    == crate::runtime::ForgeQueryContinuityMutationOutcomeClass::ContinuesAsSplitSuccessors
        }
        ForgeQueryContinuityContributionPosture::Replaced => {
            runtime_semantics.family()
                == crate::runtime::ForgeQueryContinuityMutationFamily::RebindExistingTarget
                && runtime_semantics.outcome_class()
                    == crate::runtime::ForgeQueryContinuityMutationOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor
        }
        ForgeQueryContinuityContributionPosture::CorrespondenceOnly => false,
    }
}

fn continuity_intent_from_runtime_semantics(
    runtime_semantics: &ForgeQueryContinuityRuntimeSemantics,
    payload: &ForgeQueryContinuityContributionPayload,
) -> Option<ForgeQueryContinuityMutationIntent> {
    match payload.posture() {
        ForgeQueryContinuityContributionPosture::Preserved => {
            ForgeQueryContinuityMutationIntent::rebind_existing_target(
                runtime_semantics.prior_authoritative_identity(),
                runtime_semantics
                    .successor_authoritative_identities()
                    .first()?
                    .as_str(),
            )
            .ok()
        }
        ForgeQueryContinuityContributionPosture::Split => {
            ForgeQueryContinuityMutationIntent::split_existing_target(
                runtime_semantics.prior_authoritative_identity(),
                runtime_semantics
                    .successor_authoritative_identities()
                    .iter()
                    .cloned(),
            )
            .ok()
        }
        ForgeQueryContinuityContributionPosture::Replaced => {
            ForgeQueryContinuityMutationIntent::rebind_merge_successor(
                runtime_semantics.prior_authoritative_identity(),
                runtime_semantics
                    .successor_authoritative_identities()
                    .first()?
                    .as_str(),
            )
            .ok()
        }
        ForgeQueryContinuityContributionPosture::CorrespondenceOnly => None,
    }
}

fn missing_runtime_semantics_denial(
    payload: &ForgeQueryContinuityContributionPayload,
    request_digest: &str,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::MissingCanonicalMaterializationSemantics,
        "continuity-lineage",
        crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind::AdmittedIntentPlan,
        request_digest,
        format!(
            "continuity runtime materialization requires continuity intent semantics for `{}`",
            payload.semantic_code()
        ),
    )
}

fn unsupported_posture_denial(
    payload: &ForgeQueryContinuityContributionPayload,
    request_digest: &str,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
        "continuity-lineage",
        crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind::AdmittedIntentPlan,
        request_digest,
        format!(
            "continuity runtime materialization does not support `{}` posture",
            payload.posture().as_str()
        ),
    )
}

fn inconsistent_runtime_semantics_denial(
    payload: &ForgeQueryContinuityContributionPayload,
    runtime_semantics: &ForgeQueryContinuityRuntimeSemantics,
    request_digest: &str,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::InconsistentCanonicalMaterializationSemantics,
        "continuity-lineage",
        crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind::AdmittedIntentPlan,
        request_digest,
        format!(
            "continuity runtime semantics `{}:{}` do not match `{}` posture",
            runtime_semantics.family().as_str(),
            runtime_semantics.outcome_class().as_str(),
            payload.posture().as_str()
        ),
    )
}
