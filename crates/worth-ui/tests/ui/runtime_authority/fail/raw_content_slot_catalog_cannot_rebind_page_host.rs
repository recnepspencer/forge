use worth_ui::facade::{
    WorthUiContentSlotCatalog, WorthUiPageHostPlan, WorthUiRuntimeHost, WorthUiValidationReloadEvidence,
};

fn main() {
    let runtime: WorthUiRuntimeHost = todo!();
    let current_plan: WorthUiPageHostPlan = todo!();
    let raw_slots: WorthUiContentSlotCatalog = todo!();
    let evidence: WorthUiValidationReloadEvidence = todo!();

    let _ = runtime.rebind_page_host_after_reload(&current_plan, raw_slots, &evidence);
}
