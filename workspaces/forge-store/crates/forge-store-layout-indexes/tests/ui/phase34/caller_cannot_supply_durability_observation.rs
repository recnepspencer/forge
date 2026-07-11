use forge_store_physical_backend::{
    PhysicalStoreDurabilityExecutor, StoreDurabilityExecutionObservation,
    StoreDurabilityExecutionRequest,
};

fn main() {
    let _ = core::mem::size_of::<StoreDurabilityExecutionObservation>();
}
