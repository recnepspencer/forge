use topology::derived_invalidation_operator_cutover::DerivedInvalidationPhaseEightSeed;

fn main() {
    let _ = DerivedInvalidationPhaseEightSeed {
        operator_cutover_receipt_digest: String::new(),
        projection_read_stage_receipt_digest: String::new(),
        selected_plan_digest: String::new(),
        execution_receipt_digest: String::new(),
        touched_closure_digest: String::new(),
        query_support_digest: String::new(),
        legality_support_digest: String::new(),
        counters_digest: String::new(),
        seed_digest: String::new(),
    };
}
