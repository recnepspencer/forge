//! Lifecycle-grouped runtime exports.

// --- launch ---
pub use super::active::WorthUiActiveRuntimeObservation;
pub use super::launch::{
    WorthUiLastValidObservation, WorthUiPendingActivation, WorthUiRuntimeFrameEpoch,
    WorthUiRuntimeLaunch, WorthUiRuntimeLaunchDenial, WorthUiRuntimeLifecycle,
    WorthUiRuntimeShutdownReceipt,
};

// --- replacement ---
pub use super::replacement::admission::{
    WorthUiActiveReplacementBasis, WorthUiAdmittedReplacementCandidate, WorthUiCandidateAdmission,
    WorthUiCandidateAdmissionCounters, WorthUiCandidateAdmissionDenial,
    WorthUiCandidateAdmissionReport, WorthUiQuerySupportReceipt, WorthUiQuerySupportStatus,
    WorthUiRuntimeReplacementPosture,
};
pub use super::replacement::candidate::{
    WorthUiCandidateArtifactBundle, WorthUiCandidateAuthoringLane,
    WorthUiCandidateDependencyMetadata, WorthUiCandidateLoweringBasis,
    WorthUiCandidateProvenanceHandle, WorthUiReplacementCandidate,
    WorthUiReplacementCandidateBasis, WorthUiReplacementCandidateDenial, WorthUiReplacementCause,
};
pub use super::replacement::equivalence::{
    WorthUiRuntimeArtifactComparator, WorthUiRuntimeArtifactComparison,
    WorthUiRuntimeArtifactComparisonCounters, WorthUiRuntimeArtifactComparisonDenial,
    WorthUiRuntimeArtifactComparisonOutcome, WorthUiRuntimeEquivalenceBasis,
};
pub use super::replacement::file_rust_replacement_parity::{
    WorthUiFileRustReplacementParityBoundary, WorthUiFileRustReplacementParityCounters,
    WorthUiFileRustReplacementParityDenial, WorthUiFileRustReplacementParityDenialReason,
    WorthUiFileRustReplacementParityReceipt, WorthUiFileRustReplacementPipelineReport,
    WorthUiFileRustReplacementSemanticReceipt,
};
pub use super::replacement::impact::{
    WorthUiAccessibilityImpact, WorthUiCommandImpact, WorthUiDurableStateImpactReceipts,
    WorthUiLaneImpactClassification, WorthUiRendererResourceImpact, WorthUiReplacementImpact,
    WorthUiReplacementImpactClassification, WorthUiReplacementImpactClassifier,
    WorthUiReplacementImpactCounters, WorthUiReplacementImpactDenial, WorthUiReplacementScope,
    WorthUiTokenThemeImpact, WorthUiUnsupportedReplacementImpact,
};
pub use super::replacement::matching::{
    WorthUiIdentityMatchCounters, WorthUiIdentityMatchDenial, WorthUiIdentityMatchEdge,
    WorthUiIdentityMatchGraph, WorthUiIdentityMatchNode, WorthUiIdentityMatchNodeKind,
    WorthUiIdentityMatchNodeSide, WorthUiIdentityMatchReport, WorthUiIdentitySeedContribution,
    WorthUiMovedNodeIdentity, WorthUiRepeatedTemplateIdentity,
};
pub use super::replacement::narrowing::{
    WorthUiAccessibilityInvalidation, WorthUiCommandBindingInvalidation,
    WorthUiImpactLookupCounters, WorthUiQueryDependencyInvalidation, WorthUiQueryDependencySurface,
    WorthUiRendererResourceInvalidation, WorthUiRuntimeImpactNarrower,
    WorthUiRuntimeImpactNarrowing, WorthUiRuntimeImpactNarrowingDenial, WorthUiTokenInvalidation,
};
pub use super::replacement::query_binding::{
    WorthUiQueryBindingComparison, WorthUiQueryBindingComparisonCounters,
    WorthUiQueryBindingComparisonDenial, WorthUiQueryBindingComparisonEntry,
    WorthUiQueryBindingComparisonOutcome, WorthUiQueryBindingIdentity, WorthUiQueryBindingPosture,
    WorthUiQueryBindingPostureDriftFamily,
};
pub use super::replacement::query_live_rebind::{
    WorthUiQueryBindingDriftDenial, WorthUiQueryBindingDriftDenialKind,
    WorthUiQueryBindingPreservation, WorthUiQueryBindingRebind, WorthUiQueryBindingRebindReason,
    WorthUiQueryBindingRetirement, WorthUiQueryBindingRetirementReason,
    WorthUiQueryLiveRebindCounters, WorthUiQueryLiveRebindEntry, WorthUiQueryLiveRebindOutcome,
    WorthUiQueryLiveRebindPlan, WorthUiQueryLiveRebindPlanDenial,
    WorthUiQueryRebindRequiredSurface,
};
pub use super::replacement::reconciliation::{
    WorthUiAdmittedDurableResizeInput, WorthUiDurableResizeInputPosture,
    WorthUiDurableStateCarryForward, WorthUiDurableStateReconciliationCounters,
    WorthUiDurableStateReconciliationDenial, WorthUiDurableStateReconciliationOutcome,
    WorthUiDurableStateReconciliationPlan, WorthUiDurableStateReconciliationReceipt,
    WorthUiDurableStateReplacement, WorthUiFocusChainReconciliation,
    WorthUiPanelVisibilityReconciliation, WorthUiScrollAnchorReconciliation,
    WorthUiSelectionRangeReconciliation, WorthUiSplitterPositionReconciliation,
    WorthUiTabStateReconciliation, WorthUiTextEditStateReconciliation,
};
pub use super::replacement::state_inventory::{
    WorthUiDurableStateEligibility, WorthUiDurableStateFamily, WorthUiDurableStateFamilyHook,
    WorthUiDurableStateFamilyId, WorthUiDurableStateInventory, WorthUiDurableStateInventoryBuilder,
    WorthUiDurableStateInventoryCounters, WorthUiDurableStateInventoryDenial,
    WorthUiDurableStateReplacementPolicy, WorthUiStateOwnerIdentity, WorthUiStateOwnershipClass,
    WorthUiStatePersistencePosture, WorthUiTransientInteractionPolicy,
    WorthUiTransientInteractionState,
};
pub use super::replacement::{
    WorthUiAmbiguousReplacementDenial, WorthUiNodeLifecycleTransition,
    WorthUiNodeReplacementClassification, WorthUiNodeReplacementCounters,
    WorthUiNodeReplacementPlan, WorthUiReplacementAdmissionBasis,
    WorthUiReplacementComparisonReady, WorthUiReplacementIdentityReady,
    WorthUiReplacementImpactReady, WorthUiReplacementLoweringDenial,
    WorthUiReplacementLoweringReady, WorthUiReplacementNarrowingReady,
    WorthUiReplacementNodePlanReady, WorthUiReplacementQueryComparisonReady,
    WorthUiReplacementReconciliationReady,
};

