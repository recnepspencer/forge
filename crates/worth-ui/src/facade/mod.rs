//! Public Worth UI facade.

mod app;
mod builder;
mod entry;

pub use crate::capability::{
    AdmittedCapability, CapabilityDiagnosticCode, CapabilityDiagnosticRichness,
    CapabilityDiagnosticSeverity, CapabilityIdError, CapabilityRegistrationDiagnostic,
    CapabilityRegistrationReport, CapabilitySnapshot, CapabilitySnapshotDigest,
    CapabilitySupportId, CapabilitySupportKind, CapabilitySupportPosture,
    CapabilitySupportRejection, CommandCategory, CommandDescriptor, CommandId, CommandProjectionId,
    CommandReadinessBinding, CommandRuntimeIntentBinding, ComponentAccessibilitySupport,
    ComponentChildPolicy, ComponentDescriptor, ComponentExecutionLane, ComponentFocusSupport,
    ComponentId, ComponentPropSchema, ComponentStateOwnership, DeferredCapability,
    FrozenCommandCapabilities, FrozenComponentCapabilities, FrozenMosaicRegionCapabilities,
    FrozenSurfaceCapabilities, IconId, MosaicChildRule, MosaicClippingPosture,
    MosaicFocusScopeKind, MosaicHitTestPosture, MosaicPlacementPolicyId,
    MosaicRegionKindDescriptor, MosaicRegionKindId, MosaicRegionPersistence, MosaicRegionRole,
    MosaicScrollOwnership, MosaicSizingBehavior, MosaicSizingContractId, MosaicStateSlotId,
    NativeCapabilityId, PlatformInternalCapability, PluginSlotId, RegisteredCapabilitySet,
    RuntimeOutcomeProjectionId, SettingId, SnapshotMetrics, SupportRequirement, SupportSnapshot,
    SurfaceDescriptor, SurfaceId, SurfaceKind, SurfacePlacementClass, SurfaceStateClass,
    TaskPresentationId, ThemeTokenId, UnsupportedCapability, ViewBindingId,
};
pub use app::WorthUiApp;
pub use builder::WorthUiAppBuilder;
pub use entry::WorthUi;
