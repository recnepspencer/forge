use worth_spatial::facade::planar_retained_facts::RetainedPlanarFactsReceipt;

fn main() {
    let _receipt = RetainedPlanarFactsReceipt {
        basis: fake(),
        declaration_digest: String::new(),
        progression_digest: String::new(),
        route_plan_digest: String::new(),
        query_receipt_digest: String::new(),
        envelope_digest: String::new(),
        retained_fact_digest: String::new(),
        counters: fake(),
    };
}

fn fake<T>() -> T {
    unimplemented!()
}
