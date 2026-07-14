mod artifact;
mod counters;
mod errors;
mod lowering;
mod support;
mod validation;

pub use artifact::{
    NarrowedPolicyQueryArtifact, PolicyAwareOptimizerInput, PolicyAwareValidationReport,
    PolicyNarrowingCostPosture, PolicyNarrowingWorkBudget, SavedPolicyNarrowingReuseDescriptor,
    SavedPolicyNarrowingReuseDisposition,
};
pub use counters::PolicyNarrowingCounters;
pub use errors::{PolicyNarrowingError, PolicyNarrowingFailureClass};
pub use lowering::{
    classify_saved_policy_narrowing_reuse, narrow_policy_query,
    optimizer_input_from_narrowed_policy_query,
};
pub use support::{
    runtime_backed_policy_narrowing_support_profile, PolicyNarrowingSupportProfile,
    PolicyNarrowingSupportStatus, PolicyNarrowingSurface,
};
pub(crate) use validation::validate_narrowing_budget;

#[cfg(test)]
mod tests;
