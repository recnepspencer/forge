use forge_store_physical_isolation::{StablePhysicalReadHandle, StablePhysicalReadPlan};

fn main() {
    let plan: StablePhysicalReadPlan = todo!();
    let _handle = StablePhysicalReadHandle {
        plan,
        released: false,
    };
}
