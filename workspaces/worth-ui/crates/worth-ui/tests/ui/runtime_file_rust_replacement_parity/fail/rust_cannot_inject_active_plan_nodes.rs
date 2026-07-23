use worth_ui::facade::WorthUiExecutionPlan;

fn main() {
    let _plan = WorthUiExecutionPlan {
        lowering_identity: uninitialized_field(),
        handle_receipt: uninitialized_field(),
        flat_projection: None,
        region_store: uninitialized_field(),
        construction_counters: uninitialized_field(),
        regional_evidence: uninitialized_field(),
        counters: uninitialized_field(),
    };
}

fn uninitialized_field<T>() -> T {
    unimplemented!()
}
