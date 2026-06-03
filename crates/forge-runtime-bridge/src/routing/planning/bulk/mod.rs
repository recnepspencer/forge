mod planner;
mod types;

pub use types::{
    AdmittedBridgeExecutionPlan, AdmittedPreparationPartitionSet, BridgeAdmissionProfileIdentity,
    BridgeBulkDecisionLog, BridgeBulkDecisionRecord, BridgeBulkDecisionRecordKind,
    BridgeBulkPlanningCounters, BridgeBulkPlanningFailure, BridgeBulkPlanningFailureKind,
    BridgeBulkPlanningSummary, BridgeBulkWorkloadPlan, BridgeBulkWorkloadRequest,
    BridgeBulkWorkloadSegment, BridgeCanonicalBulkPlanRecord, BridgeCanonicalPlanningIdentity,
    BridgeInvalidationReductionFamily, BridgeLocalityFootprint, BridgeParallelAdmission,
    BridgeParallelAdmissionClass, BridgeParallelAdmissionReason, BridgeParallelLegalityClass,
    BridgeParallelLegalityDecision, BridgeParallelLegalityReason, BridgeParallelProfitabilityClass,
    BridgeParallelProfitabilityDecision, BridgeParallelProfitabilityReason, BridgePreparationMode,
    BridgeWorkloadIdentity, BulkContinuityMemberIdentity, BulkPacketRegionIdentity,
    BulkTruthViewMemberIdentity, BulkWorkloadSegmentIdentity, CanonicalBridgeWorkloadRequest,
    ContinuityPacketIdentity, ContinuityRemapPacket, DisjointPacketRegionSet,
    InvalidationReductionPacket, NormalizedBridgeWorkloadSummary, ParallelPreparationLegalityProof,
    PlannedBridgePacketSet, ReducedBridgePublication, ReducedBridgeWorkloadArtifact,
    ReducedContinuityIdentity, ReducedContinuityRemap, ReducedPublicationIdentity,
    ReducedRoutingTargetIdentity, ReducedTruthViewIdentity, ReducedTruthViewMaterialization,
    ReducedWideningAggregation, ReducedWideningIdentity, ReductionPacketIdentity,
    RoutingPacketIdentity, TruthDeltaRoutingPacket, TruthViewMaterializationPacket,
    TruthViewPacketIdentity, WideningAggregationPacket, WideningPacketIdentity,
    BRIDGE_CANONICAL_BULK_PLAN_RECORD_SCHEMA_V1,
};

pub(crate) use planner::{plan_bulk_workload, plan_bulk_workload_with_route_policy};