// --- planning ---
pub(crate) use super::planning::allocation_planning::WorthUiRetainedAllocationPlanningEvidenceRegistry;
pub use super::planning::allocation_planning::{
    WorthUiAllocationPlanning, WorthUiAllocationPlanningBasis, WorthUiAllocationPlanningCounters,
    WorthUiAllocationPlanningDenial, WorthUiAllocationPlanningDenialReason,
    WorthUiAllocationPlanningInspection, WorthUiAllocationPlanningLoweringMismatch,
};
pub use super::planning::execution_plan_input::{
    WorthUiComponentLoweringHook, WorthUiComponentLoweringHookFamily, WorthUiEguiBoundaryInput,
    WorthUiExecutionPlanInput, WorthUiPlanLoweringBasis, WorthUiPlanLoweringContext,
    WorthUiPlanLoweringCounters, WorthUiPlanLoweringDenial, WorthUiPlanLoweringDenialReason,
    WorthUiPlanNodeInput, WorthUiPlanNodeInputFamily, WorthUiPlanNodeTopologyInput,
};
pub use super::planning::plan_equivalence::{
    WorthUiExecutionPlanDigest, WorthUiExecutionPlanEquivalence,
    WorthUiExecutionPlanEquivalenceBasis, WorthUiExecutionPlanEquivalenceCounters,
    WorthUiPlanReuseClassification,
};
pub use super::planning::plan_inspection::{
    WorthUiArtifactToPlanProvenance, WorthUiExecutionPlanInspection, WorthUiLaneInspection,
    WorthUiPlanInspectionCounters, WorthUiPlanInspectionDenial, WorthUiPlanInspectionDenialReason,
    WorthUiPlanNodeInspection, WorthUiPlanProvenanceSource, WorthUiQueryInspectionLinks,
};
pub use super::planning::plan_topology::{
    WorthUiEguiBoundaryContact, WorthUiEguiPlanBoundary, WorthUiExecutionPlan,
    WorthUiPlanChildRange, WorthUiPlanExecutionLane, WorthUiPlanLanePartition,
    WorthUiPlanLookupIndex, WorthUiPlanNode, WorthUiPlanNodeFamily, WorthUiPlanRegionStructure,
    WorthUiPlanTopology, WorthUiPlanTopologyCounters, WorthUiPlanTopologyDenial,
    WorthUiPlanTopologyDenialReason, WorthUiRenderResourceRef,
};
pub use super::planning::WorthUiPlanningLaneInput;

