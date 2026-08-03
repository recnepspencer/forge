use crate::capability::{
    validate_registration_candidates, CapabilityRegistrationReport, CapabilitySnapshotBuilder,
    CapabilitySnapshotFreezeInput, CapabilitySupportCatalog, CommandAcceptedRegistrationProof,
    CommandProjectionAcceptedRegistrationProof, ComponentAcceptedRegistrationProof,
    IconAcceptedRegistrationProof, IntentDefinitionAcceptedRegistrationProof,
    MosaicPlacementAcceptedRegistrationProof, MosaicRegionAcceptedRegistrationProof,
    MosaicSizingAcceptedRegistrationProof, MosaicStateSlotAcceptedRegistrationProof,
    NativeCapabilityAcceptedRegistrationProof, PluginSlotAcceptedRegistrationProof,
    RegisteredCapabilitySet, RegistrationValidationReport, RegistryFamily,
    RuntimeOutcomeProjectionAcceptedRegistrationProof, SettingAcceptedRegistrationProof,
    SurfaceAcceptedRegistrationProof, TaskPresentationAcceptedRegistrationProof,
    ThemeTokenAcceptedRegistrationProof, ViewBindingAcceptedRegistrationProof,
};

use super::CapabilityRegistrationBuilder;

impl CapabilityRegistrationBuilder {
    /// Freeze registered capabilities and return structured registration diagnostics.
    pub fn freeze_with_registration_report(self) -> CapabilityRegistrationReport {
        let validation = validate_registration_candidates(
            &self.registration_candidates,
            self.diagnostic_richness,
        );
        let accepted = AcceptedRegistryProofs::from_validation(&validation);
        let support_catalog =
            CapabilitySupportCatalog::from_registration_candidates(&self.registration_candidates);
        let (accepted_capabilities, diagnostics) = validation.into_parts();
        let freeze_input = self.freeze_input(&accepted, support_catalog, accepted_capabilities);

        CapabilityRegistrationReport::new(
            CapabilitySnapshotBuilder::new(freeze_input).freeze(),
            diagnostics,
        )
    }

    fn freeze_input(
        self,
        accepted: &AcceptedRegistryProofs,
        support_catalog: CapabilitySupportCatalog,
        registered_capabilities: RegisteredCapabilitySet,
    ) -> CapabilitySnapshotFreezeInput {
        CapabilitySnapshotFreezeInput {
            registered_capabilities,
            commands: self.command_registry.freeze(&accepted.commands),
            command_projections: self
                .command_projection_registry
                .freeze(&accepted.command_projections),
            components: self.component_registry.freeze(&accepted.components),
            icons: self.icon_registry.freeze(&accepted.icons),
            intent_definitions: self
                .intent_definition_registry
                .freeze(&accepted.intent_definitions),
            surfaces: self.surface_registry.freeze(&accepted.surfaces),
            mosaic_regions: self.mosaic_region_registry.freeze(&accepted.mosaic_regions),
            mosaic_placement_policies: self
                .mosaic_placement_registry
                .freeze(&accepted.mosaic_placements),
            mosaic_sizing_contracts: self.mosaic_sizing_registry.freeze(&accepted.mosaic_sizing),
            mosaic_state_slots: self
                .mosaic_state_slot_registry
                .freeze(&accepted.mosaic_state_slots),
            native_capabilities: self
                .native_capability_registry
                .freeze(&accepted.native_capabilities),
            plugin_slots: self.plugin_slot_registry.freeze(&accepted.plugin_slots),
            view_bindings: self.view_binding_registry.freeze(&accepted.view_bindings),
            runtime_outcome_projections: self
                .runtime_outcome_projection_registry
                .freeze(&accepted.runtime_outcome_projections),
            settings: self.settings_registry.freeze(&accepted.settings),
            task_presentations: self
                .task_presentation_registry
                .freeze(&accepted.task_presentations),
            theme_tokens: self.theme_token_registry.freeze(&accepted.theme_tokens),
            support_catalog,
        }
    }
}

