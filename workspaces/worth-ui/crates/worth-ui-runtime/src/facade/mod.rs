//! Public Worth UI runtime capability and inspection surfaces.

pub mod admission;
mod app;
mod app_derived_state;
mod app_inspection_closeout;
mod app_inspection_support;
mod app_builder;
mod builder;
pub mod declaration;
pub mod graph;
mod inspection;
mod inspection_observation;
mod inspection_receipt;
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
    UiEvidenceExpansion, UiEvidenceFamilySummary, UiEvidenceHandle, UiEvidenceIdentity,
    UiEvidenceMaterializedDetail, UiEvidenceRef, UiEvidenceSlice, UiEvidenceSliceRef,
    UiInspectionCostReceipt, UiInspectionObligationEvidenceReceipt,
    UiInspectionObligationReasonProjection,
};
pub use crate::graph::{
    project_aspect_evidence_ref, project_aspect_evidence_refs, UiAspectEvidenceLane,
    UiAspectEvidenceRefProjection, UiAspectEvidenceSubjectKind, UiGraphNodeIdentity,
    UiMountedReceiptIdentity,
};
pub use crate::lifecycle::{WorthUiRuntimeSupportInventory, PHASE3_RUNTIME_SUPPORT_INVENTORY};
pub use crate::runtime::*;
pub use app::{WorthUi, WorthUiApp};
pub use app_builder::{WorthUiAppBuilder, WorthUiBuilder};
pub use builder::CapabilityRegistrationBuilder;
pub use inspection::UiInspectionAiHarness;
pub use inspection_observation::UiInspectionFacadeObservation;
pub use inspection_receipt::UiInspectionReceipt;
pub(crate) use inspection::foreign_evidence_refs_for_obligation_record;
pub use worth_ui_dsl::WorthUiDslPackage;
pub use worth_ui_host_contract::{
    WorthUiHostAdapter, WorthUiHostCapability, WorthUiHostCapabilityPosture,
    WorthUiHostCapabilityReport, WorthUiHostContract,
};
pub use worth_ui_inspection::{
    UiAuthoredSourceProvenanceRef,
    UiEvidenceAuthorityArtifactIdentity, UiEvidenceAuthorityBinding, UiEvidenceAuthorityGeneration,
    UiEvidenceAuthorityKind, UiEvidenceBudget, UiEvidenceExpansionOutcome, UiEvidenceFamily,
    UiInspectionForeignEvidenceCitation, UiInspectionForeignEvidenceRef,
    UiEvidenceLinkKind, UiEvidenceMaterializationPosture, UiEvidenceRetentionPosture,
    UiEvidenceRichness, UiEvidenceSliceOmission, UiInspectionAdmissionHostCapability,
    UiInspectionAdmissionPosture, UiInspectionAdmissionQueryBasis,
    UiInspectionAdmissionStaleEvidence, UiInspectionAiHarnessLane, UiInspectionClosureReport,
    UiInspectionClosedSemanticLane, UiInspectionCloseoutGuarantee,
    UiInspectionCloseoutNonGoal, UiInspectionCloseoutReport, UiInspectionCostLane,
    UiInspectionDeferredPosture, UiInspectionDiagnosticOnlyPosture,
    UiInspectionDerivedIndexLane, UiInspectionEvidenceSource,
    UiInspectionDeclarationIdentity,
    UiInspectionMilestoneExpectation, UiInspectionObligationDecision,
    UiInspectionObligationDenialPosture, UiInspectionObligationDispatchPosture,
    UiInspectionObligationFamily, UiInspectionObligationLegalityReason,
    UiInspectionObligationNonSelectionReason,
    UiInspectionObligationRelevanceDetail, UiInspectionObligationSelectionReason,
    UiInspectionObligationVerdictClass, UiInspectionObligationVerdictPosture,
    UiInspectionPosture, UiInspectionQuery,
    UiInspectionRelevance, UiInspectionRelevanceAdmission, UiInspectionRelevanceOutcome,
    UiInspectionRefLifecycleLane, UiInspectionScope, UiInspectionScopeSupportRow,
    UiInspectionSliceLane, UiInspectionSupportPosture, UiInspectionSupportReason,
    UiInspectionSupportReport, UiInspectionSupportRowSchemaKind, UiInspectionSupportStatus,
    UiInspectionSupportWorld, UiInspectionTarget,
    UiInspectionTargetClass, UiInspectionTouchAspectPosture, UiInspectionTouchOriginClass,
    UiInspectionTouchRuntimeLane, UiInspectionTouchTargetClass, UiInspectionUnsupportedPosture,
    UiInspectionWrongWorldPosture, UiInspectionAspectName, UiInspectionAspectRelevanceDetail,
    UiInspectionQueryForeignEvidenceArtifactKind, UiInspectionQueryForeignEvidenceCitation,
    UiInspectionQueryForeignEvidenceKind, UiInspectionQueryForeignEvidenceRef,
    UiRelevanceFamily, UiRelevanceFilter,
    UiSourceArtifactGeneration, UiSourceArtifactIdentity,
};
