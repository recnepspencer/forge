use forge_query::facade::runtime::ForgeQuerySupportContributionAuthoring;

fn main() {
    let _ = ForgeQuerySupportContributionAuthoring::narrowed_support("routing", "detail")
        .for_lower_runtime_boundary_source("not-an-envelope-source");
}
