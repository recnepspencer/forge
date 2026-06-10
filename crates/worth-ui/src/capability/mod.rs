mod diagnostics;
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
pub use identity::{
    CapabilityIdError, CommandId, CommandProjectionId, ComponentId, IconId,
    MosaicPlacementPolicyId, MosaicRegionKindId, MosaicSizingContractId, MosaicStateSlotId,
    NativeCapabilityId, PluginSlotId, RuntimeOutcomeProjectionId, SettingId, SurfaceId,
    TaskPresentationId, ThemeTokenId, ViewBindingId,
};
pub use registered_set::RegisteredCapabilitySet;
pub(crate) use registration::{
    validate_registration_candidates, RegistrationCandidate, RegistrationCandidateDiagnostic,
    RegistrationDependency,
};
pub(crate) use registry::{
    CommandAcceptedRegistrationProof, CommandRegistry, ComponentAcceptedRegistrationProof,
    ComponentRegistry, MosaicRegionAcceptedRegistrationProof, MosaicRegionRegistry,
    SurfaceAcceptedRegistrationProof, SurfaceRegistry, COMMAND_FAMILY_NAME,
    COMMAND_PROJECTION_FAMILY_NAME, COMPONENT_FAMILY_NAME, MOSAIC_REGION_KIND_FAMILY_NAME,
    SURFACE_FAMILY_NAME, THEME_TOKEN_FAMILY_NAME, VIEW_BINDING_FAMILY_NAME,
};
pub use registry::{
    CommandCategory, CommandDescriptor, CommandReadinessBinding, CommandRuntimeIntentBinding,
    ComponentAccessibilitySupport, ComponentChildPolicy, ComponentDescriptor,
    ComponentExecutionLane, ComponentFocusSupport, ComponentPropSchema, ComponentStateOwnership,
    FrozenCommandCapabilities, FrozenComponentCapabilities, FrozenMosaicRegionCapabilities,
    FrozenSurfaceCapabilities, MosaicChildRule, MosaicClippingPosture, MosaicFocusScopeKind,
    MosaicHitTestPosture, MosaicRegionKindDescriptor, MosaicRegionPersistence, MosaicRegionRole,
    MosaicScrollOwnership, MosaicSizingBehavior, SurfaceDescriptor, SurfaceKind,
    SurfacePlacementClass, SurfaceStateClass,
};
pub use snapshot::{
    CapabilitySnapshot, CapabilitySnapshotDigest, SnapshotMetrics, SupportSnapshot,
};
pub use support::{
    AdmittedCapability, CapabilitySupportId, CapabilitySupportKind, CapabilitySupportPosture,
    CapabilitySupportRejection, DeferredCapability, PlatformInternalCapability, SupportRequirement,
    UnsupportedCapability,
};
