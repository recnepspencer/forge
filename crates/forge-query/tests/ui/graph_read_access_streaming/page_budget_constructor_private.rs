use forge_query::facade::runtime::ForgeQueryGraphReadStreamingPageBudget;

fn main() {
    let _ = ForgeQueryGraphReadStreamingPageBudget {
        digest: String::new(),
        max_page_width: 1,
        max_resident_frontier: 1,
        max_resident_visited: 1,
        max_page_result_bytes: 1,
    };
}
