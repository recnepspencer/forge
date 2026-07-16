mod artifacts;
mod branch;
mod current;
mod diff;
mod historical;
mod optimizer;

pub use artifacts::{
    PolicyAwarePlanCore, PolicyAwarePlanCostPosture, PolicyAwarePlanDigest,
    PolicyAwarePlanLoweringReport, PolicyAwarePlanWorkBudget,
};
#[cfg(test)]
pub(crate) use branch::lower_policy_aware_branch_plan;
pub use branch::{PolicyAwareBranchPlan, PolicyAwareReadBasis};
pub(crate) use current::lower_policy_aware_current_plan;
pub use current::PolicyAwareCurrentPlan;
#[cfg(test)]
pub(crate) use diff::lower_policy_aware_diff_plan;
pub use diff::{
    deny_raw_diff_scrub, PolicyAwareDiffBasisPair, PolicyAwareDiffPlan,
    PolicyAwareDiffScrubDisposition,
};
#[cfg(test)]
pub(crate) use historical::lower_policy_aware_historical_plan;
pub use historical::{
    defer_store_backed_policy_historical_plan, PolicyAwareHistoricalBasis,
    PolicyAwareHistoricalPlan,
};
#[cfg(test)]
pub(crate) use optimizer::lower_policy_aware_optimizer_input;
#[cfg(test)]
mod tests;
