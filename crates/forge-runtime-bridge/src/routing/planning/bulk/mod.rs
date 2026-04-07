mod planner;
mod types;

pub use types::{
    AdmittedBridgeExecutionPlan, AdmittedPreparationPartitionSet, BridgeAdmissionProfileIdentity,
    BridgeBulkDecisionLog, BridgeBulkDecisionRecord, BridgeBulkDecisionRecordKind,
    BridgeBulkPlanningCounters, BridgeBulkPlanningFailure, BridgeBulkPlanningFailureKind,
    BridgeBulkPlanningSummary, BridgeBulkWorkloadPlan, BridgeBulkWorkloadRequest,
    BridgeBulkWorkloadSegment, BridgeCanonicalBulkPlanRecord, BridgeCanonicalPlanningIdentity,
    BridgeLocalityFootprint, BridgeParallelAdmission, BridgeParallelAdmissionClass,
    BridgeParallelAdmissionReason, BridgeParallelLegalityClass, BridgeParallelLegalityDecision,
    BridgeParallelLegalityReason, BridgeParallelProfitabilityClass,
    BridgeParallelProfitabilityDecision, BridgeParallelProfitabilityReason,
    BridgePreparationMode, BridgeWorkloadIdentity, CanonicalBridgeWorkloadRequest,
    ContinuityPacketIdentity, ContinuityRemapPacket, DisjointPacketRegionSet,
    FallbackAggregationPacket, FallbackPacketIdentity, InvalidationReductionPacket,
    NormalizedBridgeWorkloadSummary, ParallelPreparationLegalityProof, PlannedBridgePacketSet,
    ReducedBridgePublication, ReducedBridgeWorkloadArtifact, ReducedContinuityIdentity,
    ReducedContinuityRemap, ReducedPublicationIdentity, ReducedTruthViewIdentity,
    ReducedTruthViewMaterialization, ReductionPacketIdentity, RoutingPacketIdentity,
    TruthDeltaRoutingPacket, TruthViewMaterializationPacket, TruthViewPacketIdentity,
    BRIDGE_CANONICAL_BULK_PLAN_RECORD_SCHEMA_V1,
};

pub(crate) use planner::plan_bulk_workload;
