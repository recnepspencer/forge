use topology::derived_invalidation_execution::{
    DerivedInvalidationDeniedProductExecutionRow, DerivedInvalidationExecutedProductRow,
    DerivedInvalidationExecutionCounters, DerivedInvalidationExecutionReceipt,
    DerivedInvalidationResidueExecutionRow, DerivedInvalidationUnaffectedProductExecutionRow,
};

fn main() {
    let _ = DerivedInvalidationExecutionReceipt {
        phase_four_seed_digest: String::new(),
        selected_plan_digest: String::new(),
        touched_closure_digest: String::new(),
        query_support_digest: String::new(),
        legality_support_digest: String::new(),
        executed_rows: Vec::<DerivedInvalidationExecutedProductRow>::new(),
        unaffected_rows: Vec::<DerivedInvalidationUnaffectedProductExecutionRow>::new(),
        denied_rows: Vec::<DerivedInvalidationDeniedProductExecutionRow>::new(),
        residue_rows: Vec::<DerivedInvalidationResidueExecutionRow>::new(),
        counters: fake_counters(),
        execution_receipt_digest: String::new(),
    };
}

fn fake_counters() -> DerivedInvalidationExecutionCounters {
    panic!("compile-fail fixture does not execute")
}
