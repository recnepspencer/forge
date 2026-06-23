use worth_ui::facade::{
    CommandProjectionSelectionMode, WorthUiCapabilityReloadEvidence, WorthUiDropdownProjectionPlan,
    WorthUiRuntimeHost,
};

fn main() {
    let mut runtime: WorthUiRuntimeHost = todo!();
    let current_plan: WorthUiDropdownProjectionPlan = todo!();
    let evidence: WorthUiCapabilityReloadEvidence = todo!();
    let mode = CommandProjectionSelectionMode::SingleSelect;
    let _ = runtime.rebind_dropdown_projection_after_capability_reload(&current_plan, mode, &evidence);
}
