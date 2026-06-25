use topology::derived_invalidation_operator_cutover::{
    DerivedInvalidationProjectionReadStageReceipt, ProjectionReadStageConsumptionScope,
};

fn main() {
    let _ = DerivedInvalidationProjectionReadStageReceipt {
        operator_cutover_receipt_digest: String::new(),
        execution_receipt_digest: String::new(),
        selected_plan_digest: String::new(),
        touched_closure_digest: String::new(),
        consumption_scope: ProjectionReadStageConsumptionScope::CommittedRead,
        projection_dirty_expansion_count: 0,
        receipt_digest: String::new(),
    };
}
