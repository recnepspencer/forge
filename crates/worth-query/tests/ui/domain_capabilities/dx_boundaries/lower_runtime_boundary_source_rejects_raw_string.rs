use worth_query::facade::domain::WorthQuerySupportContributionAuthoring;

fn main() {
    let _ = WorthQuerySupportContributionAuthoring::narrowed_support("routing", "detail")
        .for_lower_runtime_boundary_source("not-an-envelope-source");
}
