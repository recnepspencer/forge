use worth_store_physical_certification::{
    require_lowered_physical_simulation_plan, CertifiedPhysicalScenario,
};

fn main() {
    let scenario: CertifiedPhysicalScenario = panic!("type-check only");
    let _ = require_lowered_physical_simulation_plan(&scenario);
}
