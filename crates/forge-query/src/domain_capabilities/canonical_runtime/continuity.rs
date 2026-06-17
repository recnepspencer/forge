use forge_proof::TransitionOutcome;

use crate::domain_capabilities::denials::{
    ForgeQueryDomainCapabilityProgressionDenial, ForgeQueryDomainCapabilityProgressionDenialKind,
};
use crate::domain_capabilities::identity::domain_capability_scope_encoder;
use crate::domain_capabilities::payloads::{
    ForgeQueryContinuityContributionPayload, ForgeQueryContinuityContributionPosture,
    ForgeQueryContinuityRuntimeSemantics,
};
use crate::domain_capabilities::targets::{
    ForgeQueryAdmittedPlanBoundContributionTarget, ForgeQueryDomainCapabilityTargetBinding,
    ForgeQueryDomainCapabilityTargetKind,
};
use crate::domain_capabilities::{
    ForgeQueryCanonicalContinuityArtifact, ForgeQueryDomainCapabilityTransitionOutcome,
    ForgeQueryMaterializationReadyContinuityContribution,
};
use crate::runtime::{ForgeQueryContinuityMutationEvidence, ForgeQueryContinuityMutationIntent};
use crate::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceTag};

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
    let binding_identity = domain_capability_continuity_binding_identity(
        domain_contribution.target().kind(),
        &domain_contribution.target().binding_identity(),
    );
    let Some(runtime_semantics) = payload.runtime_semantics() else {
        return TransitionOutcome::Denied(missing_runtime_semantics_denial(
            payload,
            domain_contribution.request_identity().clone(),
        ));
    };
    if !runtime_semantics_match_posture(runtime_semantics, payload.posture()) {
        return TransitionOutcome::Denied(inconsistent_runtime_semantics_denial(
            payload,
            runtime_semantics,
            domain_contribution.request_identity().clone(),
        ));
    }
    let Some(intent) = continuity_intent_from_runtime_semantics(runtime_semantics, payload) else {
        return TransitionOutcome::Denied(unsupported_posture_denial(
            payload,
            domain_contribution.request_identity().clone(),
        ));
    };

    TransitionOutcome::Success(ForgeQueryContinuityMutationEvidence::from_intent(
        &intent,
        Some(&binding_identity),
        None,
        None,
    ))
}

fn domain_capability_continuity_binding_identity(
    target_kind: ForgeQueryDomainCapabilityTargetKind,
    binding_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    domain_capability_scope_encoder("domain_capability_continuity_binding_v1")
        .field_shape(
            ForgeQueryEvidenceTag::new("target_kind"),
            target_kind.as_str(),
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("binding"), binding_identity)
        .seal()
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
    let prior_authority =
        crate::runtime::ForgeQueryMutationAuthorityIdentity::continuity_prior_authority(
            crate::runtime::ForgeQueryContinuityPriorAuthorityLabel::new(
                runtime_semantics.prior_authoritative_source_label_for_reporting(),
            )
            .ok()?,
        )
        .ok()?;
    match payload.posture() {
        ForgeQueryContinuityContributionPosture::Preserved => {
            let successor_authority = crate::runtime::ForgeQueryMutationAuthorityIdentity::continuity_successor_authority(
                crate::runtime::ForgeQueryContinuitySuccessorAuthorityLabel::new(
                    runtime_semantics
                        .successor_authoritative_source_labels_for_reporting()
                        .first()?,
                )
                .ok()?,
            )
            .ok()?;
            ForgeQueryContinuityMutationIntent::rebind_existing_target(
                prior_authority,
                successor_authority,
            )
            .ok()
        }
        ForgeQueryContinuityContributionPosture::Split => {
            let successor_authorities = runtime_semantics
                .successor_authoritative_source_labels_for_reporting()
                .iter()
                .map(|label| {
                    crate::runtime::ForgeQueryMutationAuthorityIdentity::continuity_successor_authority(
                        crate::runtime::ForgeQueryContinuitySuccessorAuthorityLabel::new(label)?,
                    )
                })
                .collect::<Result<Vec<_>, _>>()
                .ok()?;
            ForgeQueryContinuityMutationIntent::split_existing_target(
                prior_authority,
                successor_authorities,
            )
            .ok()
        }
        ForgeQueryContinuityContributionPosture::Replaced => {
            let successor_authority = crate::runtime::ForgeQueryMutationAuthorityIdentity::continuity_successor_authority(
                crate::runtime::ForgeQueryContinuitySuccessorAuthorityLabel::new(
                    runtime_semantics
                        .successor_authoritative_source_labels_for_reporting()
                        .first()?,
                )
                .ok()?,
            )
            .ok()?;
            ForgeQueryContinuityMutationIntent::rebind_merge_successor(
                prior_authority,
                successor_authority,
            )
            .ok()
        }
        ForgeQueryContinuityContributionPosture::CorrespondenceOnly => None,
    }
}

fn missing_runtime_semantics_denial(
    payload: &ForgeQueryContinuityContributionPayload,
    request_identity: ForgeQueryEvidenceIdentity,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::MissingCanonicalMaterializationSemantics,
        "continuity-lineage",
        crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind::AdmittedIntentPlan,
        request_identity,
        format!(
            "continuity runtime materialization requires continuity intent semantics for `{}`",
            payload.semantic_code()
        ),
    )
}

fn unsupported_posture_denial(
    payload: &ForgeQueryContinuityContributionPayload,
    request_identity: ForgeQueryEvidenceIdentity,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
        "continuity-lineage",
        crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind::AdmittedIntentPlan,
        request_identity,
        format!(
            "continuity runtime materialization does not support `{}` posture",
            payload.posture().as_str()
        ),
    )
}

fn inconsistent_runtime_semantics_denial(
    payload: &ForgeQueryContinuityContributionPayload,
    runtime_semantics: &ForgeQueryContinuityRuntimeSemantics,
    request_identity: ForgeQueryEvidenceIdentity,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::InconsistentCanonicalMaterializationSemantics,
        "continuity-lineage",
        crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind::AdmittedIntentPlan,
        request_identity,
        format!(
            "continuity runtime semantics `{}:{}` do not match `{}` posture",
            runtime_semantics.family().as_str(),
            runtime_semantics.outcome_class().as_str(),
            payload.posture().as_str()
        ),
    )
}
