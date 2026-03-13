use crate::data::output::{KeyedComputation, MemoizedResultOrigin};
use crate::data::reuse::ReuseBasis;

#[derive(Debug, Clone)]
pub struct EvaluationExecutionMetadata {
    pub keyed: Option<KeyedComputation>,
    pub memoized_origin: MemoizedResultOrigin,
    pub reuse_basis: ReuseBasis,
}

impl EvaluationExecutionMetadata {
    pub fn from_keyed(computation: &KeyedComputation, reuse_basis: ReuseBasis) -> Self {
        Self {
            keyed: Some(computation.clone()),
            memoized_origin: descriptive_memoized_origin(reuse_basis),
            reuse_basis,
        }
    }
}

fn descriptive_memoized_origin(reuse_basis: ReuseBasis) -> MemoizedResultOrigin {
    match reuse_basis {
        ReuseBasis::FreshCompute => MemoizedResultOrigin::DirectCompute,
        ReuseBasis::Reused { .. } => MemoizedResultOrigin::MemoizedFromCache,
    }
}
