use worth_query::facade::runtime::WorthQueryGraphReadStreamingPageBudget;

fn main() {
    let _ = WorthQueryGraphReadStreamingPageBudget {
        digest: String::new(),
        max_page_width: 1,
        max_resident_frontier: 1,
        max_resident_visited: 1,
        max_page_result_bytes: 1,
    };
}
