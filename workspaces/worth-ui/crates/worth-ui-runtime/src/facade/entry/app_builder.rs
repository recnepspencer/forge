use crate::facade::entry::CapabilityRegistrationBuilder;
use crate::facade::host_observation::{WorthUiHostAdapter, WorthUiHostContract};
use crate::facade::inspection_bridge::UiMeasurementInspectionEvidenceBundle;
use crate::facade::lifecycle::WorthUiCapabilityRegistrationFreezeCore;
use crate::facade::registry::{
    CapabilityRegistrationReport, CommandDescriptor, CommandProjectionDescriptor,
    ComponentDescriptor, IconDescriptor, MosaicPlacementPolicyDescriptor,
    MosaicRegionKindDescriptor, MosaicSizingContractDescriptor, MosaicStateSlotDescriptor,
    NativeCapabilityDescriptor, PluginSlotDescriptor, RuntimeOutcomeProjectionDescriptor,
    SettingDescriptor, SurfaceDescriptor, TaskPresentationDescriptor, ThemeTokenDescriptor,
    WorthUiQueryViewRegistration,
};
use crate::facade::{WorthUiApp, WorthUiDslPackage};
use crate::graph::UiGraphWorldProfile;
use crate::runtime::{WorthUiSourceBackedDeclarationWitness, WorthUiSourceBackedDslPackage};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiQueryViewRegistrationError {
    Binding(worth_ui_query_binding::WorthUiQueryBindingRegistrationDenial),
    InvalidIdentity(crate::capability::CapabilityIdError),
}

/// Builder for a Worth UI application definition.
pub struct WorthUiBuilder {
    inner: CapabilityRegistrationBuilder,
    dsl_package: WorthUiDslPackage,
    host_contract: WorthUiHostContract,
    graph_world_profile: UiGraphWorldProfile,
    measurement_inspection_evidence: Vec<UiMeasurementInspectionEvidenceBundle>,
    source_backed_declaration_witness: WorthUiSourceBackedDeclarationWitnessSlot,
    query_binding_plan: worth_ui_query_binding::WorthUiQueryBindingPlan,
}

pub type WorthUiAppBuilder = WorthUiBuilder;

impl WorthUiBuilder {
    pub(crate) fn new() -> Self {
        Self {
            inner: CapabilityRegistrationBuilder::new(),
            dsl_package: WorthUiDslPackage::empty(),
            host_contract: WorthUiHostContract::headless(),
            graph_world_profile: UiGraphWorldProfile::authoritative(),
            measurement_inspection_evidence: Vec::new(),
            source_backed_declaration_witness: WorthUiSourceBackedDeclarationWitnessSlot::Absent,
            query_binding_plan: Default::default(),
        }
    }

    pub fn with_dsl_package(mut self, dsl_package: WorthUiDslPackage) -> Self {
        self.dsl_package = dsl_package;
        self.source_backed_declaration_witness = WorthUiSourceBackedDeclarationWitnessSlot::Absent;
        self
    }

    pub fn with_host<Host>(mut self, host: Host) -> Self
    where
        Host: WorthUiHostAdapter,
    {
        self.host_contract = host.host_contract();
        self
    }

    pub fn with_graph_world_profile(mut self, graph_world_profile: UiGraphWorldProfile) -> Self {
        self.graph_world_profile = graph_world_profile;
        self
    }

    pub fn with_source_backed_dsl_package(
        mut self,
        source_backed_package: WorthUiSourceBackedDslPackage,
    ) -> Self {
        let (dsl_package, source_backed_declaration_witness) = source_backed_package.into_parts();
        self.dsl_package = dsl_package;
        self.source_backed_declaration_witness =
            WorthUiSourceBackedDeclarationWitnessSlot::Present(source_backed_declaration_witness);
        self
    }

    #[cfg(test)]
    pub fn with_measurement_inspection_evidence(
        mut self,
        evidence: UiMeasurementInspectionEvidenceBundle,
    ) -> Self {
        self.measurement_inspection_evidence.push(evidence);
        self
    }

    pub fn register_command(mut self, descriptor: CommandDescriptor) -> Self {
        self.inner = self.inner.register_command(descriptor);
        self
    }

    pub fn register_command_projection(mut self, descriptor: CommandProjectionDescriptor) -> Self {
        self.inner = self.inner.register_command_projection(descriptor);
        self
    }

    pub fn register_component(mut self, descriptor: ComponentDescriptor) -> Self {
        self.inner = self.inner.register_component(descriptor);
        self
    }

    pub fn register_icon(mut self, descriptor: IconDescriptor) -> Self {
        self.inner = self.inner.register_icon(descriptor);
        self
    }

    pub fn register_surface(mut self, descriptor: SurfaceDescriptor) -> Self {
        self.inner = self.inner.register_surface(descriptor);
        self
    }

    pub fn register_mosaic_region_kind(mut self, descriptor: MosaicRegionKindDescriptor) -> Self {
        self.inner = self.inner.register_mosaic_region_kind(descriptor);
        self
    }

