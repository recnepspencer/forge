mod configuration;
mod identity;
mod planning;
mod scope;
#[cfg(test)]
mod tests;

pub use configuration::{
    UiChangeProfile, UiRebindBudgetInput, UiRebindConcurrencyInput, UiRebindLimit, UiRebindProfile,
    UiRebindProfileConstructionDenial,
};
#[cfg(any(test, feature = "certification-support"))]
pub(crate) use identity::decision_from_transition;
pub use identity::{
    UiIdentityLifecycleDecision, UiIdentityLifecycleDenial, UiIdentityLifecycleEntry,
    UiResolvedIdentityLifecycle,
};
pub(crate) use identity::{UiIdentityLifecycleResolver, UiSourceIdentityLifecycleIndex};
pub use planning::{
    UiRebindArtifactPolicy, UiRebindCancellationPolicy, UiRebindCancellationRequest,
    UiRebindConflictFootprint, UiRebindDeadlinePolicy, UiRebindDeclarativeEffect,
    UiRebindDisclosurePolicy, UiRebindEffectSet, UiRebindExecutionPolicy, UiRebindIdempotency,
    UiRebindParallelAdmission, UiRebindPlan, UiRebindPlanBasis, UiRebindPlanCost,
    UiRebindPlanTarget, UiRebindPlanningDenial, UiRebindResourceAccess, UiRebindRetryTolerance,
    UiRebindSafePoint, UiRebindSafePointPolicy, UiRebindSessionDeadline, UiRebindSubsystemKind,
    UiRebindSubsystemPlan,
};
pub(crate) use planning::{UiRebindPlanCompiler, UiRebindPlanningContext};
pub(crate) use scope::UiAffectedScopeResolver;
pub use scope::{
    UiAffectedConsumer, UiAffectedFactLookup, UiAffectedScopeBasis, UiAffectedScopeCost,
    UiAffectedScopeDenial, UiAffectedScopeGeneration, UiResolvedAffectedScope,
};
