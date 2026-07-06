mod activation_staging;
mod active;
mod admission;
mod allocation_planning;
mod atomic_plan_swap;
mod candidate;
mod canvas_spatial_lane;
mod diagnostics;
mod diagnostics_projection;
mod equivalence;
mod execution_plan_input;
mod file_rust_replacement_parity;
mod frame_activation_gate;
mod handle_allocation;
mod host;
mod host_atomic_plan_swap;
mod host_canvas_spatial_lane;
mod host_diagnostics;
#[cfg(test)]
mod host_file_rust_replacement_parity;
mod host_frame_activation_gate;
mod host_identity_state_query_certification;
mod host_lane_admission;
mod host_lane_frame_cost_certification;
mod host_lane_meaning_parity;
mod host_ordinary_lane;
mod host_plan_inspection;
mod host_realtime_overlay_lane;
mod host_reload_failure;
#[cfg(test)]
mod host_reload_storm_certification;
mod host_virtualized_data_lane;
mod identity_state_query_certification;
mod impact;
mod inspection_ai_harness;
mod lane_admission;
mod lane_frame_cost_certification;
mod lane_meaning_parity;
mod lifecycle;
mod matching;
mod measurement;
mod narrowing;
mod ordinary_lane;
mod plan_equivalence;
mod plan_inspection;
mod plan_topology;
mod preservation;
mod query_binding;
mod query_live_rebind;
mod realtime_overlay_lane;
mod reconciliation;
mod reload_counter_boundary;
mod reload_failure;
mod reload_storm_certification;
mod replacement;
mod source_ingress;
mod state_inventory;
mod steady_frame_counter_boundary;
#[cfg(test)]
mod touch_origin_certification_support;
mod virtualized_data_lane;

