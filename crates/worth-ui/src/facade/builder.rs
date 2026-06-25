use crate::capability::{
    validate_registration_candidates, CapabilityDiagnosticRichness, CapabilityRegistrationReport,
    CapabilitySnapshotBuilder, CapabilitySnapshotFreezeInput, CapabilitySupportCatalog,
    CommandAcceptedRegistrationProof, CommandDescriptor,
    CommandProjectionAcceptedRegistrationProof, CommandProjectionDescriptor,
    CommandProjectionRegistry, CommandRegistry, ComponentAcceptedRegistrationProof,
    ComponentDescriptor, ComponentRegistry, IconAcceptedRegistrationProof, IconDescriptor,
    IconRegistry, ImageAssetAcceptedRegistrationProof, ImageAssetDescriptor, ImageAssetRegistry,
    MosaicPlacementAcceptedRegistrationProof, MosaicPlacementPolicyDescriptor,
    MosaicPlacementRegistry, MosaicRegionAcceptedRegistrationProof, MosaicRegionKindDescriptor,
    MosaicRegionRegistry, MosaicSizingAcceptedRegistrationProof, MosaicSizingContractDescriptor,
    MosaicSizingRegistry, MosaicStateSlotAcceptedRegistrationProof, MosaicStateSlotDescriptor,
    MosaicStateSlotRegistry, NativeCapabilityAcceptedRegistrationProof, NativeCapabilityDescriptor,
    NativeCapabilityRegistry, PluginSlotAcceptedRegistrationProof, PluginSlotDescriptor,
    PluginSlotRegistry, RegistrationCandidate, RegistryFamily,
    RuntimeOutcomeProjectionAcceptedRegistrationProof, RuntimeOutcomeProjectionDescriptor,
    RuntimeOutcomeProjectionRegistry, SettingAcceptedRegistrationProof, SettingDescriptor,
    SettingsRegistry, SurfaceAcceptedRegistrationProof, SurfaceDescriptor, SurfaceRegistry,
    TaskPresentationAcceptedRegistrationProof, TaskPresentationDescriptor,
    TaskPresentationRegistry, ThemeTokenAcceptedRegistrationProof, ThemeTokenDescriptor,
    ThemeTokenRegistry, ViewBindingAcceptedRegistrationProof, ViewBindingDescriptor,
    ViewBindingRegistry, WorthUiAppearanceAcceptedRegistrationProof, WorthUiAppearanceRegistry,
    WorthUiAppearanceTokenDescriptor, WorthUiDensityAcceptedRegistrationProof,
    WorthUiDensityRegistry, WorthUiDensityTokenDescriptor,
};

use super::WorthUiApp;

/// Builder for a Worth UI application definition.
pub struct WorthUiAppBuilder {
    registration_candidates: Vec<RegistrationCandidate>,
    command_registry: CommandRegistry,
    command_projection_registry: CommandProjectionRegistry,
    component_registry: ComponentRegistry,
    appearance_registry: WorthUiAppearanceRegistry,
    density_registry: WorthUiDensityRegistry,
    icon_registry: IconRegistry,
    image_asset_registry: ImageAssetRegistry,
    surface_registry: SurfaceRegistry,
    mosaic_region_registry: MosaicRegionRegistry,
    mosaic_placement_registry: MosaicPlacementRegistry,
    mosaic_sizing_registry: MosaicSizingRegistry,
    mosaic_state_slot_registry: MosaicStateSlotRegistry,
    native_capability_registry: NativeCapabilityRegistry,
    plugin_slot_registry: PluginSlotRegistry,
    view_binding_registry: ViewBindingRegistry,
    runtime_outcome_projection_registry: RuntimeOutcomeProjectionRegistry,
    settings_registry: SettingsRegistry,
    task_presentation_registry: TaskPresentationRegistry,
    theme_token_registry: ThemeTokenRegistry,
    diagnostic_richness: CapabilityDiagnosticRichness,
}

