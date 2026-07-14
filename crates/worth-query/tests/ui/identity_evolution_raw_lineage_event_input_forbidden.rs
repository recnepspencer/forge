use worth_query::facade::foundation::IdentityEvolutionQueryContext;

fn main() {
    let _: fn(String) -> IdentityEvolutionQueryContext =
        IdentityEvolutionQueryContext::from_raw_lineage_event;
}
