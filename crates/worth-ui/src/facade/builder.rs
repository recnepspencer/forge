use crate::capability::{
    validate_registration_candidates, CapabilityDiagnosticRichness, CapabilityRegistrationReport,
    CapabilitySnapshot, CommandAcceptedRegistrationProof, CommandDescriptor, CommandRegistry,
    ComponentAcceptedRegistrationProof, ComponentDescriptor, ComponentRegistry,
    MosaicRegionAcceptedRegistrationProof, MosaicRegionKindDescriptor, MosaicRegionRegistry,
    RegistrationCandidate, SurfaceAcceptedRegistrationProof, SurfaceDescriptor, SurfaceRegistry,
    COMMAND_FAMILY_NAME, COMPONENT_FAMILY_NAME, MOSAIC_REGION_KIND_FAMILY_NAME,
    SURFACE_FAMILY_NAME,
};

use super::WorthUiApp;

/// Builder for a Worth UI application definition.
pub struct WorthUiAppBuilder {
    registration_candidates: Vec<RegistrationCandidate>,
    command_registry: CommandRegistry,
    component_registry: ComponentRegistry,
    surface_registry: SurfaceRegistry,
    mosaic_region_registry: MosaicRegionRegistry,
    diagnostic_richness: CapabilityDiagnosticRichness,
}

impl WorthUiAppBuilder {
    pub(crate) fn new() -> Self {
        Self {
            registration_candidates: Vec::new(),
            command_registry: CommandRegistry::empty(),
            component_registry: ComponentRegistry::empty(),
            surface_registry: SurfaceRegistry::empty(),
            mosaic_region_registry: MosaicRegionRegistry::empty(),
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

    /// Register a domain-agnostic renderable component capability.
    pub fn register_component(mut self, descriptor: ComponentDescriptor) -> Self {
        self.registration_candidates
            .push(descriptor.registration_candidate());
        self.component_registry.push(descriptor);
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
            validation_report.accepted_identity_texts_for_family(COMMAND_FAMILY_NAME),
        );
        let accepted_components = ComponentAcceptedRegistrationProof::from_identity_texts(
            validation_report.accepted_identity_texts_for_family(COMPONENT_FAMILY_NAME),
        );
        let accepted_surfaces = SurfaceAcceptedRegistrationProof::from_identity_texts(
            validation_report.accepted_identity_texts_for_family(SURFACE_FAMILY_NAME),
        );
        let accepted_mosaic_regions = MosaicRegionAcceptedRegistrationProof::from_identity_texts(
            validation_report.accepted_identity_texts_for_family(MOSAIC_REGION_KIND_FAMILY_NAME),
        );
        let (accepted_capabilities, diagnostics) = validation_report.into_parts();
        let command_capabilities = self.command_registry.freeze(&accepted_commands);
        let component_capabilities = self.component_registry.freeze(&accepted_components);
        let surface_capabilities = self.surface_registry.freeze(&accepted_surfaces);
        let mosaic_region_capabilities =
            self.mosaic_region_registry.freeze(&accepted_mosaic_regions);
        CapabilityRegistrationReport::new(
            CapabilitySnapshot::from_registered_capabilities_commands_components_surfaces_and_mosaic_regions(
                accepted_capabilities,
                command_capabilities,
                component_capabilities,
                surface_capabilities,
                mosaic_region_capabilities,
            ),
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