impl WorthUiAppBuilder {
    pub(crate) fn new() -> Self {
        Self {
            registration_candidates: Vec::new(),
            command_registry: CommandRegistry::empty(),
            command_projection_registry: CommandProjectionRegistry::empty(),
            component_registry: ComponentRegistry::empty(),
            appearance_registry: WorthUiAppearanceRegistry::empty(),
            density_registry: WorthUiDensityRegistry::empty(),
            icon_registry: IconRegistry::empty(),
            image_asset_registry: ImageAssetRegistry::empty(),
            surface_registry: SurfaceRegistry::empty(),
            mosaic_region_registry: MosaicRegionRegistry::empty(),
            mosaic_placement_registry: MosaicPlacementRegistry::empty(),
            mosaic_sizing_registry: MosaicSizingRegistry::empty(),
            mosaic_state_slot_registry: MosaicStateSlotRegistry::empty(),
            native_capability_registry: NativeCapabilityRegistry::empty(),
            plugin_slot_registry: PluginSlotRegistry::empty(),
            view_binding_registry: ViewBindingRegistry::empty(),
            runtime_outcome_projection_registry: RuntimeOutcomeProjectionRegistry::empty(),
            settings_registry: SettingsRegistry::empty(),
            task_presentation_registry: TaskPresentationRegistry::empty(),
            theme_token_registry: ThemeTokenRegistry::empty(),
            diagnostic_richness: CapabilityDiagnosticRichness::Rich,
        }
    }

    /// Register a domain-agnostic application command capability.
    pub fn register_command(mut self, descriptor: CommandDescriptor) -> Self {
        self.registration_candidates
            .push(descriptor.registration_candidate());
        self.command_registry.push(descriptor);
        self
    }

    /// Register a domain-agnostic command-spine projection capability.
    pub fn register_command_projection(mut self, descriptor: CommandProjectionDescriptor) -> Self {
        self.registration_candidates
            .push(descriptor.registration_candidate());
        self.command_projection_registry.push(descriptor);
        self
    }

    /// Register a domain-agnostic renderable component capability.
    pub fn register_component(mut self, descriptor: ComponentDescriptor) -> Self {
        self.registration_candidates
            .push(descriptor.registration_candidate());
        self.component_registry.push(descriptor);
        self
    }

    /// Register a typed appearance token capability.
    pub fn register_appearance_token(
        mut self,
        descriptor: WorthUiAppearanceTokenDescriptor,
    ) -> Self {
        self.registration_candidates
            .push(descriptor.registration_candidate());
        self.appearance_registry.push(descriptor);
        self
    }

    /// Register a typed density token capability.
    pub fn register_density_token(mut self, descriptor: WorthUiDensityTokenDescriptor) -> Self {
        self.registration_candidates
            .push(descriptor.registration_candidate());
        self.density_registry.push(descriptor);
        self
    }

    /// Register a domain-agnostic stable icon capability.
    pub fn register_icon(mut self, descriptor: IconDescriptor) -> Self {
        self.registration_candidates
            .push(descriptor.registration_candidate());
        self.icon_registry.push(descriptor);
        self
    }

    /// Register a local/static image asset capability for content anatomy.
    pub fn register_image_asset(mut self, descriptor: ImageAssetDescriptor) -> Self {
        self.registration_candidates
            .push(descriptor.registration_candidate());
        self.image_asset_registry.push(descriptor);
        self
    }

    /// Register a domain-agnostic product-facing shell surface capability.
    pub fn register_surface(mut self, descriptor: SurfaceDescriptor) -> Self {
        self.registration_candidates
            .push(descriptor.registration_candidate());
        self.surface_registry.push(descriptor);
        self
    }

