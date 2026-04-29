use forge_query::facade::ForgeQueryAspectMutationBuilder;
use std::collections::BTreeSet;

fn main() {
    let _builder = ForgeQueryAspectMutationBuilder {
        aspects: Vec::new(),
        seen_aspects: BTreeSet::new(),
        error: None,
    };
}
