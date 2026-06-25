use topology::derived_invalidation_selected_plan::{
    DerivedInvalidationDensityPolicy, DerivedInvalidationExecutionAdmission,
    DerivedInvalidationPhaseFourSeed, DerivedInvalidationSelectedPlan,
    DerivedInvalidationSelectionCounters,
};

fn main() {
    let _ = DerivedInvalidationSelectedPlan {
        phase_three_seed_digest: String::new(),
        catalog_digest: String::new(),
        touched_closure_digest: String::new(),
        query_support_digest: String::new(),
        legality_support_digest: String::new(),
        density_policy: DerivedInvalidationDensityPolicy::Sparse,
        selected_rows: Vec::new(),
        unaffected_rows: Vec::new(),
        denied_rows: Vec::new(),
        residue_rows: Vec::new(),
        counters: fake_counters(),
        execution_admission: DerivedInvalidationExecutionAdmission::Admitted,
        phase_four_seed: fake_phase_four_seed(),
        selected_plan_digest: String::new(),
    };
}

fn fake_counters() -> DerivedInvalidationSelectionCounters {
    panic!("compile-fail fixture does not execute")
}

fn fake_phase_four_seed() -> DerivedInvalidationPhaseFourSeed {
    panic!("compile-fail fixture does not execute")
}
