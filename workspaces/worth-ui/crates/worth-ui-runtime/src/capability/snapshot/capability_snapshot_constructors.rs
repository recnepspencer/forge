#[cfg(test)]
use crate::capability::{
    CapabilitySnapshotBuilder, CapabilitySnapshotFreezeInput, CapabilitySupportCatalog,
    FrozenCommandCapabilities, FrozenCommandProjectionCapabilities, FrozenComponentCapabilities,
    FrozenIconCapabilities, FrozenMosaicPlacementCapabilities, FrozenMosaicRegionCapabilities,
    FrozenMosaicSizingCapabilities, FrozenMosaicStateCapabilities, FrozenNativeCapabilities,
    FrozenPluginSlotCapabilities, FrozenRuntimeOutcomeProjectionCapabilities,
    FrozenSettingCapabilities, FrozenSurfaceCapabilities, FrozenTaskPresentationCapabilities,
    FrozenThemeTokenCapabilities, FrozenViewBindingCapabilities, RegisteredCapabilitySet,
};

use super::CapabilitySnapshot;

impl CapabilitySnapshot {
    #[cfg(test)]
    pub(crate) fn from_registered_capabilities(
        registered_capabilities: RegisteredCapabilitySet,
    ) -> Self {
        Self::from_registered_capabilities_and_commands(
            registered_capabilities,
            FrozenCommandCapabilities::empty(),
        )
    }

    #[cfg(test)]
    pub(crate) fn from_registered_capabilities_and_commands(
        registered_capabilities: RegisteredCapabilitySet,
        commands: FrozenCommandCapabilities,
    ) -> Self {
        Self::from_registered_capabilities_commands_and_components(
            registered_capabilities,
            commands,
            FrozenComponentCapabilities::empty(),
        )
    }

    #[cfg(test)]
    pub(crate) fn from_registered_capabilities_commands_and_components(
        registered_capabilities: RegisteredCapabilitySet,
        commands: FrozenCommandCapabilities,
        components: FrozenComponentCapabilities,
    ) -> Self {
        Self::from_registered_capabilities_commands_command_projections_and_components(
            registered_capabilities,
            commands,
            FrozenCommandProjectionCapabilities::empty(),
            components,
        )
    }

    #[cfg(test)]
    pub(crate) fn from_registered_capabilities_commands_command_projections_and_components(
        registered_capabilities: RegisteredCapabilitySet,
        commands: FrozenCommandCapabilities,
        command_projections: FrozenCommandProjectionCapabilities,
        components: FrozenComponentCapabilities,
    ) -> Self {
        Self::from_registered_capabilities_commands_components_icons_and_surfaces(
            registered_capabilities,
            commands,
            command_projections,
            components,
            FrozenIconCapabilities::empty(),
            FrozenSurfaceCapabilities::empty(),
        )
    }

    #[cfg(test)]
    pub(crate) fn from_registered_capabilities_commands_components_icons_and_surfaces(
        registered_capabilities: RegisteredCapabilitySet,
        commands: FrozenCommandCapabilities,
        command_projections: FrozenCommandProjectionCapabilities,
        components: FrozenComponentCapabilities,
        icons: FrozenIconCapabilities,
        surfaces: FrozenSurfaceCapabilities,
    ) -> Self {
        Self::from_registered_capabilities_commands_components_icons_surfaces_and_mosaic_regions(
            registered_capabilities,
            commands,
            command_projections,
            components,
            icons,
            surfaces,
            FrozenMosaicRegionCapabilities::empty(),
        )
    }

    #[cfg(test)]
    pub(crate) fn from_registered_capabilities_commands_components_icons_surfaces_and_mosaic_regions(
        registered_capabilities: RegisteredCapabilitySet,
        commands: FrozenCommandCapabilities,
        command_projections: FrozenCommandProjectionCapabilities,
        components: FrozenComponentCapabilities,
        icons: FrozenIconCapabilities,
        surfaces: FrozenSurfaceCapabilities,
        mosaic_regions: FrozenMosaicRegionCapabilities,
    ) -> Self {
        Self::from_registered_capabilities_commands_components_icons_surfaces_mosaic_regions_and_mosaic_placements(
            registered_capabilities,
            commands,
            command_projections,
            components,
            icons,
            surfaces,
            mosaic_regions,
            FrozenMosaicPlacementCapabilities::empty(),
        )
    }

    #[cfg(test)]
    pub(crate) fn from_registered_capabilities_commands_components_icons_surfaces_mosaic_regions_and_mosaic_placements(
        registered_capabilities: RegisteredCapabilitySet,
        commands: FrozenCommandCapabilities,
        command_projections: FrozenCommandProjectionCapabilities,
        components: FrozenComponentCapabilities,
        icons: FrozenIconCapabilities,
        surfaces: FrozenSurfaceCapabilities,
        mosaic_regions: FrozenMosaicRegionCapabilities,
        mosaic_placement_policies: FrozenMosaicPlacementCapabilities,
    ) -> Self {
        Self::from_registered_capabilities_commands_components_icons_surfaces_mosaic_regions_mosaic_placements_and_mosaic_sizing(
            registered_capabilities,
            commands,
            command_projections,
            components,
            icons,
            surfaces,
            mosaic_regions,
            mosaic_placement_policies,
            FrozenMosaicSizingCapabilities::empty(),
        )
    }

