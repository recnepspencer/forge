use crate::capability::{
    AppearanceTokenId, CommandDescriptor, CommandId, CommandProjectionDescriptor,
    CommandProjectionId, ComponentDescriptor, ComponentId, DensityTokenId,
    FrozenAppearanceCapabilities, FrozenCommandCapabilities, FrozenCommandProjectionCapabilities,
    FrozenComponentCapabilities, FrozenDensityCapabilities, FrozenIconCapabilities,
    FrozenMosaicPlacementCapabilities, FrozenMosaicRegionCapabilities,
    FrozenMosaicSizingCapabilities, FrozenMosaicStateCapabilities, FrozenNativeCapabilities,
    FrozenPluginSlotCapabilities, FrozenRuntimeOutcomeProjectionCapabilities,
    FrozenSettingCapabilities, FrozenSurfaceCapabilities, FrozenTaskPresentationCapabilities,
    FrozenThemeTokenCapabilities, FrozenViewBindingCapabilities, IconDescriptor, IconId,
    MosaicPlacementPolicyDescriptor, MosaicPlacementPolicyId, MosaicRegionKindDescriptor,
    MosaicRegionKindId, MosaicSizingContractDescriptor, MosaicSizingContractId,
    MosaicStateSlotDescriptor, MosaicStateSlotId, NativeCapabilityDescriptor, NativeCapabilityId,
    PluginSlotDescriptor, PluginSlotId, RuntimeOutcomeProjectionDescriptor,
    RuntimeOutcomeProjectionId, SettingDescriptor, SettingId, SurfaceDescriptor, SurfaceId,
    TaskPresentationDescriptor, TaskPresentationId, ThemeTokenDescriptor, ThemeTokenId,
    ViewBindingDescriptor, ViewBindingId, WorthUiAppearanceTokenDescriptor,
    WorthUiDensityTokenDescriptor, APPEARANCE_TOKEN_FAMILY_NAME, COMMAND_FAMILY_NAME,
    COMMAND_PROJECTION_FAMILY_NAME, COMPONENT_FAMILY_NAME, DENSITY_TOKEN_FAMILY_NAME,
    ICON_FAMILY_NAME, MOSAIC_PLACEMENT_POLICY_FAMILY_NAME, MOSAIC_REGION_KIND_FAMILY_NAME,
    MOSAIC_SIZING_CONTRACT_FAMILY_NAME, MOSAIC_STATE_SLOT_FAMILY_NAME,
    NATIVE_CAPABILITY_FAMILY_NAME, PLUGIN_SLOT_FAMILY_NAME, RUNTIME_OUTCOME_PROJECTION_FAMILY_NAME,
    SETTING_FAMILY_NAME, SURFACE_FAMILY_NAME, TASK_PRESENTATION_FAMILY_NAME,
    THEME_TOKEN_FAMILY_NAME, VIEW_BINDING_FAMILY_NAME,
};

use super::{SnapshotFamilyIndex, SnapshotLookupReport};

/// Immutable typed lookup surface for a frozen capability snapshot.
#[derive(Clone, Copy)]
pub struct CapabilitySnapshotIndex<'snapshot> {
    commands: &'snapshot FrozenCommandCapabilities,
    command_projections: &'snapshot FrozenCommandProjectionCapabilities,
    components: &'snapshot FrozenComponentCapabilities,
    appearance_tokens: &'snapshot FrozenAppearanceCapabilities,
    density_tokens: &'snapshot FrozenDensityCapabilities,
    icons: &'snapshot FrozenIconCapabilities,
    surfaces: &'snapshot FrozenSurfaceCapabilities,
    mosaic_regions: &'snapshot FrozenMosaicRegionCapabilities,
    mosaic_placement_policies: &'snapshot FrozenMosaicPlacementCapabilities,
    mosaic_sizing_contracts: &'snapshot FrozenMosaicSizingCapabilities,
    mosaic_state_slots: &'snapshot FrozenMosaicStateCapabilities,
    native_capabilities: &'snapshot FrozenNativeCapabilities,
    plugin_slots: &'snapshot FrozenPluginSlotCapabilities,
    view_bindings: &'snapshot FrozenViewBindingCapabilities,
    runtime_outcome_projections: &'snapshot FrozenRuntimeOutcomeProjectionCapabilities,
    settings: &'snapshot FrozenSettingCapabilities,
    task_presentations: &'snapshot FrozenTaskPresentationCapabilities,
    theme_tokens: &'snapshot FrozenThemeTokenCapabilities,
}

