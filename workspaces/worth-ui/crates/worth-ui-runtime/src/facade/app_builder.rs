use crate::facade::runtime_bridge::WorthUiCapabilityRegistrationFreezeCore;
use crate::facade::{
    CapabilityRegistrationBuilder, CapabilityRegistrationReport, CommandDescriptor,
    CommandProjectionDescriptor, ComponentDescriptor, IconDescriptor,
    MosaicPlacementPolicyDescriptor, MosaicRegionKindDescriptor, MosaicSizingContractDescriptor,
    MosaicStateSlotDescriptor, NativeCapabilityDescriptor, PluginSlotDescriptor,
    RuntimeOutcomeProjectionDescriptor, SettingDescriptor, SurfaceDescriptor,
    TaskPresentationDescriptor, ThemeTokenDescriptor, ViewBindingDescriptor, WorthUiApp,
    WorthUiDslPackage, WorthUiHostAdapter, WorthUiHostContract,
};
use crate::graph::UiGraphWorldProfile;
use worth_ui_inspection::UiInspectionScopeInventory;

/// Builder for a Worth UI application definition.
pub struct WorthUiBuilder {
    inner: CapabilityRegistrationBuilder,
    dsl_package: WorthUiDslPackage,
    host_contract: WorthUiHostContract,
    graph_world_profile: UiGraphWorldProfile,
}

pub type WorthUiAppBuilder = WorthUiBuilder;

impl WorthUiBuilder {
    pub(crate) fn new() -> Self {
        Self {
            inner: CapabilityRegistrationBuilder::new(),
            dsl_package: WorthUiDslPackage::empty(),
            host_contract: WorthUiHostContract::headless(),
            graph_world_profile: UiGraphWorldProfile::authoritative(),
        }
    }

    pub fn with_dsl_package(mut self, dsl_package: WorthUiDslPackage) -> Self {
        self.dsl_package = dsl_package;
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

    pub fn register_view_binding(mut self, descriptor: ViewBindingDescriptor) -> Self {
        self.inner = self.inner.register_view_binding(descriptor);
        self
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
        WorthUiApp::from_freeze_core(WorthUiCapabilityRegistrationFreezeCore::new(
            capability_snapshot,
            self.dsl_package,
            self.host_contract,
            self.graph_world_profile,
        ))
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

    pub(crate) fn freeze_with_inspection_scope_inventory(
        self,
        inspection_scope_inventory: UiInspectionScopeInventory,
    ) -> WorthUiApp {
        let capability_snapshot = self
            .inner
            .freeze_with_registration_report()
            .into_accepted_snapshot();
        WorthUiApp::from_freeze_core(
            WorthUiCapabilityRegistrationFreezeCore::new_with_inspection_scope_inventory(
                capability_snapshot,
                self.dsl_package,
                self.host_contract,
                self.graph_world_profile,
                inspection_scope_inventory,
            ),
        )
    }
}
