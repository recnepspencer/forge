use forge_store_certification::certify_layout_index_layout_scenario;
use forge_store_physical_certification::layout_harness::scenario::{
    layout_scenario, S8LayoutScenarioKind,
};

fn main() {
    let definition = layout_scenario(S8LayoutScenarioKind::ExactCounter);
    let _ = certify_layout_index_layout_scenario(definition);
}
