use topology::derived_invalidation_selected_plan::DerivedInvalidationPhaseFourSeed;

fn main() {
    let _ = DerivedInvalidationPhaseFourSeed {
        selected_plan_digest: String::new(),
        touched_closure_digest: String::new(),
        query_support_digest: String::new(),
        legality_support_digest: String::new(),
        selected_product_count: 0,
        denied_product_count: 0,
        unaffected_product_count: 0,
        seed_digest: String::new(),
    };
}