impl<'snapshot> CapabilitySnapshotIndex<'snapshot> {
    pub(crate) fn new(snapshot_parts: CapabilitySnapshotIndexParts<'snapshot>) -> Self {
        Self {
            commands: snapshot_parts.commands,
            command_projections: snapshot_parts.command_projections,
            components: snapshot_parts.components,
            appearance_tokens: snapshot_parts.appearance_tokens,
            density_tokens: snapshot_parts.density_tokens,
            icons: snapshot_parts.icons,
            surfaces: snapshot_parts.surfaces,
            mosaic_regions: snapshot_parts.mosaic_regions,
            mosaic_placement_policies: snapshot_parts.mosaic_placement_policies,
            mosaic_sizing_contracts: snapshot_parts.mosaic_sizing_contracts,
            mosaic_state_slots: snapshot_parts.mosaic_state_slots,
            native_capabilities: snapshot_parts.native_capabilities,
            plugin_slots: snapshot_parts.plugin_slots,
            view_bindings: snapshot_parts.view_bindings,
            runtime_outcome_projections: snapshot_parts.runtime_outcome_projections,
            settings: snapshot_parts.settings,
            task_presentations: snapshot_parts.task_presentations,
            theme_tokens: snapshot_parts.theme_tokens,
        }
    }

    pub fn commands(self) -> CommandSnapshotIndex<'snapshot> {
        CommandSnapshotIndex::new(self.commands)
    }

    pub fn command_projections(self) -> CommandProjectionSnapshotIndex<'snapshot> {
        CommandProjectionSnapshotIndex::new(self.command_projections)
    }

    pub fn components(self) -> ComponentSnapshotIndex<'snapshot> {
        ComponentSnapshotIndex::new(self.components)
    }

    pub fn appearance_tokens(self) -> AppearanceTokenSnapshotIndex<'snapshot> {
        AppearanceTokenSnapshotIndex::new(self.appearance_tokens)
    }

    pub fn density_tokens(self) -> DensityTokenSnapshotIndex<'snapshot> {
        DensityTokenSnapshotIndex::new(self.density_tokens)
    }

    pub fn icons(self) -> IconSnapshotIndex<'snapshot> {
        IconSnapshotIndex::new(self.icons)
    }

    pub fn surfaces(self) -> SurfaceSnapshotIndex<'snapshot> {
        SurfaceSnapshotIndex::new(self.surfaces)
    }

    pub fn mosaic_regions(self) -> MosaicRegionSnapshotIndex<'snapshot> {
        MosaicRegionSnapshotIndex::new(self.mosaic_regions)
    }

    pub fn mosaic_placement_policies(self) -> MosaicPlacementSnapshotIndex<'snapshot> {
        MosaicPlacementSnapshotIndex::new(self.mosaic_placement_policies)
    }

    pub fn mosaic_sizing_contracts(self) -> MosaicSizingSnapshotIndex<'snapshot> {
        MosaicSizingSnapshotIndex::new(self.mosaic_sizing_contracts)
    }

    pub fn mosaic_state_slots(self) -> MosaicStateSnapshotIndex<'snapshot> {
        MosaicStateSnapshotIndex::new(self.mosaic_state_slots)
    }

    pub fn native_capabilities(self) -> NativeCapabilitySnapshotIndex<'snapshot> {
        NativeCapabilitySnapshotIndex::new(self.native_capabilities)
    }

    pub fn plugin_slots(self) -> PluginSlotSnapshotIndex<'snapshot> {
        PluginSlotSnapshotIndex::new(self.plugin_slots)
    }

    pub fn view_bindings(self) -> ViewBindingSnapshotIndex<'snapshot> {
        ViewBindingSnapshotIndex::new(self.view_bindings)
    }

    pub fn runtime_outcome_projections(self) -> RuntimeOutcomeProjectionSnapshotIndex<'snapshot> {
        RuntimeOutcomeProjectionSnapshotIndex::new(self.runtime_outcome_projections)
    }

    pub fn settings(self) -> SettingSnapshotIndex<'snapshot> {
        SettingSnapshotIndex::new(self.settings)
    }

    pub fn task_presentations(self) -> TaskPresentationSnapshotIndex<'snapshot> {
        TaskPresentationSnapshotIndex::new(self.task_presentations)
    }

    pub fn theme_tokens(self) -> ThemeTokenSnapshotIndex<'snapshot> {
        ThemeTokenSnapshotIndex::new(self.theme_tokens)
    }
}

