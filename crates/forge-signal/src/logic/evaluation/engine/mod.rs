mod apply;
mod metadata;
mod prepared_apply;

pub(crate) use apply::collect_effect_dependency_inputs_batch;
pub use metadata::EvaluationExecutionMetadata;
#[cfg(any(test, feature = "parallel"))]
pub(crate) use prepared_apply::{
    apply_prepared_evaluation_with_policy,
};
pub(crate) use prepared_apply::{
    apply_prepared_dependency_batch, apply_prepared_evaluation_after_dependencies_with_policy,
};
