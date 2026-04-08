mod bulk;
mod canonical;
mod ingestion;
mod plan;
mod replay;
mod summaries;

use crate::identity::{BridgeIdentity, RouteIdentityTag};

pub type BridgeRouteIdentity = BridgeIdentity<RouteIdentityTag>;

pub use bulk::{
    AdmittedBridgeExecutionPlan, AdmittedPreparationPartitionSet, BridgeAdmissionProfileIdentity,
    BridgeBulkPlanningCounters, BridgeBulkPlanningSummary, BridgeBulkWorkloadPlan, BridgeBulkWorkloadRequest,
    BridgeBulkDecisionLog, BridgeBulkDecisionRecord, BridgeBulkDecisionRecordKind,
    BridgeBulkPlanningFailure, BridgeBulkPlanningFailureKind, BridgeBulkWorkloadSegment,
    BridgeCanonicalBulkPlanRecord, BRIDGE_CANONICAL_BULK_PLAN_RECORD_SCHEMA_V1,
    BridgeCanonicalPlanningIdentity, BridgeLocalityFootprint, BridgeParallelAdmission,
    BridgeParallelAdmissionClass, BridgeParallelAdmissionReason, BridgeParallelLegalityClass,
    BridgeParallelLegalityDecision, BridgeParallelLegalityReason, BridgeParallelProfitabilityClass,
    BridgeParallelProfitabilityDecision, BridgeParallelProfitabilityReason, BridgePreparationMode,
    BridgeWorkloadIdentity, CanonicalBridgeWorkloadRequest, ContinuityPacketIdentity, ContinuityRemapPacket, DisjointPacketRegionSet,
    FallbackAggregationPacket, FallbackPacketIdentity, NormalizedBridgeWorkloadSummary,
    InvalidationReductionPacket, ParallelPreparationLegalityProof, PlannedBridgePacketSet,
    ReducedBridgePublication, ReducedBridgeWorkloadArtifact, ReducedContinuityIdentity,
    ReducedContinuityRemap, ReducedFallbackAggregation, ReducedFallbackIdentity,
    ReducedPublicationIdentity, ReducedRoutingTargetIdentity, ReducedTruthViewIdentity,
    ReducedTruthViewMaterialization, ReductionPacketIdentity, RoutingPacketIdentity,
    TruthDeltaRoutingPacket, TruthViewMaterializationPacket, TruthViewPacketIdentity,
};
pub use summaries::{
    BridgeExecutionCounts, BridgePlanningProvenance, BridgePlanningSummary,
    BridgeRouteSourceSummary, BridgeRoutingSummary,
};
pub use plan::BridgePlannedRoute;

pub(crate) use ingestion::IngestedBridgePatch;
pub(crate) use bulk::plan_bulk_workload;
pub(crate) use plan::{BridgePreparedDelivery, plan_ingested_patch};
pub(crate) use replay::replay_route_record;
