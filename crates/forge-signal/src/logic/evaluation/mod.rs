mod condition;
mod effect;
mod engine;
mod output;

pub use condition::{
    ConditionEvaluationContext, ConditionResolver, DefaultConditionResolver, EvaluationRequestMode,
};
pub use effect::{
    AppliedEffectReport, DeferralReason, DiagnosticEnvelope, EvaluationVerdict, OperationalEffect,
    SuppressionReason,
};
pub(crate) use effect::{
    EffectComparison, EffectDependencyInputs, EffectRuntimeMetadata, EvaluationEffect,
    PendingDependencySnapshot, PreparedApplyResult,
};
#[cfg(test)]
pub(crate) use engine::apply_prepared_evaluation_with_policy;
pub use engine::EvaluationExecutionMetadata;
pub(crate) use engine::{
    apply_prepared_evaluation_after_dependencies_with_policy,
    collect_effect_dependency_inputs_batch,
};
pub use output::{EvaluationOutput, IntoEvaluationOutput};
