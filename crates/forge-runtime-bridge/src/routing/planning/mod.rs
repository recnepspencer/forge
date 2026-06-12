mod bulk;
mod canonical;
mod ingestion;
mod plan;
mod replay;
mod summaries;

use crate::identity::{BridgeIdentity, BridgeIdentityEvidence, RouteIdentityTag};

pub type BridgeRouteIdentity = BridgeIdentity<RouteIdentityTag>;

impl BridgeRouteIdentity {
    pub fn from_bridge_evidence(evidence_identity: &BridgeIdentityEvidence) -> Self {
        Self::new(format!(
            "bridge-route:external-authority-evidence:{}",
            evidence_identity.as_str()
        ))
    }

    pub fn from_external_authority_evidence(evidence_identity: impl AsRef<str>) -> Self {
        Self::from_bridge_evidence(&BridgeIdentityEvidence::from_external_authority(
            evidence_identity,
        ))
    }
}

pub use bulk::{
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
pub use plan::BridgePlannedRoute;
pub use summaries::{
    BridgeExecutionCounts, BridgePlanningProvenance, BridgePlanningSummary,
    BridgeRouteSourceSummary, BridgeRoutingSummary,
};

pub(crate) use bulk::{plan_bulk_workload, plan_bulk_workload_with_route_policy};
pub(crate) use ingestion::IngestedBridgePatch;
pub(crate) use plan::{plan_ingested_patch, BridgePreparedDelivery};
pub(crate) use replay::replay_route_record;