pub(crate) struct CapabilitySnapshotIndexParts<'snapshot> {
    pub(crate) commands: &'snapshot FrozenCommandCapabilities,
    pub(crate) command_projections: &'snapshot FrozenCommandProjectionCapabilities,
    pub(crate) components: &'snapshot FrozenComponentCapabilities,
    pub(crate) appearance_tokens: &'snapshot FrozenAppearanceCapabilities,
    pub(crate) density_tokens: &'snapshot FrozenDensityCapabilities,
    pub(crate) icons: &'snapshot FrozenIconCapabilities,
    pub(crate) surfaces: &'snapshot FrozenSurfaceCapabilities,
    pub(crate) mosaic_regions: &'snapshot FrozenMosaicRegionCapabilities,
    pub(crate) mosaic_placement_policies: &'snapshot FrozenMosaicPlacementCapabilities,
    pub(crate) mosaic_sizing_contracts: &'snapshot FrozenMosaicSizingCapabilities,
    pub(crate) mosaic_state_slots: &'snapshot FrozenMosaicStateCapabilities,
    pub(crate) native_capabilities: &'snapshot FrozenNativeCapabilities,
    pub(crate) plugin_slots: &'snapshot FrozenPluginSlotCapabilities,
    pub(crate) view_bindings: &'snapshot FrozenViewBindingCapabilities,
    pub(crate) runtime_outcome_projections: &'snapshot FrozenRuntimeOutcomeProjectionCapabilities,
    pub(crate) settings: &'snapshot FrozenSettingCapabilities,
    pub(crate) task_presentations: &'snapshot FrozenTaskPresentationCapabilities,
    pub(crate) theme_tokens: &'snapshot FrozenThemeTokenCapabilities,
}

macro_rules! snapshot_index_family {
    ($index_name:ident, $family_name:ident, $frozen:ty, $id:ty, $descriptor:ty) => {
        #[derive(Clone, Copy)]
        pub struct $index_name<'snapshot> {
            capabilities: &'snapshot $frozen,
            family: SnapshotFamilyIndex,
        }

        impl<'snapshot> $index_name<'snapshot> {
            fn new(capabilities: &'snapshot $frozen) -> Self {
                Self {
                    capabilities,
                    family: SnapshotFamilyIndex::new($family_name, capabilities.len()),
                }
            }

            pub fn lookup(self, id: &$id) -> SnapshotLookupReport<&'snapshot $descriptor> {
                self.family.lookup(self.capabilities.get(id))
            }
        }
    };
}

