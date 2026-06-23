mod diagnostics;
mod family_inventory;
mod identity;
mod registered_set;
mod registration;
mod registry;
mod snapshot;
mod support;

pub use diagnostics::{
    CapabilityDiagnosticCode, CapabilityDiagnosticRichness, CapabilityDiagnosticSeverity,
    CapabilityRegistrationDiagnostic, CapabilityRegistrationReport,
};
pub use family_inventory::{
    RegistryFamily, RegistryFamilyFacadeExposure, RegistryFamilyInventoryAudit,
    RegistryFamilyLifecyclePropagation,
};
pub use identity::{
    AppearanceTokenId, CapabilityIdError, CommandId, CommandProjectionId, ComponentId,
    DensityTokenId, IconId, MosaicPlacementPolicyId, MosaicRegionKindId, MosaicSizingContractId,
    MosaicStateOwnerScopeId, MosaicStateSlotId, NativeCapabilityId, PluginSlotId,
    RuntimeOutcomeProjectionId, SettingId, SurfaceId, TaskPresentationId, ThemeTokenId,
    ViewBindingId,
};
pub use registered_set::RegisteredCapabilitySet;
pub(crate) use registration::{
    validate_registration_candidates, RegistrationCandidate, RegistrationCandidateDiagnostic,
    RegistrationDependency,
};
pub use registry::{
    AmbientHostCheck, ArbitraryKeyValueSettingBag, CommandCategory, CommandDescriptor,
    CommandProjectionCommandReference, CommandProjectionDescriptor, CommandProjectionGrouping,
    CommandProjectionIconLabelPolicy, CommandProjectionKey, CommandProjectionMeaningOverride,
    CommandProjectionMosaicScope, CommandProjectionOrdering, CommandProjectionOverflowBehavior,
    CommandProjectionReadinessDisplayPolicy, CommandProjectionSelectionMode,
    CommandProjectionShortcutVisibility, CommandProjectionSurface, CommandReadinessBinding,
    CommandRuntimeIntentBinding, ComponentAccessibilitySupport, ComponentChildPolicy,
    ComponentDescriptor, ComponentExecutionLane, ComponentFocusSupport, ComponentPropSchema,
    ComponentStateOwnership, FrozenAppearanceCapabilities, FrozenCommandCapabilities,
    FrozenCommandProjectionCapabilities, FrozenCommandProjectionEntry, FrozenComponentCapabilities,
    FrozenDensityCapabilities, FrozenIconCapabilities, FrozenIconEntry,
    FrozenMosaicPlacementCapabilities, FrozenMosaicRegionCapabilities,
    FrozenMosaicSizingCapabilities, FrozenMosaicStateCapabilities, FrozenMosaicStateSlotEntry,
    FrozenNativeCapabilities, FrozenNativeCapabilityEntry, FrozenPluginSlotCapabilities,
    FrozenPluginSlotEntry, FrozenRuntimeOutcomeProjectionCapabilities,
    FrozenRuntimeOutcomeProjectionEntry, FrozenSettingCapabilities, FrozenSettingEntry,
    FrozenSurfaceCapabilities, FrozenTaskPresentationCapabilities, FrozenTaskPresentationEntry,
    FrozenThemeTokenCapabilities, FrozenThemeTokenEntry, FrozenViewBindingCapabilities,
    FrozenViewBindingEntry, IconAccessibilityPosture, IconColorSupport, IconDescriptor, IconFamily,
    IconKey, IconNativeVectorSupport, IconSizeSupport, IconSourceDescriptor, IconSourceKind,
    IconThemePosture, MeasurementConstraint, MeasurementValue, MosaicChildRule,
    MosaicClippingPosture, MosaicFocusScopeKind, MosaicHitTestPosture, MosaicMeasurementAuthority,
    MosaicOverflowBehavior, MosaicParentGrowthBehavior, MosaicPlacementAction,
    MosaicPlacementConflictBehavior, MosaicPlacementEligibility, MosaicPlacementPersistence,
    MosaicPlacementPolicyDescriptor, MosaicPlacementReloadReconciliation, MosaicPlacementSource,
    MosaicPlacementSupport, MosaicPlacementTarget, MosaicRegionKindDescriptor,
    MosaicRegionPersistence, MosaicRegionRole, MosaicResizePermission, MosaicScrollOwnership,
    MosaicSizingBehavior, MosaicSizingContractDescriptor, MosaicSizingKind,
    MosaicSizingPersistence, MosaicStableIdentityBehavior, MosaicStateOwnerIdentity,
    MosaicStatePersistencePolicy, MosaicStateReconciliationKey, MosaicStateReplacementRule,
    MosaicStateSlotDescriptor, MosaicStateSlotKind, MosaicStateTruthPosture,
    MosaicViewportConstraint, NamedMeasurementDefinition, NamedMeasurementToken,
    NativeCapabilityDescriptor, NativeCapabilityFamily, NativeCapabilityKey, NativePlatformPosture,
    NativeShellAuthorityClaim, PluginCapabilityPermission, PluginContributionFamily,
    PluginSlotContributionReference, PluginSlotDescriptor, PluginSlotDiagnostics,
    PluginSlotGlobalMutationHook, PluginSlotKey, PluginSlotOrdering, PluginSlotSupportPosture,
    QueryBasisPostureReference, QueryDenialPresentation, QueryLiveCompatibility,
    QueryResultShapeReference, QueryViewBindingKey, QueryViewCapabilityReference,
    RawColorOutsideTokenDefinition, RawIconAssetReference, RawLayoutMeasurementForDiagnostics,
    RawLayoutMeasurementKind, RuntimeOutcomeAffordance, RuntimeOutcomeDenialPosture,
    RuntimeOutcomeFamily, RuntimeOutcomePresentation, RuntimeOutcomeProjectionDescriptor,
    RuntimeOutcomeProjectionKey, RuntimeOutcomeRecoveryPosture, RuntimeOutcomeSourceReference,
    RuntimeOutcomeTone, SettingDefaultPosture, SettingDefaultValue, SettingDescriptor,
    SettingEditorHint, SettingKey, SettingMigrationPosture, SettingOwnershipMetadata, SettingScope,
    SettingValidationPosture, SettingValueSchema, SurfaceDescriptor, SurfaceKind,
    SurfacePlacementClass, SurfaceStateClass, TaskPresentationCancellationPosture,
    TaskPresentationDescriptor, TaskPresentationFailurePosture, TaskPresentationFamily,
    TaskPresentationKey, TaskPresentationLifecyclePosture, TaskPresentationProjectionEligibility,
    TaskPresentationRuntimeAuthorityPosture, ThemeColorValue, ThemeColorValueError,
    ThemeTokenAlias, ThemeTokenDescriptor, ThemeTokenFamily, ThemeTokenKey, ThemeTokenSource,
    ThemeTokenValue, ViewBindingDescriptor, ViewBindingFamily, VisibleStateBindingDeclaration,
    WorthUiAppearanceFamily, WorthUiAppearanceTokenDescriptor, WorthUiAppearanceTokenSource,
    WorthUiAppearanceValue, WorthUiBorderWidthValue, WorthUiCornerRadiusValue,
    WorthUiDensityFamily, WorthUiDensityPostureValue, WorthUiDensityTokenDescriptor,
    WorthUiDensityValue, WorthUiFontSizeValue, WorthUiLengthValue, WorthUiPaddingValue,
    WorthUiShadowValue, WorthUiSpacingValue, WorthUiStyleValueError, WorthUiStyleValueErrorReason,
};
pub(crate) use registry::{
    CommandAcceptedRegistrationProof, CommandProjectionAcceptedRegistrationProof,
    CommandProjectionRegistry, CommandRegistry, ComponentAcceptedRegistrationProof,
    ComponentRegistry, IconAcceptedRegistrationProof, IconRegistry,
    MosaicPlacementAcceptedRegistrationProof, MosaicPlacementRegistry,
    MosaicRegionAcceptedRegistrationProof, MosaicRegionRegistry,
    MosaicSizingAcceptedRegistrationProof, MosaicSizingRegistry,
    MosaicStateSlotAcceptedRegistrationProof, MosaicStateSlotRegistry,
    NativeCapabilityAcceptedRegistrationProof, NativeCapabilityRegistry,
    PluginSlotAcceptedRegistrationProof, PluginSlotRegistry,
    RuntimeOutcomeProjectionAcceptedRegistrationProof, RuntimeOutcomeProjectionRegistry,
    SettingAcceptedRegistrationProof, SettingsRegistry, SurfaceAcceptedRegistrationProof,
    SurfaceRegistry, TaskPresentationAcceptedRegistrationProof, TaskPresentationRegistry,
    ThemeTokenAcceptedRegistrationProof, ThemeTokenRegistry, ViewBindingAcceptedRegistrationProof,
    ViewBindingRegistry, WorthUiAppearanceAcceptedRegistrationProof, WorthUiAppearanceRegistry,
    WorthUiDensityAcceptedRegistrationProof, WorthUiDensityRegistry, APPEARANCE_TOKEN_FAMILY_NAME,
    COMMAND_FAMILY_NAME, COMMAND_PROJECTION_FAMILY_NAME, COMPONENT_FAMILY_NAME,
    DENSITY_TOKEN_FAMILY_NAME, ICON_FAMILY_NAME, MOSAIC_PLACEMENT_POLICY_FAMILY_NAME,
    MOSAIC_REGION_KIND_FAMILY_NAME, MOSAIC_SIZING_CONTRACT_FAMILY_NAME,
    MOSAIC_STATE_SLOT_FAMILY_NAME, NATIVE_CAPABILITY_FAMILY_NAME, PLUGIN_SLOT_FAMILY_NAME,
    RUNTIME_OUTCOME_PROJECTION_FAMILY_NAME, SETTING_FAMILY_NAME, SURFACE_FAMILY_NAME,
    TASK_PRESENTATION_FAMILY_NAME, THEME_TOKEN_FAMILY_NAME, VIEW_BINDING_FAMILY_NAME,
};
pub use snapshot::{
    CapabilitySnapshot, CapabilitySnapshotDigest, CapabilitySnapshotIndex, FrozenCapabilityFamily,
    SnapshotFamilyIndex, SnapshotFreezeReport, SnapshotLookupCounters, SnapshotLookupReport,
    SnapshotMetrics, SnapshotReferenceValidationReport, SnapshotReferenceViolation,
    SnapshotReferenceViolationKind, SupportSnapshot,
};
pub(crate) use snapshot::{
    CapabilitySnapshotBuilder, CapabilitySnapshotFreezeInput, CapabilitySnapshotIndexParts,
    CapabilitySupportCatalog,
};
pub use support::{
    AdmittedCapability, CapabilitySupportId, CapabilitySupportKind, CapabilitySupportPosture,
    CapabilitySupportRejection, DeferredCapability, PlatformInternalCapability, SupportRequirement,
    UnsupportedCapability,
};
