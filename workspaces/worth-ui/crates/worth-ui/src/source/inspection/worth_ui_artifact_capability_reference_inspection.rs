use crate::capability::{
    AdmittedCapability, CommandId, CommandProjectionId, ComponentId, IconId,
    MosaicPlacementPolicyId, MosaicRegionKindId, MosaicSizingContractId, MosaicStateSlotId,
    SurfaceId, ThemeTokenId, ViewBindingId,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum WorthUiArtifactCapabilityReferenceRole {
    PrimaryComponent,
    PrimarySurface,
    PrimaryThemeToken,
    BoundViewBinding,
    StructureRegionKind,
    StructureRegionSizingContract,
    StructureRegionStateSlot,
    StructureMountSurface,
    StructureMountPlacementPolicy,
    StructureMountStateSlot,
    SurfaceIcon,
    SurfaceCommand,
    SurfaceCommandIcon,
    SurfaceCommandProjection,
    SurfaceViewBinding,
    ThemeTokenAliasTarget,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WorthUiArtifactCapabilityReference {
    Component(AdmittedCapability<ComponentId>),
    Surface(AdmittedCapability<SurfaceId>),
    ThemeToken(AdmittedCapability<ThemeTokenId>),
    ViewBinding(AdmittedCapability<ViewBindingId>),
    Icon(AdmittedCapability<IconId>),
    Command(AdmittedCapability<CommandId>),
    CommandProjection(AdmittedCapability<CommandProjectionId>),
    MosaicRegionKind(AdmittedCapability<MosaicRegionKindId>),
    MosaicSizingContract(AdmittedCapability<MosaicSizingContractId>),
    MosaicStateSlot(AdmittedCapability<MosaicStateSlotId>),
    MosaicPlacementPolicy(AdmittedCapability<MosaicPlacementPolicyId>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiArtifactCapabilityReferenceInspection {
    role: WorthUiArtifactCapabilityReferenceRole,
    reference: WorthUiArtifactCapabilityReference,
}

impl WorthUiArtifactCapabilityReferenceInspection {
    pub(crate) fn new(
        role: WorthUiArtifactCapabilityReferenceRole,
        reference: WorthUiArtifactCapabilityReference,
    ) -> Self {
        Self { role, reference }
    }

    pub(crate) fn role(&self) -> WorthUiArtifactCapabilityReferenceRole {
        self.role
    }

    pub(crate) fn reference(&self) -> &WorthUiArtifactCapabilityReference {
        &self.reference
    }
}
