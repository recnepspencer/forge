use forge_store_physical_isolation::{StablePhysicalReadExecution, StablePhysicalReadPlan};

fn main() {
    let plan: StablePhysicalReadPlan = todo!();
    let _execution = StablePhysicalReadExecution::from_plan(plan);
}