    /// Register a domain-agnostic mosaic-owned structural region kind capability.
    pub fn register_mosaic_region_kind(mut self, descriptor: MosaicRegionKindDescriptor) -> Self {
        self.registration_candidates
            .push(descriptor.registration_candidate());
        self.mosaic_region_registry.push(descriptor);
        self
    }

    /// Register a domain-agnostic runtime-owned mosaic placement policy capability.
    pub fn register_mosaic_placement_policy(
        mut self,
        descriptor: MosaicPlacementPolicyDescriptor,
    ) -> Self {
        self.registration_candidates
            .push(descriptor.registration_candidate());
        self.mosaic_placement_registry.push(descriptor);
        self
    }

    /// Register a domain-agnostic runtime-owned mosaic sizing contract capability.
    pub fn register_mosaic_sizing_contract(
        mut self,
        descriptor: MosaicSizingContractDescriptor,
    ) -> Self {
        self.registration_candidates
            .push(descriptor.registration_candidate());
        self.mosaic_sizing_registry.push(descriptor);
        self
    }

    /// Register a runtime-owned mosaic state slot preservation capability.
    pub fn register_mosaic_state_slot(mut self, descriptor: MosaicStateSlotDescriptor) -> Self {
        self.registration_candidates
            .push(descriptor.registration_candidate());
        self.mosaic_state_slot_registry.push(descriptor);
        self
    }

    /// Register an explicit native platform capability seam.
    pub fn register_native_capability(mut self, descriptor: NativeCapabilityDescriptor) -> Self {
        self.registration_candidates
            .push(descriptor.registration_candidate());
        self.native_capability_registry.push(descriptor);
        self
    }

    /// Register a runtime-owned plugin contribution-slot capability.
    pub fn register_plugin_slot(mut self, descriptor: PluginSlotDescriptor) -> Self {
        self.registration_candidates
            .push(descriptor.registration_candidate());
        self.plugin_slot_registry.push(descriptor);
        self
    }

    /// Register a Query-owned view binding presentation capability.
    pub fn register_view_binding(mut self, descriptor: ViewBindingDescriptor) -> Self {
        self.registration_candidates
            .push(descriptor.registration_candidate());
        self.view_binding_registry.push(descriptor);
        self
    }

    /// Register a runtime-owned outcome presentation projection capability.
    pub fn register_runtime_outcome_projection(
        mut self,
        descriptor: RuntimeOutcomeProjectionDescriptor,
    ) -> Self {
        self.registration_candidates
            .push(descriptor.registration_candidate());
        self.runtime_outcome_projection_registry.push(descriptor);
        self
    }

    /// Register a typed runtime/UI setting metadata capability.
    pub fn register_setting(mut self, descriptor: SettingDescriptor) -> Self {
        self.registration_candidates
            .push(descriptor.registration_candidate());
        self.settings_registry.push(descriptor);
        self
    }

    /// Register domain-agnostic task presentation posture metadata.
    pub fn register_task_presentation(mut self, descriptor: TaskPresentationDescriptor) -> Self {
        self.registration_candidates
            .push(descriptor.registration_candidate());
        self.task_presentation_registry.push(descriptor);
        self
    }

    /// Register a named semantic theme token capability.
    pub fn register_theme_token(mut self, descriptor: ThemeTokenDescriptor) -> Self {
        self.registration_candidates
            .push(descriptor.registration_candidate());
        self.theme_token_registry.push(descriptor);
        self
    }

    /// Freeze registered capabilities into an immutable Worth UI application.
    pub fn freeze(self) -> WorthUiApp {
        WorthUiApp::from_registration_report(self.freeze_with_registration_report())
    }

