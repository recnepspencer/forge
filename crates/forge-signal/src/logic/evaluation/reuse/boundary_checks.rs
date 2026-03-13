use crate::data::reuse::{
    ArtifactSemanticBoundary, NodeReuseContract, ReuseBoundaryEvidence, ReuseBoundaryFailure,
    ReuseBoundaryProof,
};

use super::basis_resolution::ResolvedReuseDecision;

pub(crate) fn prove_reuse_boundaries(
    contract: &NodeReuseContract,
    decision: ResolvedReuseDecision,
    evidence: &ReuseBoundaryEvidence,
) -> Result<Vec<ReuseBoundaryProof>, ReuseBoundaryFailure> {
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
                    .map(|context| context.semantic_region.clone()),
                evidence.current.semantic_region.clone(),
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
