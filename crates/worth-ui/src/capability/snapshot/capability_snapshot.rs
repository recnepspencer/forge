use crate::capability::{
    CapabilitySnapshotBuilder, CapabilitySnapshotFreezeInput, CapabilitySnapshotIndex,
    CapabilitySnapshotIndexParts, FrozenCommandCapabilities, FrozenCommandProjectionCapabilities,
    FrozenComponentCapabilities, FrozenIconCapabilities, FrozenMosaicPlacementCapabilities,
    FrozenMosaicRegionCapabilities, FrozenMosaicSizingCapabilities, FrozenMosaicStateCapabilities,
    FrozenNativeCapabilities, FrozenPluginSlotCapabilities,
    FrozenRuntimeOutcomeProjectionCapabilities, FrozenSettingCapabilities,
    FrozenSurfaceCapabilities, FrozenTaskPresentationCapabilities, FrozenThemeTokenCapabilities,
    FrozenViewBindingCapabilities, RegisteredCapabilitySet, SnapshotFreezeReport,
    SnapshotReferenceValidationReport,
};

use super::{CapabilitySnapshotDigest, SnapshotMetrics};

/// Immutable capability snapshot consumed by later lowering phases.
#[derive(Debug, Eq, PartialEq)]
pub struct CapabilitySnapshot {
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
    digest: CapabilitySnapshotDigest,
    metrics: SnapshotMetrics,
    freeze_report: SnapshotFreezeReport,
    reference_validation: SnapshotReferenceValidationReport,
}

impl CapabilitySnapshot {
    #[allow(dead_code)]
    pub(crate) fn from_registered_capabilities(
        registered_capabilities: RegisteredCapabilitySet,
    ) -> Self {
        Self::from_registered_capabilities_and_commands(
            registered_capabilities,
            FrozenCommandCapabilities::empty(),
        )
    }

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
        )
    }

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
        })
        .freeze()
    }

    pub(crate) fn from_freeze_parts(
        input: CapabilitySnapshotFreezeInput,
        digest: CapabilitySnapshotDigest,
        metrics: SnapshotMetrics,
        freeze_report: SnapshotFreezeReport,
        reference_validation: SnapshotReferenceValidationReport,
    ) -> Self {
        Self {
            registered_capabilities: input.registered_capabilities,
            commands: input.commands,
            command_projections: input.command_projections,
            components: input.components,
            icons: input.icons,
            surfaces: input.surfaces,
            mosaic_regions: input.mosaic_regions,
            mosaic_placement_policies: input.mosaic_placement_policies,
            mosaic_sizing_contracts: input.mosaic_sizing_contracts,
            mosaic_state_slots: input.mosaic_state_slots,
            native_capabilities: input.native_capabilities,
            plugin_slots: input.plugin_slots,
            view_bindings: input.view_bindings,
            runtime_outcome_projections: input.runtime_outcome_projections,
            settings: input.settings,
            task_presentations: input.task_presentations,
            theme_tokens: input.theme_tokens,
            digest,
            metrics,
            freeze_report,
            reference_validation,
        }
    }

    pub fn registered_capabilities(&self) -> &RegisteredCapabilitySet {
        &self.registered_capabilities
    }

    pub fn commands(&self) -> &FrozenCommandCapabilities {
        &self.commands
    }

    pub fn command_projections(&self) -> &FrozenCommandProjectionCapabilities {
        &self.command_projections
    }

    pub fn components(&self) -> &FrozenComponentCapabilities {
        &self.components
    }

    pub fn icons(&self) -> &FrozenIconCapabilities {
        &self.icons
    }

    pub fn surfaces(&self) -> &FrozenSurfaceCapabilities {
        &self.surfaces
    }

    /// Frozen mosaic region kind capabilities admitted at registration freeze.
    pub fn mosaic_regions(&self) -> &FrozenMosaicRegionCapabilities {
        &self.mosaic_regions
    }

    /// Frozen mosaic placement policy capabilities admitted at registration freeze.
    pub fn mosaic_placement_policies(&self) -> &FrozenMosaicPlacementCapabilities {
        &self.mosaic_placement_policies
    }

    /// Frozen mosaic sizing contract capabilities admitted at registration freeze.
    pub fn mosaic_sizing_contracts(&self) -> &FrozenMosaicSizingCapabilities {
        &self.mosaic_sizing_contracts
    }

    /// Frozen mosaic state slot capabilities admitted at registration freeze.
    pub fn mosaic_state_slots(&self) -> &FrozenMosaicStateCapabilities {
        &self.mosaic_state_slots
    }

    /// Frozen native platform capabilities admitted at registration freeze.
    pub fn native_capabilities(&self) -> &FrozenNativeCapabilities {
        &self.native_capabilities
    }

    /// Frozen plugin contribution-slot capabilities admitted at registration freeze.
    pub fn plugin_slots(&self) -> &FrozenPluginSlotCapabilities {
        &self.plugin_slots
    }

    /// Frozen Query-owned view binding capabilities admitted at registration freeze.
    pub fn view_bindings(&self) -> &FrozenViewBindingCapabilities {
        &self.view_bindings
    }

    /// Frozen runtime outcome projection capabilities admitted at registration freeze.
    pub fn runtime_outcome_projections(&self) -> &FrozenRuntimeOutcomeProjectionCapabilities {
        &self.runtime_outcome_projections
    }

    /// Frozen typed settings capabilities admitted at registration freeze.
    pub fn settings(&self) -> &FrozenSettingCapabilities {
        &self.settings
    }

    /// Frozen task presentation posture capabilities admitted at registration freeze.
    pub fn task_presentations(&self) -> &FrozenTaskPresentationCapabilities {
        &self.task_presentations
    }

    /// Frozen semantic theme token capabilities admitted at registration freeze.
    pub fn theme_tokens(&self) -> &FrozenThemeTokenCapabilities {
        &self.theme_tokens
    }

    /// Deterministic digest for this frozen capability meaning.
    pub fn digest(&self) -> CapabilitySnapshotDigest {
        self.digest
    }

    /// Structural counters computed at freeze.
    pub fn metrics(&self) -> SnapshotMetrics {
        self.metrics
    }

    /// Immutable typed lookup indexes for this frozen snapshot.
    pub fn index(&self) -> CapabilitySnapshotIndex<'_> {
        CapabilitySnapshotIndex::new(CapabilitySnapshotIndexParts {
            commands: &self.commands,
            command_projections: &self.command_projections,
            components: &self.components,
            icons: &self.icons,
            surfaces: &self.surfaces,
            mosaic_regions: &self.mosaic_regions,
            mosaic_placement_policies: &self.mosaic_placement_policies,
            mosaic_sizing_contracts: &self.mosaic_sizing_contracts,
            mosaic_state_slots: &self.mosaic_state_slots,
            native_capabilities: &self.native_capabilities,
            plugin_slots: &self.plugin_slots,
            view_bindings: &self.view_bindings,
            runtime_outcome_projections: &self.runtime_outcome_projections,
            settings: &self.settings,
            task_presentations: &self.task_presentations,
            theme_tokens: &self.theme_tokens,
        })
    }

    /// Family widths and digest bases recorded at snapshot freeze.
    pub fn freeze_report(&self) -> &SnapshotFreezeReport {
        &self.freeze_report
    }

    /// Canonical reference validation summary for future lowering.
    pub fn validation_summary(&self) -> &SnapshotReferenceValidationReport {
        &self.reference_validation
    }
}
