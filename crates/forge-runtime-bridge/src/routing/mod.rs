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
    AdmittedBridgeExecutionPlan, AdmittedPreparationPartitionSet, BridgeAdmissionProfileIdentity,
    BridgeBulkPlanningCounters, BridgeBulkPlanningSummary, BridgeBulkWorkloadPlan, BridgeBulkWorkloadRequest,
    BridgeBulkDecisionLog, BridgeBulkDecisionRecord, BridgeBulkDecisionRecordKind,
    BridgeBulkPlanningFailure, BridgeBulkPlanningFailureKind, BridgeBulkWorkloadSegment,
    BridgeCanonicalBulkPlanRecord, BRIDGE_CANONICAL_BULK_PLAN_RECORD_SCHEMA_V1,
    BridgeCanonicalPlanningIdentity, BridgeExecutionCounts, BridgeLocalityFootprint,
    BridgeParallelAdmission, BridgeParallelAdmissionClass, BridgeParallelAdmissionReason,
    BridgeParallelLegalityClass, BridgeParallelLegalityDecision, BridgeParallelLegalityReason,
    BridgeParallelProfitabilityClass, BridgeParallelProfitabilityDecision,
    BridgeParallelProfitabilityReason,
    BridgePlannedRoute, BridgePlanningProvenance, BridgePlanningSummary,
    BridgePreparationMode, BridgeRouteIdentity, BridgeRouteSourceSummary, BridgeRoutingSummary, BridgeWorkloadIdentity,
    CanonicalBridgeWorkloadRequest, ContinuityPacketIdentity, ContinuityRemapPacket, DisjointPacketRegionSet,
    FallbackAggregationPacket, FallbackPacketIdentity, NormalizedBridgeWorkloadSummary,
    InvalidationReductionPacket, ParallelPreparationLegalityProof, PlannedBridgePacketSet,
    ReducedBridgePublication, ReducedBridgeWorkloadArtifact, ReducedContinuityIdentity,
    ReducedContinuityRemap, ReducedFallbackAggregation, ReducedFallbackIdentity,
    ReducedPublicationIdentity, ReducedRoutingTargetIdentity, ReducedTruthViewIdentity,
    ReducedTruthViewMaterialization, ReductionPacketIdentity, RoutingPacketIdentity,
    TruthDeltaRoutingPacket, TruthViewMaterializationPacket, TruthViewPacketIdentity,
};
pub use proof::BridgeRouteContractProof;
pub use result::{
    BridgeBulkResultSummary, BridgeBulkWorkloadResult, BridgeRouteResult, BridgeRouteResultSummary,
};
