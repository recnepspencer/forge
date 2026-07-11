use forge_store_certification::certify_s8_layout_scenario;
use forge_store_physical_certification::layout_harness::scenario::{
    layout_scenario, S8LayoutScenarioKind,
};

fn main() {
    let definition = layout_scenario(S8LayoutScenarioKind::ExactCounter);
    let _ = certify_s8_layout_scenario(definition);
}