    #[cfg(test)]
    pub(crate) fn from_registered_capabilities_commands_components_icons_surfaces_mosaic_regions_mosaic_placements_and_mosaic_sizing(
        registered_capabilities: RegisteredCapabilitySet,
        commands: FrozenCommandCapabilities,
        command_projections: FrozenCommandProjectionCapabilities,
        components: FrozenComponentCapabilities,
        icons: FrozenIconCapabilities,
        surfaces: FrozenSurfaceCapabilities,
        mosaic_regions: FrozenMosaicRegionCapabilities,
        mosaic_placement_policies: FrozenMosaicPlacementCapabilities,
        mosaic_sizing_contracts: FrozenMosaicSizingCapabilities,
    ) -> Self {
        Self::from_registered_capabilities_commands_components_icons_surfaces_mosaic_regions_mosaic_placements_mosaic_sizing_and_mosaic_state(
            registered_capabilities,
            commands,
            command_projections,
            components,
            icons,
            surfaces,
            mosaic_regions,
            mosaic_placement_policies,
            mosaic_sizing_contracts,
            FrozenMosaicStateCapabilities::empty(),
        )
    }

    #[cfg(test)]
    pub(crate) fn from_registered_capabilities_commands_components_icons_surfaces_mosaic_regions_mosaic_placements_mosaic_sizing_and_mosaic_state(
        registered_capabilities: RegisteredCapabilitySet,
        commands: FrozenCommandCapabilities,
        command_projections: FrozenCommandProjectionCapabilities,
        components: FrozenComponentCapabilities,
        icons: FrozenIconCapabilities,
        surfaces: FrozenSurfaceCapabilities,
        mosaic_regions: FrozenMosaicRegionCapabilities,
        mosaic_placement_policies: FrozenMosaicPlacementCapabilities,
        mosaic_sizing_contracts: FrozenMosaicSizingCapabilities,
        mosaic_state_slots: FrozenMosaicStateCapabilities,
    ) -> Self {
        Self::from_registered_capabilities_commands_components_icons_surfaces_mosaic_regions_mosaic_placements_mosaic_sizing_mosaic_state_and_view_bindings(
            registered_capabilities,
            commands,
            command_projections,
            components,
            icons,
            surfaces,
            mosaic_regions,
            mosaic_placement_policies,
            mosaic_sizing_contracts,
            mosaic_state_slots,
            FrozenViewBindingCapabilities::empty(),
        )
    }

    #[cfg(test)]
    pub(crate) fn from_registered_capabilities_commands_components_icons_surfaces_mosaic_regions_mosaic_placements_mosaic_sizing_mosaic_state_and_view_bindings(
        registered_capabilities: RegisteredCapabilitySet,
        commands: FrozenCommandCapabilities,
        command_projections: FrozenCommandProjectionCapabilities,
        components: FrozenComponentCapabilities,
        icons: FrozenIconCapabilities,
        surfaces: FrozenSurfaceCapabilities,
        mosaic_regions: FrozenMosaicRegionCapabilities,
        mosaic_placement_policies: FrozenMosaicPlacementCapabilities,
        mosaic_sizing_contracts: FrozenMosaicSizingCapabilities,
        mosaic_state_slots: FrozenMosaicStateCapabilities,
        view_bindings: FrozenViewBindingCapabilities,
    ) -> Self {
        Self::from_registered_capabilities_commands_command_projections_components_icons_surfaces_mosaic_regions_mosaic_placements_mosaic_sizing_mosaic_state_native_capabilities_plugin_slots_view_bindings_runtime_outcome_projections_settings_task_presentations_and_theme_tokens(
            registered_capabilities,
            commands,
            command_projections,
            components,
            icons,
            surfaces,
            mosaic_regions,
            mosaic_placement_policies,
            mosaic_sizing_contracts,
            mosaic_state_slots,
            FrozenNativeCapabilities::empty(),
            FrozenPluginSlotCapabilities::empty(),
            view_bindings,
            FrozenRuntimeOutcomeProjectionCapabilities::empty(),
            FrozenSettingCapabilities::empty(),
            FrozenTaskPresentationCapabilities::empty(),
            FrozenThemeTokenCapabilities::empty(),
            CapabilitySupportCatalog::empty(),
        )
    }

    #[cfg(test)]
    pub(crate) fn from_registered_capabilities_commands_command_projections_components_icons_surfaces_mosaic_regions_mosaic_placements_mosaic_sizing_mosaic_state_native_capabilities_plugin_slots_view_bindings_runtime_outcome_projections_settings_task_presentations_and_theme_tokens(
        registered_capabilities: RegisteredCapabilitySet,
        commands: FrozenCommandCapabilities,
        command_projections: FrozenCommandProjectionCapabilities,
        components: FrozenComponentCapabilities,
        icons: FrozenIconCapabilities,
        surfaces: FrozenSurfaceCapabilities,
        mosaic_regions: FrozenMosaicRegionCapabilities,
        mosaic_placement_policies: FrozenMosaicPlacementCapabilities,
        mosaic_sizing_contracts: FrozenMosaicSizingCapabilities,
        mosaic_state_slots: FrozenMosaicStateCapabilities,
        native_capabilities: FrozenNativeCapabilities,
        plugin_slots: FrozenPluginSlotCapabilities,
        view_bindings: FrozenViewBindingCapabilities,
        runtime_outcome_projections: FrozenRuntimeOutcomeProjectionCapabilities,
        settings: FrozenSettingCapabilities,
        task_presentations: FrozenTaskPresentationCapabilities,
        theme_tokens: FrozenThemeTokenCapabilities,
        support_catalog: CapabilitySupportCatalog,
    ) -> Self {
        CapabilitySnapshotBuilder::new(CapabilitySnapshotFreezeInput {
            registered_capabilities,
            commands,
            command_projections,
            components,
            icons,
            surfaces,
            mosaic_regions,
            mosaic_placement_policies,
            mosaic_sizing_contracts,
            mosaic_state_slots,
            native_capabilities,
            plugin_slots,
            view_bindings,
            runtime_outcome_projections,
            settings,
            task_presentations,
            theme_tokens,
            support_catalog,
        })
        .freeze()
    }
}
