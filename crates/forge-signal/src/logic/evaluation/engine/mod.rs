mod apply;
mod metadata;
mod prepared_apply;

pub(crate) use apply::collect_effect_dependency_inputs_iter;
pub use metadata::EvaluationExecutionMetadata;
pub(crate) use prepared_apply::{
    apply_prepared_evaluation_after_dependencies_with_policy,
};
#[cfg(feature = "parallel")]
pub(crate) use prepared_apply::{
    build_prepared_apply_commit_packet, record_reuse_rejection_telemetry, ApplyCommitBuildError,
};
#[cfg(test)]
pub(crate) use prepared_apply::apply_prepared_evaluation_with_policy;
