use crate::data::output::MemoizedResultOrigin;
use crate::data::reuse::{ReuseBasis, ReuseCrossing, ReuseOrigin, ReuseSource, ReuseStrategy};
use crate::logic::evaluation::EvaluationExecutionMetadata;
use crate::logic::prepared::PreparedEvaluationOrigin;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedReuseDecision {
    pub basis: ReuseBasis,
    pub strategy: Option<ReuseStrategy>,
    pub origin: ReuseOrigin,
    pub source: ReuseSource,
    pub crossing: ReuseCrossing,
    pub memoized_origin: MemoizedResultOrigin,
    pub recomputed: bool,
}

impl ResolvedReuseDecision {
    fn fresh_compute() -> Self {
        Self {
            basis: ReuseBasis::fresh_compute(),
            strategy: None,
            origin: ReuseOrigin::FreshCompute,
            source: ReuseSource::None,
            crossing: ReuseCrossing::None,
            memoized_origin: MemoizedResultOrigin::DirectCompute,
            recomputed: true,
        }
    }

    fn reused(
        strategy: ReuseStrategy,
        origin: ReuseOrigin,
        source: ReuseSource,
        crossing: ReuseCrossing,
        memoized_origin: MemoizedResultOrigin,
    ) -> Self {
        Self {
            basis: ReuseBasis::strategy(strategy, source, crossing),
            strategy: Some(strategy),
            origin,
            source,
            crossing,
            memoized_origin,
            recomputed: false,
        }
    }
}

pub(crate) fn resolve_prepared_reuse_decision(
    prepared_origin: PreparedEvaluationOrigin,
    execution_metadata: Option<&EvaluationExecutionMetadata>,
) -> ResolvedReuseDecision {
    match execution_metadata {
        Some(metadata) => {
            if metadata.reuse_basis.is_fresh_compute() {
                ResolvedReuseDecision::fresh_compute()
            } else {
                ResolvedReuseDecision::reused(
                    metadata
                        .reuse_basis
                        .strategy
                        .unwrap_or(ReuseStrategy::MemoizedArtifactReuse),
                    metadata.reuse_origin,
                    metadata.reuse_basis.source,
                    metadata.reuse_basis.crossing,
                    metadata.memoized_origin,
                )
            }
        }
        None => match prepared_origin {
            PreparedEvaluationOrigin::DirectPrecompute => ResolvedReuseDecision::fresh_compute(),
            PreparedEvaluationOrigin::MemoizedReuse => ResolvedReuseDecision::reused(
                ReuseStrategy::MemoizedArtifactReuse,
                ReuseOrigin::MemoizedArtifactReuse,
                ReuseSource::MemoizedArtifact,
                ReuseCrossing::None,
                MemoizedResultOrigin::MemoizedFromCache,
            ),
            PreparedEvaluationOrigin::CrossIdentityPersistentReuse => {
                ResolvedReuseDecision::reused(
                    ReuseStrategy::CrossIdentityPersistentMatch,
                    ReuseOrigin::CrossIdentityPersistentReuse,
                    ReuseSource::PersistentCorrespondence,
                    ReuseCrossing::PersistentIdentityBoundary,
                    MemoizedResultOrigin::MemoizedFromCache,
                )
            }
            PreparedEvaluationOrigin::PartialArtifactSplice => ResolvedReuseDecision::reused(
                ReuseStrategy::PartialArtifactSplicing,
                ReuseOrigin::PartialArtifactSplice,
                ReuseSource::PartialComposition,
                ReuseCrossing::CompositionBoundary,
                MemoizedResultOrigin::MemoizedFromCache,
            ),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepared_memoized_reuse_resolves_to_reuse_basis_without_metadata() {
        let decision =
            resolve_prepared_reuse_decision(PreparedEvaluationOrigin::MemoizedReuse, None);

        assert_eq!(
            decision.basis,
            ReuseBasis::strategy(
                ReuseStrategy::MemoizedArtifactReuse,
                ReuseSource::MemoizedArtifact,
                ReuseCrossing::None,
            )
        );
        assert_eq!(decision.origin, ReuseOrigin::MemoizedArtifactReuse);
        assert_eq!(
            decision.memoized_origin,
            MemoizedResultOrigin::MemoizedFromCache
        );
        assert!(!decision.recomputed);
    }
}