// --- activation ---
pub use super::activation::activation_staging::{
    WorthUiActivationReadiness, WorthUiActivationStagingCounters, WorthUiActivationStagingDenial,
    WorthUiActivationStagingDenialReason, WorthUiActivationStagingReport,
    WorthUiPendingExecutionPlanLoweringInput, WorthUiStagedReplacement,
};
pub use super::activation::atomic_plan_swap::{
    WorthUiAtomicPlanSwapCounters, WorthUiPlanSwapDenialReason, WorthUiPlanSwapReceipt,
    WorthUiPlanSwapRollback, WorthUiPriorValidPlanObservation,
};
pub use super::activation::frame_activation_gate::{
    WorthUiActivationGateCounters, WorthUiActivationGateDenial, WorthUiActivationGateDenialReason,
    WorthUiActivationGateReceipt, WorthUiFrameBoundary, WorthUiFrameBoundaryPosture,
    WorthUiReadyActivation,
};
pub use super::activation::WorthUiActivationLaneInput;

// --- execution ---
pub use super::execution::canvas_spatial_lane::{
    WorthUiCanvasDrawHook, WorthUiCanvasOverlayPlan, WorthUiCanvasSpatialCertification,
    WorthUiCanvasSpatialCounters, WorthUiCanvasSpatialFrameDenial,
    WorthUiCanvasSpatialFrameDenialReason, WorthUiCanvasSpatialFrameReceipt,
    WorthUiCanvasSpatialFrameTarget, WorthUiCanvasSpatialLane, WorthUiCanvasSpatialNode,
    WorthUiCanvasSpatialPlan, WorthUiCanvasSpatialPlanDenial, WorthUiCanvasSpatialPlanDenialReason,
    WorthUiCanvasViewportPlan, WorthUiCanvasViewportPlanDenial,
    WorthUiCanvasViewportPlanDenialReason, WorthUiSpatialHitTestHook, WorthUiSpatialHitTestPlan,
    WorthUiSpatialToolStateHook, WorthUiSpatialViewportPoint,
};
pub use super::execution::handle_allocation::{
    WorthUiChildRangeHandle, WorthUiCommandHandle, WorthUiComponentHandle,
    WorthUiHandlePlanGeneration, WorthUiLaneHandle, WorthUiRuntimeHandle,
    WorthUiRuntimeHandleAllocation, WorthUiRuntimeHandleAllocationBasis,
    WorthUiRuntimeHandleAllocationCounters, WorthUiRuntimeHandleAllocationDenial,
    WorthUiRuntimeHandleAllocationDenialReason, WorthUiRuntimeHandleAllocationReceipt,
    WorthUiRuntimeHandleFamilyWidths, WorthUiStateSlotHandle, WorthUiTokenHandle,
    WorthUiViewBindingHandle,
};
pub use super::execution::lane_admission::{
    WorthUiExecutionLane, WorthUiExecutionLaneDescriptor, WorthUiExecutionLaneSupport,
    WorthUiExtensionHookAdmission, WorthUiLaneAdapterHook, WorthUiLaneAdapterHookKind,
    WorthUiLaneAdmission, WorthUiLaneAdmissionCounters, WorthUiLaneAdmissionDenial,
    WorthUiLaneAdmissionDenialReason, WorthUiLaneCostRegime, WorthUiLaneFailureMode,
    WorthUiLaneSupportDiagnostic, WorthUiLaneSupportRow, WorthUiLaneSupportStatus,
    WorthUiLaneTeachingPosture, WorthUiQueryLaneSupportLinks, WorthUiUnsupportedHookDenial,
    WorthUiUnsupportedHookDenialReason,
};
pub use super::execution::lane_frame_cost_certification::{
    WorthUiBroadScanRegressionDenial, WorthUiFrameCostCertification,
    WorthUiLaneAndFrameCostCertification, WorthUiLaneCertification,
    WorthUiLaneFrameCostCertificationCounters, WorthUiLaneFrameCostCertificationDenial,
    WorthUiLaneFrameCostCertificationDenialReason, WorthUiLaneFrameCostCertificationScenario,
    WorthUiLaneFrameCostFoundationalReadiness, WorthUiLaneScaleVariationProof,
    WorthUiNoSourceFrameProof,
};
pub use super::execution::lane_meaning_parity::{
    WorthUiCrossLaneSemanticAuthority, WorthUiCrossLaneSemanticFamily,
    WorthUiCrossLaneSemanticReference, WorthUiLaneMeaningParity, WorthUiLaneParityCertification,
    WorthUiLaneParityCounters, WorthUiLaneParityDenial, WorthUiLaneParityDenialReason,
    WorthUiLaneParityReport, WorthUiLaneTransitionParity,
};
pub use super::execution::ordinary_lane::{
    WorthUiOrdinaryExecutionLane, WorthUiOrdinaryFrameTarget, WorthUiOrdinaryLaneCertification,
    WorthUiOrdinaryLaneCounters, WorthUiOrdinaryLaneFrameDenial,
    WorthUiOrdinaryLaneFrameDenialReason, WorthUiOrdinaryLaneFrameReceipt, WorthUiOrdinaryLaneNode,
    WorthUiOrdinaryLanePlan, WorthUiOrdinaryLanePlanDenial, WorthUiOrdinaryLanePlanDenialReason,
};
pub use super::execution::realtime_overlay_lane::{
    WorthUiHighFrequencyFramePolicy, WorthUiHighFrequencyFramePolicyDenial,
    WorthUiHighFrequencyFramePolicyDenialReason, WorthUiHudNode, WorthUiHudPlan,
    WorthUiHudPlanDenial, WorthUiHudPlanDenialReason, WorthUiRealtimeCertification,
    WorthUiRealtimeFrameDenial, WorthUiRealtimeFrameDenialReason, WorthUiRealtimeFramePriority,
    WorthUiRealtimeFrameReceipt, WorthUiRealtimeFrameTarget, WorthUiRealtimeLaneCounters,
    WorthUiRealtimeOverlayHook, WorthUiRealtimeOverlayLane, WorthUiRendererSurfaceAdmission,
    WorthUiRendererSurfaceHandle,
};
pub use super::execution::reload_counter_boundary::{
    WorthUiCertifiedReloadLoweringCounterReceipt, WorthUiReloadCounterBoundary,
    WorthUiReloadCounterBoundaryDenial, WorthUiReloadCounterBoundaryDenialReason,
    WorthUiReloadCounterStopStage, WorthUiReloadLoweringCounterReceipt,
    WorthUiReloadLoweringCounterReceiptBuilder, WorthUiReloadLoweringFoundationalBridge,
    WorthUiReloadLoweringFoundationalEvidence,
};
pub use super::execution::steady_frame_counter_boundary::{
    WorthUiCertifiedFrameExecutionReceipt, WorthUiFrameExecutionReceipt,
    WorthUiFrameReportMaterializationBoundary, WorthUiLaneFrameReceipt,
    WorthUiLaneFrameReceiptKind, WorthUiRenderCostReceipt, WorthUiSteadyFrameCounterBoundary,
    WorthUiSteadyFrameCounterDenial, WorthUiSteadyFrameCounterDenialReason,
    WorthUiSteadyFrameCounterReceiptBuilder, WorthUiSteadyFrameCounters,
    WorthUiSteadyFrameDiagnosticPolicy, WorthUiSteadyFrameFoundationalBridge,
    WorthUiSteadyFrameFoundationalEvidence, WorthUiSteadyFrameReportPlan,
    WorthUiSteadyFrameReportPlanner,
};
pub use super::execution::virtualized_data_lane::{
    WorthUiQueryPatchPosture, WorthUiVirtualizedDataCertification, WorthUiVirtualizedDataCounters,
    WorthUiVirtualizedDataFrameDenial, WorthUiVirtualizedDataFrameDenialReason,
    WorthUiVirtualizedDataFrameReceipt, WorthUiVirtualizedDataFrameTarget,
    WorthUiVirtualizedDataLane, WorthUiVirtualizedDataNode, WorthUiVirtualizedDataPlan,
    WorthUiVirtualizedDataPlanDenial, WorthUiVirtualizedDataPlanDenialReason, WorthUiVisibleRange,
    WorthUiVisibleRangeDenial, WorthUiVisibleRangeDenialReason,
};
pub use super::execution::WorthUiExecutionLaneInput;

