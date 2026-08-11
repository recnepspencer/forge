mod artifact;
mod budget;
mod counters;
mod errors;
mod lowering;
mod optimizer_input;
mod reuse;
mod support;
mod validation;

pub use artifact::NarrowedPolicyQueryArtifact;
pub use budget::{PolicyNarrowingCostPosture, PolicyNarrowingWorkBudget};
pub use counters::PolicyNarrowingCounters;
pub use errors::{PolicyNarrowingError, PolicyNarrowingFailureClass};
pub use lowering::{
    classify_saved_policy_narrowing_reuse, narrow_policy_query,
    optimizer_input_from_narrowed_policy_query,
};
pub use optimizer_input::PolicyAwareOptimizerInput;
pub use reuse::{SavedPolicyNarrowingReuseDescriptor, SavedPolicyNarrowingReuseDisposition};
pub use support::{
    runtime_backed_policy_narrowing_support_profile, PolicyNarrowingSupportProfile,
    PolicyNarrowingSupportStatus, PolicyNarrowingSurface,
};
pub(crate) use validation::validate_narrowing_budget;
pub use validation::PolicyAwareValidationReport;

#[cfg(test)]
mod tests;