    /// Freeze registered capabilities and return structured registration diagnostics.
    pub fn freeze_with_registration_report(self) -> CapabilityRegistrationReport {
        let validation_report = validate_registration_candidates(
            &self.registration_candidates,
            self.diagnostic_richness,
        );
        let accepted_commands = CommandAcceptedRegistrationProof::from_identity_texts(
            validation_report.accepted_identity_texts_for_registry_family(RegistryFamily::Command),
        );
        let accepted_command_projections =
            CommandProjectionAcceptedRegistrationProof::from_identity_texts(
                validation_report
                    .accepted_identity_texts_for_registry_family(RegistryFamily::CommandProjection),
            );
        let accepted_components = ComponentAcceptedRegistrationProof::from_identity_texts(
            validation_report
                .accepted_identity_texts_for_registry_family(RegistryFamily::Component),
        );
        let accepted_appearance_tokens =
            WorthUiAppearanceAcceptedRegistrationProof::from_identity_texts(
                validation_report
                    .accepted_identity_texts_for_registry_family(RegistryFamily::AppearanceToken),
            );
        let accepted_density_tokens = WorthUiDensityAcceptedRegistrationProof::from_identity_texts(
            validation_report
                .accepted_identity_texts_for_registry_family(RegistryFamily::DensityToken),
        );
        let accepted_icons = IconAcceptedRegistrationProof::from_identity_texts(
            validation_report.accepted_identity_texts_for_registry_family(RegistryFamily::Icon),
        );
        let accepted_image_assets = ImageAssetAcceptedRegistrationProof::from_identity_texts(
            validation_report
                .accepted_identity_texts_for_registry_family(RegistryFamily::ImageAsset),
        );
        let accepted_surfaces = SurfaceAcceptedRegistrationProof::from_identity_texts(
            validation_report.accepted_identity_texts_for_registry_family(RegistryFamily::Surface),
        );
        let accepted_mosaic_regions = MosaicRegionAcceptedRegistrationProof::from_identity_texts(
            validation_report
                .accepted_identity_texts_for_registry_family(RegistryFamily::MosaicRegionKind),
        );
        let accepted_mosaic_placements =
            MosaicPlacementAcceptedRegistrationProof::from_identity_texts(
                validation_report.accepted_identity_texts_for_registry_family(
                    RegistryFamily::MosaicPlacementPolicy,
                ),
            );
        let accepted_mosaic_sizing = MosaicSizingAcceptedRegistrationProof::from_identity_texts(
            validation_report
                .accepted_identity_texts_for_registry_family(RegistryFamily::MosaicSizingContract),
        );
        let accepted_mosaic_state_slots =
            MosaicStateSlotAcceptedRegistrationProof::from_identity_texts(
                validation_report
                    .accepted_identity_texts_for_registry_family(RegistryFamily::MosaicStateSlot),
            );
        let accepted_plugin_slots = PluginSlotAcceptedRegistrationProof::from_identity_texts(
            validation_report
                .accepted_identity_texts_for_registry_family(RegistryFamily::PluginSlot),
        );
        let accepted_native_capabilities =
            NativeCapabilityAcceptedRegistrationProof::from_identity_texts(
                validation_report
                    .accepted_identity_texts_for_registry_family(RegistryFamily::NativeCapability),
            );
        let accepted_view_bindings = ViewBindingAcceptedRegistrationProof::from_identity_texts(
            validation_report
                .accepted_identity_texts_for_registry_family(RegistryFamily::ViewBinding),
        );
        let accepted_runtime_outcome_projections =
            RuntimeOutcomeProjectionAcceptedRegistrationProof::from_identity_texts(
                validation_report.accepted_identity_texts_for_registry_family(
                    RegistryFamily::RuntimeOutcomeProjection,
                ),
            );
        let accepted_settings = SettingAcceptedRegistrationProof::from_identity_texts(
            validation_report.accepted_identity_texts_for_registry_family(RegistryFamily::Setting),
        );
        let accepted_task_presentations =
            TaskPresentationAcceptedRegistrationProof::from_identity_texts(
                validation_report
                    .accepted_identity_texts_for_registry_family(RegistryFamily::TaskPresentation),
            );
        let accepted_theme_tokens = ThemeTokenAcceptedRegistrationProof::from_identity_texts(
            validation_report
                .accepted_identity_texts_for_registry_family(RegistryFamily::ThemeToken),
        );
        let support_catalog =
            CapabilitySupportCatalog::from_registration_candidates(&self.registration_candidates);
        let (accepted_capabilities, diagnostics) = validation_report.into_parts();
        let command_capabilities = self.command_registry.freeze(&accepted_commands);
        let command_projection_capabilities = self
            .command_projection_registry
            .freeze(&accepted_command_projections);
        let component_capabilities = self.component_registry.freeze(&accepted_components);
        let appearance_capabilities = self.appearance_registry.freeze(&accepted_appearance_tokens);
        let density_capabilities = self.density_registry.freeze(&accepted_density_tokens);
        let icon_capabilities = self.icon_registry.freeze(&accepted_icons);
        let image_asset_capabilities = self.image_asset_registry.freeze(&accepted_image_assets);
        let surface_capabilities = self.surface_registry.freeze(&accepted_surfaces);
        let mosaic_region_capabilities =
            self.mosaic_region_registry.freeze(&accepted_mosaic_regions);
        let mosaic_placement_capabilities = self
            .mosaic_placement_registry
            .freeze(&accepted_mosaic_placements);
        let mosaic_sizing_capabilities =
            self.mosaic_sizing_registry.freeze(&accepted_mosaic_sizing);
        let mosaic_state_capabilities = self
            .mosaic_state_slot_registry
            .freeze(&accepted_mosaic_state_slots);
        let native_capabilities = self
            .native_capability_registry
            .freeze(&accepted_native_capabilities);
        let plugin_slot_capabilities = self.plugin_slot_registry.freeze(&accepted_plugin_slots);
        let view_binding_capabilities = self.view_binding_registry.freeze(&accepted_view_bindings);
        let runtime_outcome_projection_capabilities = self
            .runtime_outcome_projection_registry
            .freeze(&accepted_runtime_outcome_projections);
        let setting_capabilities = self.settings_registry.freeze(&accepted_settings);
        let task_presentation_capabilities = self
            .task_presentation_registry
            .freeze(&accepted_task_presentations);
        let theme_token_capabilities = self.theme_token_registry.freeze(&accepted_theme_tokens);
        CapabilityRegistrationReport::new(
            CapabilitySnapshotBuilder::new(CapabilitySnapshotFreezeInput {
                registered_capabilities: accepted_capabilities,
                commands: command_capabilities,
                command_projections: command_projection_capabilities,
                components: component_capabilities,
                appearance_tokens: appearance_capabilities,
                density_tokens: density_capabilities,
                icons: icon_capabilities,
                image_assets: image_asset_capabilities,
                surfaces: surface_capabilities,
                mosaic_regions: mosaic_region_capabilities,
                mosaic_placement_policies: mosaic_placement_capabilities,
                mosaic_sizing_contracts: mosaic_sizing_capabilities,
                mosaic_state_slots: mosaic_state_capabilities,
                native_capabilities,
                plugin_slots: plugin_slot_capabilities,
                view_bindings: view_binding_capabilities,
                runtime_outcome_projections: runtime_outcome_projection_capabilities,
                settings: setting_capabilities,
                task_presentations: task_presentation_capabilities,
                theme_tokens: theme_token_capabilities,
                support_catalog,
            })
            .freeze(),
            diagnostics,
        )
    }

    /// Materialize only stable diagnostic codes and required report topology.
    pub fn with_minimal_registration_diagnostics(mut self) -> Self {
        self.diagnostic_richness = CapabilityDiagnosticRichness::Minimal;
        self
    }

    /// Materialize richer diagnostic context without changing snapshot meaning.
    pub fn with_rich_registration_diagnostics(mut self) -> Self {
        self.diagnostic_richness = CapabilityDiagnosticRichness::Rich;
        self
    }
}
