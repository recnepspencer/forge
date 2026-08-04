mod admission;
mod execution;
mod matching;
mod planning;
mod scope_contract;

pub use admission::{
    LocalityAdmissionClass, LocalityBreadthBudget, LocalityCostPosture, LocalityMaintenanceClass,
    LocalityPerformanceStatus, LocalityScopeAdmission, LocalitySemanticBasis,
    LocalityWideningBudget, LocalityWideningPolicy, StreamLoweringAdmissionClass,
    StreamLoweringCostPosture, StreamMemberWidthBudget, StreamWindowWidthBudget,
};
pub use execution::{
    RegionScopedExecutionReport, RegionScopedLiveError, RegionScopedLiveExecutionEnvelope,
};
pub use matching::{
    LocalityAwareRelevanceContract, LocalityMatchClass, LocalityMatchKind,
    LocalityWideningDecision, PartitionSliceMatch, RegionScopedSubscriptionIdentity,
    RegionSliceMatch,
};
pub use planning::{RegionScopedLivePlan, RegionScopedPlanningReport};
pub use scope_contract::{LocalityPredicateContract, LocalityScopeDigest, LocalityScopeKind};