pub use activation_staging::{
    WorthUiActivationReadiness, WorthUiActivationStagingCounters, WorthUiActivationStagingDenial,
    WorthUiActivationStagingDenialReason, WorthUiActivationStagingReport,
    WorthUiPendingExecutionPlanLoweringInput, WorthUiStagedReplacement,
};
pub use active::WorthUiActiveRuntimeObservation;
pub use admission::{
    WorthUiActiveReplacementBasis, WorthUiAdmittedReplacementCandidate, WorthUiCandidateAdmission,
    WorthUiCandidateAdmissionCounters, WorthUiCandidateAdmissionDenial,
    WorthUiCandidateAdmissionReport, WorthUiQuerySupportReceipt, WorthUiQuerySupportStatus,
    WorthUiRuntimeReplacementPosture,
};
pub use allocation_planning::{
    WorthUiAllocationPlanning, WorthUiAllocationPlanningBasis, WorthUiAllocationPlanningCounters,
    WorthUiAllocationPlanningDenial, WorthUiAllocationPlanningDenialReason,
    WorthUiAllocationPlanningInspection, WorthUiAllocationPlanningLoweringMismatch,
};
pub(crate) use allocation_planning::{
    planning_pair_for_certification_suite, WorthUiRetainedAllocationPlanningEvidenceRegistry,
};
pub use atomic_plan_swap::{
    WorthUiAtomicPlanSwapCounters, WorthUiPlanSwapDenialReason, WorthUiPlanSwapReceipt,
    WorthUiPlanSwapRollback, WorthUiPriorValidPlanObservation,
};
pub use candidate::{
    WorthUiCandidateArtifactBundle, WorthUiCandidateAuthoringLane,
    WorthUiCandidateDependencyMetadata, WorthUiCandidateLoweringBasis,
    WorthUiCandidateProvenanceHandle, WorthUiReplacementCandidate,
    WorthUiReplacementCandidateBasis, WorthUiReplacementCandidateDenial, WorthUiReplacementCause,
};
pub(crate) use candidate::rust_authored_replacement_candidate;
pub use canvas_spatial_lane::{
    WorthUiCanvasDrawHook, WorthUiCanvasOverlayPlan, WorthUiCanvasSpatialCertification,
    WorthUiCanvasSpatialCounters, WorthUiCanvasSpatialFrameDenial,
    WorthUiCanvasSpatialFrameDenialReason, WorthUiCanvasSpatialFrameReceipt,
    WorthUiCanvasSpatialFrameTarget, WorthUiCanvasSpatialLane, WorthUiCanvasSpatialNode,
    WorthUiCanvasSpatialPlan, WorthUiCanvasSpatialPlanDenial, WorthUiCanvasSpatialPlanDenialReason,
    WorthUiCanvasViewportPlan, WorthUiCanvasViewportPlanDenial,
    WorthUiCanvasViewportPlanDenialReason, WorthUiSpatialHitTestHook, WorthUiSpatialHitTestPlan,
    WorthUiSpatialToolStateHook, WorthUiSpatialViewportPoint,
};
pub use diagnostics::{
    WorthUiDiagnosticMaterialization, WorthUiDiagnosticProjectionHook,
    WorthUiDiagnosticRichnessPolicy, WorthUiDiagnosticRichnessTier, WorthUiDiagnosticSource,
    WorthUiDiagnosticSupportReport, WorthUiPlanDiagnostic, WorthUiReloadDiagnostic,
    WorthUiRuntimeActivationStatus, WorthUiRuntimeDiagnostic, WorthUiRuntimeDiagnosticCode,
    WorthUiRuntimeDiagnosticCounters, WorthUiRuntimeDiagnosticFamily,
    WorthUiRuntimeDiagnosticPolicy, WorthUiRuntimeDiagnosticReport, WorthUiSupportReportPolicy,
};
pub use diagnostics_projection::{
    WorthUiDiagnosticsProjection, WorthUiDiagnosticsProjectionCounters,
    WorthUiDiagnosticsProjectionDenial, WorthUiDiagnosticsProjectionDenialReason,
    WorthUiDiagnosticsProjectionHook, WorthUiDiagnosticsProjectionHookEffect,
    WorthUiDiagnosticsProjectionRequest, WorthUiDiagnosticsSurfaceBinding, WorthUiFrameCostRow,
    WorthUiFrameCostSurface, WorthUiFrameCostSurfaceKind, WorthUiPlanInspectionSurface,
    WorthUiQueryStatusRow, WorthUiQueryStatusSurface, WorthUiReloadStatusSurface,
    WorthUiRuntimeDiagnosticsProjection,
};
pub use equivalence::{
    WorthUiRuntimeArtifactComparator, WorthUiRuntimeArtifactComparison,
    WorthUiRuntimeArtifactComparisonCounters, WorthUiRuntimeArtifactComparisonDenial,
    WorthUiRuntimeArtifactComparisonOutcome, WorthUiRuntimeEquivalenceBasis,
};
pub use execution_plan_input::{
    WorthUiComponentLoweringHook, WorthUiComponentLoweringHookFamily, WorthUiEguiBoundaryInput,
    WorthUiExecutionPlanInput, WorthUiPlanLoweringBasis, WorthUiPlanLoweringContext,
    WorthUiPlanLoweringCounters, WorthUiPlanLoweringDenial, WorthUiPlanLoweringDenialReason,
    WorthUiPlanNodeInput, WorthUiPlanNodeInputFamily, WorthUiPlanNodeTopologyInput,
};
pub use file_rust_replacement_parity::{
    WorthUiFileRustReplacementParityBoundary, WorthUiFileRustReplacementParityCounters,
    WorthUiFileRustReplacementParityDenial, WorthUiFileRustReplacementParityDenialReason,
    WorthUiFileRustReplacementParityReceipt, WorthUiFileRustReplacementPipelineReport,
    WorthUiFileRustReplacementSemanticReceipt,
};
pub use frame_activation_gate::{
    WorthUiActivationGateCounters, WorthUiActivationGateDenial, WorthUiActivationGateDenialReason,
    WorthUiActivationGateReceipt, WorthUiFrameBoundary, WorthUiFrameBoundaryPosture,
    WorthUiReadyActivation,
};
pub use handle_allocation::{
    WorthUiChildRangeHandle, WorthUiCommandHandle, WorthUiComponentHandle,
    WorthUiHandlePlanGeneration, WorthUiLaneHandle, WorthUiRuntimeHandle,
    WorthUiRuntimeHandleAllocation, WorthUiRuntimeHandleAllocationBasis,
    WorthUiRuntimeHandleAllocationCounters, WorthUiRuntimeHandleAllocationDenial,
    WorthUiRuntimeHandleAllocationDenialReason, WorthUiRuntimeHandleAllocationReceipt,
    WorthUiRuntimeHandleFamilyWidths, WorthUiStateSlotHandle, WorthUiTokenHandle,
    WorthUiViewBindingHandle,
};
pub use host::{WorthUiRuntimeHost, WorthUiRuntimeLaunch, WorthUiRuntimeLaunchDenial};
pub use host_diagnostics::{WorthUiRuntimeDiagnosticRequest, WorthUiRuntimeDiagnostics};
pub use identity_state_query_certification::{
    WorthUiIdentityStateCertification, WorthUiIdentityStateQueryCertificationCounters,
    WorthUiIdentityStateQueryCertificationDenial,
    WorthUiIdentityStateQueryCertificationDenialReason,
    WorthUiIdentityStateQueryCertificationScenario, WorthUiQueryDriftCertification,
    WorthUiQueryDriftCertificationScenarioStep, WorthUiStateCarryForwardReceipt,
    WorthUiStateCertificationScenarioStep, WorthUiStateLifecycleReceipt,
    WorthUiStateQueryResidueScan,
};
pub use impact::{
    WorthUiAccessibilityImpact, WorthUiCommandImpact, WorthUiDurableStateImpactReceipts,
    WorthUiLaneImpactClassification, WorthUiRendererResourceImpact, WorthUiReplacementImpact,
    WorthUiReplacementImpactClassification, WorthUiReplacementImpactClassifier,
    WorthUiReplacementImpactCounters, WorthUiReplacementImpactDenial, WorthUiReplacementScope,
    WorthUiTokenThemeImpact, WorthUiUnsupportedReplacementImpact,
};
pub use inspection_ai_harness::WorthUiRuntimeInspectionAiHarness;
pub use lane_admission::{
    WorthUiExecutionLane, WorthUiExecutionLaneDescriptor, WorthUiExecutionLaneSupport,
    WorthUiExtensionHookAdmission, WorthUiLaneAdapterHook, WorthUiLaneAdapterHookKind,
    WorthUiLaneAdmission, WorthUiLaneAdmissionCounters, WorthUiLaneAdmissionDenial,
    WorthUiLaneAdmissionDenialReason, WorthUiLaneCostRegime, WorthUiLaneFailureMode,
    WorthUiLaneSupportDiagnostic, WorthUiLaneSupportRow, WorthUiLaneSupportStatus,
    WorthUiLaneTeachingPosture, WorthUiQueryLaneSupportLinks, WorthUiUnsupportedHookDenial,
    WorthUiUnsupportedHookDenialReason,
};
pub use lane_frame_cost_certification::{
    WorthUiBroadScanRegressionDenial, WorthUiFrameCostCertification,
    WorthUiLaneAndFrameCostCertification, WorthUiLaneCertification,
    WorthUiLaneFrameCostCertificationCounters, WorthUiLaneFrameCostCertificationDenial,
    WorthUiLaneFrameCostCertificationDenialReason, WorthUiLaneFrameCostCertificationScenario,
    WorthUiLaneFrameCostFoundationalReadiness, WorthUiLaneScaleVariationProof,
    WorthUiNoSourceFrameProof,
};
pub use lane_meaning_parity::{
    WorthUiCrossLaneSemanticAuthority, WorthUiCrossLaneSemanticFamily,
    WorthUiCrossLaneSemanticReference, WorthUiLaneMeaningParity, WorthUiLaneParityCertification,
    WorthUiLaneParityCounters, WorthUiLaneParityDenial, WorthUiLaneParityDenialReason,
    WorthUiLaneParityReport, WorthUiLaneTransitionParity,
};
pub use lifecycle::{
    WorthUiPendingActivation, WorthUiRuntimeFrameEpoch, WorthUiRuntimeLifecycle,
    WorthUiRuntimeShutdownReceipt,
};
pub use matching::{
    WorthUiIdentityMatchCounters, WorthUiIdentityMatchDenial, WorthUiIdentityMatchEdge,
    WorthUiIdentityMatchGraph, WorthUiIdentityMatchNode, WorthUiIdentityMatchNodeKind,
    WorthUiIdentityMatchNodeSide, WorthUiIdentityMatchReport, WorthUiIdentitySeedContribution,
    WorthUiMovedNodeIdentity, WorthUiRepeatedTemplateIdentity,
};
pub use measurement::{
    WorthUiCertifiedMeasurementPacket, WorthUiComplexityContract, WorthUiCounterAuthority,
    WorthUiCounterCaptureRichness, WorthUiCounterPacketBuilder, WorthUiCounterValueKind,
    WorthUiFoundationalCounterBridge, WorthUiFoundationalCounterEvidence, WorthUiFrameCostCounter,
    WorthUiMeasurementBoundary, WorthUiMeasurementCertificationDenial,
    WorthUiMeasurementCounterPacket, WorthUiMeasurementQueryEvidence,
    WorthUiMeasurementQueryEvidenceKind, WorthUiRuntimeCounterFamily,
};
pub use narrowing::{
    WorthUiAccessibilityInvalidation, WorthUiCommandBindingInvalidation,
    WorthUiImpactLookupCounters, WorthUiQueryDependencyInvalidation, WorthUiQueryDependencySurface,
    WorthUiRendererResourceInvalidation, WorthUiRuntimeImpactNarrower,
    WorthUiRuntimeImpactNarrowing, WorthUiRuntimeImpactNarrowingDenial, WorthUiTokenInvalidation,
};
pub use ordinary_lane::{
    WorthUiOrdinaryExecutionLane, WorthUiOrdinaryFrameTarget, WorthUiOrdinaryLaneCertification,
    WorthUiOrdinaryLaneCounters, WorthUiOrdinaryLaneFrameDenial,
    WorthUiOrdinaryLaneFrameDenialReason, WorthUiOrdinaryLaneFrameReceipt, WorthUiOrdinaryLaneNode,
    WorthUiOrdinaryLanePlan, WorthUiOrdinaryLanePlanDenial, WorthUiOrdinaryLanePlanDenialReason,
};
pub use plan_equivalence::{
    WorthUiExecutionPlanDigest, WorthUiExecutionPlanEquivalence,
    WorthUiExecutionPlanEquivalenceBasis, WorthUiExecutionPlanEquivalenceCounters,
    WorthUiPlanReuseClassification,
};
pub use plan_inspection::{
    WorthUiArtifactToPlanProvenance, WorthUiExecutionPlanInspection, WorthUiLaneInspection,
    WorthUiPlanInspectionCounters, WorthUiPlanInspectionDenial, WorthUiPlanInspectionDenialReason,
    WorthUiPlanNodeInspection, WorthUiPlanProvenanceSource, WorthUiQueryInspectionLinks,
};
pub use plan_topology::{
    WorthUiEguiBoundaryContact, WorthUiEguiPlanBoundary, WorthUiExecutionPlan,
    WorthUiPlanChildRange, WorthUiPlanExecutionLane, WorthUiPlanLanePartition,
    WorthUiPlanLookupIndex, WorthUiPlanNode, WorthUiPlanNodeFamily, WorthUiPlanRegionStructure,
    WorthUiPlanTopology, WorthUiPlanTopologyCounters, WorthUiPlanTopologyDenial,
    WorthUiPlanTopologyDenialReason, WorthUiRenderResourceRef,
};
pub use preservation::WorthUiLastValidObservation;
pub use query_binding::{
    WorthUiQueryBindingComparison, WorthUiQueryBindingComparisonCounters,
    WorthUiQueryBindingComparisonDenial, WorthUiQueryBindingComparisonEntry,
    WorthUiQueryBindingComparisonOutcome, WorthUiQueryBindingIdentity, WorthUiQueryBindingPosture,
    WorthUiQueryBindingPostureDriftFamily,
};
pub use query_live_rebind::{
    WorthUiQueryBindingDriftDenial, WorthUiQueryBindingDriftDenialKind,
    WorthUiQueryBindingPreservation, WorthUiQueryBindingRebind, WorthUiQueryBindingRebindReason,
    WorthUiQueryBindingRetirement, WorthUiQueryBindingRetirementReason,
    WorthUiQueryLiveRebindCounters, WorthUiQueryLiveRebindEntry, WorthUiQueryLiveRebindOutcome,
    WorthUiQueryLiveRebindPlan, WorthUiQueryLiveRebindPlanDenial,
    WorthUiQueryRebindRequiredSurface,
};
pub use realtime_overlay_lane::{
    WorthUiHighFrequencyFramePolicy, WorthUiHighFrequencyFramePolicyDenial,
    WorthUiHighFrequencyFramePolicyDenialReason, WorthUiHudNode, WorthUiHudPlan,
    WorthUiHudPlanDenial, WorthUiHudPlanDenialReason, WorthUiRealtimeCertification,
    WorthUiRealtimeFrameDenial, WorthUiRealtimeFrameDenialReason, WorthUiRealtimeFramePriority,
    WorthUiRealtimeFrameReceipt, WorthUiRealtimeFrameTarget, WorthUiRealtimeLaneCounters,
    WorthUiRealtimeOverlayHook, WorthUiRealtimeOverlayLane, WorthUiRendererSurfaceAdmission,
    WorthUiRendererSurfaceHandle,
};
pub use reconciliation::{
    WorthUiAdmittedDurableResizeInput, WorthUiDurableResizeInputPosture,
    WorthUiDurableStateCarryForward, WorthUiDurableStateReconciliationCounters,
    WorthUiDurableStateReconciliationDenial, WorthUiDurableStateReconciliationOutcome,
    WorthUiDurableStateReconciliationPlan, WorthUiDurableStateReconciliationReceipt,
    WorthUiDurableStateReplacement, WorthUiFocusChainReconciliation,
    WorthUiPanelVisibilityReconciliation, WorthUiScrollAnchorReconciliation,
    WorthUiSelectionRangeReconciliation, WorthUiSplitterPositionReconciliation,
    WorthUiTabStateReconciliation, WorthUiTextEditStateReconciliation,
};
pub use reload_counter_boundary::{
    WorthUiCertifiedReloadLoweringCounterReceipt, WorthUiReloadCounterBoundary,
    WorthUiReloadCounterBoundaryDenial, WorthUiReloadCounterBoundaryDenialReason,
    WorthUiReloadCounterStopStage, WorthUiReloadLoweringCounterReceipt,
    WorthUiReloadLoweringCounterReceiptBuilder, WorthUiReloadLoweringFoundationalBridge,
    WorthUiReloadLoweringFoundationalEvidence,
};
pub use reload_failure::{
    WorthUiFailedActivationReport, WorthUiReloadCheckedStopPosture, WorthUiReloadDenial,
    WorthUiReloadFailure, WorthUiReloadFailureCounters, WorthUiReloadFailureStage,
    WorthUiReloadPreservationReceipt,
};
pub use reload_storm_certification::{
    WorthUiReloadCertificationBundle, WorthUiReloadLatencyCounters,
    WorthUiReloadStormCandidateDenialReason, WorthUiReloadStormCandidateStep,
    WorthUiReloadStormCandidateStepKind, WorthUiReloadStormCertification,
    WorthUiReloadStormCertificationDenial, WorthUiReloadStormCertificationDenialReason,
    WorthUiReloadStormDeniedIteration, WorthUiReloadStormIterationOutcome,
    WorthUiReloadStormNoOpIteration, WorthUiReloadStormOrderedTruth,
    WorthUiReloadStormReceiptBinding, WorthUiReloadStormScenario,
    WorthUiReloadStormSuccessfulIteration,
};
pub use replacement::{
    WorthUiAmbiguousReplacementDenial, WorthUiNodeLifecycleTransition,
    WorthUiNodeReplacementClassification, WorthUiNodeReplacementCounters,
    WorthUiNodeReplacementPlan,
};
pub(crate) use source_ingress::WorthUiSourceBackedDeclarationWitness;
pub use source_ingress::{
    WorthUiCandidateOrderingReceipt, WorthUiDebouncedWatcherBatch, WorthUiReloadDebounce,
    WorthUiSourceBackedDslPackage, WorthUiSourceIngressCounters, WorthUiSourceIngressDenial,
    WorthUiSourceIngressDenialReason, WorthUiSourceIngressHook, WorthUiSourceIngressSession,
    WorthUiSourcePackageRevision, WorthUiSourceProvider, WorthUiSourceProviderKind,
    WorthUiSourceWatcher, WorthUiWatchedArtifactInput, WorthUiWatchedCandidateSubmission,
    WorthUiWatchedCandidateSubmissionDenial, WorthUiWatcherEvent,
};
pub use state_inventory::{
    WorthUiDurableStateEligibility, WorthUiDurableStateFamily, WorthUiDurableStateFamilyHook,
    WorthUiDurableStateFamilyId, WorthUiDurableStateInventory, WorthUiDurableStateInventoryBuilder,
    WorthUiDurableStateInventoryCounters, WorthUiDurableStateInventoryDenial,
    WorthUiDurableStateReplacementPolicy, WorthUiStateOwnerIdentity, WorthUiStateOwnershipClass,
    WorthUiStatePersistencePosture, WorthUiTransientInteractionPolicy,
    WorthUiTransientInteractionState,
};
pub use steady_frame_counter_boundary::{
    WorthUiCertifiedFrameExecutionReceipt, WorthUiFrameExecutionReceipt,
    WorthUiFrameReportMaterializationBoundary, WorthUiLaneFrameReceipt,
    WorthUiLaneFrameReceiptKind, WorthUiRenderCostReceipt, WorthUiSteadyFrameCounterBoundary,
    WorthUiSteadyFrameCounterDenial, WorthUiSteadyFrameCounterDenialReason,
    WorthUiSteadyFrameCounterReceiptBuilder, WorthUiSteadyFrameCounters,
    WorthUiSteadyFrameDiagnosticPolicy, WorthUiSteadyFrameFoundationalBridge,
    WorthUiSteadyFrameFoundationalEvidence, WorthUiSteadyFrameReportPlan,
    WorthUiSteadyFrameReportPlanner,
};
#[cfg(test)]
pub use touch_origin_certification_support::{
    runtime_origin_fixture, WorthUiTouchOriginCertificationFixture,
    WorthUiTouchOriginFixtureVariant,
};
pub use virtualized_data_lane::{
    WorthUiQueryPatchPosture, WorthUiVirtualizedDataCertification, WorthUiVirtualizedDataCounters,
    WorthUiVirtualizedDataFrameDenial, WorthUiVirtualizedDataFrameDenialReason,
    WorthUiVirtualizedDataFrameReceipt, WorthUiVirtualizedDataFrameTarget,
    WorthUiVirtualizedDataLane, WorthUiVirtualizedDataNode, WorthUiVirtualizedDataPlan,
    WorthUiVirtualizedDataPlanDenial, WorthUiVirtualizedDataPlanDenialReason, WorthUiVisibleRange,
    WorthUiVisibleRangeDenial, WorthUiVisibleRangeDenialReason,
};

#[cfg(test)]
mod runtime_test_modules;
#[cfg(test)]
pub(crate) use runtime_test_modules::{
    dependency_impact_narrowing_test_support, replacement_impact_test_support,
    source_ingress_test_support,
};