snapshot_index_family!(
    CommandSnapshotIndex,
    COMMAND_FAMILY_NAME,
    FrozenCommandCapabilities,
    CommandId,
    CommandDescriptor
);
snapshot_index_family!(
    CommandProjectionSnapshotIndex,
    COMMAND_PROJECTION_FAMILY_NAME,
    FrozenCommandProjectionCapabilities,
    CommandProjectionId,
    CommandProjectionDescriptor
);
snapshot_index_family!(
    ComponentSnapshotIndex,
    COMPONENT_FAMILY_NAME,
    FrozenComponentCapabilities,
    ComponentId,
    ComponentDescriptor
);
snapshot_index_family!(
    AppearanceTokenSnapshotIndex,
    APPEARANCE_TOKEN_FAMILY_NAME,
    FrozenAppearanceCapabilities,
    AppearanceTokenId,
    WorthUiAppearanceTokenDescriptor
);
snapshot_index_family!(
    DensityTokenSnapshotIndex,
    DENSITY_TOKEN_FAMILY_NAME,
    FrozenDensityCapabilities,
    DensityTokenId,
    WorthUiDensityTokenDescriptor
);
snapshot_index_family!(
    IconSnapshotIndex,
    ICON_FAMILY_NAME,
    FrozenIconCapabilities,
    IconId,
    IconDescriptor
);
snapshot_index_family!(
    SurfaceSnapshotIndex,
    SURFACE_FAMILY_NAME,
    FrozenSurfaceCapabilities,
    SurfaceId,
    SurfaceDescriptor
);
snapshot_index_family!(
    MosaicRegionSnapshotIndex,
    MOSAIC_REGION_KIND_FAMILY_NAME,
    FrozenMosaicRegionCapabilities,
    MosaicRegionKindId,
    MosaicRegionKindDescriptor
);
snapshot_index_family!(
    MosaicPlacementSnapshotIndex,
    MOSAIC_PLACEMENT_POLICY_FAMILY_NAME,
    FrozenMosaicPlacementCapabilities,
    MosaicPlacementPolicyId,
    MosaicPlacementPolicyDescriptor
);
snapshot_index_family!(
    MosaicSizingSnapshotIndex,
    MOSAIC_SIZING_CONTRACT_FAMILY_NAME,
    FrozenMosaicSizingCapabilities,
    MosaicSizingContractId,
    MosaicSizingContractDescriptor
);
snapshot_index_family!(
    MosaicStateSnapshotIndex,
    MOSAIC_STATE_SLOT_FAMILY_NAME,
    FrozenMosaicStateCapabilities,
    MosaicStateSlotId,
    MosaicStateSlotDescriptor
);
snapshot_index_family!(
    NativeCapabilitySnapshotIndex,
    NATIVE_CAPABILITY_FAMILY_NAME,
    FrozenNativeCapabilities,
    NativeCapabilityId,
    NativeCapabilityDescriptor
);
snapshot_index_family!(
    PluginSlotSnapshotIndex,
    PLUGIN_SLOT_FAMILY_NAME,
    FrozenPluginSlotCapabilities,
    PluginSlotId,
    PluginSlotDescriptor
);
snapshot_index_family!(
    ViewBindingSnapshotIndex,
    VIEW_BINDING_FAMILY_NAME,
    FrozenViewBindingCapabilities,
    ViewBindingId,
    ViewBindingDescriptor
);
snapshot_index_family!(
    RuntimeOutcomeProjectionSnapshotIndex,
    RUNTIME_OUTCOME_PROJECTION_FAMILY_NAME,
    FrozenRuntimeOutcomeProjectionCapabilities,
    RuntimeOutcomeProjectionId,
    RuntimeOutcomeProjectionDescriptor
);
snapshot_index_family!(
    SettingSnapshotIndex,
    SETTING_FAMILY_NAME,
    FrozenSettingCapabilities,
    SettingId,
    SettingDescriptor
);
snapshot_index_family!(
    TaskPresentationSnapshotIndex,
    TASK_PRESENTATION_FAMILY_NAME,
    FrozenTaskPresentationCapabilities,
    TaskPresentationId,
    TaskPresentationDescriptor
);
snapshot_index_family!(
    ThemeTokenSnapshotIndex,
    THEME_TOKEN_FAMILY_NAME,
    FrozenThemeTokenCapabilities,
    ThemeTokenId,
    ThemeTokenDescriptor
);
