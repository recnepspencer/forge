use crate::data::reuse::{
    ArtifactSemanticBoundary, NodeReuseContract, ReuseBoundaryEvidence, ReuseBoundaryFailure,
    ReuseBoundaryProof, ReuseStrategyBoundaryAuthority,
};

use super::basis_resolution::ResolvedReuseDecision;

pub(crate) fn prove_reuse_boundaries(
    contract: &NodeReuseContract,
    decision: &ResolvedReuseDecision,
    evidence: &ReuseBoundaryEvidence,
) -> Result<Vec<ReuseBoundaryProof>, ReuseBoundaryFailure> {
    if let Some(strategy) = decision.strategy {
        if !contract.equivalence.supports_strategy(strategy) {
            return Err(ReuseBoundaryFailure::ContractStrategyDisallowed(strategy));
        }
        match strategy {
            crate::data::reuse::ReuseStrategy::CrossIdentityPersistentMatch
                if !contract
                    .equivalence
                    .required_boundaries
                    .contains(&ArtifactSemanticBoundary::PersistentCorrespondence) =>
            {
                return Err(ReuseBoundaryFailure::ContractStrategyDisallowed(strategy));
            }
            crate::data::reuse::ReuseStrategy::PartialArtifactSplicing
                if !contract
                    .equivalence
                    .required_boundaries
                    .contains(&ArtifactSemanticBoundary::CompositionRegionSet) =>
            {
                return Err(ReuseBoundaryFailure::ContractStrategyDisallowed(strategy));
            }
            _ => {}
        }
    }
    if matches!(
        decision.crossing,
        crate::data::reuse::ReuseCrossing::SnapshotRestore
    ) && !contract.equivalence.allows_snapshot_restore_reuse
    {
        return Err(ReuseBoundaryFailure::SnapshotReuseNotAllowed);
    }
    if matches!(
        decision.crossing,
        crate::data::reuse::ReuseCrossing::AuthorityBoundary
    ) && !contract.equivalence.allows_authority_reconciliation_reuse
    {
        return Err(ReuseBoundaryFailure::AuthorityReuseNotAllowed);
    }

    let mut proofs = Vec::new();
    for boundary in &contract.equivalence.required_boundaries {
        let proof = match boundary {
            ArtifactSemanticBoundary::TopologyRegime => prove_boundary(
                *boundary,
                evidence
                    .previous
                    .as_ref()
                    .map(|context| context.topology_regime),
                evidence.current.topology_regime,
            )?,
            ArtifactSemanticBoundary::ToleranceRegime => prove_boundary(
                *boundary,
                evidence
                    .previous
                    .as_ref()
                    .map(|context| context.tolerance_regime.clone()),
                evidence.current.tolerance_regime.clone(),
            )?,
            ArtifactSemanticBoundary::SemanticRegionIdentity => prove_boundary(
                *boundary,
                evidence
                    .previous
                    .as_ref()
                    .map(|context| context.semantic_region_digest),
                evidence.current.semantic_region_digest,
            )?,
            ArtifactSemanticBoundary::SnapshotLineage
                if !contract.equivalence.allows_snapshot_restore_reuse
                    && matches!(
                        decision.crossing,
                        crate::data::reuse::ReuseCrossing::SnapshotRestore
                    ) =>
            {
                return Err(ReuseBoundaryFailure::BoundaryMismatch(*boundary));
            }
            ArtifactSemanticBoundary::AuthorityLane => {
                if !contract.equivalence.allows_authority_reconciliation_reuse
                    && matches!(
                        decision.crossing,
                        crate::data::reuse::ReuseCrossing::AuthorityBoundary
                    )
                {
                    return Err(ReuseBoundaryFailure::BoundaryMismatch(*boundary));
                }
                prove_boundary(
                    *boundary,
                    evidence
                        .previous
                        .as_ref()
                        .map(|context| context.authority_policy),
                    evidence.current.authority_policy,
                )?
            }
            ArtifactSemanticBoundary::ArtifactFamilyBasis => prove_boundary(
                *boundary,
                evidence
                    .previous
                    .as_ref()
                    .map(|context| context.artifact_family.clone()),
                evidence.current.artifact_family.clone(),
            )?,
            ArtifactSemanticBoundary::StructuralDependencyBasis => prove_boundary(
                *boundary,
                evidence
                    .previous
                    .as_ref()
                    .map(|context| context.structural_dependency_basis),
                evidence.current.structural_dependency_basis,
            )?,
            ArtifactSemanticBoundary::PartitionRegionBasis => prove_boundary(
                *boundary,
                evidence.previous.as_ref().map(|context| {
                    (
                        context.partition_region_basis_digest,
                        context.partition_region_basis_count,
                    )
                }),
                (
                    evidence.current.partition_region_basis_digest,
                    evidence.current.partition_region_basis_count,
                ),
            )?,
            ArtifactSemanticBoundary::PersistentCorrespondence => {
                if !matches!(
                    decision.strategy,
                    Some(crate::data::reuse::ReuseStrategy::CrossIdentityPersistentMatch)
                ) {
                    continue;
                }
                let Some(previous) =
                    evidence
                        .previous
                        .as_ref()
                        .and_then(|context| match context.strategy_detail {
                            ReuseStrategyBoundaryAuthority::CrossIdentity {
                                persistent_correspondence_kind,
                                persistent_correspondence_digest,
                                persistent_correspondence_valid,
                            } => Some((
                                persistent_correspondence_kind,
                                persistent_correspondence_digest,
                                persistent_correspondence_valid,
                            )),
                            ReuseStrategyBoundaryAuthority::None
                            | ReuseStrategyBoundaryAuthority::PartialArtifactSplice { .. } => None,
                        })
                else {
                    return Err(ReuseBoundaryFailure::PersistentCorrespondenceEvidenceMissing);
                };
                let Some(current) = (match evidence.current.strategy_detail {
                    ReuseStrategyBoundaryAuthority::CrossIdentity {
                        persistent_correspondence_kind,
                        persistent_correspondence_digest,
                        persistent_correspondence_valid,
                    } => Some((
                        persistent_correspondence_kind,
                        persistent_correspondence_digest,
                        persistent_correspondence_valid,
                    )),
                    ReuseStrategyBoundaryAuthority::None
                    | ReuseStrategyBoundaryAuthority::PartialArtifactSplice { .. } => None,
                }) else {
                    return Err(ReuseBoundaryFailure::PersistentCorrespondenceEvidenceMissing);
                };
                if !previous.2 || !current.2 || previous != current {
                    return Err(ReuseBoundaryFailure::PersistentCorrespondenceEvidenceInvalid);
                }
                ReuseBoundaryProof {
                    boundary: *boundary,
                    satisfied: true,
                }
            }
            ArtifactSemanticBoundary::CompositionRegionSet => {
                if !matches!(
                    decision.strategy,
                    Some(crate::data::reuse::ReuseStrategy::PartialArtifactSplicing)
                ) {
                    continue;
                }
                let Some(current_regions) = (match evidence.current.strategy_detail {
                    ReuseStrategyBoundaryAuthority::PartialArtifactSplice {
                        composition_region_digest,
                        composition_region_count,
                    } => Some((composition_region_digest, composition_region_count)),
                    ReuseStrategyBoundaryAuthority::None
                    | ReuseStrategyBoundaryAuthority::CrossIdentity { .. } => None,
                }) else {
                    return Err(ReuseBoundaryFailure::CompositionRegionLegalityFailure);
                };
                if current_regions.1 == 0 {
                    return Err(ReuseBoundaryFailure::CompositionRegionLegalityFailure);
                }
                prove_boundary(
                    *boundary,
                    evidence
                        .previous
                        .as_ref()
                        .and_then(|context| match context.strategy_detail {
                            ReuseStrategyBoundaryAuthority::PartialArtifactSplice {
                                composition_region_digest,
                                composition_region_count,
                            } => Some((composition_region_digest, composition_region_count)),
                            ReuseStrategyBoundaryAuthority::None
                            | ReuseStrategyBoundaryAuthority::CrossIdentity { .. } => None,
                        }),
                    current_regions,
                )?
            }
            ArtifactSemanticBoundary::SnapshotLineage => ReuseBoundaryProof {
                boundary: *boundary,
                satisfied: true,
            },
        };
        proofs.push(proof);
    }
    Ok(proofs)
}

fn prove_boundary<T: PartialEq>(
    boundary: ArtifactSemanticBoundary,
    previous: Option<T>,
    current: T,
) -> Result<ReuseBoundaryProof, ReuseBoundaryFailure> {
    match previous {
        Some(previous) if previous == current => Ok(ReuseBoundaryProof {
            boundary,
            satisfied: true,
        }),
        Some(_) => Err(ReuseBoundaryFailure::BoundaryMismatch(boundary)),
        None => Err(ReuseBoundaryFailure::BoundaryContextUnavailable(boundary)),
    }
}
