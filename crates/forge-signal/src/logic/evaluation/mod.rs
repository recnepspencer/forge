mod condition;
mod effect;
mod engine;
mod output;

pub use condition::{
    ConditionEvaluationContext, ConditionResolver, DefaultConditionResolver, EvaluationRequestMode,
};
pub use effect::{
    AppliedEffectReport, DeferralReason, EvaluationEffect, EvaluationVerdict, SuppressionReason,
};
pub use output::{EvaluationOutput, IntoEvaluationOutput};
pub(crate) use effect::{
    EffectComparison, EffectDependencyInputs, PendingDependencySnapshot, PreparedApplyResult,
};
#[cfg(any(test, feature = "parallel"))]
pub(crate) use engine::apply_prepared_evaluation_with_policy;
pub(crate) use engine::{
    apply_prepared_dependency_batch, apply_prepared_evaluation_after_dependencies_with_policy,
    collect_effect_dependency_inputs_batch,
};
pub use engine::EvaluationExecutionMetadata;