struct AcceptedRegistryProofs {
    commands: CommandAcceptedRegistrationProof,
    command_projections: CommandProjectionAcceptedRegistrationProof,
    components: ComponentAcceptedRegistrationProof,
    icons: IconAcceptedRegistrationProof,
    intent_definitions: IntentDefinitionAcceptedRegistrationProof,
    surfaces: SurfaceAcceptedRegistrationProof,
    mosaic_regions: MosaicRegionAcceptedRegistrationProof,
    mosaic_placements: MosaicPlacementAcceptedRegistrationProof,
    mosaic_sizing: MosaicSizingAcceptedRegistrationProof,
    mosaic_state_slots: MosaicStateSlotAcceptedRegistrationProof,
    native_capabilities: NativeCapabilityAcceptedRegistrationProof,
    plugin_slots: PluginSlotAcceptedRegistrationProof,
    view_bindings: ViewBindingAcceptedRegistrationProof,
    runtime_outcome_projections: RuntimeOutcomeProjectionAcceptedRegistrationProof,
    settings: SettingAcceptedRegistrationProof,
    task_presentations: TaskPresentationAcceptedRegistrationProof,
    theme_tokens: ThemeTokenAcceptedRegistrationProof,
}

impl AcceptedRegistryProofs {
    fn from_validation(validation: &RegistrationValidationReport) -> Self {
        Self {
            commands: accepted(validation, RegistryFamily::Command),
            command_projections: accepted(validation, RegistryFamily::CommandProjection),
            components: accepted(validation, RegistryFamily::Component),
            icons: accepted(validation, RegistryFamily::Icon),
            intent_definitions: accepted(validation, RegistryFamily::IntentDefinition),
            surfaces: accepted(validation, RegistryFamily::Surface),
            mosaic_regions: accepted(validation, RegistryFamily::MosaicRegionKind),
            mosaic_placements: accepted(validation, RegistryFamily::MosaicPlacementPolicy),
            mosaic_sizing: accepted(validation, RegistryFamily::MosaicSizingContract),
            mosaic_state_slots: accepted(validation, RegistryFamily::MosaicStateSlot),
            native_capabilities: accepted(validation, RegistryFamily::NativeCapability),
            plugin_slots: accepted(validation, RegistryFamily::PluginSlot),
            view_bindings: accepted(validation, RegistryFamily::ViewBinding),
            runtime_outcome_projections: accepted(
                validation,
                RegistryFamily::RuntimeOutcomeProjection,
            ),
            settings: accepted(validation, RegistryFamily::Setting),
            task_presentations: accepted(validation, RegistryFamily::TaskPresentation),
            theme_tokens: accepted(validation, RegistryFamily::ThemeToken),
        }
    }
}

fn accepted<Proof>(validation: &RegistrationValidationReport, family: RegistryFamily) -> Proof
where
    Proof: FromAcceptedIdentityTexts,
{
    Proof::from_accepted_identity_texts(
        validation.accepted_identity_texts_for_registry_family(family),
    )
}

trait FromAcceptedIdentityTexts: Sized {
    fn from_accepted_identity_texts(identities: std::collections::BTreeSet<String>) -> Self;
}

macro_rules! accepted_identity_proof {
    ($proof:ty) => {
        impl FromAcceptedIdentityTexts for $proof {
            fn from_accepted_identity_texts(
                identities: std::collections::BTreeSet<String>,
            ) -> Self {
                Self::from_identity_texts(identities)
            }
        }
    };
}

accepted_identity_proof!(CommandAcceptedRegistrationProof);
accepted_identity_proof!(CommandProjectionAcceptedRegistrationProof);
accepted_identity_proof!(ComponentAcceptedRegistrationProof);
accepted_identity_proof!(IconAcceptedRegistrationProof);
accepted_identity_proof!(IntentDefinitionAcceptedRegistrationProof);
accepted_identity_proof!(MosaicPlacementAcceptedRegistrationProof);
accepted_identity_proof!(MosaicRegionAcceptedRegistrationProof);
accepted_identity_proof!(MosaicSizingAcceptedRegistrationProof);
accepted_identity_proof!(MosaicStateSlotAcceptedRegistrationProof);
accepted_identity_proof!(NativeCapabilityAcceptedRegistrationProof);
accepted_identity_proof!(PluginSlotAcceptedRegistrationProof);
accepted_identity_proof!(RuntimeOutcomeProjectionAcceptedRegistrationProof);
accepted_identity_proof!(SettingAcceptedRegistrationProof);
accepted_identity_proof!(SurfaceAcceptedRegistrationProof);
accepted_identity_proof!(TaskPresentationAcceptedRegistrationProof);
accepted_identity_proof!(ThemeTokenAcceptedRegistrationProof);
accepted_identity_proof!(ViewBindingAcceptedRegistrationProof);
