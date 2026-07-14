use worth_store_layout_indexes::{
    access_execution, S8ExecutedAccessEvidence, S8ExecutionReadyAccessPlan, S8LoweredAccessPlan,
};

fn main() {
    let _ = access_execution();
    let _ = core::mem::size_of::<S8ExecutedAccessEvidence>();
    let _ = core::mem::size_of::<S8ExecutionReadyAccessPlan>();
    let _ = core::mem::size_of::<S8LoweredAccessPlan>();
}
