//! Public API boundary for `forge-runtime-bridge`.
//! External crates should import through this module rather than reaching into
//! internal crate structure directly.

use std::sync::Arc;

pub use crate::adapter::{
    BridgeHistoricalLineageAuthority, BridgeHistoricalLineageRequest,
    BridgeHistoricalLineageTopology, BridgeSourceAdapter, CommittedPatchSource,
    ContinuityLineageSource, InvalidationSink, RelationalBridgeSource, RelationalBridgeSourceError,
    RelationalCommittedPatchRequest, SignalBridgeSink, SignalBridgeSinkError, SnapshotReadSource,
    SnapshotReaderPool, TruthBranchHeadSource, TruthWritebackAuthority,
    TruthWritebackAuthorityError, TruthWritebackReceipt, TruthWritebackRequest,
};
pub use crate::builder::RuntimeBridgeBuilder;
pub use crate::continuity::{
    BridgeContinuityArtifact, BridgeContinuityAuthorityBasis, BridgeContinuityAuthorityKind,
    BridgeContinuityClass, BridgeContinuityCounters, BridgeContinuityDigestBasisKind,
    BridgeContinuityIdentity, BridgeContinuityOutcomeClass, BridgeContinuityRejectionClass,
    BridgeEligibleContinuityRequestSet, BridgeHistoricalLineagePacket,
    BridgeHistoricalLineagePacketEntry, BridgePlannedContinuityRequest,
    BridgePlannedContinuityRequestSet, BridgeUnsupportedContinuityClass, PriorSubscriptionSlice,
    ResolvedLineageContinuity, ResolvedLineageContinuitySet,
};
pub use crate::delivery::{
    BridgeDeliveryReceipt, BridgePreparedDeliveryRequest, BridgeSignalEvaluationRequest,
};
use crate::diagnostics::DiagnosticSink;
pub use crate::diagnostics::{
    BridgeBulkPlanExplanation, BridgeCanonicalContinuityRecord,
    BridgeCanonicalHistoricalEvaluationRecord, BridgeCanonicalMergeRecord,
    BridgeCanonicalRouteRecord, BridgeCanonicalStructuralBranchComparisonRecord,
    BridgeCanonicalStructuralRemapRecord, BridgeContinuityExplanation,
    BridgeContinuityReplaySummary, BridgeContractDiagnosticsRecord,
    BridgeDeliveredContinuityResult, BridgeDiagnosticsFacade, BridgeDiagnosticsHandle,
    BridgeFailureClass, BridgeFailureRecord, BridgeHistoricalEvaluationCounters,
    BridgeHistoricalEvaluationDecisionLog, BridgeHistoricalEvaluationExplanation,
    BridgeHistoricalEvaluationFailureClass, BridgeHistoricalEvaluationFailureRecord,
    BridgeHistoricalEvaluationRecord, BridgeHistoricalEvaluationReplaySummary, BridgePolicyExplanation,
    BridgePolicyExplanationRow, BridgePolicyRejectionExplanation,
    BridgeHistoricalMaterializationPath, BridgeLoweringDiagnosticsRecord, BridgeMergeExplanation,
    BridgeMergeRecord, BridgeMergeRecordIdentity, BridgeMergeReplaySummary,
    BridgePreviewDiscardExplanation, BridgePreviewExecutionExplanation,
    BridgePreviewPromotionExplanation, BridgePreviewReplayExplanation, BridgeReplayRecord,
    BridgeReplaySummary, BridgeRouteExplanation, BridgeRouteExplanationEntry, BridgeRouteRecord,
    BridgeRouteRecordEntry, BridgeRouteRecordMatch, BridgeRouteSourceRecord,
    BridgeRoutingDiagnosticsRecord, BridgeSourceFailureExplanation,
    BridgeSourceMaterializationExplanation, BridgeStreamCheckpointExplanation,
    BridgeStreamReplayExplanation, BridgeStreamResumeSummary,
    BridgeStructuralBranchComparisonExplanation, BridgeStructuralBranchComparisonRecord,
    BridgeStructuralBranchComparisonReplaySummary, BridgeStructuralCounters,
    BridgeStructuralRemapExplanation, BridgeStructuralRemapRecord,
    BridgeStructuralRemapReplaySummary, BridgeMappedWritebackFamilyInputExplanation,
    BridgeWritebackAdmissionExplanation,
    BridgeWritebackCandidateExplanation,
    BridgeWritebackExecutionExplanation, BridgeWritebackLoopPreventionExplanation,
    BridgeWritebackMapperEnvelopeExplanation, BridgeWritebackMapperExplanation,
    BridgeWritebackOutcomeExplanation, BridgeWritebackReplayExplanation,
    BridgeWritebackReplayRecordExplanation, BridgeWritebackStrategyCompatibilityExplanation,
    BRIDGE_CANONICAL_CONTINUITY_RECORD_SCHEMA_V1,
    BRIDGE_CANONICAL_HISTORICAL_EVALUATION_RECORD_SCHEMA_V1,
    BRIDGE_CANONICAL_MERGE_RECORD_SCHEMA_V1, BRIDGE_CANONICAL_ROUTE_RECORD_SCHEMA_V3,
    BRIDGE_CANONICAL_STRUCTURAL_BRANCH_COMPARISON_RECORD_SCHEMA_V1,
    BRIDGE_CANONICAL_STRUCTURAL_REMAP_RECORD_SCHEMA_V1,
};
pub use crate::error::{
    BridgeBuildError, BridgeBuildErrorKind, BridgeContinuityError, BridgeContinuityErrorKind,
    BridgeDeliveryError, BridgeDeliveryErrorKind, BridgeErrorContext, BridgeLineageSourceError,
    BridgeLineageSourceErrorKind, BridgeMergeError, BridgeMergeErrorKind, BridgePatchCoordinate,
    BridgeReplayError, BridgeReplayErrorKind, BridgeRouteError, BridgeRouteErrorKind,
    BridgeSnapshotReadCoordinate, BridgeSpeculationError, BridgeSpeculationErrorKind,
    BridgeStreamError, BridgeStreamErrorKind, BridgeWritebackError, BridgeWritebackErrorKind,
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
use crate::mapping::{FrozenAspectMappingRegistry, FrozenMappingRegistry};
pub use crate::merge::{
    AdmittedMergeHistoryContract, AdmittedMergeRegistry,
    BridgeMergeAuthoritativeLineageDisposition, BridgeMergeAuthorityBasis,
    BridgeMergeAuthorityBasisIdentity, BridgeMergeAuthorityBasisKind,
    BridgeMergeCausalFrontierDisposition, BridgeMergeConsumptionClass, BridgeMergeCounters,
    BridgeMergeDenialClass, BridgeMergeOntologyLoweringKind, BridgeMergeOntologyMappingEntry,
    BridgeMergeOntologyMappingSurface, BridgeMergeOntologyMappingSurfaceIdentity,
    BridgeMergeParentOrderDigestBasis, BridgeMergeParentOrderProof,
    BridgeMergeParentOrderProofIdentity, BridgeMergePrecedenceStage,
    BridgeMergeRoutingOutcomeClass, BridgeMergeSchemaPolicyDisposition,
    BridgeMergeStageDecisionClass, BridgeMergeStructuralAdvisoryDisposition,
    CanonicalRelationalMergeClass, LoweredMergeHistoryPacketSet, MergeDecisionLogEntry,
    MergeHistoryDeclaration, MergeHistoryDeclarationIdentity, MergePrecedenceStageOutput,
    MergeReplayCertificationBundle, PublishedMergeContinuityArtifact,
    PublishedMergeExplanationArtifact, PublishedMergeRemapArtifact, ReducedMergeRoutingArtifact,
    ValidatedMergeHistoryDeclaration,
};
pub use crate::policy::{
    AdmittedBridgePolicyContract, BridgeArtifactPolicyBaseline, BridgeDiagnosticsPolicyBaseline,
    BridgeDiagnosticsRetentionBudget, BridgeDiagnosticsTier, BridgePolicyCounters,
    BridgeExecutionPolicyBaseline, BridgeExecutionPolicyClass, BridgePolicyAuthorityInputs,
    BridgePolicyDeclaration, BridgePolicyDeclarationIdentity, BridgePolicyFieldKind,
    BridgePolicyProvenanceEntry, BridgePolicyProvenanceRecord, BridgePolicyProvenanceReport,
    BridgePolicyProvenanceReportRow, BridgePolicyReplayBundle, BridgePolicyRejection,
    BridgePolicyRejectionKind, BridgePolicyRejectionStage, BridgePolicyResolution, BridgePolicySourceClass,
    BridgeRoutePlanningPolicy, BridgeRuntimePolicy, BridgeRuntimePosture,
    LoweredBridgeExecutionPolicy,
    ValidatedBridgePolicyDeclaration,
};
pub use crate::routing::{
    AdmittedBridgeExecutionPlan, AdmittedPreparationPartitionSet, BridgeAdmissionProfileIdentity,
    BridgeBulkDecisionLog, BridgeBulkDecisionRecord, BridgeBulkDecisionRecordKind,
    BridgeBulkPlanningCounters, BridgeBulkPlanningFailure, BridgeBulkPlanningFailureKind,
    BridgeBulkPlanningSummary, BridgeBulkResultSummary, BridgeBulkWorkloadPlan,
    BridgeBulkWorkloadRequest, BridgeBulkWorkloadResult, BridgeBulkWorkloadSegment,
    BridgeCanonicalBulkPlanRecord, BridgeCanonicalPlanningIdentity, BridgeExecutionCounts,
    BridgeInvalidationArtifact, BridgeInvalidationIdentity, BridgeInvalidationTarget,
    BridgeLineageContext, BridgeLocalityFootprint, BridgeLoweringPlanSummary,
    BridgeLoweringProvenance, BridgeLoweringSummary, BridgeMappingContext, BridgeParallelAdmission,
    BridgeParallelAdmissionClass, BridgeParallelAdmissionReason, BridgeParallelLegalityClass,
    BridgeParallelLegalityDecision, BridgeParallelLegalityReason, BridgeParallelProfitabilityClass,
    BridgeParallelProfitabilityDecision, BridgeParallelProfitabilityReason, BridgePlannedRoute,
    BridgePlanningProvenance, BridgePlanningSummary, BridgePreparationMode,
    BridgeRouteContractProof, BridgeRouteIdentity, BridgeRouteOutcomeReference, BridgeRouteResult,
    BridgeRouteResultSummary, BridgeRouteSourceSummary, BridgeRoutingCounters,
    BridgeRoutingSummary, BridgeSignalInvalidationDelivery, BridgeSubscriptionSlice,
    BridgeSubscriptionSliceIdentity, BridgeWorkloadIdentity, CanonicalBridgeWorkloadRequest,
    CanonicalInvalidationTargets, CanonicalSubscriptionSlices, ContinuityPacketIdentity,
    ContinuityRemapPacket, DisjointPacketRegionSet, FallbackAggregationPacket,
    FallbackPacketIdentity, FineGrainedMatchOutcome, FineGrainedMatchStatus,
    InvalidationReductionPacket, NormalizedBridgeWorkloadSummary, ParallelPreparationLegalityProof,
    PlannedBridgePacketSet, ReducedBridgePublication, ReducedBridgeWorkloadArtifact,
    ReducedContinuityIdentity, ReducedContinuityRemap, ReducedFallbackAggregation,
    ReducedFallbackIdentity, ReducedPublicationIdentity, ReducedRoutingTargetIdentity,
    ReducedTruthViewIdentity, ReducedTruthViewMaterialization, ReductionPacketIdentity,
    RoutingPacketIdentity, TruthDeltaRoutingPacket, TruthViewMaterializationPacket,
    TruthViewPacketIdentity, BRIDGE_CANONICAL_BULK_PLAN_RECORD_SCHEMA_V1,
};
pub use crate::snapshot::{
    AdmittedSnapshotContext, BridgeDeliveryIntent, BridgeReplayMode, BridgeSnapshotContext,
    BridgeSnapshotReadError, BridgeSnapshotToken, BridgeTruthViewKind,
    BridgeTruthViewPolicyRejection, BridgeTruthViewPolicyResolution, BridgeTruthViewSelector,
    BridgeTruthViewSelectorIdentity, HistoricalEvaluationDeclaration,
    HistoricalEvaluationDeclarationIdentity, LoweredHistoricalEvaluationArtifact,
    LoweredHistoricalEvaluationArtifactIdentity, ResolvedTruthViewPolicy, SnapshotReadPacket,
    SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadRequest, TruthSnapshotIdentity,
    TruthSnapshotReader, TruthViewObservationReader, TruthViewPolicyRejectionKind,
    TruthViewReplayCompatibility, TruthViewRetentionAdmission, TruthViewSourceCapability,
    ValidatedSnapshotReadPacketResult, ValidatedTruthViewSelectorSet,
};
pub use crate::source::{
    AdmittedSourceContract, AdmittedSourceRegistry, BridgeSourceCapability,
    BridgeSourceCapabilitySet, MaterializedTruthViewPacketSet, PlannedSourceReadPacketSet,
    SourceDeclaration, SourceDeclarationIdentity, SourceFailureClass, SourceFailureRecord,
    SourceFailureRecordIdentity, SourceMaterializationCounters, SourceMaterializationRecord,
    SourceMaterializationRecordIdentity, ValidatedSourceDeclaration,
};
pub use crate::speculation::{
    BridgePreviewDiscardCleanupOutcome, BridgePreviewDiscardRecord, BridgePreviewExecutionRecord,
    BridgePreviewLifecycleStateKind, BridgePreviewLifecycleTransitionKind,
    BridgePreviewPromotionRecord, BridgePreviewPromotionRecordIdentity, BridgePreviewReplayBundle,
    BridgePreviewResidueClass, BridgePreviewResidueReport, BridgePreviewReuseEquivalence,
    BridgePreviewSession, BridgePreviewSessionDeclaration, BridgePreviewSessionDeclarationIdentity,
    BridgePreviewSessionIdentity, BridgePromotionAdmissibilityProof, BridgeRequestKind,
    BridgeSignalBranchIdentity, BridgeSpeculationCounters, BridgeSpeculationFailureClass,
    BridgeSpeculativeBranchBinding, BridgeSpeculativeBranchBindingIdentity, PreviewActive,
    PreviewAdmitted, PreviewDeclared, PreviewDiscarded, PreviewExecutionRecordIdentity,
    PreviewPromoted, ValidatedBridgePreviewSessionDeclaration,
};
pub use crate::stream::{
    AdmittedConsumerContract, BackpressureDecisionRecord, CanonicalStreamMember,
    CanonicalStreamPosition, CanonicalStreamReplayRecord, ChangeStreamDeclaration,
    ChangeStreamDeclarationIdentity, ConsumerCheckpointToken, ConsumerContractIdentity,
    LoweredConsumedChangeSet, PlannedChangeStreamWindow, StreamCheckpointFrontierKind,
    StreamCheckpointPublicationMode, StreamCoalescingFamily, StreamCoalescingIntent,
    StreamConsumerShape, StreamDeliveryIntent, StreamDiagnosticsPolicyClass,
    StreamProtocolCounters, StreamProtocolIdentity, StreamReplayAuditResult,
    StreamReplayAuditSummary, StreamReplayMode, StreamReplayRecordIdentity, StreamResumeMode,
    StreamWindowDeliveryResult, StreamWindowDeliverySummary, StreamWindowIdentity,
    ValidatedStreamProtocol,
};
pub use crate::writeback::{
    AdmittedBridgeWritebackContract, BridgeDerivedWritebackEffect,
    BridgeWritebackAuthorityInputs, BridgeWritebackAuthorityOutcome, BridgeWritebackCausalityBasis,
    BridgeWritebackCausalityIdentity, BridgeWritebackContractIdentity, BridgeWritebackCounters,
    BridgeWritebackDeclaration, BridgeWritebackDeclarationIdentity,
    BridgeWritebackEffectClass, BridgeWritebackEffectIdentity, BridgeWritebackFailureClass,
    BridgeWritebackFamilyBasis, BridgeWritebackFamilyIdentity, BridgeWritebackFamilyKind,
    BridgeWritebackFamilyAdmissionRecord, BridgeWritebackFamilyAdmissionRecordIdentity,
    BridgeWritebackFeedbackProvenance, BridgeWritebackExecutionRecord,
    BridgeWritebackExecutionRecordIdentity, BridgeWritebackIdempotenceBasis,
    BridgeWritebackIdempotenceClass,
    BridgeWritebackIdempotenceIdentity, BridgeWritebackOutcomeClass,
    BridgeWritebackLoopDisposition, BridgeWritebackLoopPreventionIdentity,
    BridgeWritebackLoopPreventionReport,
    BridgeWritebackMapperEnvelope, BridgeWritebackMapperEnvelopeIdentity,
    BridgeMappedWritebackFamilyInput, BridgeMappedWritebackFamilyInputIdentity,
    BridgeWritebackMapperRecord, BridgeWritebackMapperRecordIdentity,
    BridgeWritebackMapperWitness, BridgeWritebackMapperWitnessIdentity,
    BridgeValidatedWritebackCandidate, BridgeWritebackCandidateIdentity,
    BridgeWritebackReplayBundle, BridgeWritebackReplayRecord,
    BridgeWritebackReplayRecordIdentity, BridgeWritebackRequestMode,
    BridgeWritebackRetryDisposition, BridgeWritebackStrategyBasis, BridgeWritebackStrategyClass,
    BridgeWritebackStrategyCompatibilityDisposition,
    BridgeWritebackStrategyCompatibilityIdentity,
    BridgeWritebackStrategyCompatibilityReport, BridgeWritebackStrategyIdentity,
    ValidatedBridgeWritebackDeclaration,
};
pub use crate::structural::{
    AdmittedStructuralComparisonContract, AdmittedStructuralRegistry,
    PlannedStructuralMatchPacketSet, PublishedBranchComparisonArtifact,
    PublishedStructuralRemapArtifact, ReducedStructuralMatchSet, StructuralCandidateIdentity,
    StructuralCandidateSearchScope, StructuralComparisonMode, StructuralFingerprint,
    StructuralFingerprintEquivalenceContract, StructuralFingerprintFamily,
    StructuralFingerprintIdentity, StructuralFingerprintNormalizationRule,
    StructuralFingerprintOmissionPolicy, StructuralFingerprintOrderingRule,
    StructuralIdentityDeclaration, StructuralIdentityDeclarationIdentity, StructuralMatchCandidate,
    StructuralMatchCandidateKind, StructuralMatchOutcomeClass, StructuralMismatchClass,
    StructuralSchemaIdentity, StructuralTruthViewBasis, StructuralTruthViewBasisIdentity,
    StructuralTruthViewBasisKind, ValidatedStructuralIdentityDeclaration,
};

mod request;
mod runtime;

pub use request::BridgeRouteRequest;
pub use runtime::RuntimeBridge;

#[cfg(test)]
mod tests;
