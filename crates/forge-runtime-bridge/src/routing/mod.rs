//! Routing proof chain for canonical bridge planning and lowering.

pub(crate) mod canonicalization;
pub(crate) mod context;
pub(crate) mod counters;
pub(crate) mod eligibility;
pub(crate) mod lowering;
pub(crate) mod matching;
pub(crate) mod outcome;
pub(crate) mod planning;
pub(crate) mod proof;
pub(crate) mod result;
pub(crate) mod scope;
pub(crate) mod surfaces;

pub(crate) use planning::replay_route_record;
pub(crate) use planning::IngestedBridgePatch;

pub use counters::BridgeRoutingCounters;
pub use context::{BridgeLineageContext, BridgeMappingContext};
pub use matching::{FineGrainedMatchOutcome, FineGrainedMatchStatus};
pub use lowering::{
    BridgeInvalidationArtifact, BridgeInvalidationIdentity, BridgeInvalidationTarget,
    BridgeLoweringPlanSummary, BridgeLoweringProvenance, BridgeLoweringSummary,
    BridgeSignalInvalidationDelivery, BridgeSubscriptionSlice, BridgeSubscriptionSliceIdentity,
    CanonicalInvalidationTargets, CanonicalSubscriptionSlices,
};
pub use outcome::BridgeRouteOutcomeReference;
pub use planning::{
    BridgeExecutionCounts, BridgePlannedRoute, BridgePlanningProvenance, BridgePlanningSummary,
    BridgeRouteIdentity, BridgeRouteSourceSummary, BridgeRoutingSummary,
};
pub use proof::BridgeRouteContractProof;
pub use result::{BridgeRouteResult, BridgeRouteResultSummary};
