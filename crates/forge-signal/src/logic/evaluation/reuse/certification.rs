use crate::data::reuse::{
    NodeReuseContract, ReuseBoundaryEvidence, ReuseCertificationFailure, ReuseCertificationRecord,
};

use super::basis_resolution::ResolvedReuseDecision;
use super::boundary_checks::prove_reuse_boundaries;

pub(crate) fn certify_reuse_decision(
    contract: &NodeReuseContract,
    decision: &ResolvedReuseDecision,
    evidence: &ReuseBoundaryEvidence,
) -> Result<Option<ReuseCertificationRecord>, ReuseCertificationFailure> {
    if decision.basis.is_fresh_compute() {
        return Ok(None);
    }

    let proofs = prove_reuse_boundaries(contract, decision, evidence).map_err(|failure| {
        ReuseCertificationFailure {
            strategy: decision.strategy,
            source: decision.source,
            crossing: decision.crossing,
            failure,
        }
    })?;

    if !contract.retain_certification {
        return Ok(None);
    }

    Ok(Some(ReuseCertificationRecord {
        strategy: decision
            .strategy
            .expect("non-fresh reuse decisions must carry a strategy"),
        origin: decision.origin,
        source: decision.source,
        crossing: decision.crossing,
        proofs,
    }))
}

