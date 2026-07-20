use worth_ui::facade::{
    registry::{CommandId, CommandProjectionId, ComponentId, IconId, MosaicPlacementPolicyId, MosaicRegionKindId, MosaicSizingContractId, MosaicStateSlotId, NativeCapabilityId, PluginSlotId, RuntimeOutcomeProjectionId, SettingId, SurfaceId, TaskPresentationId, ThemeTokenId, ViewBindingId},
};

fn main() {
    let _ = CommandId::new("app.command.save").expect("valid command id");
    let _ = ComponentId::new("app.component.editor").expect("valid component id");
    let _ = SurfaceId::new("app.surface.main").expect("valid surface id");
    let _ =
        MosaicRegionKindId::new("platform.mosaic_region.primary").expect("valid region id");
    let _ = MosaicPlacementPolicyId::new("platform.mosaic_placement.docked")
        .expect("valid placement id");
    let _ = MosaicSizingContractId::new("platform.mosaic_sizing.flex").expect("valid sizing id");
    let _ = MosaicStateSlotId::new("app.mosaic_state.editor_tabs").expect("valid state id");
    let _ = ViewBindingId::new("app.view_binding.tasks").expect("valid binding id");
    let _ = RuntimeOutcomeProjectionId::new("app.runtime_outcome.build")
        .expect("valid outcome id");
    let _ = SettingId::new("app.setting.theme").expect("valid setting id");
    let _ =
        TaskPresentationId::new("app.task_presentation.default").expect("valid task id");
    let _ = ThemeTokenId::new("app.theme_token.accent").expect("valid token id");
    let _ = IconId::new("app.icon.save").expect("valid icon id");
    let _ = CommandProjectionId::new("app.command_projection.toolbar")
        .expect("valid projection id");
    let _ = PluginSlotId::new("app.plugin_slot.theme").expect("valid plugin slot id");
    let _ = NativeCapabilityId::new("platform.native.clipboard").expect("valid native id");
}