// --- host observation ---
pub use super::host_observation::diagnostics::{
    WorthUiDiagnosticMaterialization, WorthUiDiagnosticProjectionHook,
    WorthUiDiagnosticRichnessPolicy, WorthUiDiagnosticRichnessTier, WorthUiDiagnosticSource,
    WorthUiDiagnosticSupportReport, WorthUiPlanDiagnostic, WorthUiReloadDiagnostic,
    WorthUiRuntimeActivationStatus, WorthUiRuntimeDiagnostic, WorthUiRuntimeDiagnosticCode,
    WorthUiRuntimeDiagnosticCounters, WorthUiRuntimeDiagnosticFamily,
    WorthUiRuntimeDiagnosticPolicy, WorthUiRuntimeDiagnosticReport, WorthUiSupportReportPolicy,
};
pub use super::host_observation::diagnostics_projection::{
    WorthUiDiagnosticsProjection, WorthUiDiagnosticsProjectionCounters,
    WorthUiDiagnosticsProjectionDenial, WorthUiDiagnosticsProjectionDenialReason,
    WorthUiDiagnosticsProjectionHook, WorthUiDiagnosticsProjectionHookEffect,
    WorthUiDiagnosticsProjectionRequest, WorthUiDiagnosticsSurfaceBinding, WorthUiFrameCostRow,
    WorthUiFrameCostSurface, WorthUiFrameCostSurfaceKind, WorthUiPlanInspectionSurface,
    WorthUiQueryStatusRow, WorthUiQueryStatusSurface, WorthUiReloadStatusSurface,
    WorthUiRuntimeDiagnosticsProjection,
};
pub use super::host_observation::identity_state_query_certification::{
    WorthUiIdentityStateCertification, WorthUiIdentityStateQueryCertificationCounters,
    WorthUiIdentityStateQueryCertificationDenial,
    WorthUiIdentityStateQueryCertificationDenialReason,
    WorthUiIdentityStateQueryCertificationScenario, WorthUiQueryDriftCertification,
    WorthUiQueryDriftCertificationScenarioStep, WorthUiStateCarryForwardReceipt,
    WorthUiStateCertificationScenarioStep, WorthUiStateLifecycleReceipt,
    WorthUiStateQueryResidueScan,
};
pub use super::host_observation::reload_failure::{
    WorthUiFailedActivationReport, WorthUiReloadCheckedStopPosture, WorthUiReloadDenial,
    WorthUiReloadFailure, WorthUiReloadFailureCounters, WorthUiReloadFailureStage,
    WorthUiReloadPreservationReceipt,
};
#[cfg(test)]
pub use super::host_observation::reload_storm_certification::{
    WorthUiReloadCertificationBundle, WorthUiReloadLatencyCounters,
    WorthUiReloadStormCandidateDenialReason, WorthUiReloadStormCandidateStep,
    WorthUiReloadStormCandidateStepKind, WorthUiReloadStormCertification,
    WorthUiReloadStormCertificationDenial, WorthUiReloadStormCertificationDenialReason,
    WorthUiReloadStormDeniedIteration, WorthUiReloadStormIterationOutcome,
    WorthUiReloadStormNoOpIteration, WorthUiReloadStormOrderedTruth,
    WorthUiReloadStormReceiptBinding, WorthUiReloadStormScenario,
    WorthUiReloadStormSuccessfulIteration,
};
pub use super::host_observation::{
    WorthUiRuntimeDiagnosticRequest, WorthUiRuntimeDiagnostics, WorthUiRuntimeInspectionAiHarness,
};

