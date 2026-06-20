use forge_query::facade::runtime::{
    ForgeQueryGraphReadStreamingPageBudget, ForgeQueryGraphReadStreamingPlan,
};

fn main() {
    let _ = ForgeQueryGraphReadStreamingPlan {
        digest: String::new(),
        admission_digest: String::new(),
        requirement_set_digest: String::new(),
        page_budget: ForgeQueryGraphReadStreamingPageBudget {
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