#[cfg(test)]
mod tests {
    use crate::data::reuse::{
        ArtifactEquivalenceContract, ArtifactSemanticBoundary, NodeReuseContract,
        PersistentCorrespondenceEvidence, ReuseBasis, ReuseBoundaryContext, ReuseBoundaryEvidence,
        ReuseBoundaryFailure, ReuseCrossing, ReuseOrigin, ReuseSemanticRegionIdentity, ReuseSource,
        ReuseStrategy, ReuseStrategyBoundaryContext,
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
                artifact_family: None,
                structural_dependency_basis: crate::data::dependency::DependencySnapshotId::EMPTY,
                partition_region_basis: Default::default(),
                strategy_detail: ReuseStrategyBoundaryContext::None,
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
                artifact_family: None,
                structural_dependency_basis: crate::data::dependency::DependencySnapshotId::EMPTY,
                partition_region_basis: Default::default(),
                strategy_detail: ReuseStrategyBoundaryContext::None,
            }),
        }
    }

    #[test]
    fn snapshot_crossing_reuse_requires_snapshot_allowance() {
        let contract = NodeReuseContract {
            equivalence: ArtifactEquivalenceContract {
                required_boundaries: vec![ArtifactSemanticBoundary::SnapshotLineage],
                supported_strategies: vec![ReuseStrategy::SnapshotRestoreReuse],
                allows_snapshot_restore_reuse: false,
                allows_authority_reconciliation_reuse: false,
            },
            retain_certification: true,
        };
        let decision = ResolvedReuseDecision {
            basis: ReuseBasis::strategy(
                ReuseStrategy::SnapshotRestoreReuse,
                ReuseSource::SnapshotArtifact,
                ReuseCrossing::SnapshotRestore,
            ),
            strategy: Some(ReuseStrategy::SnapshotRestoreReuse),
            origin: ReuseOrigin::SnapshotRestore,
            source: ReuseSource::SnapshotArtifact,
            crossing: ReuseCrossing::SnapshotRestore,
            memoized_origin: crate::data::output::MemoizedResultOrigin::MemoizedFromCache,
            recomputed: false,
        };

        let failure = certify_reuse_decision(&contract, &decision, &evidence()).unwrap_err();
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
                supported_strategies: vec![ReuseStrategy::ReconciliationAdoption],
                allows_snapshot_restore_reuse: false,
                allows_authority_reconciliation_reuse: false,
            },
            retain_certification: true,
        };
        let decision = ResolvedReuseDecision {
            basis: ReuseBasis::strategy(
                ReuseStrategy::ReconciliationAdoption,
                ReuseSource::AuthorityReconciliation,
                ReuseCrossing::AuthorityBoundary,
            ),
            strategy: Some(ReuseStrategy::ReconciliationAdoption),
            origin: ReuseOrigin::ReconciliationAdoption,
            source: ReuseSource::AuthorityReconciliation,
            crossing: ReuseCrossing::AuthorityBoundary,
            memoized_origin: crate::data::output::MemoizedResultOrigin::MemoizedFromCache,
            recomputed: false,
        };

        let failure = certify_reuse_decision(&contract, &decision, &evidence()).unwrap_err();
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
            basis: ReuseBasis::strategy(
                ReuseStrategy::MemoizedArtifactReuse,
                ReuseSource::MemoizedArtifact,
                ReuseCrossing::None,
            ),
            strategy: Some(ReuseStrategy::MemoizedArtifactReuse),
            origin: ReuseOrigin::MemoizedArtifactReuse,
            source: ReuseSource::MemoizedArtifact,
            crossing: ReuseCrossing::None,
            memoized_origin: crate::data::output::MemoizedResultOrigin::MemoizedFromCache,
            recomputed: false,
        };

        assert_eq!(
            certify_reuse_decision(&contract, &decision, &evidence()).unwrap(),
            None
        );
    }

    #[test]
    fn missing_prior_boundary_context_fails_certification_honestly() {
        let contract = NodeReuseContract::strict();
        let decision = ResolvedReuseDecision {
            basis: ReuseBasis::strategy(
                ReuseStrategy::MemoizedArtifactReuse,
                ReuseSource::MemoizedArtifact,
                ReuseCrossing::None,
            ),
            strategy: Some(ReuseStrategy::MemoizedArtifactReuse),
            origin: ReuseOrigin::MemoizedArtifactReuse,
            source: ReuseSource::MemoizedArtifact,
            crossing: ReuseCrossing::None,
            memoized_origin: crate::data::output::MemoizedResultOrigin::MemoizedFromCache,
            recomputed: false,
        };
        let mut evidence = evidence();
        evidence.previous = None;

        let failure = certify_reuse_decision(&contract, &decision, &evidence).unwrap_err();
        assert_eq!(
            failure.failure,
            ReuseBoundaryFailure::BoundaryContextUnavailable(
                ArtifactSemanticBoundary::TopologyRegime
            )
        );
    }

    #[test]
    fn cross_identity_persistent_match_requires_explicit_correspondence_boundary() {
        let contract = NodeReuseContract {
            equivalence: ArtifactEquivalenceContract {
                required_boundaries: vec![ArtifactSemanticBoundary::PersistentCorrespondence],
                supported_strategies: vec![ReuseStrategy::CrossIdentityPersistentMatch],
                allows_snapshot_restore_reuse: false,
                allows_authority_reconciliation_reuse: false,
            },
            retain_certification: true,
        };
        let decision = ResolvedReuseDecision {
            basis: ReuseBasis::strategy(
                ReuseStrategy::CrossIdentityPersistentMatch,
                ReuseSource::PersistentCorrespondence,
                ReuseCrossing::PersistentIdentityBoundary,
            ),
            strategy: Some(ReuseStrategy::CrossIdentityPersistentMatch),
            origin: ReuseOrigin::CrossIdentityPersistentReuse,
            source: ReuseSource::PersistentCorrespondence,
            crossing: ReuseCrossing::PersistentIdentityBoundary,
            memoized_origin: crate::data::output::MemoizedResultOrigin::MemoizedFromCache,
            recomputed: false,
        };

        let failure = certify_reuse_decision(&contract, &decision, &evidence()).unwrap_err();
        assert_eq!(
            failure.failure,
            ReuseBoundaryFailure::PersistentCorrespondenceEvidenceMissing
        );
    }

    #[test]
    fn partial_artifact_splicing_requires_explicit_composition_region_basis() {
        let contract = NodeReuseContract {
            equivalence: ArtifactEquivalenceContract {
                required_boundaries: vec![ArtifactSemanticBoundary::CompositionRegionSet],
                supported_strategies: vec![ReuseStrategy::PartialArtifactSplicing],
                allows_snapshot_restore_reuse: false,
                allows_authority_reconciliation_reuse: false,
            },
            retain_certification: true,
        };
        let decision = ResolvedReuseDecision {
            basis: ReuseBasis::strategy(
                ReuseStrategy::PartialArtifactSplicing,
                ReuseSource::PartialComposition,
                ReuseCrossing::CompositionBoundary,
            ),
            strategy: Some(ReuseStrategy::PartialArtifactSplicing),
            origin: ReuseOrigin::PartialArtifactSplice,
            source: ReuseSource::PartialComposition,
            crossing: ReuseCrossing::CompositionBoundary,
            memoized_origin: crate::data::output::MemoizedResultOrigin::MemoizedFromCache,
            recomputed: false,
        };
        let mut evidence = evidence();
        evidence.current.strategy_detail = ReuseStrategyBoundaryContext::None;

        let failure = certify_reuse_decision(&contract, &decision, &evidence).unwrap_err();
        assert_eq!(
            failure.failure,
            ReuseBoundaryFailure::CompositionRegionLegalityFailure
        );
    }

    #[test]
    fn cross_identity_match_accepts_explicit_allowed_evidence() {
        let contract = NodeReuseContract {
            equivalence: ArtifactEquivalenceContract {
                required_boundaries: vec![ArtifactSemanticBoundary::PersistentCorrespondence],
                supported_strategies: vec![ReuseStrategy::CrossIdentityPersistentMatch],
                allows_snapshot_restore_reuse: false,
                allows_authority_reconciliation_reuse: false,
            },
            retain_certification: true,
        };
        let decision = ResolvedReuseDecision {
            basis: ReuseBasis::strategy(
                ReuseStrategy::CrossIdentityPersistentMatch,
                ReuseSource::PersistentCorrespondence,
                ReuseCrossing::PersistentIdentityBoundary,
            ),
            strategy: Some(ReuseStrategy::CrossIdentityPersistentMatch),
            origin: ReuseOrigin::CrossIdentityPersistentReuse,
            source: ReuseSource::PersistentCorrespondence,
            crossing: ReuseCrossing::PersistentIdentityBoundary,
            memoized_origin: crate::data::output::MemoizedResultOrigin::MemoizedFromCache,
            recomputed: false,
        };
        let mut evidence = evidence();
        let correspondence =
            PersistentCorrespondenceEvidence::HostSuppliedKey("mesh-001".to_string());
        evidence.current.strategy_detail = ReuseStrategyBoundaryContext::CrossIdentity {
            persistent_correspondence: correspondence.clone(),
        };
        evidence
            .previous
            .as_mut()
            .expect("previous context")
            .strategy_detail = ReuseStrategyBoundaryContext::CrossIdentity {
            persistent_correspondence: correspondence,
        };

        let certification = certify_reuse_decision(&contract, &decision, &evidence).unwrap();
        assert!(certification.is_some());
        assert_eq!(
            certification.unwrap().origin,
            ReuseOrigin::CrossIdentityPersistentReuse
        );
    }
}
