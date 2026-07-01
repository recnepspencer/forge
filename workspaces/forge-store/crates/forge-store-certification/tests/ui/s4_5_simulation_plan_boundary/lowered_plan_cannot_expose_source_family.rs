use forge_store_physical_certification::PhysicalSimulationPlan;

fn main() {
    let plan: PhysicalSimulationPlan = panic!("type-check only");
    let _ = plan.family();
}
