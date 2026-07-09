use worth_query::facade::runtime::{
    WorthQueryGraphReadStreamingPageBudget, WorthQueryGraphReadStreamingPlan,
};

fn main() {
    let _ = WorthQueryGraphReadStreamingPlan {
        digest: String::new(),
        admission_digest: String::new(),
        requirement_set_digest: String::new(),
        page_budget: WorthQueryGraphReadStreamingPageBudget {
            digest: String::new(),
            max_page_width: 1,
            max_resident_frontier: 1,
            max_resident_visited: 1,
            max_page_result_bytes: 1,
        },
        canonical_result_basis_digest: String::new(),
        replay_basis_digest: String::new(),
    };
}
