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
            command_projections: accepted_command_projections(validation),
            components: accepted(validation, RegistryFamily::Component),
            icons: accepted(validation, RegistryFamily::Icon),
            intent_definitions: accepted_intent_definitions(validation),
            surfaces: accepted(validation, RegistryFamily::Surface),
            mosaic_regions: accepted_mosaic_regions(validation),
            mosaic_placements: accepted_mosaic_placements(validation),
            mosaic_sizing: accepted_mosaic_sizing(validation),
            mosaic_state_slots: accepted_mosaic_state_slots(validation),
            native_capabilities: accepted_native_capabilities(validation),
            plugin_slots: accepted_plugin_slots(validation),
            view_bindings: accepted_view_bindings(validation),
            runtime_outcome_projections: accepted_runtime_outcomes(validation),
            settings: accepted(validation, RegistryFamily::Setting),
            task_presentations: accepted_task_presentations(validation),
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
accepted_identity_proof!(ComponentAcceptedRegistrationProof);
accepted_identity_proof!(IconAcceptedRegistrationProof);
accepted_identity_proof!(SettingAcceptedRegistrationProof);
accepted_identity_proof!(SurfaceAcceptedRegistrationProof);
accepted_identity_proof!(ThemeTokenAcceptedRegistrationProof);

macro_rules! accepted_for_family {
    ($function:ident, $proof:ty, $family:expr) => {
        fn $function(validation: &RegistrationValidationReport) -> $proof {
            <$proof>::from_identity_texts(
                validation.accepted_identity_texts_for_registry_family($family),
            )
        }
    };
}

accepted_for_family!(
    accepted_command_projections,
    CommandProjectionAcceptedRegistrationProof,
    RegistryFamily::CommandProjection
);
accepted_for_family!(
    accepted_intent_definitions,
    IntentDefinitionAcceptedRegistrationProof,
    RegistryFamily::IntentDefinition
);
accepted_for_family!(
    accepted_mosaic_regions,
    MosaicRegionAcceptedRegistrationProof,
    RegistryFamily::MosaicRegionKind
);
accepted_for_family!(
    accepted_mosaic_placements,
    MosaicPlacementAcceptedRegistrationProof,
    RegistryFamily::MosaicPlacementPolicy
);
accepted_for_family!(
    accepted_mosaic_sizing,
    MosaicSizingAcceptedRegistrationProof,
    RegistryFamily::MosaicSizingContract
);
accepted_for_family!(
    accepted_mosaic_state_slots,
    MosaicStateSlotAcceptedRegistrationProof,
    RegistryFamily::MosaicStateSlot
);
accepted_for_family!(
    accepted_native_capabilities,
    NativeCapabilityAcceptedRegistrationProof,
    RegistryFamily::NativeCapability
);
accepted_for_family!(
    accepted_plugin_slots,
    PluginSlotAcceptedRegistrationProof,
    RegistryFamily::PluginSlot
);
accepted_for_family!(
    accepted_view_bindings,
    ViewBindingAcceptedRegistrationProof,
    RegistryFamily::ViewBinding
);
accepted_for_family!(
    accepted_runtime_outcomes,
    RuntimeOutcomeProjectionAcceptedRegistrationProof,
    RegistryFamily::RuntimeOutcomeProjection
);
accepted_for_family!(
    accepted_task_presentations,
    TaskPresentationAcceptedRegistrationProof,
    RegistryFamily::TaskPresentation
);
