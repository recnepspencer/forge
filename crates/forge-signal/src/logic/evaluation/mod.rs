mod condition;
mod effect;
mod engine;
mod output;
mod reuse;

pub use condition::{
    ConditionEvaluationContext, ConditionResolver, DefaultConditionResolver, EvaluationRequestMode,
};
pub use effect::{
    AppliedEffectReport, DeferralReason, DiagnosticEnvelope, EvaluationVerdict, OperationalEffect,
    SuppressionReason,
};
pub(crate) use effect::{
    DependencyInputContext, EffectComparison, EffectDependencyInputs, EffectRuntimeMetadata,
    EvaluationEffect, PendingDependencySnapshot, PreparedApplyResult,
};
#[cfg(test)]
pub(crate) use engine::apply_prepared_evaluation_with_policy;
pub use engine::EvaluationExecutionMetadata;
pub(crate) use engine::{
    apply_prepared_evaluation_after_dependencies_with_policy,
    build_effect_dependency_inputs_for_dependencies,
};
pub(crate) use engine::collect_effect_dependency_inputs_iter;
#[cfg(feature = "parallel")]
pub(crate) use engine::{
    build_prepared_apply_commit_packet, record_reuse_rejection_telemetry, ApplyCommitBuildError,
};
pub use output::{EvaluationOutput, IntoEvaluationOutput};
#[cfg(feature = "parallel")]
pub(crate) use reuse::resolve_reuse_boundary_context_with_policy;
pub(crate) use reuse::{
    certify_reuse_decision, resolve_prepared_reuse_decision, resolve_reuse_boundary_context,
};