    pub fn register_mosaic_placement_policy(
        mut self,
        descriptor: MosaicPlacementPolicyDescriptor,
    ) -> Self {
        self.inner = self.inner.register_mosaic_placement_policy(descriptor);
        self
    }

    pub fn register_mosaic_sizing_contract(
        mut self,
        descriptor: MosaicSizingContractDescriptor,
    ) -> Self {
        self.inner = self.inner.register_mosaic_sizing_contract(descriptor);
        self
    }

    pub fn register_mosaic_state_slot(mut self, descriptor: MosaicStateSlotDescriptor) -> Self {
        self.inner = self.inner.register_mosaic_state_slot(descriptor);
        self
    }

    pub fn register_native_capability(mut self, descriptor: NativeCapabilityDescriptor) -> Self {
        self.inner = self.inner.register_native_capability(descriptor);
        self
    }

    pub fn register_plugin_slot(mut self, descriptor: PluginSlotDescriptor) -> Self {
        self.inner = self.inner.register_plugin_slot(descriptor);
        self
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn register_view_binding(
        mut self,
        descriptor: crate::facade::registry::ViewBindingDescriptor,
    ) -> Self {
        self.inner = self.inner.register_view_binding(descriptor);
        self
    }

    /// Register an installed Query view as one coherent definition and
    /// runtime-affine authority. Query posture cannot be assembled piecemeal.
    pub fn register_query_view(
        mut self,
        registration: impl Into<WorthUiQueryViewRegistration>,
    ) -> Result<Self, WorthUiQueryViewRegistrationError> {
        let (view, visible_state_bindings, denial_presentation) = registration.into().into_parts();
        let definition = view.definition().clone();
        let id = crate::capability::ViewBindingId::new(definition.identity().as_str())
            .map_err(WorthUiQueryViewRegistrationError::InvalidIdentity)?;
        let family = match definition.shape() {
            worth_ui_query_binding::WorthUiQueryViewShape::Collection => {
                crate::capability::ViewBindingFamily::collection()
            }
            worth_ui_query_binding::WorthUiQueryViewShape::Detail => {
                crate::capability::ViewBindingFamily::detail()
            }
        };
        self.query_binding_plan = self
            .query_binding_plan
            .register_view(view)
            .map_err(WorthUiQueryViewRegistrationError::Binding)?;
        let descriptor = visible_state_bindings.into_iter().fold(
            crate::capability::ViewBindingDescriptor::from_definition(id, family, definition)
                .with_denial_presentation(denial_presentation),
            crate::capability::ViewBindingDescriptor::with_visible_state_binding,
        );
        self.inner = self.inner.register_view_binding(descriptor);
        Ok(self)
    }

    pub fn register_runtime_outcome_projection(
        mut self,
        descriptor: RuntimeOutcomeProjectionDescriptor,
    ) -> Self {
        self.inner = self.inner.register_runtime_outcome_projection(descriptor);
        self
    }

    pub fn register_setting(mut self, descriptor: SettingDescriptor) -> Self {
        self.inner = self.inner.register_setting(descriptor);
        self
    }

    pub fn register_task_presentation(mut self, descriptor: TaskPresentationDescriptor) -> Self {
        self.inner = self.inner.register_task_presentation(descriptor);
        self
    }

    pub fn register_theme_token(mut self, descriptor: ThemeTokenDescriptor) -> Self {
        self.inner = self.inner.register_theme_token(descriptor);
        self
    }

    pub fn freeze(self) -> WorthUiApp {
        let capability_snapshot = self
            .inner
            .freeze_with_registration_report()
            .into_accepted_snapshot();
        WorthUiApp::from_freeze_core_with_query_binding(
            WorthUiCapabilityRegistrationFreezeCore::freeze_from_registration(
                capability_snapshot,
                self.dsl_package,
                self.host_contract,
                self.graph_world_profile,
                self.measurement_inspection_evidence.into_boxed_slice(),
                self.source_backed_declaration_witness.into_option(),
            ),
            self.query_binding_plan,
        )
    }

    pub fn freeze_with_registration_report(self) -> CapabilityRegistrationReport {
        self.inner.freeze_with_registration_report()
    }

    pub fn with_minimal_registration_diagnostics(mut self) -> Self {
        self.inner = self.inner.with_minimal_registration_diagnostics();
        self
    }

    pub fn with_rich_registration_diagnostics(mut self) -> Self {
        self.inner = self.inner.with_rich_registration_diagnostics();
        self
    }
}

enum WorthUiSourceBackedDeclarationWitnessSlot {
    Absent,
    Present(WorthUiSourceBackedDeclarationWitness),
}

impl WorthUiSourceBackedDeclarationWitnessSlot {
    fn into_option(self) -> Option<WorthUiSourceBackedDeclarationWitness> {
        match self {
            Self::Absent => None,
            Self::Present(witness) => Some(witness),
        }
    }
}
