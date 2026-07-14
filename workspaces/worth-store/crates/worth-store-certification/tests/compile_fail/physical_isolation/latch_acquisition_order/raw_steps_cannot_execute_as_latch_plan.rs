use worth_store_physical_isolation::{LatchAcquisitionPlan, LatchAcquisitionStep};

fn execute(_: LatchAcquisitionPlan) {}

fn main() {
    let steps: Vec<LatchAcquisitionStep> = Vec::new();
    execute(steps);
}