// --- source ingress ---
pub(crate) use super::source_ingress::WorthUiSourceBackedDeclarationWitness;
pub use super::source_ingress::{
    WorthUiCandidateOrderingReceipt, WorthUiDebouncedWatcherBatch, WorthUiReloadDebounce,
    WorthUiSourceBackedDslPackage, WorthUiSourceIngressCounters, WorthUiSourceIngressDenial,
    WorthUiSourceIngressDenialReason, WorthUiSourceIngressHook, WorthUiSourceIngressSession,
    WorthUiSourcePackageRevision, WorthUiSourceProvider, WorthUiSourceProviderKind,
    WorthUiSourceWatcher, WorthUiWatchedArtifactInput, WorthUiWatchedCandidateSubmission,
    WorthUiWatchedCandidateSubmissionDenial, WorthUiWatcherEvent,
};

// --- measurement ---
pub use super::measurement::{
    WorthUiCertifiedMeasurementPacket, WorthUiComplexityContract, WorthUiCounterAuthority,
    WorthUiCounterCaptureRichness, WorthUiCounterPacketBuilder, WorthUiCounterValueKind,
    WorthUiFoundationalCounterBridge, WorthUiFoundationalCounterEvidence, WorthUiFrameCostCounter,
    WorthUiMeasurementBoundary, WorthUiMeasurementCertificationDenial,
    WorthUiMeasurementCounterPacket, WorthUiMeasurementQueryEvidence,
    WorthUiMeasurementQueryEvidenceKind, WorthUiRuntimeCounterFamily,
};
