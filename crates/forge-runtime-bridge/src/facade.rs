//! Public API boundary for `forge-runtime-bridge`.
//! External crates should import through this module rather than reaching into
//! internal crate structure directly.

use std::sync::Arc;

use crate::mapping::{FrozenAspectMappingRegistry, FrozenMappingRegistry};
pub use crate::adapter::{
    BridgeHistoricalLineageAuthority, BridgeHistoricalLineageRequest, BridgeHistoricalLineageTopology, CommittedPatchSource,
    ContinuityLineageSource, InvalidationSink, RelationalBridgeSource, RelationalBridgeSourceError,
    RelationalCommittedPatchRequest, SignalBridgeSink, SignalBridgeSinkError, SnapshotReadSource,
    SnapshotReaderPool, TruthBranchHeadSource,
};
use crate::diagnostics::DiagnosticSink;
pub use crate::builder::RuntimeBridgeBuilder;
pub use crate::delivery::{
    BridgeDeliveryReceipt, BridgePreparedDeliveryRequest, BridgeSignalEvaluationRequest,
};
pub use crate::diagnostics::{
    BridgeBulkPlanExplanation, BridgeCanonicalContinuityRecord, BridgeCanonicalRouteRecord,
    BridgeContinuityExplanation, BridgeContinuityReplaySummary, BridgeDeliveredContinuityResult,
    BridgeDiagnosticsFacade, BridgeDiagnosticsHandle,
    BridgeContractDiagnosticsRecord, BridgeFailureClass, BridgeFailureRecord,
    BridgeCanonicalHistoricalEvaluationRecord, BridgeHistoricalEvaluationCounters,
    BridgeHistoricalEvaluationDecisionLog, BridgeHistoricalEvaluationExplanation,
    BridgeHistoricalEvaluationFailureClass, BridgeHistoricalEvaluationFailureRecord,
    BridgeHistoricalEvaluationRecord,
    BridgeHistoricalEvaluationReplaySummary, BridgeHistoricalMaterializationPath,
    BridgeLoweringDiagnosticsRecord, BridgeReplayRecord, BridgeReplaySummary,
    BridgeStreamCheckpointExplanation, BridgeStreamReplayExplanation,
    BridgeStreamResumeSummary,
    BridgeRouteExplanation, BridgeRouteExplanationEntry, BridgeRouteRecord, BridgeRouteRecordMatch,
    BridgeRouteRecordEntry, BridgeRouteSourceRecord, BridgeRoutingDiagnosticsRecord,
    BRIDGE_CANONICAL_HISTORICAL_EVALUATION_RECORD_SCHEMA_V1,
    BRIDGE_CANONICAL_CONTINUITY_RECORD_SCHEMA_V1,
    BRIDGE_CANONICAL_ROUTE_RECORD_SCHEMA_V3,
};
pub use crate::error::{
    BridgeBuildError, BridgeBuildErrorKind, BridgeContinuityError, BridgeContinuityErrorKind,
    BridgeDeliveryError, BridgeDeliveryErrorKind, BridgeErrorContext, BridgeLineageSourceError,
    BridgeLineageSourceErrorKind, BridgePatchCoordinate, BridgeReplayError,
    BridgeReplayErrorKind, BridgeRouteError, BridgeRouteErrorKind, BridgeSnapshotReadCoordinate,
    BridgeStreamError, BridgeStreamErrorKind,
};
pub use crate::input::envelope::{
    BridgeCommittedPatchBody, BridgeCommittedPatchDigest, BridgeCommittedPatchEnvelope,
    BridgeCommittedPatchItem, BridgeCommittedPatchSummary, BridgeProducerAuthorityKind,
    BridgeProducerMetadata, RawCommittedPatchEnvelope, TruthBranchIdentity, TruthCommitIdentity,
    TruthPatchIdentity, BRIDGE_PRODUCER_EXPORT_SCHEMA_V1,
};
pub use crate::mapping::{
    BridgeAspectRegistration, BridgeAspectRegistrationId, BridgeMappingFallbackClass,
    BridgeMappingId, BridgeMappingRegistration, CoarseRoutingMode, MappingSelector,
    SignalInvalidationScope, SliceFallbackPolicy, SubscriptionSliceKind, TruthDeltaSurfaceKind,
    TruthPatchScope,
};
pub use crate::continuity::{
    BridgeContinuityAuthorityBasis, BridgeContinuityAuthorityKind, BridgeContinuityClass,
    BridgeContinuityArtifact, BridgeContinuityCounters, BridgeContinuityIdentity,
    BridgeContinuityDigestBasisKind, BridgeContinuityOutcomeClass,
    BridgeContinuityRejectionClass, BridgeEligibleContinuityRequestSet,
    BridgeHistoricalLineagePacket, BridgeHistoricalLineagePacketEntry,
    BridgePlannedContinuityRequest, BridgePlannedContinuityRequestSet,
    BridgeUnsupportedContinuityClass, PriorSubscriptionSlice, ResolvedLineageContinuity,
    ResolvedLineageContinuitySet,
};
pub use crate::policy::{BridgeDiagnosticsRetentionBudget, BridgeDiagnosticsTier, BridgeRuntimePolicy};
pub use crate::routing::{
    AdmittedBridgeExecutionPlan, AdmittedPreparationPartitionSet, BridgeAdmissionProfileIdentity,
    BridgeBulkPlanningCounters, BridgeBulkPlanningSummary, BridgeBulkWorkloadPlan, BridgeBulkWorkloadRequest,
    BridgeBulkDecisionLog, BridgeBulkDecisionRecord, BridgeBulkDecisionRecordKind,
    BridgeBulkPlanningFailure, BridgeBulkPlanningFailureKind, BridgeBulkWorkloadSegment,
    BridgeCanonicalBulkPlanRecord, BRIDGE_CANONICAL_BULK_PLAN_RECORD_SCHEMA_V1,
    BridgeCanonicalPlanningIdentity, BridgeExecutionCounts, BridgeLocalityFootprint,
    BridgeInvalidationArtifact, BridgeInvalidationIdentity, BridgeInvalidationTarget,
    BridgeLoweringPlanSummary, BridgeLoweringProvenance, BridgeLoweringSummary,
    BridgeParallelAdmission, BridgeParallelAdmissionClass, BridgeParallelAdmissionReason,
    BridgeParallelLegalityClass, BridgeParallelLegalityDecision, BridgeParallelLegalityReason,
    BridgeParallelProfitabilityClass, BridgeParallelProfitabilityDecision,
    BridgeParallelProfitabilityReason,
    BridgePlannedRoute, BridgePlanningProvenance, BridgePlanningSummary,
    BridgePreparationMode, BridgeRouteIdentity, BridgeRouteContractProof, BridgeRouteOutcomeReference,
    BridgeBulkResultSummary, BridgeBulkWorkloadResult, BridgeRouteResult, BridgeRouteResultSummary,
    BridgeRoutingCounters, BridgeRoutingSummary,
    BridgeLineageContext, BridgeMappingContext, BridgeRouteSourceSummary, BridgeWorkloadIdentity,
    CanonicalBridgeWorkloadRequest, ContinuityPacketIdentity, ContinuityRemapPacket, DisjointPacketRegionSet,
    FallbackAggregationPacket, FallbackPacketIdentity, NormalizedBridgeWorkloadSummary,
    InvalidationReductionPacket, ParallelPreparationLegalityProof, PlannedBridgePacketSet,
    ReducedBridgePublication, ReducedBridgeWorkloadArtifact, ReducedContinuityIdentity,
    ReducedContinuityRemap, ReducedFallbackAggregation, ReducedFallbackIdentity,
    ReducedPublicationIdentity, ReducedRoutingTargetIdentity, ReducedTruthViewIdentity,
    ReducedTruthViewMaterialization, ReductionPacketIdentity, RoutingPacketIdentity,
    TruthDeltaRoutingPacket, TruthViewMaterializationPacket, TruthViewPacketIdentity,
    FineGrainedMatchOutcome, FineGrainedMatchStatus, BridgeSignalInvalidationDelivery, BridgeSubscriptionSlice,
    BridgeSubscriptionSliceIdentity, CanonicalInvalidationTargets, CanonicalSubscriptionSlices,
};
pub use crate::snapshot::{
    AdmittedSnapshotContext, BridgeDeliveryIntent, BridgeReplayMode, BridgeSnapshotContext,
    BridgeSnapshotReadError, BridgeSnapshotToken, BridgeTruthViewKind,
    BridgeTruthViewPolicyRejection, BridgeTruthViewPolicyResolution, BridgeTruthViewSelector,
    BridgeTruthViewSelectorIdentity, HistoricalEvaluationDeclaration,
    HistoricalEvaluationDeclarationIdentity, LoweredHistoricalEvaluationArtifact,
    LoweredHistoricalEvaluationArtifactIdentity, ResolvedTruthViewPolicy, SnapshotReadPacket,
    SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadRequest, TruthSnapshotIdentity,
    TruthSnapshotReader, TruthViewObservationReader, TruthViewPolicyRejectionKind, TruthViewReplayCompatibility,
    TruthViewRetentionAdmission, TruthViewSourceCapability, ValidatedSnapshotReadPacketResult,
    ValidatedTruthViewSelectorSet,
};
pub use crate::stream::{
    AdmittedConsumerContract, BackpressureDecisionRecord, CanonicalStreamMember,
    CanonicalStreamPosition, CanonicalStreamReplayRecord, ChangeStreamDeclaration,
    ChangeStreamDeclarationIdentity, ConsumerCheckpointToken, ConsumerContractIdentity,
    LoweredConsumedChangeSet,
    PlannedChangeStreamWindow, StreamCheckpointFrontierKind,
    StreamCheckpointPublicationMode, StreamCoalescingFamily, StreamCoalescingIntent,
    StreamConsumerShape, StreamDeliveryIntent, StreamDiagnosticsPolicyClass,
    StreamProtocolIdentity, StreamReplayAuditResult, StreamReplayAuditSummary,
    StreamProtocolCounters,
    StreamReplayMode, StreamReplayRecordIdentity, StreamResumeMode,
    StreamWindowDeliveryResult, StreamWindowDeliverySummary,
    StreamWindowIdentity,
    ValidatedStreamProtocol,
};


mod request;
mod runtime;

pub use request::BridgeRouteRequest;
pub use runtime::RuntimeBridge;

#[cfg(test)]
mod tests;
