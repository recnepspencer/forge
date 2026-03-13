use crate::data::output::MemoizedResultOrigin;
use crate::data::reuse::{ReuseBasis, ReuseCrossing, ReuseSource};
use crate::logic::evaluation::EvaluationExecutionMetadata;
use crate::logic::prepared::PreparedEvaluationOrigin;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ResolvedReuseDecision {
    pub basis: ReuseBasis,
    pub source: ReuseSource,
    pub crossing: ReuseCrossing,
    pub memoized_origin: MemoizedResultOrigin,
    pub recomputed: bool,
}

impl ResolvedReuseDecision {
    fn fresh_compute() -> Self {
        Self {
            basis: ReuseBasis::FreshCompute,
            source: ReuseSource::None,
            crossing: ReuseCrossing::None,
            memoized_origin: MemoizedResultOrigin::DirectCompute,
            recomputed: true,
        }
    }

    fn reused(
        source: ReuseSource,
        crossing: ReuseCrossing,
        memoized_origin: MemoizedResultOrigin,
    ) -> Self {
        Self {
            basis: ReuseBasis::Reused { source, crossing },
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
        Some(metadata) => match metadata.reuse_basis {
            ReuseBasis::FreshCompute => ResolvedReuseDecision::fresh_compute(),
            ReuseBasis::Reused { source, crossing } => {
                ResolvedReuseDecision::reused(source, crossing, metadata.memoized_origin)
            }
        },
        None => match prepared_origin {
            PreparedEvaluationOrigin::DirectPrecompute => ResolvedReuseDecision::fresh_compute(),
            PreparedEvaluationOrigin::MemoizedReuse => ResolvedReuseDecision::reused(
                ReuseSource::MemoizedArtifact,
                ReuseCrossing::None,
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
            ReuseBasis::Reused {
                source: ReuseSource::MemoizedArtifact,
                crossing: ReuseCrossing::None,
            }
        );
        assert_eq!(
            decision.memoized_origin,
            MemoizedResultOrigin::MemoizedFromCache
        );
        assert!(!decision.recomputed);
    }
}
