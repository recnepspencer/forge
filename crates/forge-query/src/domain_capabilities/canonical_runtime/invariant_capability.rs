use forge_proof::TransitionOutcome;

use crate::domain_capabilities::denials::{
    ForgeQueryDomainCapabilityProgressionDenial, ForgeQueryDomainCapabilityProgressionDenialKind,
};
use crate::domain_capabilities::payloads::{
    ForgeQueryGraphInvariantDenialRuntimeSemantics,
    ForgeQueryInvariantCapabilityContributionPayload,
    ForgeQueryInvariantCapabilityContributionPosture,
};
use crate::domain_capabilities::targets::{
    ForgeQueryDomainCapabilityTargetBinding, ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
};
use crate::domain_capabilities::{
    ForgeQueryCanonicalInvariantCapabilityArtifact, ForgeQueryDomainCapabilityTransitionOutcome,
    ForgeQueryMaterializationReadyInvariantCapabilityContribution,
};
use crate::runtime::ForgeQueryGraphCompositionCapabilitySupportRow;
use crate::runtime::{
    ForgeQueryGraphCompositionDomainInvariantDenial,
    ForgeQueryGraphCompositionDomainInvariantSummary,
};

pub fn materialize_canonical_invariant_capability_artifact<T>(
    contribution: ForgeQueryMaterializationReadyInvariantCapabilityContribution<T>,
) -> ForgeQueryCanonicalInvariantCapabilityArtifact<T>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    super::artifacts::materialize_domain_capability_canonical_runtime_artifact(contribution)
}

pub fn materialize_graph_composition_capability_support_row(
    contribution: ForgeQueryMaterializationReadyInvariantCapabilityContribution<
        ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    >,
) -> ForgeQueryDomainCapabilityTransitionOutcome<ForgeQueryGraphCompositionCapabilitySupportRow> {
    let domain_contribution = contribution.payload();
    let payload = domain_contribution.payload();
    let Some(graph_capability) = payload.graph_capability() else {
        return TransitionOutcome::Denied(missing_runtime_semantics_denial(
            payload,
            domain_contribution.request_digest(),
            domain_contribution.target().kind(),
            "graph capability",
        ));
    };

    match payload.posture() {
        ForgeQueryInvariantCapabilityContributionPosture::CapabilityGap
        | ForgeQueryInvariantCapabilityContributionPosture::SupportSummary => {
            TransitionOutcome::Success(ForgeQueryGraphCompositionCapabilitySupportRow::new(
                graph_capability.capability_family(),
                graph_capability.capability_class(),
            ))
        }
        ForgeQueryInvariantCapabilityContributionPosture::InvariantDenial
        | ForgeQueryInvariantCapabilityContributionPosture::InvariantRegistration => {
            TransitionOutcome::Denied(unsupported_posture_denial(
                payload,
                domain_contribution.request_digest(),
                domain_contribution.target().kind(),
                "graph capability",
                "capability-gap and support-summary",
            ))
        }
    }
}

pub fn materialize_graph_composition_domain_invariant_denial(
    contribution: ForgeQueryMaterializationReadyInvariantCapabilityContribution<
        ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    >,
) -> ForgeQueryDomainCapabilityTransitionOutcome<ForgeQueryGraphCompositionDomainInvariantDenial> {
    let domain_contribution = contribution.payload();
    let payload = domain_contribution.payload();
    let Some(graph_invariant_denial) = payload.graph_invariant_denial() else {
        return TransitionOutcome::Denied(missing_runtime_semantics_denial(
            payload,
            domain_contribution.request_digest(),
            domain_contribution.target().kind(),
            "graph invariant denial",
        ));
    };

    match payload.posture() {
        ForgeQueryInvariantCapabilityContributionPosture::InvariantDenial => {
            TransitionOutcome::Success(
                ForgeQueryGraphCompositionDomainInvariantDenial::from_contributed(
                    graph_invariant_denial.invariant_family(),
                    payload.detail(),
                    graph_invariant_summary(graph_invariant_denial),
                ),
            )
        }
        ForgeQueryInvariantCapabilityContributionPosture::CapabilityGap
        | ForgeQueryInvariantCapabilityContributionPosture::SupportSummary
        | ForgeQueryInvariantCapabilityContributionPosture::InvariantRegistration => {
            TransitionOutcome::Denied(unsupported_posture_denial(
                payload,
                domain_contribution.request_digest(),
                domain_contribution.target().kind(),
                "graph invariant denial",
                "invariant-denial",
            ))
        }
    }
}

fn graph_invariant_summary(
    semantics: &ForgeQueryGraphInvariantDenialRuntimeSemantics,
) -> ForgeQueryGraphCompositionDomainInvariantSummary {
    ForgeQueryGraphCompositionDomainInvariantSummary::from_parts(
        semantics.declared_collections().to_vec(),
        semantics.declared_symbols().to_vec(),
        semantics.target_combination_families().to_vec(),
        semantics.lifecycle_families().to_vec(),
        semantics.program_digest().to_string(),
        semantics.breadth_digest().to_string(),
        semantics.counter_snapshot().to_string(),
    )
}

fn missing_runtime_semantics_denial(
    payload: &ForgeQueryInvariantCapabilityContributionPayload,
    request_digest: &str,
    target_kind: crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind,
    runtime_family: &str,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::MissingCanonicalMaterializationSemantics,
        "invariant-capability",
        target_kind,
        request_digest,
        format!(
            "{runtime_family} runtime materialization requires matching runtime semantics for `{}`",
            payload.semantic_code()
        ),
    )
}

fn unsupported_posture_denial(
    payload: &ForgeQueryInvariantCapabilityContributionPayload,
    request_digest: &str,
    target_kind: crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind,
    runtime_family: &str,
    supported_postures: &str,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
        "invariant-capability",
        target_kind,
        request_digest,
        format!(
            "{runtime_family} runtime materialization only supports {supported_postures} postures; got `{}`",
            payload.posture().as_str()
        ),
    )
}
