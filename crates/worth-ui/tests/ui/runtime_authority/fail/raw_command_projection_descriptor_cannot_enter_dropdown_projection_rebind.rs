use worth_ui::facade::{
    CommandProjectionDescriptor, CommandProjectionId, CommandProjectionSurface,
    WorthUiCapabilityReloadEvidence, WorthUiDropdownProjectionPlan, WorthUiRuntimeHost,
};

fn main() {
    let mut runtime: WorthUiRuntimeHost = todo!();
    let current_plan: WorthUiDropdownProjectionPlan = todo!();
    let evidence: WorthUiCapabilityReloadEvidence = todo!();
    let descriptor = CommandProjectionDescriptor::new(
        CommandProjectionId::new("workspace.header.file").unwrap(),
        CommandProjectionSurface::menu_bar(),
    );
    let _ =
        runtime.rebind_dropdown_projection_after_capability_reload(&current_plan, descriptor, &evidence);
}
