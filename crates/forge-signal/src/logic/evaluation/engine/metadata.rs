use crate::data::output::{KeyedComputation, MemoizedResultOrigin};

#[derive(Debug, Clone)]
pub struct EvaluationExecutionMetadata {
    pub keyed: Option<KeyedComputation>,
    pub memoized_origin: MemoizedResultOrigin,
}

impl EvaluationExecutionMetadata {
    pub fn from_keyed(
        computation: &KeyedComputation,
        memoized_origin: MemoizedResultOrigin,
    ) -> Self {
        Self {
            keyed: Some(computation.clone()),
            memoized_origin,
        }
    }
}
