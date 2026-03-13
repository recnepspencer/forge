use crate::data::reuse::{
    NodeReuseContract, ReuseBoundaryEvidence, ReuseCertificationFailure, ReuseCertificationRecord,
};

use super::basis_resolution::ResolvedReuseDecision;
use super::boundary_checks::prove_reuse_boundaries;

pub(crate) fn certify_reuse_decision(
    contract: &NodeReuseContract,
    decision: ResolvedReuseDecision,
    evidence: &ReuseBoundaryEvidence,
) -> Result<Option<ReuseCertificationRecord>, ReuseCertificationFailure> {
    if matches!(decision.basis, crate::data::reuse::ReuseBasis::FreshCompute) {
        return Ok(None);
    }

    let proofs = prove_reuse_boundaries(contract, decision, evidence).map_err(|failure| {
        ReuseCertificationFailure {
            source: decision.source,
            crossing: decision.crossing,
            failure,
        }
    })?;

    if !contract.retain_certification {
        return Ok(None);
    }

    Ok(Some(ReuseCertificationRecord {
        source: decision.source,
        crossing: decision.crossing,
        proofs,
    }))
}

#[cfg(test)]
mod tests {
    use crate::data::reuse::{
        ArtifactEquivalenceContract, ArtifactSemanticBoundary, NodeReuseContract, ReuseBasis,
        ReuseBoundaryContext, ReuseBoundaryEvidence, ReuseBoundaryFailure, ReuseCrossing,
        ReuseSemanticRegionIdentity, ReuseSource,
    };
    use crate::data::{
        comparator::VersionComparatorPolicy, node::ContextRequirement, performance::AuthorityPolicy,
    };

    use super::*;
    use crate::logic::evaluation::reuse::basis_resolution::ResolvedReuseDecision;

    fn evidence() -> ReuseBoundaryEvidence {
        ReuseBoundaryEvidence {
            current: ReuseBoundaryContext {
                topology_regime: 7,
                tolerance_regime: VersionComparatorPolicy::Tolerance { epsilon: 2 },
                semantic_region: ReuseSemanticRegionIdentity::new(
                    crate::data::handle::NodeId::new(0, 0),
                    true,
                    Vec::new(),
                    ContextRequirement::None,
                ),
                authority_policy: AuthorityPolicy::SpeculativeThenReconcile,
            },
            previous: Some(ReuseBoundaryContext {
                topology_regime: 7,
                tolerance_regime: VersionComparatorPolicy::Tolerance { epsilon: 2 },
                semantic_region: ReuseSemanticRegionIdentity::new(
                    crate::data::handle::NodeId::new(0, 0),
                    true,
                    Vec::new(),
                    ContextRequirement::None,
                ),
                authority_policy: AuthorityPolicy::SpeculativeThenReconcile,
            }),
        }
    }

    #[test]
    fn snapshot_crossing_reuse_requires_snapshot_allowance() {
        let contract = NodeReuseContract {
            equivalence: ArtifactEquivalenceContract {
                required_boundaries: vec![ArtifactSemanticBoundary::SnapshotLineage],
                allows_snapshot_restore_reuse: false,
                allows_authority_reconciliation_reuse: false,
            },
            retain_certification: true,
        };
        let decision = ResolvedReuseDecision {
            basis: ReuseBasis::Reused {
                source: ReuseSource::SnapshotArtifact,
                crossing: ReuseCrossing::SnapshotRestore,
            },
            source: ReuseSource::SnapshotArtifact,
            crossing: ReuseCrossing::SnapshotRestore,
            memoized_origin: crate::data::output::MemoizedResultOrigin::MemoizedFromCache,
            recomputed: false,
        };

        let failure = certify_reuse_decision(&contract, decision, &evidence()).unwrap_err();
        assert_eq!(
            failure.failure,
            ReuseBoundaryFailure::SnapshotReuseNotAllowed
        );
    }

    #[test]
    fn authority_crossing_reuse_requires_authority_allowance() {
        let contract = NodeReuseContract {
            equivalence: ArtifactEquivalenceContract {
                required_boundaries: vec![ArtifactSemanticBoundary::AuthorityLane],
                allows_snapshot_restore_reuse: false,
                allows_authority_reconciliation_reuse: false,
            },
            retain_certification: true,
        };
        let decision = ResolvedReuseDecision {
            basis: ReuseBasis::Reused {
                source: ReuseSource::MemoizedArtifact,
                crossing: ReuseCrossing::AuthorityBoundary,
            },
            source: ReuseSource::MemoizedArtifact,
            crossing: ReuseCrossing::AuthorityBoundary,
            memoized_origin: crate::data::output::MemoizedResultOrigin::MemoizedFromCache,
            recomputed: false,
        };

        let failure = certify_reuse_decision(&contract, decision, &evidence()).unwrap_err();
        assert_eq!(
            failure.failure,
            ReuseBoundaryFailure::AuthorityReuseNotAllowed
        );
    }

    #[test]
    fn retained_certification_can_be_disabled_without_skipping_boundary_enforcement() {
        let contract = NodeReuseContract {
            equivalence: ArtifactEquivalenceContract::strict(),
            retain_certification: false,
        };
        let decision = ResolvedReuseDecision {
            basis: ReuseBasis::Reused {
                source: ReuseSource::MemoizedArtifact,
                crossing: ReuseCrossing::None,
            },
            source: ReuseSource::MemoizedArtifact,
            crossing: ReuseCrossing::None,
            memoized_origin: crate::data::output::MemoizedResultOrigin::MemoizedFromCache,
            recomputed: false,
        };

        assert_eq!(
            certify_reuse_decision(&contract, decision, &evidence()).unwrap(),
            None
        );
    }

    #[test]
    fn missing_prior_boundary_context_fails_certification_honestly() {
        let contract = NodeReuseContract::strict();
        let decision = ResolvedReuseDecision {
            basis: ReuseBasis::Reused {
                source: ReuseSource::MemoizedArtifact,
                crossing: ReuseCrossing::None,
            },
            source: ReuseSource::MemoizedArtifact,
            crossing: ReuseCrossing::None,
            memoized_origin: crate::data::output::MemoizedResultOrigin::MemoizedFromCache,
            recomputed: false,
        };
        let mut evidence = evidence();
        evidence.previous = None;

        let failure = certify_reuse_decision(&contract, decision, &evidence).unwrap_err();
        assert_eq!(
            failure.failure,
            ReuseBoundaryFailure::BoundaryContextUnavailable(
                ArtifactSemanticBoundary::TopologyRegime
            )
        );
    }
}
