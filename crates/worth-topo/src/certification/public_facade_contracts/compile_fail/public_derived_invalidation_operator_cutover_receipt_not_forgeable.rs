use topology::derived_invalidation_operator_cutover::{
    DerivedInvalidationOperatorCutoverCounters, DerivedInvalidationOperatorCutoverReceipt,
};

fn main() {
    let _ = DerivedInvalidationOperatorCutoverReceipt {
        phase_seven_seed_digest: String::new(),
        operator_touched_basis_digest: String::new(),
        selected_plan_digest: String::new(),
        execution_receipt_digest: String::new(),
        touched_closure_digest: String::new(),
        query_support_digest: String::new(),
        legality_support_digest: String::new(),
        graph_obligation_envelope_digest: String::new(),
        graph_obligation_dispatch_digest: Some(String::new()),
        counters: fake_counters(),
        receipt_digest: String::new(),
    };
}

fn fake_counters() -> DerivedInvalidationOperatorCutoverCounters {
    panic!("compile-fail fixture does not execute")
}
