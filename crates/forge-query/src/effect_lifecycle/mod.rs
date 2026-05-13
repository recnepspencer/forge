mod admission;
mod authoring_basis;
mod counters;
mod eligibility;
mod execution;
mod intent;
mod lowering;
mod normalized;
mod planning;
mod support;
mod taxonomy;

pub use admission::{admit_effect_intent, evaluate_effect_eligibility};
pub use authoring_basis::EffectAuthoringBasis;
pub use counters::EffectLifecycleCounters;
pub use eligibility::{
    AdmittedEffectIntent, DeferredEffectEligibility, DeniedEffectEligibility, EffectEligibility,
    EffectEligibilityDecisionTrace, EffectEligibilityOutcome, RebindRequiredEffectEligibility,
};
pub use execution::{
    execute_lowered_effect_plan, EffectExecutionAuthority, EffectExecutionDenial,
    EffectExecutionDenialKind, ExecutedEffectAuthorityArtifact, ExecutedEffectPlan,
};
pub use intent::{normalize_raw_effect_intent, RawEffectIntent};
pub use lowering::{
    lower_authority_scoped_effect_plan, EffectLoweringDenial, EffectLoweringDenialKind,
    LoweredEffectExecutionArtifact, LoweredEffectExecutionPlan,
};
pub use normalized::{EffectIntentDenial, EffectOperationInput, NormalizedEffectIntent};
pub use planning::{
    scope_admitted_effect_plan, AuthorityScopedEffectPlan, EffectArtifactPolicy,
    EffectAuthorityOwner, EffectConflictFootprint, EffectInvariantScope,
    EffectPermittedLoweringFamily, EffectPolicyPosture, EffectPreviewPosture,
    EffectStrategyIdentityTarget,
};
pub use support::{
    discover_effect_lifecycle_support, effect_lifecycle_support_matrix,
    EffectLifecycleSupportDiscovery, EffectLifecycleSupportMatrix, EffectLifecycleSupportRow,
    EffectSupportPosture,
};
pub use taxonomy::{
    DeniedEffectEligibilityKind, EffectAuthorityLane, EffectFamily, EffectIntentDenialKind,
};

#[cfg(test)]
mod tests;
