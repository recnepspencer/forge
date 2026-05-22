use forge_foundational::{
    boundary_evidence, derive_foundational_profile_identity, BoundaryHandle,
    CanonicalizationRuleVersion, FoundationalBoundaryEvidenceCanonicalDigestBasis,
    FoundationalBoundaryEvidenceFreshnessPosture, FoundationalBoundaryEvidenceProfileBasis,
    FoundationalBoundaryEvidenceProvenanceArtifact, FoundationalBoundaryEvidenceSourceBasis,
    FoundationalBoundaryEvidenceStrategyBasis,
    FoundationalBoundaryEvidenceSupportContextAttachment, FoundationalTransitionStrategyFamily,
    FoundationalTransitionStrategyId, FoundationalTransitionStrategyIdentity,
    FoundationalTransitionStrategyOwnershipClass, FoundationalTransitionStrategySemanticName,
    FoundationalTransitionStrategyVersion,
};
use forge_proof::TransitionOutcome;

use super::super::materialization::{
    ForgeQueryDomainCapabilityDescriptiveArtifactKind,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
    ForgeQueryDomainCapabilityProfileProgression,
};
use super::super::targets::ForgeQueryDomainCapabilityTargetBinding;
use super::super::{
    ForgeQueryDomainCapabilityPayload, ForgeQueryDomainCapabilitySemanticPosture,
    ForgeQueryMaterializationReadyDomainCapabilityContribution,
};
use super::rows::ForgeQueryDomainCapabilityDiagnosticRows;

pub(crate) fn build_provenance<P, T>(
    contribution: &ForgeQueryMaterializationReadyDomainCapabilityContribution<P, T>,
    profile_progression: &ForgeQueryDomainCapabilityProfileProgression,
    rows: &ForgeQueryDomainCapabilityDiagnosticRows,
    artifact_kind: ForgeQueryDomainCapabilityDescriptiveArtifactKind,
) -> Result<
    FoundationalBoundaryEvidenceProvenanceArtifact,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    let category = contribution.payload().category();
    let profile_identity = match derive_foundational_profile_identity(
        CanonicalizationRuleVersion::new("forge.query.domain-capabilities.v1")
            .expect("valid canonicalization rule version"),
        profile_progression.admitted(),
    ) {
        TransitionOutcome::Success(identity) => identity,
        TransitionOutcome::Denied(denial) => {
            return Err(
                ForgeQueryDomainCapabilityDescriptiveMaterializationDenial::ProfileIdentity {
                    category,
                    artifact_kind,
                    denial,
                },
            )
        }
        outcome => panic!("unexpected foundational profile identity outcome: {outcome:?}"),
    };

    let semantic_posture = contribution.payload().payload().semantic_posture();
    let step = boundary_evidence()
        .provenance()
        .current(source_basis_for(contribution))
        .profile_basis(FoundationalBoundaryEvidenceProfileBasis::profile(
            profile_identity.clone(),
        ))
        .canonical_digest_basis(FoundationalBoundaryEvidenceCanonicalDigestBasis::digest(
            profile_identity.digest().clone(),
        ));
    let step = if semantic_posture.is_policy_or_inferred() {
        step.strategy_basis(strategy_basis_for(contribution, semantic_posture))
    } else {
        step
    }
    .attach_support_context(
        FoundationalBoundaryEvidenceSupportContextAttachment::diagnostic_scope(rows.scope.clone()),
    )
    .attach_support_context(
        FoundationalBoundaryEvidenceSupportContextAttachment::diagnostic_code(
            rows.primary_code.clone(),
        ),
    );

    match step.with_freshness(freshness_for()) {
        TransitionOutcome::Success(provenance) => Ok(provenance),
        TransitionOutcome::Denied(denial) => panic!("unexpected provenance denial: {denial:?}"),
        outcome => panic!("unexpected provenance outcome: {outcome:?}"),
    }
}

fn source_basis_for<P, T>(
    contribution: &ForgeQueryMaterializationReadyDomainCapabilityContribution<P, T>,
) -> FoundationalBoundaryEvidenceSourceBasis
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    let locator = forge_foundational::BoundaryArtifactLocator::new(
        super::rows::boundary_artifact_id(contribution.payload().target().binding_digest()),
        forge_foundational::BoundaryArtifactField::Basis,
    );
    FoundationalBoundaryEvidenceSourceBasis::boundary_artifact(locator)
}

fn freshness_for() -> FoundationalBoundaryEvidenceFreshnessPosture {
    FoundationalBoundaryEvidenceFreshnessPosture::FreshRetained
}

fn strategy_basis_for<P, T>(
    contribution: &ForgeQueryMaterializationReadyDomainCapabilityContribution<P, T>,
    semantic_posture: ForgeQueryDomainCapabilitySemanticPosture,
) -> FoundationalBoundaryEvidenceStrategyBasis
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    let handle = BoundaryHandle::new(
        super::rows::boundary_artifact_id(contribution.payload().request_digest()).get(),
    );
    let family =
        FoundationalTransitionStrategyFamily::new(contribution.payload().category().as_str())
            .expect("category names are valid strategy families");
    let semantic_name = FoundationalTransitionStrategySemanticName::new(semantic_posture.as_str())
        .expect("semantic posture names are valid strategy semantic names");
    let version = FoundationalTransitionStrategyVersion::new("v1")
        .expect("static strategy version should always be valid");

    FoundationalBoundaryEvidenceStrategyBasis::strategy(
        FoundationalTransitionStrategyIdentity::new(
            FoundationalTransitionStrategyId::new(handle),
            family,
            semantic_name,
            version,
            FoundationalTransitionStrategyOwnershipClass::CompatibilityLowered,
        ),
    )
}
