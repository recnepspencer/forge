//! Public API boundary for `forge-runtime-bridge`.
//! External crates should import through this module rather than reaching into
//! internal crate structure directly.
//!
//! This file is the authoritative bridge API surface. The standard path, the
//! advanced controls, and the specialist proof surfaces are all exposed here so
//! callers can learn one import path and stay there.
//!
//! ```no_run
//! use forge_runtime_bridge::facade::{
//!     BridgeMappingId, BridgeMappingRegistration, CoarseRoutingMode, MappingSelector,
//!     RuntimeBridge, SignalInvalidationScope, TruthPatchScope,
//! };
//!
//! fn facade_example<
//!     TruthSource,
//!     BranchHeads,
//!     ComputeSink,
//! >(
//!     truth_source: TruthSource,
//!     branch_heads: BranchHeads,
//!     compute_sink: ComputeSink,
//! ) -> Result<(), Box<dyn std::error::Error>>
//! where
//!     TruthSource: forge_runtime_bridge::facade::RelationalBridgeSource + Clone + 'static,
//!     BranchHeads: forge_runtime_bridge::facade::TruthBranchHeadSource + Clone + 'static,
//!     ComputeSink: forge_runtime_bridge::facade::SignalBridgeSink + Clone + 'static,
//! {
//!     let bridge = RuntimeBridge::builder()
//!         .with_truth_source(truth_source)
//!         .with_truth_branch_head_source(branch_heads)
//!         .with_compute_sink(compute_sink)
//!         .register_mapping(BridgeMappingRegistration::new(
//!             BridgeMappingId::new("pricing:steel"),
//!             TruthPatchScope::new(
//!                 MappingSelector::exact("component:steel"),
//!                 MappingSelector::exact("cost"),
//!                 MappingSelector::exact("usd"),
//!             ),
//!             SignalInvalidationScope::new("price:bicycle"),
//!             CoarseRoutingMode::Direct,
//!         ))
//!         .build()?;
//!
//!     let route = bridge.route("commit:steel-main")?;
//!     let evaluation = bridge.evaluate_current(route.target())?;
//!     let diagnostics = bridge.diagnostics().explain_last();
//!
//!     let _ = evaluation;
//!     let _ = diagnostics;
//!     Ok(())
//! }
//! ```

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
    BridgeHistoricalEvaluationRecord, BridgeHistoricalEvaluationReplaySummary,
    BridgeHistoricalMaterializationPath, BridgeLoweringDiagnosticsRecord,
    BridgeMappedWritebackFamilyInputExplanation, BridgeMergeExplanation, BridgeMergeRecord,
    BridgeMergeRecordIdentity, BridgeMergeReplaySummary, BridgePolicyExplanation,
    BridgePolicyExplanationRow, BridgePolicyRejectionExplanation, BridgePreviewDiscardExplanation,
    BridgePreviewExecutionExplanation, BridgePreviewPromotionExplanation,
    BridgePreviewReplayExplanation, BridgeReplayRecord, BridgeReplaySummary,
    BridgeRouteExplanation, BridgeRouteExplanationEntry, BridgeRouteRecord, BridgeRouteRecordEntry,
    BridgeRouteRecordMatch, BridgeRouteSourceRecord, BridgeRoutingDiagnosticsRecord,
    BridgeSourceFailureExplanation, BridgeSourceMaterializationExplanation,
    BridgeStreamCheckpointExplanation, BridgeStreamReplayExplanation, BridgeStreamResumeSummary,
    BridgeStructuralBranchComparisonExplanation, BridgeStructuralBranchComparisonRecord,
    BridgeStructuralBranchComparisonReplaySummary, BridgeStructuralCounters,
    BridgeStructuralRemapExplanation, BridgeStructuralRemapRecord,
    BridgeStructuralRemapReplaySummary, BridgeWritebackAdmissionExplanation,
    BridgeWritebackCandidateExplanation, BridgeWritebackExecutionExplanation,
    BridgeWritebackLoopPreventionExplanation, BridgeWritebackMapperEnvelopeExplanation,
    BridgeWritebackMapperExplanation, BridgeWritebackOutcomeExplanation,
    BridgeWritebackReplayExplanation, BridgeWritebackReplayRecordExplanation,
    BridgeWritebackStrategyCompatibilityExplanation, BRIDGE_CANONICAL_CONTINUITY_RECORD_SCHEMA_V1,
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
    BridgeDiagnosticsRetentionBudget, BridgeDiagnosticsTier, BridgeExecutionPolicyBaseline,
    BridgeExecutionPolicyClass, BridgePolicyAuthorityInputs, BridgePolicyCounters,
    BridgePolicyDeclaration, BridgePolicyDeclarationIdentity, BridgePolicyFieldKind,
    BridgePolicyProvenanceEntry, BridgePolicyProvenanceRecord, BridgePolicyProvenanceReport,
    BridgePolicyProvenanceReportRow, BridgePolicyRejection, BridgePolicyRejectionKind,
    BridgePolicyRejectionStage, BridgePolicyReplayBundle, BridgePolicyResolution,
    BridgePolicySourceClass, BridgeRoutePlanningPolicy, BridgeRuntimePolicy, BridgeRuntimePosture,
    LoweredBridgeExecutionPolicy, ValidatedBridgePolicyDeclaration,
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
    materialize_bridge_grouped_truth_view_from_projection, materialize_bridge_row_set,
    AdmittedSourceContract, AdmittedSourceRegistry, BridgeGroupedLaneValue, BridgeGroupedMemberRow,
    BridgeGroupedTruthViewArtifact, BridgeGroupedTruthViewDigest, BridgeGroupedTruthViewError,
    BridgeMaterializedFieldValue, BridgeMaterializedRowArtifact, BridgeMaterializedRowSetArtifact,
    BridgeMaterializedRowSetDigest, BridgeRowIdentity, BridgeRowSetMaterializationError,
    BridgeSourceCapability, BridgeSourceCapabilitySet, GroupedProjectionMemberSource,
    GroupedProjectionSource, MaterializedTruthViewPacketSet, PlannedSourceReadPacketSet,
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
use crate::subscription::FrozenSubscriptionFamilyRegistry;
pub use crate::subscription::{
    AdmittedBridgeSubscription, BridgeActiveSubscription, BridgeActiveSubscriptionIdentity,
    BridgeAdmittedSubscriptionIdentity, BridgePreviewActiveSubscription,
    BridgePreviewActiveSubscriptionIdentity, BridgeRetainedSubscriptionBundle,
    BridgeSignalStrategyDescriptor, BridgeSignalStrategyIdentity, BridgeSignalStrategyKind,
    BridgeSubscriptionAcknowledgementFrontier, BridgeSubscriptionAcknowledgementFrontierIdentity,
    BridgeSubscriptionAcknowledgementFrontierRejection,
    BridgeSubscriptionAcknowledgementFrontierRejectionKind, BridgeSubscriptionActivationReady,
    BridgeSubscriptionAdmissionRejection, BridgeSubscriptionAdmissionRejectionKind,
    BridgeSubscriptionBasisIdentity, BridgeSubscriptionBasisKind, BridgeSubscriptionBasisRequest,
    BridgeSubscriptionBasisResolutionFailure, BridgeSubscriptionBasisResolutionFailureKind,
    BridgeSubscriptionCheckpoint, BridgeSubscriptionCheckpointIdentity,
    BridgeSubscriptionCheckpointReady, BridgeSubscriptionCheckpointReadyIdentity,
    BridgeSubscriptionCheckpointRejection, BridgeSubscriptionCheckpointRejectionKind,
    BridgeSubscriptionConsumerBackpressurePosture, BridgeSubscriptionConsumerContract,
    BridgeSubscriptionConsumerContractFamily, BridgeSubscriptionConsumerContractIdentity,
    BridgeSubscriptionConsumerContractRejection, BridgeSubscriptionConsumerContractRejectionKind,
    BridgeSubscriptionConsumerDiagnosticsRetention, BridgeSubscriptionConsumerPacingCapability,
    BridgeSubscriptionContinuationCandidate, BridgeSubscriptionContinuationCandidateIdentity,
    BridgeSubscriptionContinuationCandidateInput, BridgeSubscriptionContinuationChild,
    BridgeSubscriptionContinuationChildIdentity, BridgeSubscriptionContinuationDecision,
    BridgeSubscriptionContinuationDecisionIdentity, BridgeSubscriptionContinuationIndex,
    BridgeSubscriptionContinuationIndexIdentity, BridgeSubscriptionContinuationIndexRejection,
    BridgeSubscriptionContinuationIndexRejectionKind, BridgeSubscriptionContinuationKind,
    BridgeSubscriptionContinuationRejection, BridgeSubscriptionContinuationRejectionKind,
    BridgeSubscriptionCounters, BridgeSubscriptionDeactivated, BridgeSubscriptionDeclaration,
    BridgeSubscriptionDeclarationFamilyKind, BridgeSubscriptionDeclarationIdentity,
    BridgeSubscriptionDeclarationRejection, BridgeSubscriptionDeclarationRejectionKind,
    BridgeSubscriptionDeliveryBufferLifecycleIdentity, BridgeSubscriptionDeliveryBufferPlan,
    BridgeSubscriptionDeliveryCostProfile, BridgeSubscriptionDeliveryCostProfileIdentity,
    BridgeSubscriptionDeliveryCostProfileRejection,
    BridgeSubscriptionDeliveryCostProfileRejectionKind, BridgeSubscriptionDeliveryDensityPosture,
    BridgeSubscriptionDeliveryDiagnosticsReference,
    BridgeSubscriptionDeliveryDiagnosticsReferenceIdentity, BridgeSubscriptionDeliveryFamily,
    BridgeSubscriptionDeliveryFamilyIdentity, BridgeSubscriptionDeliveryFamilyKind,
    BridgeSubscriptionDeliveryIntentClass, BridgeSubscriptionDeliveryMemberClass,
    BridgeSubscriptionDeliveryMemberIdentity, BridgeSubscriptionDeliveryMemberInput,
    BridgeSubscriptionDeliveryMemberRecord, BridgeSubscriptionDeliveryReplayPlan,
    BridgeSubscriptionDeliveryReplayPlanIdentity, BridgeSubscriptionDeliveryReplayPlanRejection,
    BridgeSubscriptionDeliveryReplayPlanRejectionKind,
    BridgeSubscriptionDeliveryReplayReadinessClass,
    BridgeSubscriptionDeliveryReplayReadinessIdentity, BridgeSubscriptionDeliveryWindowIdentity,
    BridgeSubscriptionDeliveryWindowOpen, BridgeSubscriptionDeliveryWindowRejection,
    BridgeSubscriptionDeliveryWindowRejectionKind, BridgeSubscriptionDeliveryWindowReplayReadiness,
    BridgeSubscriptionDeliveryWindowSealed, BridgeSubscriptionDuplicateReplayPolicy,
    BridgeSubscriptionDuplicateReplayPolicyIdentity, BridgeSubscriptionDuplicateReplayPolicyKind,
    BridgeSubscriptionExplanation, BridgeSubscriptionFamilyRegistryIdentity,
    BridgeSubscriptionFanoutAcknowledgementPolicyClass, BridgeSubscriptionFanoutConsumerBinding,
    BridgeSubscriptionFanoutConsumerBindingIdentity, BridgeSubscriptionFanoutDeliveryProjection,
    BridgeSubscriptionFanoutDeliveryProjectionIdentity,
    BridgeSubscriptionFanoutDeliveryProjectionSet,
    BridgeSubscriptionFanoutDeliveryProjectionSetIdentity,
    BridgeSubscriptionFanoutDiagnosticsPolicyClass, BridgeSubscriptionFanoutLayout,
    BridgeSubscriptionFanoutLayoutIdentity, BridgeSubscriptionFanoutPlan,
    BridgeSubscriptionFanoutPlanIdentity, BridgeSubscriptionFanoutPlanRejection,
    BridgeSubscriptionFanoutPlanRejectionKind, BridgeSubscriptionFanoutProjectionRejection,
    BridgeSubscriptionFanoutProjectionRejectionKind, BridgeSubscriptionFanoutProjectionValidation,
    BridgeSubscriptionFanoutProjectionValidationIdentity,
    BridgeSubscriptionFanoutProjectionValidationRejection,
    BridgeSubscriptionFanoutProjectionValidationRejectionKind, BridgeSubscriptionLifecycleIdentity,
    BridgeSubscriptionLifecycleRecord, BridgeSubscriptionLifecycleStateKind,
    BridgeSubscriptionPayloadOmissionReason, BridgeSubscriptionPreviewBasisBinding,
    BridgeSubscriptionPreviewBasisIdentity, BridgeSubscriptionPreviewBasisRejection,
    BridgeSubscriptionPreviewBasisRejectionKind, BridgeSubscriptionPreviewDiscardResidueProof,
    BridgeSubscriptionPreviewDiscardResidueProofIdentity,
    BridgeSubscriptionPreviewDiscardResidueRejection,
    BridgeSubscriptionPreviewDiscardResidueRejectionKind,
    BridgeSubscriptionPreviewLifecycleIdentity, BridgeSubscriptionPreviewParentBasisIdentity,
    BridgeSubscriptionPreviewResidueArtifactIdentity,
    BridgeSubscriptionPreviewResidueArtifactInput, BridgeSubscriptionPreviewResidueArtifactRecord,
    BridgeSubscriptionPreviewResidueCategory, BridgeSubscriptionPreviewResidueScopeIdentity,
    BridgeSubscriptionPreviewResidueScopeIndex, BridgeSubscriptionPreviewResidueScopeIndexIdentity,
    BridgeSubscriptionPreviewScopeIdentity, BridgeSubscriptionReplayIdentity,
    BridgeSubscriptionReplayMismatch, BridgeSubscriptionReplayMismatchKind,
    BridgeSubscriptionReplaySummary, BridgeSubscriptionResumeAdmission,
    BridgeSubscriptionResumeAdmissionIdentity, BridgeSubscriptionResumeAdmissionRejection,
    BridgeSubscriptionResumeAdmissionRejectionKind, BridgeSubscriptionResumePlan,
    BridgeSubscriptionResumePlanIdentity, BridgeSubscriptionRetainedDeliveryReplaySeed,
    BridgeSubscriptionRetainedDeliveryReplaySeedIdentity,
    BridgeSubscriptionRetainedDeliveryWindowSeed,
    BridgeSubscriptionRetainedDeliveryWindowSeedIdentity,
    BridgeSubscriptionSharingEligibilityIdentity, BridgeSubscriptionSharingEligibilityWitness,
    NormalizedSubscriptionSliceIntent, NormalizedSubscriptionSliceIntentError,
    NormalizedSubscriptionSliceIntentErrorKind, ValidatedSubscriptionBasisBinding,
};
pub use crate::writeback::{
    AdmittedBridgeWritebackContract, BridgeDerivedWritebackEffect,
    BridgeMappedWritebackFamilyInput, BridgeMappedWritebackFamilyInputIdentity,
    BridgeValidatedWritebackCandidate, BridgeWritebackAuthorityInputs,
    BridgeWritebackAuthorityOutcome, BridgeWritebackCandidateIdentity,
    BridgeWritebackCausalityBasis, BridgeWritebackCausalityIdentity,
    BridgeWritebackContractIdentity, BridgeWritebackCounters, BridgeWritebackDeclaration,
    BridgeWritebackDeclarationIdentity, BridgeWritebackEffectClass, BridgeWritebackEffectIdentity,
    BridgeWritebackExecutionRecord, BridgeWritebackExecutionRecordIdentity,
    BridgeWritebackFailureClass, BridgeWritebackFamilyAdmissionRecord,
    BridgeWritebackFamilyAdmissionRecordIdentity, BridgeWritebackFamilyBasis,
    BridgeWritebackFamilyIdentity, BridgeWritebackFamilyKind, BridgeWritebackFeedbackProvenance,
    BridgeWritebackIdempotenceBasis, BridgeWritebackIdempotenceClass,
    BridgeWritebackIdempotenceIdentity, BridgeWritebackLoopDisposition,
    BridgeWritebackLoopPreventionIdentity, BridgeWritebackLoopPreventionReport,
    BridgeWritebackMapperEnvelope, BridgeWritebackMapperEnvelopeIdentity,
    BridgeWritebackMapperRecord, BridgeWritebackMapperRecordIdentity, BridgeWritebackMapperWitness,
    BridgeWritebackMapperWitnessIdentity, BridgeWritebackOutcomeClass, BridgeWritebackReplayBundle,
    BridgeWritebackReplayRecord, BridgeWritebackReplayRecordIdentity, BridgeWritebackRequestMode,
    BridgeWritebackRetryDisposition, BridgeWritebackStrategyBasis, BridgeWritebackStrategyClass,
    BridgeWritebackStrategyCompatibilityDisposition, BridgeWritebackStrategyCompatibilityIdentity,
    BridgeWritebackStrategyCompatibilityReport, BridgeWritebackStrategyIdentity,
    ValidatedBridgeWritebackDeclaration,
};

mod request;
mod runtime;
mod standard_path;

pub use request::BridgeRouteRequest;
pub use runtime::RuntimeBridge;
pub use standard_path::{
    BridgeDiagnostics, BridgeEvaluationTarget, BridgeRoute, BridgeSpeculativeComparison,
    BridgeSpeculativeDiscardOutcome, BridgeSpeculativePromotionOutcome,
    BridgeSpeculativePromotionRequest, BridgeSpeculativeSessionHandle,
    BridgeSpeculativeSessionRequest, BridgeStandardDiagnosticsExplanation,
    BridgeStandardRouteError, BridgeStandardSessionExplanation, BridgeTruthViewEvaluation,
    BridgeTruthViewEvaluationRequest,
};

#[doc(hidden)]
pub mod everyday {
    pub use super::*;
}

#[doc(hidden)]
pub mod advanced {
    pub use super::*;
}

#[doc(hidden)]
pub mod specialist {
    pub use super::*;
}

#[cfg(test)]
mod tests;
