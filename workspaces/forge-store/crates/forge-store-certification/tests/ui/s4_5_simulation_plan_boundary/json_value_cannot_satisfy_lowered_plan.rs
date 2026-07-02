use forge_store_physical_certification::{
    require_lowered_physical_simulation_plan, PhysicalSimulationPlan,
};

fn requires_plan(plan: &PhysicalSimulationPlan) {
    let _ = require_lowered_physical_simulation_plan(plan);
}

fn main() {
    let json = serde_json::json!({"driver": "production-boundary-yieldpoint"});
    requires_plan(&json);
}
