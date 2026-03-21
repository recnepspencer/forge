use crate::data::output::{KeyedComputation, MemoizedResultOrigin};
use crate::data::reuse::{ReuseBasis, ReuseOrigin, ReuseStrategy};

#[derive(Debug, Clone)]
pub struct EvaluationExecutionMetadata {
    pub keyed: Option<KeyedComputation>,
    pub memoized_origin: MemoizedResultOrigin,
    pub reuse_basis: ReuseBasis,
    pub reuse_origin: ReuseOrigin,
}

impl EvaluationExecutionMetadata {
    pub fn from_keyed(computation: &KeyedComputation, reuse_basis: ReuseBasis) -> Self {
        Self {
            keyed: Some(computation.clone()),
            memoized_origin: descriptive_memoized_origin(&reuse_basis),
            reuse_origin: descriptive_reuse_origin(&reuse_basis),
            reuse_basis,
        }
    }
}

fn descriptive_memoized_origin(reuse_basis: &ReuseBasis) -> MemoizedResultOrigin {
    if reuse_basis.is_fresh_compute() {
        MemoizedResultOrigin::DirectCompute
    } else {
        MemoizedResultOrigin::MemoizedFromCache
    }
}

fn descriptive_reuse_origin(reuse_basis: &ReuseBasis) -> ReuseOrigin {
    match reuse_basis.strategy {
        Some(ReuseStrategy::OutputSuppression) => ReuseOrigin::OutputSuppressed,
        Some(ReuseStrategy::MemoizedArtifactReuse) => ReuseOrigin::MemoizedArtifactReuse,
        Some(ReuseStrategy::SnapshotRestoreReuse) => ReuseOrigin::SnapshotRestore,
        Some(ReuseStrategy::ReconciliationAdoption) => ReuseOrigin::ReconciliationAdoption,
        Some(ReuseStrategy::CrossIdentityPersistentMatch) => {
            ReuseOrigin::CrossIdentityPersistentReuse
        }
        Some(ReuseStrategy::PartialArtifactSplicing) => ReuseOrigin::PartialArtifactSplice,
        None => ReuseOrigin::FreshCompute,
    }
}
