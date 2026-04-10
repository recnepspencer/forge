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
    BridgeBulkDecisionLog, BridgeBulkDecisionRecord, BridgeBulkDecisionRecordKind,
    BridgeBulkPlanningCounters, BridgeBulkPlanningFailure, BridgeBulkPlanningFailureKind,
    BridgeBulkPlanningSummary, BridgeBulkWorkloadPlan, BridgeBulkWorkloadRequest,
    BridgeBulkWorkloadSegment, BridgeCanonicalBulkPlanRecord, BridgeCanonicalPlanningIdentity,
    BridgeLocalityFootprint, BridgeParallelAdmission, BridgeParallelAdmissionClass,
    BridgeParallelAdmissionReason, BridgeParallelLegalityClass, BridgeParallelLegalityDecision,
    BridgeParallelLegalityReason, BridgeParallelProfitabilityClass,
    BridgeParallelProfitabilityDecision, BridgeParallelProfitabilityReason, BridgePreparationMode,
    BridgeWorkloadIdentity, CanonicalBridgeWorkloadRequest, ContinuityPacketIdentity,
    ContinuityRemapPacket, DisjointPacketRegionSet, FallbackAggregationPacket,
    FallbackPacketIdentity, InvalidationReductionPacket, NormalizedBridgeWorkloadSummary,
    ParallelPreparationLegalityProof, PlannedBridgePacketSet, ReducedBridgePublication,
    ReducedBridgeWorkloadArtifact, ReducedContinuityIdentity, ReducedContinuityRemap,
    ReducedFallbackAggregation, ReducedFallbackIdentity, ReducedPublicationIdentity,
    ReducedRoutingTargetIdentity, ReducedTruthViewIdentity, ReducedTruthViewMaterialization,
    ReductionPacketIdentity, RoutingPacketIdentity, TruthDeltaRoutingPacket,
    TruthViewMaterializationPacket, TruthViewPacketIdentity,
    BRIDGE_CANONICAL_BULK_PLAN_RECORD_SCHEMA_V1,
};
pub use plan::BridgePlannedRoute;
pub use summaries::{
    BridgeExecutionCounts, BridgePlanningProvenance, BridgePlanningSummary,
    BridgeRouteSourceSummary, BridgeRoutingSummary,
};

pub(crate) use bulk::{plan_bulk_workload, plan_bulk_workload_with_route_policy};
pub(crate) use ingestion::IngestedBridgePatch;
pub(crate) use plan::{plan_ingested_patch, BridgePreparedDelivery};
pub(crate) use replay::replay_route_record;
