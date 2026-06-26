use topology::derived_invalidation_milestone_ten_closeout::{
    DerivedInvalidationMilestoneElevenSeed, DerivedInvalidationMilestoneTenCloseout,
    DerivedInvalidationMilestoneTenCounters, DerivedInvalidationMilestoneTenPerformanceProof,
    DerivedInvalidationMilestoneTenProductSummaryReport,
};

fn main() {
    let _ = DerivedInvalidationMilestoneTenCloseout {
        catalog_digest: String::new(),
        selected_plan_digest: String::new(),
        execution_receipt_digest: String::new(),
        migration_sweep_digest: String::new(),
        operator_cutover_digest: String::new(),
        deletion_closeout_digest: String::new(),
        product_summary: fake_product_summary(),
        performance_proof: fake_performance_proof(),
        counters: fake_counters(),
        milestone_eleven_seed: fake_milestone_eleven_seed(),
        closeout_digest: String::new(),
    };
}

fn fake_product_summary() -> DerivedInvalidationMilestoneTenProductSummaryReport {
    panic!("compile-fail fixture does not execute")
}

fn fake_performance_proof() -> DerivedInvalidationMilestoneTenPerformanceProof {
    panic!("compile-fail fixture does not execute")
}

fn fake_counters() -> DerivedInvalidationMilestoneTenCounters {
    panic!("compile-fail fixture does not execute")
}

fn fake_milestone_eleven_seed() -> DerivedInvalidationMilestoneElevenSeed {
    panic!("compile-fail fixture does not execute")
}
