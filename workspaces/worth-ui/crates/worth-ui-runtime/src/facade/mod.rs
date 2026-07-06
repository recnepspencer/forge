//! Public Worth UI runtime capability and inspection surfaces.

pub mod admission;
mod app;
mod app_builder;
mod app_derived_state;
mod app_inspection_closeout;
mod app_inspection_support;
mod builder;
pub mod declaration;
pub mod graph;
mod inspection;
mod inspection_observation;
mod inspection_receipt;
mod measurement_inspection_evidence;
#[cfg(test)]
mod measurement_inspection_query_denial_tests;
#[cfg(test)]
mod measurement_inspection_test_support;
#[cfg(test)]
mod measurement_inspection_tests;
mod obligation_inspection;
pub mod obligations;
pub mod query_binding;
mod retained_obligation_registry;
mod runtime_bridge;

pub use crate::capability::{
    AdmittedCapability, AmbientHostCheck, ArbitraryKeyValueSettingBag, CapabilityDiagnosticCode,
    CapabilityDiagnosticRichness, CapabilityDiagnosticSeverity, CapabilityIdError,
    CapabilityRegistrationDiagnostic, CapabilityRegistrationReport, CapabilitySnapshot,
    CapabilitySnapshotDigest, CapabilitySnapshotIndex, CapabilitySupportId, CapabilitySupportKind,
    CapabilitySupportPosture, CapabilitySupportRejection, CommandCategory, CommandDescriptor,
    CommandId, CommandProjectionCommandReference, CommandProjectionDescriptor,
    CommandProjectionGrouping, CommandProjectionIconLabelPolicy, CommandProjectionId,
    CommandProjectionKey, CommandProjectionMeaningOverride, CommandProjectionMosaicScope,
    CommandProjectionOrdering, CommandProjectionOverflowBehavior,
    CommandProjectionReadinessDisplayPolicy, CommandProjectionShortcutVisibility,
    CommandProjectionSurface, CommandReadinessBinding, CommandRuntimeIntentBinding,
    ComponentAccessibilitySupport, ComponentChildPolicy, ComponentDescriptor,
    ComponentExecutionLane, ComponentFocusSupport, ComponentId, ComponentPropSchema,
    ComponentStateOwnership, DeferredCapability, FrozenCapabilityFamily, FrozenCommandCapabilities,
    FrozenCommandProjectionCapabilities, FrozenCommandProjectionEntry, FrozenComponentCapabilities,
    FrozenIconCapabilities, FrozenIconEntry, FrozenMosaicPlacementCapabilities,
    FrozenMosaicRegionCapabilities, FrozenMosaicSizingCapabilities, FrozenMosaicStateCapabilities,
    FrozenMosaicStateSlotEntry, FrozenNativeCapabilities, FrozenNativeCapabilityEntry,
    FrozenPluginSlotCapabilities, FrozenPluginSlotEntry,
    FrozenRuntimeOutcomeProjectionCapabilities, FrozenRuntimeOutcomeProjectionEntry,
    FrozenSettingCapabilities, FrozenSettingEntry, FrozenSurfaceCapabilities,
    FrozenTaskPresentationCapabilities, FrozenTaskPresentationEntry, FrozenThemeTokenCapabilities,
    FrozenThemeTokenEntry, FrozenViewBindingCapabilities, FrozenViewBindingEntry,
    IconAccessibilityPosture, IconColorSupport, IconDescriptor, IconFamily, IconId, IconKey,
    IconSizeSupport, IconSourceDescriptor, IconSourceKind, IconThemePosture, MeasurementConstraint,
    MeasurementValue, MosaicChildRule, MosaicClippingPosture, MosaicFocusScopeKind,
    MosaicHitTestPosture, MosaicMeasurementAuthority, MosaicOverflowBehavior,
    MosaicParentGrowthBehavior, MosaicPlacementAction, MosaicPlacementConflictBehavior,
    MosaicPlacementEligibility, MosaicPlacementPersistence, MosaicPlacementPolicyDescriptor,
    MosaicPlacementPolicyId, MosaicPlacementReloadReconciliation, MosaicPlacementSource,
    MosaicPlacementSupport, MosaicPlacementTarget, MosaicRegionKindDescriptor, MosaicRegionKindId,
    MosaicRegionPersistence, MosaicRegionRole, MosaicResizePermission, MosaicScrollOwnership,
    MosaicSizingBehavior, MosaicSizingContractDescriptor, MosaicSizingContractId, MosaicSizingKind,
    MosaicSizingPersistence, MosaicStableIdentityBehavior, MosaicStateOwnerIdentity,
    MosaicStateOwnerScopeId, MosaicStatePersistencePolicy, MosaicStateReconciliationKey,
    MosaicStateReplacementRule, MosaicStateSlotDescriptor, MosaicStateSlotId, MosaicStateSlotKind,
    MosaicStateTruthPosture, MosaicViewportConstraint, NamedMeasurementDefinition,
    NamedMeasurementToken, NativeCapabilityDescriptor, NativeCapabilityFamily, NativeCapabilityId,
    NativeCapabilityKey, NativePlatformPosture, NativeShellAuthorityClaim,
    PlatformInternalCapability, PluginCapabilityPermission, PluginContributionFamily,
    PluginSlotContributionReference, PluginSlotDescriptor, PluginSlotDiagnostics,
    PluginSlotGlobalMutationHook, PluginSlotId, PluginSlotKey, PluginSlotOrdering,
    PluginSlotSupportPosture, QueryBasisPostureReference, QueryDenialPresentation,
    QueryLiveCompatibility, QueryResultShapeReference, QueryViewBindingKey,
    QueryViewCapabilityReference, RawColorOutsideTokenDefinition, RawIconAssetReference,
    RawLayoutMeasurementForDiagnostics, RawLayoutMeasurementKind, RegisteredCapabilitySet,
    RegistryFamily, RegistryFamilyFacadeExposure, RegistryFamilyInventoryAudit,
    RegistryFamilyLifecyclePropagation, RuntimeOutcomeAffordance, RuntimeOutcomeDenialPosture,
    RuntimeOutcomeFamily, RuntimeOutcomePresentation, RuntimeOutcomeProjectionDescriptor,
    RuntimeOutcomeProjectionId, RuntimeOutcomeProjectionKey, RuntimeOutcomeRecoveryPosture,
    RuntimeOutcomeSourceReference, RuntimeOutcomeTone, SettingDefaultPosture, SettingDefaultValue,
    SettingDescriptor, SettingEditorHint, SettingId, SettingKey, SettingMigrationPosture,
    SettingOwnershipMetadata, SettingScope, SettingValidationPosture, SettingValueSchema,
    SnapshotFamilyIndex, SnapshotFreezeReport, SnapshotLookupCounters, SnapshotLookupReport,
    SnapshotMetrics, SnapshotReferenceValidationReport, SnapshotReferenceViolation,
    SnapshotReferenceViolationKind, SupportRequirement, SupportSnapshot, SurfaceDescriptor,
    SurfaceId, SurfaceKind, SurfacePlacementClass, SurfaceStateClass,
    TaskPresentationCancellationPosture, TaskPresentationDescriptor,
    TaskPresentationFailurePosture, TaskPresentationFamily, TaskPresentationId,
    TaskPresentationKey, TaskPresentationLifecyclePosture, TaskPresentationProjectionEligibility,
    TaskPresentationRuntimeAuthorityPosture, ThemeColorValue, ThemeColorValueError,
    ThemeTokenAlias, ThemeTokenDescriptor, ThemeTokenFamily, ThemeTokenId, ThemeTokenKey,
    ThemeTokenSource, ThemeTokenValue, UnsupportedCapability, ViewBindingDescriptor,
    ViewBindingFamily, ViewBindingId, VisibleStateBindingDeclaration,
};
pub use crate::evidence::{
    admit_measurement_basis, certify_activation_boundary_suite,
    certify_allocation_inspection_suite, certify_allocation_neighborhood_suite,
    certify_allocation_planning_determinism, certify_allocation_planning_suite,
    certify_bounded_reconciliation_suite, certify_constraint_edge_suite,
    certify_durable_resize_input_suite, certify_equal_share_suite,
    certify_intrinsic_return_flow_suite, certify_parent_child_propagation_suite,
    certify_plan_handoff_suite, certify_sibling_negotiation_suite,
    certify_special_input_suite,
    certify_measurement_basis_determinism,
    certify_measurement_basis_determinism_for_scenarios,
    consume_declared_measurement_projection_facts, MeasurementEvidenceInput,
    UiAllocationPlanningCertificationReport, UiAllocationPlanningCostClass,
    UiAllocationPlanningCertificationSuiteKind, UiAllocationPlanningCostReceipt,
    UiAllocationPlanningDeniedBroadeningReason, UiAllocationPlanningDeterminismPosture,
    UiAllocationSolveConvergencePosture,
    UiAllocationSolvePass, UiAllocationSolveRemainderPolicy, UiAllocationSolveTrace,
    UiCurrentMeasurementResult, UiEvidenceExpansion, UiEvidenceFamilySummary, UiEvidenceHandle,
    UiEvidenceIdentity, UiEvidenceMaterializedDetail, UiEvidenceRef, UiEvidenceSlice,
    UiEvidenceSliceRef, UiInspectionCostReceipt, UiInspectionObligationEvidenceReceipt,
    UiInspectionObligationReasonProjection, UiMeasurementBasis,
    UiMeasurementBasisCertificationHostRequest, UiMeasurementBasisCertificationOutcome,
    UiMeasurementBasisCertificationReport, UiMeasurementBasisCertificationScenario,
    UiMeasurementBasisCertificationScenarioError, UiMeasurementBasisDenial,
    UiMeasurementBasisDeterminismPosture, UiMeasurementBasisGeneration, UiMeasurementBasisPosture,
    UiMeasurementCoordinateSpace, UiMeasurementDependencyLineage,
    UiMeasurementDependencyLineageEntry, UiMeasurementDependencyLineageKind,
    UiMeasurementEvidenceCategory, UiMeasurementEvidenceSlot, UiMeasurementGenerationCompatibility,
    UiMeasurementNeighborhoodClassHint, UiMeasurementResult, UiMeasurementRoundingPosture,
    UiMeasurementUnitPosture, UiMeasurementValue, UiProjectionFactReceipt,
    UiProjectionFactReceiptDenial,
};
pub use crate::graph::{
    project_aspect_evidence_ref, project_aspect_evidence_refs, UiAspectEvidenceLane,
    UiAspectEvidenceRefProjection, UiAspectEvidenceSubjectKind, UiGraphNodeIdentity,
    UiMountedReceiptIdentity,
};
pub use crate::host::{
    admit_current_host_measurement_evidence, collect_host_measurement_evidence,
    freeze_measurement_request, UiHostMeasurementAssumptionProfile,
    UiHostMeasurementEvidenceDenial, UiHostMeasurementExecutionDenial,
    UiHostMeasurementFreshnessWitness, UiHostMeasurementInvalidationReason, UiHostMeasurementNeed,
    UiHostMeasurementNormalizationContext, UiHostMeasurementNormalizationDenial,
};
pub use crate::lifecycle::{WorthUiRuntimeSupportInventory, PHASE3_RUNTIME_SUPPORT_INVENTORY};
pub use crate::runtime::*;
pub use app::{WorthUi, WorthUiApp};
pub use app_builder::{WorthUiAppBuilder, WorthUiBuilder};
pub use builder::CapabilityRegistrationBuilder;
pub(crate) use inspection::foreign_evidence_refs_for_obligation_record;
pub use inspection::UiInspectionAiHarness;
pub use inspection_observation::UiInspectionFacadeObservation;
pub use inspection_receipt::UiInspectionReceipt;
pub use measurement_inspection_evidence::UiMeasurementInspectionEvidenceBundle;
pub use worth_ui_dsl::WorthUiDslPackage;
pub use worth_ui_host_contract::{
    UiDpiScaleFactorObservation, UiDpiScaleFactorRequest, UiFontMeasurementKey,
    UiFontMetricsObservation, UiFontMetricsRequest, UiForbiddenHostAuthorityAsk, UiHostObservation,
    UiHostObservationContractDenial, UiHostObservationValue, UiMeasurementCapabilityPosture,
    UiMeasurementEvidenceFamily, UiMeasurementRequest, UiMeasurementRequestDenial,
    UiMeasurementRequestFamily, UiMeasurementRequestIdentity,
    UiNativeControlIntrinsicSizeObservation, UiNativeControlIntrinsicSizeRequest,
    UiNativeControlKind, UiPortalAnchorRectObservation, UiPortalAnchorRectRequest,
    UiScrollContainerViewportObservation, UiScrollContainerViewportRequest,
    UiTextBaselineMetricsObservation, UiTextBaselineMetricsRequest, UiTextIntrinsicSizeObservation,
    UiTextIntrinsicSizeRequest, UiViewportExtentObservation, UiViewportExtentRequest,
    WorthUiHostAdapter, WorthUiHostCapability, WorthUiHostCapabilityPosture,
    WorthUiHostCapabilityReport, WorthUiHostContract, WorthUiMeasurementHostAdapter,
};
pub use worth_ui_inspection::{
    UiAuthoredSourceProvenanceRef, UiEvidenceAuthorityArtifactIdentity, UiEvidenceAuthorityBinding,
    UiEvidenceAuthorityGeneration, UiEvidenceAuthorityKind, UiEvidenceBudget,
    UiEvidenceExpansionOutcome, UiEvidenceFamily, UiEvidenceLinkKind,
    UiEvidenceMaterializationPosture, UiEvidenceRetentionPosture, UiEvidenceRichness,
    UiEvidenceSliceOmission, UiInspectionAdmissionHostCapability, UiInspectionAdmissionPosture,
    UiInspectionAdmissionQueryBasis, UiInspectionAdmissionStaleEvidence, UiInspectionAiHarnessLane,
    UiInspectionAspectName, UiInspectionAspectRelevanceDetail, UiInspectionClosedSemanticLane,
    UiInspectionCloseoutGuarantee, UiInspectionCloseoutNonGoal, UiInspectionCloseoutReport,
    UiInspectionClosureReport, UiInspectionCostLane, UiInspectionDeclarationIdentity,
    UiInspectionDeferredPosture, UiInspectionDerivedIndexLane, UiInspectionDiagnosticOnlyPosture,
    UiInspectionEvidenceSource, UiInspectionForeignEvidenceCitation,
    UiInspectionForeignEvidenceRef, UiInspectionMeasurementBasisInput,
    UiInspectionMeasurementBasisPosture, UiInspectionMeasurementBasisSource,
    UiInspectionMeasurementDenialPosture, UiInspectionMeasurementDependencyLineageEntry,
    UiInspectionMeasurementDependencyLineageKind, UiInspectionMeasurementEvidenceCategory,
    UiInspectionMeasurementEvidenceSlot, UiInspectionMeasurementEvidenceView,
    UiInspectionMeasurementFailureSource, UiInspectionMeasurementGenerationCompatibility,
    UiInspectionMeasurementNeighborhoodClassHint, UiInspectionMeasurementOwnershipPosture,
    UiInspectionMilestoneExpectation, UiInspectionObligationDecision,
    UiInspectionObligationDenialPosture, UiInspectionObligationDispatchPosture,
    UiInspectionObligationFamily, UiInspectionObligationLegalityReason,
    UiInspectionObligationNonSelectionReason, UiInspectionObligationRelevanceDetail,
    UiInspectionObligationSelectionReason, UiInspectionObligationVerdictClass,
    UiInspectionObligationVerdictPosture, UiInspectionPosture, UiInspectionQuery,
    UiInspectionQueryForeignEvidenceArtifactKind, UiInspectionQueryForeignEvidenceCitation,
    UiInspectionQueryForeignEvidenceKind, UiInspectionQueryForeignEvidenceRef,
    UiInspectionRefLifecycleLane, UiInspectionRelevance, UiInspectionRelevanceAdmission,
    UiInspectionRelevanceOutcome, UiInspectionScope, UiInspectionScopeSupportRow,
    UiInspectionSliceLane, UiInspectionSupportPosture, UiInspectionSupportReason,
    UiInspectionSupportReport, UiInspectionSupportRowSchemaKind, UiInspectionSupportStatus,
    UiInspectionSupportWorld, UiInspectionTarget, UiInspectionTargetClass,
    UiInspectionTouchAspectPosture, UiInspectionTouchOriginClass, UiInspectionTouchRuntimeLane,
    UiInspectionTouchTargetClass, UiInspectionUnsupportedPosture, UiInspectionWrongWorldPosture,
    UiRelevanceFamily, UiRelevanceFilter, UiSourceArtifactGeneration, UiSourceArtifactIdentity,
};
