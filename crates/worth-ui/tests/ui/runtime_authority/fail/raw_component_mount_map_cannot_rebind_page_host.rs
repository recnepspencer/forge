use std::collections::BTreeMap;

use worth_ui::facade::{
    ComponentId, WorthUiCapabilityReloadEvidence, WorthUiPageHostPlan, WorthUiRuntimeHost,
};

fn main() {
    let runtime: WorthUiRuntimeHost = todo!();
    let current_plan: WorthUiPageHostPlan = todo!();
    let raw_component_mounts: BTreeMap<String, ComponentId> = BTreeMap::new();
    let evidence: WorthUiCapabilityReloadEvidence = todo!();

    let _ = runtime.rebind_page_host_after_capability_reload(
        &current_plan,
        raw_component_mounts,
        &evidence,
    );
}
