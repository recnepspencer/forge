use crate::capability::{
    CapabilitySnapshotBuilder, CapabilitySnapshotFreezeInput, CapabilitySnapshotIndex,
    CapabilitySnapshotIndexParts, CapabilitySupportCatalog, FrozenAppearanceCapabilities,
    FrozenCommandCapabilities, FrozenCommandProjectionCapabilities, FrozenComponentCapabilities,
    FrozenDensityCapabilities, FrozenIconCapabilities, FrozenImageAssetCapabilities,
    FrozenMosaicPlacementCapabilities, FrozenMosaicRegionCapabilities,
    FrozenMosaicSizingCapabilities, FrozenMosaicStateCapabilities, FrozenNativeCapabilities,
    FrozenPluginSlotCapabilities, FrozenRuntimeOutcomeProjectionCapabilities,
    FrozenSettingCapabilities, FrozenSurfaceCapabilities, FrozenTaskPresentationCapabilities,
    FrozenThemeTokenCapabilities, FrozenViewBindingCapabilities, RegisteredCapabilitySet,
    SnapshotFreezeReport, SnapshotReferenceValidationReport,
};

use super::{CapabilitySnapshotDigest, SnapshotMetrics};

/// Immutable capability snapshot consumed by later lowering phases.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilitySnapshot {
    registered_capabilities: RegisteredCapabilitySet,
    commands: FrozenCommandCapabilities,
    command_projections: FrozenCommandProjectionCapabilities,
    components: FrozenComponentCapabilities,
    appearance_tokens: FrozenAppearanceCapabilities,
    density_tokens: FrozenDensityCapabilities,
    icons: FrozenIconCapabilities,
    image_assets: FrozenImageAssetCapabilities,
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
    digest: CapabilitySnapshotDigest,
    metrics: SnapshotMetrics,
    freeze_report: SnapshotFreezeReport,
    reference_validation: SnapshotReferenceValidationReport,
}

impl CapabilitySnapshot {
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
            appearance_tokens: input.appearance_tokens,
            density_tokens: input.density_tokens,
            icons: input.icons,
            image_assets: input.image_assets,
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
            support_catalog: input.support_catalog,
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

    /// Frozen typed appearance token capabilities admitted at registration freeze.
    pub fn appearance_tokens(&self) -> &FrozenAppearanceCapabilities {
        &self.appearance_tokens
    }

    /// Frozen typed density token capabilities admitted at registration freeze.
    pub fn density_tokens(&self) -> &FrozenDensityCapabilities {
        &self.density_tokens
    }

    pub fn icons(&self) -> &FrozenIconCapabilities {
        &self.icons
    }

    pub fn image_assets(&self) -> &FrozenImageAssetCapabilities {
        &self.image_assets
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

    pub(crate) fn support_catalog(&self) -> &CapabilitySupportCatalog {
        &self.support_catalog
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
            appearance_tokens: &self.appearance_tokens,
            density_tokens: &self.density_tokens,
            icons: &self.icons,
            image_assets: &self.image_assets,
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

    pub(crate) fn with_theme_tokens_replaced(
        &self,
        theme_tokens: FrozenThemeTokenCapabilities,
    ) -> Self {
        self.with_replaced_families(
            self.commands.clone(),
            self.command_projections.clone(),
            self.components.clone(),
            self.appearance_tokens.clone(),
            self.density_tokens.clone(),
            theme_tokens,
        )
    }

    pub(crate) fn with_commands_replaced(&self, commands: FrozenCommandCapabilities) -> Self {
        self.with_replaced_families(
            commands,
            self.command_projections.clone(),
            self.components.clone(),
            self.appearance_tokens.clone(),
            self.density_tokens.clone(),
            self.theme_tokens.clone(),
        )
    }

    pub(crate) fn with_command_projections_replaced(
        &self,
        command_projections: FrozenCommandProjectionCapabilities,
    ) -> Self {
        self.with_replaced_families(
            self.commands.clone(),
            command_projections,
            self.components.clone(),
            self.appearance_tokens.clone(),
            self.density_tokens.clone(),
            self.theme_tokens.clone(),
        )
    }

    pub(crate) fn with_components_replaced(&self, components: FrozenComponentCapabilities) -> Self {
        self.with_replaced_families(
            self.commands.clone(),
            self.command_projections.clone(),
            components,
            self.appearance_tokens.clone(),
            self.density_tokens.clone(),
            self.theme_tokens.clone(),
        )
    }

    pub(crate) fn with_appearance_tokens_replaced(
        &self,
        appearance_tokens: FrozenAppearanceCapabilities,
    ) -> Self {
        self.with_replaced_families(
            self.commands.clone(),
            self.command_projections.clone(),
            self.components.clone(),
            appearance_tokens,
            self.density_tokens.clone(),
            self.theme_tokens.clone(),
        )
    }

    pub(crate) fn with_density_tokens_replaced(
        &self,
        density_tokens: FrozenDensityCapabilities,
    ) -> Self {
        self.with_replaced_families(
            self.commands.clone(),
            self.command_projections.clone(),
            self.components.clone(),
            self.appearance_tokens.clone(),
            density_tokens,
            self.theme_tokens.clone(),
        )
    }

    fn with_replaced_families(
        &self,
        commands: FrozenCommandCapabilities,
        command_projections: FrozenCommandProjectionCapabilities,
        components: FrozenComponentCapabilities,
        appearance_tokens: FrozenAppearanceCapabilities,
        density_tokens: FrozenDensityCapabilities,
        theme_tokens: FrozenThemeTokenCapabilities,
    ) -> Self {
        CapabilitySnapshotBuilder::new(CapabilitySnapshotFreezeInput {
            registered_capabilities: self.registered_capabilities.clone(),
            commands,
            command_projections,
            components,
            appearance_tokens,
            density_tokens,
            icons: self.icons.clone(),
            image_assets: self.image_assets.clone(),
            surfaces: self.surfaces.clone(),
            mosaic_regions: self.mosaic_regions.clone(),
            mosaic_placement_policies: self.mosaic_placement_policies.clone(),
            mosaic_sizing_contracts: self.mosaic_sizing_contracts.clone(),
            mosaic_state_slots: self.mosaic_state_slots.clone(),
            native_capabilities: self.native_capabilities.clone(),
            plugin_slots: self.plugin_slots.clone(),
            view_bindings: self.view_bindings.clone(),
            runtime_outcome_projections: self.runtime_outcome_projections.clone(),
            settings: self.settings.clone(),
            task_presentations: self.task_presentations.clone(),
            theme_tokens,
            support_catalog: self.support_catalog.clone(),
        })
        .freeze()
    }
}
