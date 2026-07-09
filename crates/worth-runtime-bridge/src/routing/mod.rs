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

pub use context::{BridgeLineageContext, BridgeMappingContext};
pub use counters::BridgeRoutingCounters;
pub use lowering::{
    BridgeInvalidationArtifact, BridgeInvalidationIdentity, BridgeInvalidationTarget,
    BridgeInvalidationTargetIdentity, BridgeLoweringPlanSummary, BridgeLoweringProvenance,
    BridgeLoweringSummary, BridgeSignalInvalidationDelivery, BridgeSubscriptionSlice,
    BridgeSubscriptionSliceIdentity, CanonicalInvalidationTargets, CanonicalSubscriptionSlices,
};
pub use matching::{FineGrainedMatchOutcome, FineGrainedMatchStatus};
pub use outcome::BridgeRouteOutcomeReference;
pub use planning::{
    AdmittedBridgeExecutionPlan, AdmittedPreparationPartitionSet, BridgeAdmissionProfileIdentity,
    BridgeBulkDecisionLog, BridgeBulkDecisionRecord, BridgeBulkDecisionRecordKind,
    BridgeBulkPlanningCounters, BridgeBulkPlanningFailure, BridgeBulkPlanningFailureKind,
    BridgeBulkPlanningSummary, BridgeBulkWorkloadPlan, BridgeBulkWorkloadRequest,
    BridgeBulkWorkloadSegment, BridgeCanonicalBulkPlanRecord, BridgeCanonicalPlanningIdentity,
    BridgeExecutionCounts, BridgeInvalidationReductionFamily, BridgeLocalityFootprint,
    BridgeParallelAdmission, BridgeParallelAdmissionClass, BridgeParallelAdmissionReason,
    BridgeParallelLegalityClass, BridgeParallelLegalityDecision, BridgeParallelLegalityReason,
    BridgeParallelProfitabilityClass, BridgeParallelProfitabilityDecision,
    BridgeParallelProfitabilityReason, BridgePlannedRoute, BridgePlanningProvenance,
    BridgePlanningSummary, BridgePreparationMode, BridgeRouteIdentity, BridgeRouteSourceSummary,
    BridgeRoutingSummary, BridgeWorkloadIdentity, BulkContinuityMemberIdentity,
    BulkPacketRegionIdentity, BulkTruthViewMemberIdentity, BulkWorkloadSegmentIdentity,
    CanonicalBridgeWorkloadRequest, ContinuityPacketIdentity, ContinuityRemapPacket,
    DisjointPacketRegionSet, InvalidationReductionPacket, NormalizedBridgeWorkloadSummary,
    ParallelPreparationLegalityProof, PlannedBridgePacketSet, ReducedBridgePublication,
    ReducedBridgeWorkloadArtifact, ReducedContinuityIdentity, ReducedContinuityRemap,
    ReducedPublicationIdentity, ReducedRoutingTargetIdentity, ReducedTruthViewIdentity,
    ReducedTruthViewMaterialization, ReducedWideningAggregation, ReducedWideningIdentity,
    ReductionPacketIdentity, RoutingPacketIdentity, TruthDeltaRoutingPacket,
    TruthViewMaterializationPacket, TruthViewPacketIdentity, WideningAggregationPacket,
    WideningPacketIdentity, BRIDGE_CANONICAL_BULK_PLAN_RECORD_SCHEMA_V1,
};
pub use proof::BridgeRouteContractProof;
pub use result::{
    BridgeBulkResultSummary, BridgeBulkWorkloadResult, BridgeRouteResult, BridgeRouteResultSummary,
};
