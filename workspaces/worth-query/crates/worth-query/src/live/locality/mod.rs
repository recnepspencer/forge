mod admission;
#[cfg(test)]
mod execution;
mod matching;
#[cfg(test)]
mod planning;
mod scope_contract;

pub use admission::{
    LocalityAdmissionClass, LocalityBreadthBudget, LocalityCostPosture, LocalityMaintenanceClass,
    LocalityPerformanceStatus, LocalityScopeAdmission, LocalitySemanticBasis,
    LocalityWideningBudget, LocalityWideningPolicy, StreamLoweringAdmissionClass,
    StreamLoweringCostPosture, StreamMemberWidthBudget, StreamWindowWidthBudget,
};
#[cfg(test)]
pub use execution::{
    RegionScopedExecutionReport, RegionScopedLiveError, RegionScopedLiveExecutionEnvelope,
};
#[cfg(test)]
pub use matching::RegionScopedSubscriptionIdentity;
pub use matching::{
    LocalityAwareRelevanceContract, LocalityMatchClass, LocalityMatchKind,
    LocalityWideningDecision, PartitionSliceMatch, RegionSliceMatch,
};
#[cfg(test)]
pub use planning::{RegionScopedLivePlan, RegionScopedPlanningReport};
pub use scope_contract::{LocalityPredicateContract, LocalityScopeDigest, LocalityScopeKind};
