//! Public Worth UI runtime capability and inspection surfaces.

mod app;
mod app_builder;
mod builder;
pub mod declaration;
pub mod graph;
mod inspection_observation;
mod inspection_receipt;
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
pub use crate::lifecycle::{WorthUiRuntimeSupportInventory, PHASE3_RUNTIME_SUPPORT_INVENTORY};
pub use crate::runtime::*;
pub use app::{WorthUi, WorthUiApp};
pub use app_builder::{WorthUiAppBuilder, WorthUiBuilder};
pub use builder::CapabilityRegistrationBuilder;
pub use inspection_observation::UiInspectionFacadeObservation;
pub use inspection_receipt::UiInspectionReceipt;
pub use worth_ui_dsl::WorthUiDslPackage;
pub use worth_ui_host_contract::{WorthUiHostAdapter, WorthUiHostContract};
pub use worth_ui_host_contract::WorthUiHostCapability;
pub use worth_ui_inspection::{
    UiEvidenceBudget, UiEvidenceRichness, UiInspectionClosureReport, UiInspectionEvidenceSource,
    UiInspectionMilestoneExpectation, UiInspectionPosture, UiInspectionQuery,
    UiInspectionRelevance, UiInspectionScope, UiInspectionScopeSupportRow,
    UiInspectionSupportReason, UiInspectionSupportReport, UiInspectionSupportStatus,
    UiInspectionTarget, UiInspectionUnsupportedPosture,
};
