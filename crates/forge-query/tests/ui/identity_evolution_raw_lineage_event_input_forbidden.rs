use forge_query::facade::IdentityEvolutionQueryContext;

fn main() {
    let _: fn(String) -> IdentityEvolutionQueryContext =
        IdentityEvolutionQueryContext::from_raw_lineage_event;
}
