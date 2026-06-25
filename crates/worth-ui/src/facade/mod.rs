//! Public Worth UI facade.

mod app;
mod builder;
mod entry;
mod exports;

pub use exports::*;
mod runtime_launch;
mod runtime_launch_diagnostics;

pub use crate::capability::{
    standard_expression_operator_descriptor, standard_expression_operator_descriptors,
    AdmittedCapability, AmbientHostCheck, AppearanceTokenId, ArbitraryKeyValueSettingBag,
    CapabilityDiagnosticCode, CapabilityDiagnosticRichness, CapabilityDiagnosticSeverity,
    CapabilityIdError, CapabilityRegistrationDiagnostic, CapabilityRegistrationReport,
    CapabilitySnapshot, CapabilitySnapshotDigest, CapabilitySnapshotIndex, CapabilitySupportId,
    CapabilitySupportKind, CapabilitySupportPosture, CapabilitySupportRejection, CommandCategory,
    CommandDescriptor, CommandId, CommandProjectionCommandReference, CommandProjectionDescriptor,
    CommandProjectionGrouping, CommandProjectionIconLabelPolicy, CommandProjectionId,
    CommandProjectionKey, CommandProjectionMeaningOverride, CommandProjectionMosaicScope,
    CommandProjectionOrdering, CommandProjectionOverflowBehavior,
    CommandProjectionReadinessDisplayPolicy, CommandProjectionSelectionMode,
    CommandProjectionShortcutVisibility, CommandProjectionSurface, CommandReadinessBinding,
    CommandRuntimeIntentBinding, ComponentAccessibilitySupport, ComponentChildPolicy,
    ComponentDescriptor, ComponentExecutionLane, ComponentFocusSupport, ComponentId,
    ComponentPropSchema, ComponentStateOwnership, DeferredCapability, DensityTokenId,
    FrozenAppearanceCapabilities, FrozenCapabilityFamily, FrozenCommandCapabilities,
    FrozenCommandProjectionCapabilities, FrozenCommandProjectionEntry, FrozenComponentCapabilities,
    FrozenDensityCapabilities, FrozenIconCapabilities, FrozenIconEntry,
    FrozenImageAssetCapabilities, FrozenMosaicPlacementCapabilities,
    FrozenMosaicRegionCapabilities, FrozenMosaicSizingCapabilities, FrozenMosaicStateCapabilities,
    FrozenMosaicStateSlotEntry, FrozenNativeCapabilities, FrozenNativeCapabilityEntry,
    FrozenPluginSlotCapabilities, FrozenPluginSlotEntry,
    FrozenRuntimeOutcomeProjectionCapabilities, FrozenRuntimeOutcomeProjectionEntry,
    FrozenSettingCapabilities, FrozenSettingEntry, FrozenSurfaceCapabilities,
    FrozenTaskPresentationCapabilities, FrozenTaskPresentationEntry, FrozenThemeTokenCapabilities,
    FrozenThemeTokenEntry, FrozenViewBindingCapabilities, FrozenViewBindingEntry,
    IconAccessibilityPosture, IconColorSupport, IconDescriptor, IconFamily, IconId, IconKey,
    IconNativeVectorSupport, IconSizeSupport, IconSourceDescriptor, IconSourceKind,
    IconThemePosture, ImageAssetDescriptor, ImageAssetId, ImageAssetSourceKind,
    MeasurementConstraint, MeasurementValue, MosaicChildRule, MosaicClippingPosture,
    MosaicFocusScopeKind, MosaicHitTestPosture, MosaicMeasurementAuthority, MosaicOverflowBehavior,
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
    ViewBindingFamily, ViewBindingId, VisibleStateBindingDeclaration, WorthUiAppearanceFamily,
    WorthUiAppearanceTokenDescriptor, WorthUiAppearanceTokenSource, WorthUiAppearanceValue,
    WorthUiBorderWidthValue, WorthUiCornerRadiusValue, WorthUiDensityFamily,
    WorthUiDensityPostureValue, WorthUiDensityTokenDescriptor, WorthUiDensityValue,
    WorthUiExpressionArity, WorthUiExpressionCostPosture, WorthUiExpressionDependencyContract,
    WorthUiExpressionDiagnosticsPosture, WorthUiExpressionInputKind,
    WorthUiExpressionOperatorDescriptor, WorthUiExpressionOperatorId, WorthUiExpressionOutputKind,
    WorthUiFontSizeValue, WorthUiLengthValue, WorthUiPaddingValue, WorthUiShadowValue,
    WorthUiSpacingValue, WorthUiStyleValueError, WorthUiStyleValueErrorReason, AND_OPERATOR,
    DATA_PAYLOAD_OBJECT_OPERATOR, EMPTY_OPERATOR, EQUALS_OPERATOR, FIELD_OPERATOR,
    LITERAL_TEXT_OPERATOR, NON_EMPTY_OPERATOR, NORMALIZE_TRIM_OPERATOR, NOT_OPERATOR,
    ONE_OF_OPERATOR, OR_OPERATOR, PAYLOAD_OBJECT_OPERATOR, PRESENT_OPERATOR,
};
pub use crate::source::WorthUiArtifactSubtreeDigest;
pub use crate::source::{
    WorthUiContentSlotAssignment, WorthUiContentSlotCatalog, WorthUiContentSlotDiagnostic,
    WorthUiContentSlotDiagnosticCode, WorthUiContentSlotReport, WorthUiLayoutAxis,
    WorthUiLayoutDimension, WorthUiLayoutSizingSpec, WorthUiLayoutSizingValue,
    WorthUiLayoutSlotNode, WorthUiLayoutTopologyCatalog, WorthUiLayoutTopologyChild,
    WorthUiLayoutTopologyDiagnostic, WorthUiLayoutTopologyDiagnosticCode,
    WorthUiLayoutTopologyNode, WorthUiLayoutTopologyReport, WorthUiPageContentSlots,
    WorthUiPageLayoutTopology,
};
pub use app::WorthUiApp;
pub use builder::WorthUiAppBuilder;
pub use entry::WorthUi;
pub use runtime_launch::{
    WorthUiPreparedRuntimeAuthoring, WorthUiRuntimeLaunchBuilder,
    WorthUiRuntimeLaunchPreparationDenial, WorthUiRuntimeSourceModule,
};
